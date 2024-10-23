use glacier_reflect::{member::MemberInfoFlags, type_info::TypeInfo, type_registry::TypeRegistry};

#[derive(Default, Clone, Debug)]
#[repr(C)]
pub struct EbxPartitionFieldDescriptor {
    pub field_name_hash: u32,
    pub flags: MemberInfoFlags,
    pub field_type: u16,
    pub field_offset: u32,
    pub secondary_offset: u32,
}

#[derive(Default, Clone, Debug)]
#[repr(C)]
pub struct EbxPartitionTypeDescriptor {
    pub type_name_hash: u32,
    pub layout_descriptor: u32,
    pub field_count: u8,
    pub alignment: u8,
    pub type_flags: MemberInfoFlags,
    pub instance_size: u16,
    pub secondary_instance_size: u16,
}

impl EbxPartitionTypeDescriptor {
    pub fn is_shared_type_descriptor_reference(&self) -> bool {
        self.layout_descriptor & 0x80000000 != 0
    }

    pub fn type_info(&self, type_registry: &TypeRegistry) -> Option<&'static TypeInfo> {
        type_registry.type_by_hash(self.type_name_hash)
        // if self.is_shared_type_descriptor_reference() {
        //     let shared_type_descriptor = type_registry.get_type_by_hash(self.type_name_hash);

        //     if let Some(shared_type_descriptor) = shared_type_descriptor {
        //         return Some(shared_type_descriptor);
        //     }
        // }

        // None
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

    pub fn type_by_name_hash(&self, name_hash: u32) -> Option<&EbxPartitionTypeDescriptor> {
        self.type_descriptors
            .iter()
            .find(|td| td.type_name_hash == name_hash)
    }

    pub fn field_by_index(&self, index: u32) -> &EbxPartitionFieldDescriptor {
        self.field_descriptors
            .get(index as usize)
            .expect("Index out of bounds while resolving EBX field descriptor")
    }

    pub fn type_by_index(&self, index: u32) -> &EbxPartitionTypeDescriptor {
        self.type_descriptors
            .get(index as usize)
            .expect("Index out of bounds while resolving EBX type descriptor")
    }
}
