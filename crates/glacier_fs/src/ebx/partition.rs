use std::mem;

use bytes::{Buf, BytesMut};
use glacier_reflect::{type_info::TypeInfo, type_registry::TypeRegistry};
use glacier_util::{endian::endian_swap, guid::Guid, math::roundup};

use super::type_resolver::{
    EbxPartitionFieldDescriptor, EbxPartitionTypeDescriptor, EbxPartitionTypeResolver,
};

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
struct EbxPartitionImportEntry {
    partition_guid: Guid,
    instance_guid: Guid,
}

#[derive(Default, Clone)]
struct EbxPartitionInstanceRange {
    type_descriptor_index: u16,
    instance_count: u16,
}

#[derive(Default, Clone)]
struct EbxPartitionArrayEntry {
    offset: u32,
    element_count: u32,
    type_descriptor_index: u32,
}

#[derive(Default, Clone)]
struct EbxPartitionBoxedValueEntry {
    offset: u32,
    type_id: u16,
    type_code: u16,
}

#[derive(Default)]
pub struct EbxPartitionInitData {
    partition_guid: Guid,
}

pub struct EbxPartitionReader<'a> {
    partition_name: String,
    type_registry: &'a TypeRegistry,

    header: EbxPartitionHeader,
    state: ReaderState,
    endian_swap: bool,

    init_data: EbxPartitionInitData,
    type_resolver: EbxPartitionTypeResolver,

    metadata_outstanding: u32,
    payload_outstanding: u32,

    metadata: BytesMut,

    import_entries: Vec<EbxPartitionImportEntry>,
    meta_string_block: Vec<String>,

    instance_ranges: Vec<EbxPartitionInstanceRange>,
    array_entries: Vec<EbxPartitionArrayEntry>,
    boxed_value_entries: Vec<EbxPartitionBoxedValueEntry>,

    type_infos: Vec<Option<&'static TypeInfo>>,
}

impl<'a> EbxPartitionReader<'a> {
    pub fn new(partition_name: String, type_registry: &'a TypeRegistry) -> Self {
        Self {
            partition_name,
            type_registry,
            header: EbxPartitionHeader::default(),
            state: ReaderState::Initial,
            endian_swap: false,
            init_data: EbxPartitionInitData::default(),
            type_resolver: EbxPartitionTypeResolver::default(),
            metadata_outstanding: 0,
            payload_outstanding: 0,
            metadata: BytesMut::new(),
            import_entries: Vec::new(),
            meta_string_block: Vec::new(),
            instance_ranges: Vec::new(),
            array_entries: Vec::new(),
            boxed_value_entries: Vec::new(),
        }
    }

    pub fn read(&mut self, data: &mut BytesMut) {
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

                    self.init_data.partition_guid = self.header.partition_guid;

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
                    self.handle_layout();
                }
                ReaderState::Payloads => {}
                ReaderState::PreparePayload => todo!(),
                ReaderState::Fixup => todo!(),
                ReaderState::Done => todo!(),
            }
        }
    }

    pub fn handle_layout(&mut self) {
        self.type_infos = vec![None; 64];

        let mut instance_count = 0;
        for range in &self.instance_ranges {
            self.type_infos[range.type_descriptor_index as usize] = self.type_resolver.resolve_type(range.type_descriptor_index as u32);

            instance_count += range.instance_count as usize;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ebx_partition_reader() {
        let data = include_bytes!("../../tests/data/DefaultLodGroup.bin");
        let mut bytes = BytesMut::from(&data[..]);

        let registry = TypeRegistry::default();

        let mut reader = EbxPartitionReader::new("systems/Mesh/DefaultLoadGroup".to_owned(), &registry);
        reader.read(&mut bytes);
    }
}
