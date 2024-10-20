use glacier_reflect::{member::MemberInfoFlags, type_info::TypeInfo, type_registry::TypeRegistry};

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

impl EbxPartitionTypeDescriptor {
    pub fn is_shared_type_descriptor_reference(&self) -> bool {
        self.layout_descriptor & 0x80000000 != 0
    }

    pub fn type_info(&self, type_registry: &TypeRegistry) -> Option<&'static TypeInfo> {
        if self.is_shared_type_descriptor_reference() {
            let shared_type_descriptor_index = self.layout_descriptor & 0x7FFFFFFF;
            let shared_type_descriptor = type_registry.get_type_by_hash(shared_type_descriptor_index);

            if let Some(shared_type_descriptor) = shared_type_descriptor {
                return Some(shared_type_descriptor);
            }
        }

        None
    }
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

    pub fn resolve_type(&self, type_index: u32) -> &EbxPartitionTypeDescriptor {
        let td = self
            .type_descriptors
            .get(type_index as usize)
            .expect("Index out of bounds while resolving EBX type descriptor");

        if td.is_shared_type_descriptor_reference() {
            panic!("Shared type descriptor references are not supported yet");
        }

        td
    }
}
