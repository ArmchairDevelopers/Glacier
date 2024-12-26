use std::{borrow::Cow, mem};

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        BoxedTypeObject, FieldInfoData, LockedTypeObject, TypeInfo, TypeInfoData, TypeObject,
    },
    type_registry::TypeRegistry,
};
use glacier_util::{endian::endian_swap, guid::Guid, math::roundup};

use crate::{
    db::partition::{DatabasePartition, PartitionInitData},
    io::native_reader::NativeReader,
    util::trait_coercion::data_ptr_from_trait_object,
};

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

    data: NativeReader,

    header: EbxPartitionHeader,
    state: ReaderState,
    endian_swap: bool,

    init_data: PartitionInitData,
    type_resolver: EbxPartitionTypeResolver,

    metadata_outstanding: u32,
    payload_outstanding: u32,

    full_size: usize,
    payload_start: usize,

    import_entries: Vec<EbxPartitionImportEntry>,
    imports: Vec<Option<LockedTypeObject>>,

    instance_ranges: Vec<EbxPartitionInstanceRange>,
    array_entries: Vec<EbxPartitionArrayEntry>,
    boxed_value_entries: Vec<EbxPartitionBoxedValueEntry>,

    type_infos: Vec<Option<&'static TypeInfo>>,
    containers: Vec<LockedTypeObject>,
}

impl<'a> EbxPartitionReader<'a> {
    pub fn new(partition_name: String, type_registry: &'a TypeRegistry) -> Self {
        Self {
            partition_name,
            type_registry,
            data: NativeReader::new(),
            header: EbxPartitionHeader::default(),
            state: ReaderState::Initial,
            endian_swap: false,
            init_data: PartitionInitData::default(),
            type_resolver: EbxPartitionTypeResolver::default(),
            metadata_outstanding: 0,
            payload_outstanding: 0,
            full_size: 0,
            payload_start: 0,
            import_entries: Vec::new(),
            imports: Vec::new(),
            instance_ranges: Vec::new(),
            array_entries: Vec::new(),
            boxed_value_entries: Vec::new(),
            type_infos: Vec::new(),
            containers: Vec::new(),
        }
    }

    pub async fn read(&mut self, data: Vec<u8>) {
        self.data = NativeReader::from_bytes(data);

        let in_byte_count = self.data.len();
        self.full_size = in_byte_count;

        let mut complete = false;

        while self.data.remaining() > 0 || in_byte_count == 0 {
            let byte_count = self.data.len() as u32;
            match self.state {
                ReaderState::Initial => {
                    assert!(byte_count >= mem::size_of::<EbxPartitionHeader>() as u32);

                    let mut header_data = [0u8; mem::size_of::<EbxPartitionHeader>()];
                    self.data.copy_to_slice(&mut header_data);

                    self.header = unsafe { mem::transmute(header_data) };

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
                    if complete {
                        self.state = ReaderState::Prepare;
                    } else {
                        panic!(
                            "Invalid EBX partition size while reading '{}'",
                            self.partition_name
                        );
                    }

                    //data.skip(bytes_to_copy as usize);
                }
                ReaderState::Prepare => {
                    self.import_entries = unsafe {
                        let mut values = vec![
                            EbxPartitionImportEntry::default();
                            self.header.import_count as usize
                        ];
                        let raw = values.as_mut_ptr() as *mut u8;
                        let len = values.len() * mem::size_of::<EbxPartitionImportEntry>();
                        let slice = std::slice::from_raw_parts_mut(raw, len);
                        self.data.copy_to_slice(slice);
                        values
                    };

                    // TODO: handle type descriptors
                    self.data.skip(self.header.type_string_table_size as usize);

                    let field_descriptors = unsafe {
                        let mut values = vec![
                            EbxPartitionFieldDescriptor::default();
                            self.header.field_descriptor_count as usize
                        ];
                        let raw = values.as_mut_ptr() as *mut u8;
                        let len = values.len() * mem::size_of::<EbxPartitionFieldDescriptor>();
                        let slice = std::slice::from_raw_parts_mut(raw, len);
                        slice.clone_from_slice(&self.data[..len]);
                        self.data.skip(roundup(len, 16));
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
                        self.data.copy_to_slice(slice);
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
                        slice.clone_from_slice(&self.data[..len]);
                        self.data.skip(roundup(len, 16));
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
                        slice.clone_from_slice(&self.data[..len]);
                        self.data.skip(roundup(len, 16));
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
                        slice.clone_from_slice(&self.data[..len]);
                        self.data.skip(roundup(len, 16));
                        values
                    };

                    self.type_resolver.init(field_descriptors, type_descriptors);

                    self.resolve_imports();
                    self.state = ReaderState::Layout;
                }
                ReaderState::Layout => {
                    self.handle_layout().await;
                    self.state = ReaderState::Payloads;
                }
                ReaderState::Payloads => {
                    self.payload_start =
                        self.header.meta_size as usize + self.header.string_table_size as usize;
                    self.data.seek(self.payload_start);

                    self.state = ReaderState::Fixup;
                }
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

    fn resolve_imports(&mut self) {
        self.imports = vec![None; self.import_entries.len()];

        for i in 0..self.import_entries.len() {
            // TODO
            self.imports[i] = None;
        }
    }

    pub fn finalize(self) -> DatabasePartition {
        DatabasePartition::new(self.init_data)
    }

    async fn handle_layout(&mut self) {
        self.type_infos = vec![None; 512];

        let mut instance_count = 0;

        let mut i = 0;
        for range in &self.instance_ranges {
            // let td = self
            //     .type_resolver
            //     .resolve_type(range.type_descriptor_index as u32);
            // let type_info = td.type_info(self.type_registry);
            //println!("{}", range.type_descriptor_index);

            let td = self
                .type_resolver
                .type_by_index(range.type_descriptor_index as u32);
            //println!("t: {}", td.type_name_hash);

            let type_info = self.type_registry.type_by_hash(td.type_name_hash);

            self.type_infos[range.type_descriptor_index as usize] = type_info;

            let type_info = type_info.unwrap();
            let class_info = if let TypeInfoData::Class(class_info) = &type_info.data {
                class_info
            } else {
                panic!("Invalid type info");
            };

            //println!("name: {}", type_info.name);

            let is_exported_range = i < self.header.exported_range_count;

            for _ in 0..range.instance_count {
                let container = class_info.create();

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
    }

    async fn handle_fixup(&mut self) {
        let mut i = 0;
        for range in self.instance_ranges.clone() {
            let td = self
                .type_resolver
                .resolve_type(range.type_descriptor_index as u32)
                .clone();

            //let type_info = self.type_infos[range.type_descriptor_index as usize].unwrap();

            // println!(
            //     "Parsing range {} with {} bytes",
            //     type_info.name,
            //     self.data.len()
            // );

            for _ in 0..range.instance_count {
                self.align_payload(td.alignment as usize);

                let container = self.containers[i].clone();

                let mut container = container.lock().await;
                let dc_core = container
                    .data_container_core_mut()
                    .expect("Invalid container");
                dc_core.instance_guid = if dc_core.exported {
                    self.data.get_guid()
                } else {
                    Guid::random()
                };

                if td.alignment != 4 {
                    self.data.skip(8);
                }

                Self::marshal_fields(self, td.clone(), Some(&mut *container), 0, 0).await;
                i += 1;
            }
        }
    }

    #[async_recursion::async_recursion]
    async fn marshal_fields(
        inst: &mut EbxPartitionReader,
        type_desc: EbxPartitionTypeDescriptor,
        container: Option<&mut dyn TypeObject>,
        offset: isize,
        depth: u32,
    ) {
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

        let raw = data_ptr_from_trait_object(container);

        for i in 0..type_desc.field_count as u32 {
            let field_desc = inst
                .type_resolver
                .field_by_index(type_desc.layout_descriptor + i)
                .clone();
            let field_type_desc = inst
                .type_resolver
                .type_by_index(field_desc.field_type as u32)
                .clone();

            let is_superclass = field_desc.field_name_hash == SUPER_CLASS_FIELD_HASH;

            let field_info = if is_superclass {
                let superclass_name = if let TypeInfoData::Class(class_info) = &type_info.data {
                    class_info.super_class.expect("Super class not found").name
                } else {
                    panic!("Invalid type info");
                };

                Some(Cow::<FieldInfoData>::Owned(FieldInfoData {
                    name: "_glacier_base",
                    name_hash: SUPER_CLASS_FIELD_HASH,
                    field_type: superclass_name,
                    rust_offset: 0,
                    flags: MemberInfoFlags::default(),
                }))
            } else {
                if let TypeInfoData::Class(class_info) = &type_info.data {
                    class_info.field_by_hash(field_desc.field_name_hash)
                } else if let TypeInfoData::ValueType(value_type_info) = &type_info.data {
                    value_type_info.field_by_hash(field_desc.field_name_hash)
                } else {
                    panic!("Invalid type info");
                }
                .map(Cow::Borrowed)
            };

            let field_info = match field_info {
                Some(field_info) => field_info,
                None => panic!("Field not found: {}", field_desc.field_name_hash),
            };

            let target_ptr = unsafe {
                raw.data_ptr
                    .offset(offset)
                    .offset(field_info.rust_offset as isize)
            };

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

            match &field_type_info.data {
                TypeInfoData::Uint8 => todo!(),
                TypeInfoData::Int8 => todo!(),
                TypeInfoData::Uint16 => todo!(),
                TypeInfoData::Int16 => todo!(),
                TypeInfoData::Uint32 => {
                    let value = inst.data.get_u32();
                    unsafe { *(target_ptr as *mut u32) = value };
                }
                TypeInfoData::Int32 => {
                    let value = inst.data.get_i32();
                    unsafe { *(target_ptr as *mut i32) = value };
                }
                TypeInfoData::Uint64 => todo!(),
                TypeInfoData::Int64 => todo!(),
                TypeInfoData::Float32 => {
                    let value = inst.data.get_f32();
                    unsafe { *(target_ptr as *mut f32) = value };
                }
                TypeInfoData::Float64 => todo!(),
                TypeInfoData::Boolean => {
                    let value = inst.data.get_u8();
                    unsafe { *(target_ptr as *mut bool) = value != 0 };
                }
                TypeInfoData::CString => {
                    let str_offset = inst.data.get_u32();
                    let string = inst.payload_string(str_offset);
                    unsafe {
                        (*(target_ptr as *mut String)).push_str(&string);
                    };
                }
                TypeInfoData::ResourceRef => todo!(),
                TypeInfoData::TypeRef => todo!(),
                TypeInfoData::FileRef => todo!(),
                TypeInfoData::BoxedValueRef => todo!(),
                TypeInfoData::SHA1 => todo!(),
                TypeInfoData::Guid => {
                    let guid = inst.data.get_guid();
                    //println!("Guid: {:?}", guid);
                    unsafe { (*(target_ptr as *mut Guid)) = guid };
                }
                TypeInfoData::Array(element_type_name) => {
                    let index = inst.data.get_i32();
                    let payload_offset = inst.data.pos();

                    let array = &inst.array_entries[index as usize];
                    inst.data.seek(
                        inst.payload_start
                            + inst.header.array_offset as usize
                            + array.offset as usize,
                    );

                    let array_field_desc = inst
                        .type_resolver
                        .field_by_index(field_type_desc.layout_descriptor as u32)
                        .clone();

                    let type_desc = inst
                        .type_resolver
                        .type_by_index(array_field_desc.field_type as u32)
                        .clone();

                    let element_type_info = inst
                        .type_registry
                        .type_by_name(element_type_name)
                        .expect(&format!(
                            "Array element type not found while marshalling EBX field {}",
                            field_type_info.name,
                        ));

                    // println!(
                    //     "Array: {:?} {} {} {}",
                    //     array,
                    //     element_type_info.name,
                    //     type_desc.type_name_hash,
                    //     element_type_info.name.hash_quick()
                    // );

                    let count = array.element_count;
                    match &element_type_info.data {
                        TypeInfoData::Uint8 => {
                            let vec = unsafe { &mut *(target_ptr as *mut Vec<u8>) };
                            vec.reserve_exact(count as usize);

                            for _ in 0..count as usize {
                                let value = inst.data.get_u8();
                                vec.push(value);
                            }
                        }
                        TypeInfoData::Int8 => todo!(),
                        TypeInfoData::Uint16 => todo!(),
                        TypeInfoData::Int16 => todo!(),
                        TypeInfoData::Uint32 => {
                            let vec = unsafe { &mut *(target_ptr as *mut Vec<u32>) };
                            vec.reserve_exact(count as usize);

                            for _ in 0..count as usize {
                                let value = inst.data.get_u32();
                                vec.push(value);
                            }
                        }
                        TypeInfoData::Int32 => {
                            let vec = unsafe { &mut *(target_ptr as *mut Vec<i32>) };
                            vec.reserve_exact(count as usize);

                            for _ in 0..count as usize {
                                let value = inst.data.get_i32();
                                vec.push(value);
                            }
                        }
                        TypeInfoData::Uint64 => todo!(),
                        TypeInfoData::Int64 => todo!(),
                        TypeInfoData::Float32 => {
                            let vec = unsafe { &mut *(target_ptr as *mut Vec<f32>) };
                            vec.reserve_exact(count as usize);

                            for _ in 0..count as usize {
                                let value = inst.data.get_f32();
                                vec.push(value);
                            }
                        }
                        TypeInfoData::Float64 => todo!(),
                        TypeInfoData::Boolean => todo!(),
                        TypeInfoData::CString => todo!(),
                        TypeInfoData::ResourceRef => todo!(),
                        TypeInfoData::TypeRef => todo!(),
                        TypeInfoData::FileRef => todo!(),
                        TypeInfoData::BoxedValueRef => todo!(),
                        TypeInfoData::SHA1 => todo!(),
                        TypeInfoData::Guid => {
                            let vec = unsafe { &mut *(target_ptr as *mut Vec<Guid>) };

                            let mut buf = vec![0u8; mem::size_of::<Guid>() * count as usize];
                            inst.data.copy_to_slice(&mut buf);

                            let raw = buf.as_mut_ptr() as *mut Guid;
                            let slice =
                                unsafe { std::slice::from_raw_parts_mut(raw, count as usize) };

                            vec.extend_from_slice(slice);
                        }
                        TypeInfoData::Array(_) => todo!(),
                        TypeInfoData::Class(_) => {
                            let vec =
                                unsafe { &mut *(target_ptr as *mut Vec<Option<LockedTypeObject>>) };
                            vec.reserve_exact(count as usize);

                            for _ in 0..count {
                                let mut dc_ref: Option<LockedTypeObject> = None;
                                inst.marshal_reference(
                                    &mut dc_ref as *mut Option<LockedTypeObject> as *mut u8,
                                );
                                vec.push(dc_ref);
                            }
                        }
                        TypeInfoData::ValueType(value_type_data) => {
                            inst.align_payload(type_desc.alignment as usize);

                            let vec = unsafe { &mut *(target_ptr as *mut Vec<BoxedTypeObject>) };
                            vec.reserve_exact(count as usize);

                            for _ in 0..count {
                                let mut boxed = value_type_data.create_boxed();
                                Self::marshal_fields(
                                    inst,
                                    type_desc.clone(),
                                    Some(&mut *boxed),
                                    0,
                                    depth + 1,
                                )
                                .await;
                                vec.push(boxed);
                            }
                        }
                        TypeInfoData::Enum => todo!(),
                        TypeInfoData::Unknown => todo!(),
                    }

                    inst.data.seek(payload_offset);
                }
                TypeInfoData::Class(class_info) => {
                    if is_superclass {
                        Self::marshal_fields(
                            inst,
                            field_type_desc.clone(),
                            Some(container),
                            class_info.super_class_offset as isize,
                            depth + 1,
                        )
                        .await;
                    } else {
                        inst.marshal_reference(target_ptr);
                    }
                }
                TypeInfoData::ValueType(_) => {
                    inst.align_payload(field_type_desc.alignment as usize);

                    Self::marshal_fields(
                        inst,
                        field_type_desc.clone(),
                        Some(container),
                        offset + field_info.rust_offset as isize,
                        depth + 1,
                    )
                    .await;
                }
                TypeInfoData::Enum => {
                    let value = inst.data.get_i32() as i64;
                    unsafe { *(target_ptr as *mut i64) = value };
                }
                TypeInfoData::Unknown => todo!(),
            }
        }

        match type_info.data {
            TypeInfoData::Boolean => {}
            _ => inst.align_payload(type_desc.alignment as usize),
        }
    }

    fn marshal_reference(&mut self, target: *mut u8) {
        let index = self.data.get_u32();

        //println!("Reference index: {}", index);

        if index == 0 {
            unsafe { *(target as *mut Option<LockedTypeObject>) = None };
            return;
        }

        if (index & 0x8000_0000) == 0 {
            // Internal reference
            assert!(
                index - 1 < self.containers.len() as u32,
                "Invalid container index {}/{}",
                index,
                self.containers.len()
            );
            let container = self.containers[(index - 1) as usize].clone();
            unsafe { *(target as *mut Option<LockedTypeObject>) = Some(container) };
        } else {
            let i = index & 0x7FFF_FFFF;
            //println!("Import index: {}", i);

            assert!(i < self.imports.len() as u32);

            let import = self.imports[i as usize].clone();
            if import.is_none() {
                // Lazy resolve required
                unsafe { *(target as *mut Option<LockedTypeObject>) = None };
                //todo!("Lazy resolve required");
            } else {
                unsafe { *(target as *mut Option<LockedTypeObject>) = Some(import.unwrap()) };
            }
        }
    }

    fn align_payload(&mut self, alignment: usize) {
        self.data.align(alignment);
    }

    fn payload_string(&mut self, offset: u32) -> String {
        if offset == !0 {
            return String::new();
        }

        self.data
            .get(self.header.meta_size as usize + offset as usize..)
            .and_then(|slice| {
                let end = slice.iter().position(|&x| x == 0).unwrap_or(slice.len());
                let string = unsafe { String::from_utf8_unchecked(slice[..end].to_vec()) };
                Some(string)
            })
            .expect("Invalid string offset")
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
    async fn test_level_list_report() {
        let data = include_bytes!("../../tests/data/LevelListReport.bin");

        let mut registry = TypeRegistry::default();
        register_mod_types(&mut registry);

        let mut reader = EbxPartitionReader::new("LevelListReport".to_owned(), &registry);
        reader.read(data.to_vec()).await;

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

    #[tokio::test]
    async fn test_droideka_state_machine() {
        let data = include_bytes!("../../tests/data/DroidekaStateMachine.bin");

        let mut registry = TypeRegistry::default();
        register_mod_types(&mut registry);

        let mut reader = EbxPartitionReader::new("DroidekaStateMachine".to_owned(), &registry);
        reader.read(data.to_vec()).await;

        let partition = reader.finalize();
        let primary_instance = partition.primary_instance().unwrap();
        let primary_instance = primary_instance.lock().await;

        println!("{:#?}", primary_instance);
    }

    #[tokio::test]
    async fn test_default_state_machine() {
        let data = include_bytes!("../../tests/data/DefaultSoldierStateMachine.bin");

        let mut registry = TypeRegistry::default();
        register_mod_types(&mut registry);

        let mut reader =
            EbxPartitionReader::new("DefaultSoldierStateMachine".to_owned(), &registry);
        reader.read(data.to_vec()).await;

        let partition = reader.finalize();
        let primary_instance = partition.primary_instance().unwrap();
        let primary_instance = primary_instance.lock().await;

        println!("{:#?}", primary_instance);
    }
}
