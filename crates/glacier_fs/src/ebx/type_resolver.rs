use glacier_reflect::member::MemberInfoFlags;

#[derive(Default, Clone)]
pub struct EbxPartitionFieldDescriptor {
    field_name_hash: u32,
    flags: MemberInfoFlags,
    field_type: u16,
    field_offset: u32,
    secondary_offset: u32,
}

#[derive(Default, Clone)]
pub struct EbxPartitionTypeDescriptor {
    field_name_hash: u32,
    layout_descriptor: u32,
    field_count: u8,
    alignment: u8,
    type_flags: MemberInfoFlags,
    instance_size: u16,
    secondary_instance_size: u16,
}

#[derive(Default)]
pub struct EbxPartitionTypeResolver {
    field_descriptors: Vec<EbxPartitionFieldDescriptor>,
    type_descriptors: Vec<EbxPartitionTypeDescriptor>,
    meta_string_block: Vec<u8>,
}

impl EbxPartitionTypeResolver {
    pub fn init(
        &mut self,
        field_descriptors: Vec<EbxPartitionFieldDescriptor>,
        type_descriptors: Vec<EbxPartitionTypeDescriptor>,
        meta_string_block: Vec<u8>,
    ) {
        self.field_descriptors = field_descriptors;
        self.type_descriptors = type_descriptors;
        self.meta_string_block = meta_string_block;
    }
}
