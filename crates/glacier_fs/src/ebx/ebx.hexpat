struct AlignTo<auto Alignment> {
    padding[Alignment- ((($ - 1) % Alignment) + 1)];
} [[hidden]];

struct EbxPartitionHeader {
    u32 magic;
    u32 meta_size;
    u32 payload_size;
    u32 import_count;
    u16 range_count;
    u16 exported_range_count;
    u16 type_count;
    u16 type_descriptor_count;
    u16 field_descriptor_count;
    u16 type_string_table_size;
    u32 string_table_size;
    u32 array_count;
    u32 array_offset;
    u8 partition_guid[16];
    u32 boxed_value_count;
    u32 boxed_value_offset;
};

struct EbxPartitionImportEntry {
    u8 partition_guid[16];
    u8 instance_guid[16];
};

struct EbxPartitionFieldDescriptor {
    u32 field_name_hash;
    u16 flags;
    u16 field_type;
    u32 field_offset;
    u32 secondary_offset;
};

struct EbxPartitionTypeDescriptor {
    u32 type_name_hash;
    u32 layout_descriptor;
    u8 field_count;
    u8 alignment;
    u16 type_flags;
    u16 instance_size;
    u16 secondary_instance_size;
};

struct EbxPartitionInstanceRange {
    u16 type_descriptor_index;
    u16 instance_count;
};

struct EbxPartitionArrayEntry {
    u32 offset;
    u32 element_count;
    u32 type_descriptor_index;
};

struct EbxPayload {
    
};

struct EbxPartition {
    EbxPartitionHeader header;
    EbxPartitionImportEntry imports[header.import_count];
    u8 type_string_table[header.type_string_table_size];
    EbxPartitionFieldDescriptor field_descriptors[header.field_descriptor_count];
    AlignTo<16>;
    EbxPartitionTypeDescriptor type_descriptors[header.type_descriptor_count];
    EbxPartitionInstanceRange ranges[header.range_count];
    AlignTo<16>;
    EbxPartitionArrayEntry arrays[header.array_count];
    
    u8 string_table[header.string_table_size] @ header.meta_size;
};

EbxPartition a @ 0x0;
