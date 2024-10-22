use std::{borrow::Cow, mem, sync::Arc};

use bytes::{Buf, Bytes, BytesMut};
use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{FieldInfoData, LockedTypeObject, TypeInfo, TypeInfoData, TypeObject},
    type_registry::TypeRegistry,
};
use glacier_util::{
    endian::endian_swap,
    guid::{BytesGuidExt, Guid},
    math::roundup,
};
use tokio::sync::Mutex;
use tracing::{error, warn};

use crate::db::partition::{DatabasePartition, PartitionInitData};

use super::type_resolver::{
    EbxPartitionFieldDescriptor, EbxPartitionTypeDescriptor, EbxPartitionTypeResolver,
};

const SUPER_CLASS_FIELD_HASH: u32 = 177537;

#[derive(Debug, Default)]
#[repr(C)]
#[repr(align(16))]
struct EbxPartitionHeader {
    magic: u32,
    meta_size: u32,
    payload_size: u32,
    import_count: u32,
    range_count: u16,
    exported_range_count: u16,
    type_count: u16,
    type_descriptor_count: u16,
    field_descriptor_count: u16,
    type_string_table_size: u16,
    string_table_size: u32,
    array_count: u32,
    array_offset: u32,
    partition_guid: Guid,
    boxed_value_count: u32,
    boxed_value_offset: u32,
}

impl EbxPartitionHeader {
    pub const MAGIC: u32 = 0xfb4d1ce;
}

#[derive(Default)]
enum ReaderState {
    #[default]
    Initial,
    Metadata,
    Prepare,
    Payloads,
    PreparePayload,
    Layout,
    Fixup,
    Done,
}

#[derive(Default, Clone)]
#[repr(C)]
struct EbxPartitionImportEntry {
    partition_guid: Guid,
    instance_guid: Guid,
}

#[derive(Default, Clone)]
#[repr(C)]
struct EbxPartitionInstanceRange {
    type_descriptor_index: u16,
    instance_count: u16,
}

#[derive(Default, Clone, Debug)]
#[repr(C)]
struct EbxPartitionArrayEntry {
    offset: u32,
    element_count: u32,
    type_descriptor_index: u32,
}

#[derive(Default, Clone)]
#[repr(C)]
struct EbxPartitionBoxedValueEntry {
    offset: u32,
    type_id: u16,
    type_code: u16,
}

pub struct EbxPartitionReader<'a> {
    partition_name: String,
    type_registry: &'a TypeRegistry,

    header: EbxPartitionHeader,
    state: ReaderState,
    endian_swap: bool,

    init_data: PartitionInitData,
    type_resolver: EbxPartitionTypeResolver,

    metadata_outstanding: u32,
    payload_outstanding: u32,

    metadata: BytesMut,

    string_table: Bytes,

    payload_start_ptr: usize, // BytesMut internal pointer
    payload_start: BytesMut,
    payload: BytesMut,

    import_entries: Vec<EbxPartitionImportEntry>,
    meta_string_block: Vec<String>,

    instance_ranges: Vec<EbxPartitionInstanceRange>,
    array_entries: Vec<EbxPartitionArrayEntry>,
    boxed_value_entries: Vec<EbxPartitionBoxedValueEntry>,

    type_infos: Vec<Option<&'static TypeInfo>>,
    containers: Vec<LockedTypeObject>,
}

#[repr(C)]
struct RawTypeObject {
    data_ptr: *mut u8,
    vtable_ptr: *mut (),
}

unsafe impl Send for RawTypeObject {}

fn data_ptr_from_trait_object(data: &mut dyn TypeObject) -> RawTypeObject {
    let ptr = data as *mut dyn TypeObject;
    let (data_ptr, _vtable_ptr): (*mut u8, *mut ()) = unsafe { std::mem::transmute(ptr) };
    RawTypeObject {
        data_ptr,
        vtable_ptr: _vtable_ptr,
    }
}

impl<'a> EbxPartitionReader<'a> {
    pub fn new(partition_name: String, type_registry: &'a TypeRegistry) -> Self {
        Self {
            partition_name,
            type_registry,
            header: EbxPartitionHeader::default(),
            state: ReaderState::Initial,
            endian_swap: false,
            init_data: PartitionInitData::default(),
            type_resolver: EbxPartitionTypeResolver::default(),
            metadata_outstanding: 0,
            payload_outstanding: 0,
            metadata: BytesMut::new(),
            string_table: Bytes::new(),
            payload_start_ptr: 0,
            payload_start: BytesMut::new(),
            payload: BytesMut::new(),
            import_entries: Vec::new(),
            meta_string_block: Vec::new(),
            instance_ranges: Vec::new(),
            array_entries: Vec::new(),
            boxed_value_entries: Vec::new(),
            type_infos: Vec::new(),
            containers: Vec::new(),
        }
    }

    pub async fn read(&mut self, data: &mut BytesMut) {
        let start_of_data = data.clone();

        let in_byte_count = data.len();

        let mut complete = false;

        while data.remaining() > 0 || in_byte_count == 0 {
            let byte_count = data.len() as u32;
            match self.state {
                ReaderState::Initial => {
                    assert!(byte_count >= mem::size_of::<EbxPartitionHeader>() as u32);

                    let mut header_data = [0u8; mem::size_of::<EbxPartitionHeader>()];
                    data.copy_to_slice(&mut header_data);

                    self.header = unsafe { mem::transmute(header_data) };
                    println!("{:?}", self.header);

                    if self.header.magic == EbxPartitionHeader::MAGIC {
                        self.endian_swap = false;
                    } else if self.header.magic == endian_swap(EbxPartitionHeader::MAGIC) {
                        self.endian_swap = true;
                        panic!("Endian swap is not implemented yet");
                    } else {
                        panic!(
                            "Invalid EBX magic {} while reading '{}'",
                            self.header.magic, self.partition_name
                        );
                    }

                    if (self.header.payload_size + self.header.meta_size) == byte_count {
                        complete = true;
                    } else {
                        panic!(
                            "Invalid EBX partition size while reading '{}'",
                            self.partition_name
                        );
                    }

                    self.metadata_outstanding =
                        self.header.meta_size - mem::size_of::<EbxPartitionHeader>() as u32;
                    self.payload_outstanding = self.header.payload_size;

                    self.init_data.guid = self.header.partition_guid;
                    self.init_data.instances = Vec::new();

                    self.state = ReaderState::Metadata;
                }
                ReaderState::Metadata => {
                    let bytes_to_copy = byte_count.min(self.metadata_outstanding);

                    if complete {
                        self.metadata = data.clone();
                        self.state = ReaderState::Prepare;
                    } else {
                        panic!(
                            "Invalid EBX partition size while reading '{}'",
                            self.partition_name
                        );
                    }

                    data.advance(bytes_to_copy as usize);
                }
                ReaderState::Prepare => {
                    let mut metadata_cursor = self.metadata.clone();

                    self.import_entries = unsafe {
                        let mut values = vec![
                            EbxPartitionImportEntry::default();
                            self.header.import_count as usize
                        ];
                        let raw = values.as_mut_ptr() as *mut u8;
                        let len = values.len() * mem::size_of::<EbxPartitionImportEntry>();
                        let slice = std::slice::from_raw_parts_mut(raw, len);
                        metadata_cursor.copy_to_slice(slice);
                        values
                    };

                    // TODO: handle type descriptors
                    metadata_cursor.advance(self.header.type_string_table_size as usize);

                    let field_descriptors = unsafe {
                        let mut values = vec![
                            EbxPartitionFieldDescriptor::default();
                            self.header.field_descriptor_count as usize
                        ];
                        let raw = values.as_mut_ptr() as *mut u8;
                        let len = values.len() * mem::size_of::<EbxPartitionFieldDescriptor>();
                        let slice = std::slice::from_raw_parts_mut(raw, len);
                        slice.clone_from_slice(&metadata_cursor[..len]);
                        metadata_cursor.advance(roundup(len, 16));
                        values
                    };

                    let type_descriptors = unsafe {
                        let mut values = vec![
                            EbxPartitionTypeDescriptor::default();
                            self.header.type_descriptor_count as usize
                        ];
                        let raw = values.as_mut_ptr() as *mut u8;
                        let len = values.len() * mem::size_of::<EbxPartitionTypeDescriptor>();
                        let slice = std::slice::from_raw_parts_mut(raw, len);
                        metadata_cursor.copy_to_slice(slice);
                        values
                    };

                    self.instance_ranges = unsafe {
                        let mut values = vec![
                            EbxPartitionInstanceRange::default();
                            self.header.range_count as usize
                        ];
                        let raw = values.as_mut_ptr() as *mut u8;
                        let len = values.len() * mem::size_of::<EbxPartitionInstanceRange>();
                        let slice = std::slice::from_raw_parts_mut(raw, len);
                        slice.clone_from_slice(&metadata_cursor[..len]);
                        metadata_cursor.advance(roundup(len, 16));
                        values
                    };

                    self.array_entries = unsafe {
                        let mut values = vec![
                            EbxPartitionArrayEntry::default();
                            self.header.array_count as usize
                        ];
                        let raw = values.as_mut_ptr() as *mut u8;
                        let len = values.len() * mem::size_of::<EbxPartitionArrayEntry>();
                        let slice = std::slice::from_raw_parts_mut(raw, len);
                        slice.clone_from_slice(&metadata_cursor[..len]);
                        metadata_cursor.advance(roundup(len, 16));
                        values
                    };

                    self.boxed_value_entries = unsafe {
                        let mut values = vec![
                            EbxPartitionBoxedValueEntry::default();
                            self.header.boxed_value_count as usize
                        ];
                        let raw = values.as_mut_ptr() as *mut u8;
                        let len = values.len() * mem::size_of::<EbxPartitionBoxedValueEntry>();
                        let slice = std::slice::from_raw_parts_mut(raw, len);
                        slice.clone_from_slice(&metadata_cursor[..len]);
                        metadata_cursor.advance(roundup(len, 16));
                        values
                    };

                    self.type_resolver.init(
                        field_descriptors,
                        type_descriptors,
                        metadata_cursor.to_vec(),
                    );

                    // TODO: Resolve imports

                    self.state = ReaderState::Layout;
                }
                ReaderState::Layout => {
                    self.handle_layout().await;
                    self.state = ReaderState::Payloads;
                }
                ReaderState::Payloads => {
                    self.payload = start_of_data.clone();

                    let mut string_table = self.payload.clone();
                    string_table.advance(self.header.meta_size as usize);
                    self.string_table = string_table.freeze();

                    self.payload.advance(
                        self.header.meta_size as usize + self.header.string_table_size as usize,
                    );

                    self.payload_start_ptr = self.payload.as_ptr() as usize;
                    self.payload_start = self.payload.clone();

                    self.state = ReaderState::Fixup;
                }
                ReaderState::PreparePayload => todo!(),
                ReaderState::Fixup => {
                    self.handle_fixup().await;
                    self.state = ReaderState::Done;
                }
                ReaderState::Done => {
                    self.init_data.instances = self.containers.clone();
                    break;
                }
            }
        }
    }

    fn finalize(self) -> DatabasePartition {
        DatabasePartition::new(self.init_data)
    }

    async fn handle_layout(&mut self) {
        self.type_infos = vec![None; 64];

        let mut instance_count = 0;

        let mut i = 0;
        for range in &self.instance_ranges {
            // let td = self
            //     .type_resolver
            //     .resolve_type(range.type_descriptor_index as u32);
            // let type_info = td.type_info(self.type_registry);
            println!("{}", range.type_descriptor_index);

            let td = self
                .type_resolver
                .type_by_index(range.type_descriptor_index as u32);
            println!("t: {}", td.type_name_hash);

            let type_info = self.type_registry.type_by_hash(td.type_name_hash);

            self.type_infos[range.type_descriptor_index as usize] = type_info;

            let type_info = type_info.unwrap();
            let creation_func = if let TypeInfoData::Class(class_info) = &type_info.data {
                class_info.functions.create
            } else {
                panic!("Invalid type info");
            };

            println!("name: {}", type_info.name);

            let is_exported_range = i < self.header.exported_range_count;

            for _ in 0..range.instance_count {
                let container = (creation_func)();

                {
                    let mut container = container.lock().await;
                    let dc_core = container
                        .data_container_core_mut()
                        .expect("Invalid container");
                    dc_core.exported = is_exported_range;
                    dc_core.partition_guid = self.init_data.guid;
                }

                self.containers.push(container);
            }

            instance_count += range.instance_count as usize;

            i += 1;
        }

        println!("Instance count: {}", instance_count);
    }

    async fn handle_fixup(&mut self) {
        let mut i = 0;
        for range in self.instance_ranges.clone() {
            let td = self
                .type_resolver
                .resolve_type(range.type_descriptor_index as u32)
                .clone();

            let type_info = self.type_infos[range.type_descriptor_index as usize].unwrap();

            println!(
                "Parsing range {} with {} bytes",
                type_info.name,
                self.payload.len()
            );

            for _ in 0..range.instance_count {
                self.align_payload(td.alignment as usize);

                let container = self.containers[i].clone();

                let mut container = container.lock().await;
                let dc_core = container
                    .data_container_core_mut()
                    .expect("Invalid container");
                dc_core.instance_guid = if dc_core.exported {
                    self.payload.get_guid()
                } else {
                    Guid::random()
                };

                println!("Instance GUID: {:?}", dc_core.instance_guid);

                if td.alignment != 4 {
                    self.payload.advance(8);
                }

                Self::marshal_fields(self, td.clone(), Some(&mut *container), 0).await;

                i += 1;
            }
        }
    }

    #[async_recursion::async_recursion]
    async fn marshal_fields(
        inst: &mut EbxPartitionReader,
        type_desc: EbxPartitionTypeDescriptor,
        container: Option<&mut dyn TypeObject>,
        depth: u32,
    ) {
        if depth > 100 {
            println!("Depth limit reached");
            return;
        }

        let container = match container {
            Some(container) => container,
            None => {
                // TODO: Alignment
                panic!("Container is None");
            }
        };

        let type_info = type_desc
            .type_info(inst.type_registry)
            .expect("Type not found");

        println!(
            "Marshalling {}: {} {:?}",
            type_info.name, type_desc.field_count, type_desc
        );

        let data_ptr = data_ptr_from_trait_object(container);

        for i in 0..type_desc.field_count as u32 {
            let field_type = inst
                .type_resolver
                .field_by_index(type_desc.layout_descriptor + i)
                .clone();

            let is_superclass = field_type.field_name_hash == SUPER_CLASS_FIELD_HASH;

            let field_info = if is_superclass {
                let superclass_name = if let TypeInfoData::Class(class_info) = &type_info.data {
                    class_info.super_class.expect("Super class not found").name
                } else {
                    panic!("Invalid type info");
                };

                Some(Cow::<FieldInfoData>::Owned(FieldInfoData {
                    name: "_glacier_base",
                    field_type: superclass_name,
                    rust_offset: 0,
                    flags: MemberInfoFlags::default(),
                }))
            } else {
                if let TypeInfoData::Class(class_info) = &type_info.data {
                    class_info.field_by_hash(field_type.field_name_hash)
                } else if let TypeInfoData::ValueType(value_type_info) = &type_info.data {
                    value_type_info.field_by_hash(field_type.field_name_hash)
                } else {
                    panic!("Invalid type info");
                }
                .map(Cow::Borrowed)
            };

            let field_info = match field_info {
                Some(field_info) => field_info,
                None => panic!("Field not found: {}", field_type.field_name_hash),
            };

            let target_ptr = unsafe { data_ptr.data_ptr.offset(field_info.rust_offset as isize) };

            let field_type_info = field_info.field_type(inst.type_registry);
            match &field_type_info.data {
                TypeInfoData::FileRef
                | TypeInfoData::ResourceRef
                | TypeInfoData::BoxedValueRef
                | TypeInfoData::TypeRef
                | TypeInfoData::Uint64
                | TypeInfoData::Int64
                | TypeInfoData::Float64 => {
                    inst.align_payload(8);
                }
                TypeInfoData::Array(_) | TypeInfoData::Class(_) => {
                    inst.align_payload(4);
                }
                _ => {}
            }

            println!("Field: {:?}", field_info);

            //let mut source_data = inst.payload.clone();
            //source_data.advance(field_type.offset as usize);

            match &field_type_info.data {
                TypeInfoData::Uint8 => todo!(),
                TypeInfoData::Int8 => todo!(),
                TypeInfoData::Uint16 => todo!(),
                TypeInfoData::Int16 => todo!(),
                TypeInfoData::Uint32 => todo!(),
                TypeInfoData::Int32 => todo!(),
                TypeInfoData::Uint64 => todo!(),
                TypeInfoData::Int64 => todo!(),
                TypeInfoData::Float32 => todo!(),
                TypeInfoData::Float64 => todo!(),
                TypeInfoData::Boolean => todo!(),
                TypeInfoData::CString => {
                    println!("Current offset: {}", inst.full_reader_index());
                    let offset = inst.payload.get_u32_le();
                    let string = inst.payload_string(offset).unwrap();
                    println!("String: {}", string);
                    println!("Current offset2: {}", inst.full_reader_index());

                    unsafe {
                        *(target_ptr as *mut String) = string;
                    }
                }
                TypeInfoData::ResourceRef => todo!(),
                TypeInfoData::TypeRef => todo!(),
                TypeInfoData::FileRef => todo!(),
                TypeInfoData::BoxedValueRef => todo!(),
                TypeInfoData::SHA1 => todo!(),
                TypeInfoData::Guid => todo!(),
                TypeInfoData::Array(element_type_name) => {
                    let mut payload = inst.payload_start.clone();
                    payload.advance(inst.header.array_offset as usize);

                    let index = inst.payload.get_i32_le();
                    let array = &inst.array_entries[index as usize];
                    payload.advance(array.offset as usize);

                    let array_type_info = inst
                        .type_registry
                        .type_by_name(element_type_name)
                        .expect(&format!(
                            "Array element type not found while marshalling EBX field {}",
                            field_type_info.name,
                        ));

                    for j in 0..array.element_count {
                        match &array_type_info.data {
                            TypeInfoData::Uint8 => todo!(),
                            TypeInfoData::Int8 => todo!(),
                            TypeInfoData::Uint16 => todo!(),
                            TypeInfoData::Int16 => todo!(),
                            TypeInfoData::Uint32 => todo!(),
                            TypeInfoData::Int32 => todo!(),
                            TypeInfoData::Uint64 => todo!(),
                            TypeInfoData::Int64 => todo!(),
                            TypeInfoData::Float32 => todo!(),
                            TypeInfoData::Float64 => todo!(),
                            TypeInfoData::Boolean => todo!(),
                            TypeInfoData::CString => todo!(),
                            TypeInfoData::ResourceRef => todo!(),
                            TypeInfoData::TypeRef => todo!(),
                            TypeInfoData::FileRef => todo!(),
                            TypeInfoData::BoxedValueRef => todo!(),
                            TypeInfoData::SHA1 => todo!(),
                            TypeInfoData::Guid => {
                                let guid = payload.get_guid();
                                unsafe { (*(target_ptr as *mut Vec<Guid>)).push(guid) };
                            }
                            TypeInfoData::Array(_) => todo!(),
                            TypeInfoData::Class(_) => todo!(),
                            TypeInfoData::ValueType(_) => todo!(),
                            TypeInfoData::Enum => todo!(),
                            TypeInfoData::Unknown => todo!(),
                        }
                    }
                }
                TypeInfoData::Class(_) => {
                    let tfd = inst
                        .type_resolver
                        .type_by_name_hash(field_type_info.name_hash())
                        .expect(
                            format!(
                                "Type not found while marshalling EBX field '{}' in '{}'",
                                field_type.field_name_hash, field_type_info.name,
                            )
                            .as_str(),
                        )
                        .clone();

                    Self::marshal_fields(inst, tfd, Some(container), depth + 1).await;
                }
                TypeInfoData::ValueType(_) => todo!(),
                TypeInfoData::Enum => todo!(),
                TypeInfoData::Unknown => todo!(),
            }
        }
    }

    fn payload_reader_index(&self) -> usize {
        self.payload.as_ptr() as usize - self.payload_start_ptr
    }

    fn full_reader_index(&self) -> usize {
        self.header.meta_size as usize
            + self.header.string_table_size as usize
            + self.payload_reader_index()
    }

    fn align_payload(&mut self, alignment: usize) {
        if alignment == 1 || alignment == 0 {
            return;
        }

        let offset = self.full_reader_index();

        let mut padding = 0;

        while (offset + padding) % alignment != 0 {
            padding += 1;
        }

        println!("offset: {}, aligning to {}", offset, alignment);

        // let aligned_offset = roundup(offset, alignment);
        // let padding = aligned_offset - offset;

        println!("padding: {}", padding);
        self.payload.advance(padding);
    }

    fn payload_string(&mut self, offset: u32) -> Option<String> {
        self.string_table.get(offset as usize..).and_then(|slice| {
            let end = slice.iter().position(|&x| x == 0).unwrap_or(slice.len());
            let string = String::from_utf8(slice[..end].to_vec()).unwrap();
            Some(string)
        })
    }
}

#[cfg(test)]
mod tests {
    use glacier_reflect_swbf2::{
        core::AssetTrait,
        gameplay_sim::{LevelReportingAsset, LevelReportingAssetTrait},
        register_mod_types,
    };

    use super::*;

    #[tokio::test]
    async fn test_ebx_partition_reader() {
        let data = include_bytes!("../../tests/data/LevelListReport.bin");
        let mut bytes = BytesMut::from(&data[..]);

        let mut registry = TypeRegistry::default();
        register_mod_types(&mut registry);

        let mut reader = EbxPartitionReader::new("LevelListReport".to_owned(), &registry);
        reader.read(&mut bytes).await;

        let partition = reader.finalize();
        let primary_instance = partition.primary_instance().unwrap();
        let primary_instance = primary_instance.lock().await;

        let level_list = primary_instance
            .as_any()
            .downcast_ref::<LevelReportingAsset>()
            .unwrap();

        assert_eq!(level_list.name(), "LevelListReport");
        assert_eq!(level_list.built_levels().len(), 44);
        assert_eq!(
            level_list.built_levels()[0].to_string(),
            "fdbbb005-6179-5cef-b271-4b008858d46a"
        );

        println!("{:#?}", primary_instance);
    }
}
