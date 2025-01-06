use std::{
    collections::{HashMap, VecDeque},
    io::SeekFrom,
    mem,
    sync::Arc,
};

use glacier_reflect::{
    type_info::{
        BoxedTypeObject, FieldInfoData, LockedTypeObject, TypeInfo, TypeInfoData, TypeObject,
    },
    type_registry::TypeRegistry,
};
use glacier_reflect_swbf2::core::DATACONTAINER_TYPE_INFO;
use glacier_util::guid::Guid;
use tokio::io::{
    AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, AsyncWrite, AsyncWriteExt, BufReader,
    BufWriter,
};
use tracing::{info, warn};

use crate::{
    db::partition::{DatabasePartition, PartitionInitData},
    util::trait_coercion::{data_ptr_from_trait_object, SendPtr},
};

#[derive(thiserror::Error, Debug)]
pub enum DbxReaderError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("int parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("bool parse error: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),
    #[error("float parse error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("invalid xml")]
    InvalidXml,
    #[error("invalid closing tag")]
    InvalidClosingTag,
    #[error("invalid type")]
    InvalidType,
    #[error("file not found")]
    FileNotFound,
}

#[derive(Clone, Debug)]
struct XmlNode {
    name: String,
    attributes: HashMap<String, String>,
}

struct XmlReader<W: AsyncRead + AsyncSeek + Unpin> {
    reader: BufReader<W>,
    node_stack: VecDeque<XmlNode>,
}

impl<W: AsyncRead + AsyncSeek + Unpin> XmlReader<W> {
    pub fn new(reader: W) -> Self {
        Self {
            reader: BufReader::new(reader),
            node_stack: VecDeque::new(),
        }
    }

    pub async fn read_doctype(&mut self) -> Result<(), DbxReaderError> {
        let mut c = self.reader.read_u8().await?;
        while c != b'<' {
            c = self.reader.read_u8().await?;
        }

        let mut doctype = String::new();
        c = self.reader.read_u8().await?;
        while c != b'>' {
            doctype.push(c as char);
            c = self.reader.read_u8().await?;
        }

        Ok(())
    }

    pub async fn read_node(&mut self) -> Result<XmlNode, DbxReaderError> {
        let mut opening_tag = self.reader.read_u8().await?;
        while opening_tag.is_ascii_whitespace() {
            opening_tag = self.reader.read_u8().await?;
        }

        if opening_tag != b'<' {
            return Err(DbxReaderError::InvalidXml);
        }

        let mut name = String::new();
        let mut attributes = HashMap::new();

        let mut c = self.reader.read_u8().await?;
        while c != b'>' {
            if c == b' ' {
                let mut attribute_name = String::new();
                let mut attribute_value = String::new();

                c = self.reader.read_u8().await?;
                while c != b'=' {
                    attribute_name.push(c as char);
                    c = self.reader.read_u8().await?;
                }

                c = self.reader.read_u8().await?;
                if c != b'"' {
                    return Err(DbxReaderError::InvalidXml);
                }

                c = self.reader.read_u8().await?;
                while c != b'"' {
                    attribute_value.push(c as char);
                    c = self.reader.read_u8().await?;
                }

                attributes.insert(attribute_name, attribute_value);
            } else {
                name.push(c as char);
            }

            c = self.reader.read_u8().await?;
        }

        let node = XmlNode { name, attributes };
        self.node_stack.push_back(node.clone());
        Ok(node)
    }

    pub async fn try_next_node(&mut self, name: &str) -> Result<Option<XmlNode>, DbxReaderError> {
        let current_pos = self.reader.stream_position().await?;
        if let Ok(node) = self.read_node().await
            && node.name == name
        {
            Ok(Some(node))
        } else {
            self.reader.seek(SeekFrom::Start(current_pos)).await?;
            Ok(None)
        }
    }

    pub async fn next_node(&mut self) -> Result<Option<XmlNode>, DbxReaderError> {
        let current_pos = self.reader.stream_position().await?;

        let last_node = self.node_stack.back().cloned();
        let node = self.read_node().await?;

        if let Some(last_node) = last_node {
            if node.name == format!("/{}", last_node.name) {
                self.node_stack.pop_back();
                self.reader.seek(SeekFrom::Start(current_pos)).await?;
                return Ok(None);
            }
        }

        Ok(Some(node))
    }

    pub async fn end_node(&mut self, node: &XmlNode) -> Result<(), DbxReaderError> {
        let last_node = self.node_stack.back().ok_or(DbxReaderError::InvalidXml)?;
        if node.name != last_node.name {
            eprintln!(
                "Expected closing tag: {}, got: {}",
                last_node.name, node.name
            );
            return Err(DbxReaderError::InvalidXml);
        }

        'outer: loop {
            let mut c = self.reader.read_u8().await?;
            while c != b'<' {
                c = self.reader.read_u8().await?;
            }

            let mut closing_tag = String::new();
            c = self.reader.read_u8().await?;
            while c != b'>' {
                closing_tag.push(c as char);
                c = self.reader.read_u8().await?;

                if c == b' ' || c == b'@' {
                    continue 'outer;
                }
            }

            if closing_tag == format!("/{}", last_node.name) {
                self.node_stack.pop_back();
                return Ok(());
            } else {
                eprintln!("Closing tag: {}", closing_tag);
                return Err(DbxReaderError::InvalidClosingTag);
            }
        }
    }

    pub async fn read_text(&mut self) -> Result<String, DbxReaderError> {
        let mut text = String::new();
        let mut c = self.reader.read_u8().await?;
        while c != b'<' {
            text.push(c as char);
            c = self.reader.read_u8().await?;
        }

        self.reader.seek(SeekFrom::Current(-1)).await?;
        Ok(text)
    }
}

#[async_trait::async_trait]
pub trait DbxPartitionImportLoader: Sync {
    async fn store_instance(
        &mut self,
        partition_guid: Guid,
        instance_guid: Guid,
        instance: LockedTypeObject,
    );

    async fn load_partition(
        &mut self,
        partition_guid: Guid,
        instance_guid: Guid,
    ) -> Option<LockedTypeObject>;
}

#[derive(Debug)]
struct DbxDeferredReference {
    instance_guid: Guid,
    target_instance_guid: Guid,
    target: SendPtr<u8>,
}

pub struct DbxPartitionReader<'a> {
    name: String,
    partition_guid: Guid,

    type_registry: &'a TypeRegistry,
    importer: Option<&'a mut dyn DbxPartitionImportLoader>,

    instances: HashMap<Guid, LockedTypeObject>,

    current_instance_index: usize,
    instance_indices: HashMap<usize, Guid>,

    deferred_references: Vec<DbxDeferredReference>,
}

unsafe impl<'a> Send for DbxPartitionReader<'a> {}

impl<'a> DbxPartitionReader<'a> {
    pub fn new(
        name: String,
        type_registry: &'a TypeRegistry,
        importer: Option<&'a mut dyn DbxPartitionImportLoader>,
    ) -> Self {
        Self {
            name,
            partition_guid: Guid::default(),
            type_registry,
            importer,
            instances: HashMap::new(),
            current_instance_index: 0,
            instance_indices: HashMap::new(),
            deferred_references: Vec::new(),
        }
    }

    pub async fn read<W: AsyncRead + AsyncSeek + Unpin + Send>(
        &mut self,
        reader: W,
    ) -> Result<(), DbxReaderError> {
        let mut xml_reader = XmlReader::new(reader);
        xml_reader.read_doctype().await?;

        let partition_node = xml_reader.read_node().await?;
        if partition_node.name != "partition" {
            return Err(DbxReaderError::InvalidXml);
        }

        let partition_guid = Guid::from_str(
            partition_node
                .attributes
                .get("guid")
                .ok_or(DbxReaderError::InvalidXml)?,
        );

        self.partition_guid = partition_guid;

        // let primary_instance_guid = Guid::from_str(
        //     partition_node
        //         .attributes
        //         .get("primaryInstance")
        //         .ok_or(DbxReaderError::InvalidXml)?,
        // );

        //let mut raw_partitions = Vec::new();

        while let Some(node) = xml_reader.try_next_node("instance").await? {
            //println!("Reading instance {:?}", node);
            self.read_instance(node.clone(), &mut xml_reader).await?;
            xml_reader.end_node(&node).await?;
        }

        //println!("Found {} partitions", raw_partitions.len());

        Ok(())
    }

    pub async fn finalize(self) -> DatabasePartition {
        let mut instances = Vec::new();

        for i in 0..self.current_instance_index {
            let guid = self
                .instance_indices
                .get(&i)
                .expect("Instance index not found");

            let instance = self.instances.get(guid).expect("Instance not found");

            // Resolve deferred references. God this sucks.
            for reference in &self.deferred_references {
                if reference.instance_guid == *guid {
                    let _ = instance.lock().await;

                    let target_instance = self
                        .instances
                        .get(&reference.target_instance_guid)
                        .expect("Target of deferred reference not found");

                    unsafe {
                        *(reference.target.0 as *mut Option<LockedTypeObject>) =
                            Some(target_instance.clone());
                    }
                }
            }

            instances.push(instance.clone());
        }

        let init_data = PartitionInitData {
            name: self.name,
            guid: self.partition_guid,
            instances,
        };

        DatabasePartition::new(init_data)
    }

    async fn read_instance<W: AsyncRead + AsyncSeek + Unpin + Send>(
        &mut self,
        node: XmlNode,
        xml_reader: &mut XmlReader<W>,
    ) -> Result<(), DbxReaderError> {
        let instance_guid = Guid::from_str(
            node.attributes
                .get("guid")
                .ok_or(DbxReaderError::InvalidXml)
                .unwrap(),
        );

        let type_name = node
            .attributes
            .get("type")
            .ok_or(DbxReaderError::InvalidXml)?
            .split(".")
            .last()
            .expect("DBX type name is not in the format {module}.{name}");

        let type_info = self
            .type_registry
            .type_by_name(type_name)
            .ok_or(DbxReaderError::InvalidType)?;

        let is_exported = node.attributes.contains_key("exported");

        let class_info = if let TypeInfoData::Class(class_info) = &type_info.data {
            class_info
        } else {
            panic!("Invalid type info");
        };

        let container = class_info.create();

        {
            let mut container = container.lock().await;
            let dc_core = container
                .data_container_core_mut()
                .expect("Invalid container");
            dc_core.exported = is_exported;
            dc_core.partition_guid = self.partition_guid;
            dc_core.instance_guid = instance_guid;
        }

        self.instances.insert(instance_guid, container.clone());

        self.instance_indices
            .insert(self.current_instance_index, instance_guid);
        self.current_instance_index += 1;

        let mut container = container.lock().await;

        self.read_fields(
            container.type_info(),
            &instance_guid,
            xml_reader,
            &mut *container,
            0,
        )
        .await?;
        Ok(())
    }

    #[async_recursion::async_recursion]
    async fn read_fields<W: AsyncRead + AsyncSeek + Unpin + Send>(
        &mut self,
        type_info: &'static TypeInfo,
        instance_guid: &Guid,
        xml_reader: &mut XmlReader<W>,
        instance: &mut dyn TypeObject,
        offset: isize,
    ) -> Result<(), DbxReaderError> {
        let raw = data_ptr_from_trait_object(instance);

        loop {
            let node = xml_reader.next_node().await?;
            if let Some(node) = node {
                if node.name == "field" || node.name == "array" || node.name == "complex" {
                    let field_name = node
                        .attributes
                        .get("name")
                        .ok_or(DbxReaderError::InvalidXml)?;

                    //println!("Reading field: {}", field_name);

                    let field_info = if let TypeInfoData::Class(class_info) = &type_info.data {
                        class_info.field(field_name)
                    } else if let TypeInfoData::ValueType(value_type_info) = &type_info.data {
                        value_type_info.field(field_name)
                    } else {
                        panic!("Invalid type info");
                    };

                    let field_info = match field_info {
                        Some(field_info) => field_info,
                        None => {
                            warn!("Field {} not found in type {}", field_name, type_info.name);
                            continue;
                        }
                    };

                    let target_ptr = SendPtr(unsafe {
                        raw.data_ptr
                            .offset(offset)
                            .offset(field_info.rust_offset as isize)
                    });

                    let field_type = field_info.field_type(&self.type_registry);
                    match &field_type.data {
                        TypeInfoData::Uint8 => {
                            let value = xml_reader.read_text().await?.parse::<u8>()?;
                            unsafe {
                                *(target_ptr.0 as *mut u8) = value;
                            };
                        }
                        TypeInfoData::Int8 => todo!(),
                        TypeInfoData::Uint16 => {
                            let value = xml_reader.read_text().await?.parse::<u16>()?;
                            unsafe {
                                *(target_ptr.0 as *mut u16) = value;
                            };
                        }
                        TypeInfoData::Int16 => todo!(),
                        TypeInfoData::Uint32 => {
                            let value = xml_reader.read_text().await?.parse::<u32>()?;
                            unsafe {
                                *(target_ptr.0 as *mut u32) = value;
                            };
                        }
                        TypeInfoData::Int32 => {
                            let value = xml_reader.read_text().await?.parse::<i32>()?;
                            unsafe {
                                *(target_ptr.0 as *mut i32) = value;
                            };
                        }
                        TypeInfoData::Uint64 => {
                            let value = xml_reader.read_text().await?.parse::<u64>()?;
                            unsafe {
                                *(target_ptr.0 as *mut u64) = value;
                            };
                        }
                        TypeInfoData::Int64 => todo!(),
                        TypeInfoData::Float32 => {
                            let value = xml_reader.read_text().await?.parse::<f32>()?;
                            unsafe {
                                *(target_ptr.0 as *mut f32) = value;
                            };
                        }
                        TypeInfoData::Float64 => todo!(),
                        TypeInfoData::Boolean => {
                            let value = xml_reader.read_text().await?.parse::<bool>()?;
                            unsafe {
                                *(target_ptr.0 as *mut bool) = value;
                            };
                        }
                        TypeInfoData::CString => {
                            let str = xml_reader.read_text().await?;
                            unsafe {
                                (*(target_ptr.0 as *mut String)).push_str(&str);
                            };
                        }
                        TypeInfoData::ResourceRef => {
                            let _value = xml_reader.read_text().await?;
                            // TODO
                        }
                        TypeInfoData::TypeRef => {
                            let _value = xml_reader.read_text().await?;
                            // TODO
                        }
                        TypeInfoData::FileRef => {
                            let _value = xml_reader.read_text().await?;
                            // TODO
                        }
                        TypeInfoData::BoxedValueRef => {
                            let _value = xml_reader.read_text().await?;
                            // TODO
                        }
                        TypeInfoData::SHA1 => todo!(),
                        TypeInfoData::Guid => {
                            let value = xml_reader.read_text().await?;
                            let guid = Guid::from_str(&value);
                            unsafe {
                                *(target_ptr.0 as *mut Guid) = guid;
                            };
                        }
                        TypeInfoData::Array(array_type) => {
                            let array_type = self
                                .type_registry
                                .type_by_name(array_type)
                                .expect("Invalid type");

                            match &array_type.data {
                                TypeInfoData::Uint8 => {
                                    let vec = unsafe { &mut *(target_ptr.0 as *mut Vec<u8>) };

                                    while let Some(node) = xml_reader.next_node().await? {
                                        if node.name == "array" {
                                            eprintln!("Arrays of arrays are not supported!");
                                            break;
                                        }

                                        let value = xml_reader.read_text().await?.parse::<u8>()?;
                                        vec.push(value);

                                        xml_reader.end_node(&node).await?;
                                    }
                                }
                                TypeInfoData::Int8 => todo!(),
                                TypeInfoData::Uint16 => {
                                    let vec = unsafe { &mut *(target_ptr.0 as *mut Vec<u16>) };

                                    while let Some(node) = xml_reader.next_node().await? {
                                        if node.name == "array" {
                                            eprintln!("Arrays of arrays are not supported!");
                                            break;
                                        }

                                        let value = xml_reader.read_text().await?.parse::<u16>()?;
                                        vec.push(value);

                                        xml_reader.end_node(&node).await?;
                                    }
                                }
                                TypeInfoData::Int16 => todo!(),
                                TypeInfoData::Uint32 => {
                                    let vec = unsafe { &mut *(target_ptr.0 as *mut Vec<u32>) };

                                    while let Some(node) = xml_reader.next_node().await? {
                                        if node.name == "array" {
                                            eprintln!("Arrays of arrays are not supported!");
                                            break;
                                        }

                                        let value = xml_reader.read_text().await?.parse::<u32>()?;
                                        vec.push(value);

                                        xml_reader.end_node(&node).await?;
                                    }
                                }
                                TypeInfoData::Int32 => {
                                    let vec = unsafe { &mut *(target_ptr.0 as *mut Vec<i32>) };

                                    while let Some(node) = xml_reader.next_node().await? {
                                        if node.name == "array" {
                                            eprintln!("Arrays of arrays are not supported!");
                                            break;
                                        }

                                        let value = xml_reader.read_text().await?.parse::<i32>()?;
                                        vec.push(value);

                                        xml_reader.end_node(&node).await?;
                                    }
                                }
                                TypeInfoData::Uint64 => todo!(),
                                TypeInfoData::Int64 => todo!(),
                                TypeInfoData::Float32 => {
                                    let vec = unsafe { &mut *(target_ptr.0 as *mut Vec<f32>) };

                                    while let Some(node) = xml_reader.next_node().await? {
                                        if node.name == "array" {
                                            eprintln!("Arrays of arrays are not supported!");
                                            break;
                                        }

                                        let value = xml_reader.read_text().await?.parse::<f32>()?;
                                        vec.push(value);

                                        xml_reader.end_node(&node).await?;
                                    }
                                }
                                TypeInfoData::Float64 => todo!(),
                                TypeInfoData::Boolean => {
                                    let vec = unsafe { &mut *(target_ptr.0 as *mut Vec<bool>) };

                                    while let Some(node) = xml_reader.next_node().await? {
                                        if node.name == "array" {
                                            eprintln!("Arrays of arrays are not supported!");
                                            break;
                                        }

                                        let value = xml_reader.read_text().await?.parse::<bool>()?;
                                        vec.push(value);

                                        xml_reader.end_node(&node).await?;
                                    }
                                }
                                TypeInfoData::CString => {
                                    let vec = unsafe { &mut *(target_ptr.0 as *mut Vec<String>) };

                                    while let Some(node) = xml_reader.next_node().await? {
                                        if node.name == "array" {
                                            eprintln!("Arrays of arrays are not supported!");
                                            break;
                                        }

                                        let str = xml_reader.read_text().await?;
                                        vec.push(str);

                                        xml_reader.end_node(&node).await?;
                                    }
                                }
                                TypeInfoData::ResourceRef => todo!(),
                                TypeInfoData::TypeRef => todo!(),
                                TypeInfoData::FileRef => todo!(),
                                TypeInfoData::BoxedValueRef => todo!(),
                                TypeInfoData::SHA1 => todo!(),
                                TypeInfoData::Guid => {
                                    let vec = unsafe { &mut *(target_ptr.0 as *mut Vec<Guid>) };

                                    while let Some(node) = xml_reader.next_node().await? {
                                        if node.name == "array" {
                                            eprintln!("Arrays of arrays are not supported!");
                                            break;
                                        }

                                        let value = xml_reader.read_text().await?;
                                        let guid = Guid::from_str(&value);
                                        vec.push(guid);

                                        xml_reader.end_node(&node).await?;
                                    }
                                }
                                TypeInfoData::Array(_) => todo!(),
                                TypeInfoData::Class(_) => {
                                    let mut nodes = Vec::new();

                                    while let Some(node) = xml_reader.next_node().await? {
                                        if node.name == "array" {
                                            eprintln!("Arrays of arrays are not supported!");
                                            break;
                                        }
                                        xml_reader.end_node(&node).await?;
                                        nodes.push(node);
                                    }

                                    let vec = unsafe {
                                        &mut *(target_ptr.0 as *mut Vec<Option<LockedTypeObject>>)
                                    };
                                    vec.reserve_exact(nodes.len());

                                    // Allocate new vec element and get address
                                    for node in nodes {
                                        vec.push(None);

                                        let len = vec.len();
                                        let element: *mut u8 =
                                            &mut vec[len - 1] as *mut _ as *mut u8;

                                        self.marshal_reference(
                                            instance_guid,
                                            &node,
                                            SendPtr(element),
                                        )
                                        .await;
                                    }
                                }
                                TypeInfoData::ValueType(value_type_data) => {
                                    let vec = unsafe {
                                        &mut *(target_ptr.0 as *mut Vec<BoxedTypeObject>)
                                    };

                                    while let Some(node) = xml_reader.next_node().await? {
                                        if node.name == "array" {
                                            eprintln!("Arrays of arrays are not supported!");
                                            break;
                                        }

                                        let mut boxed = value_type_data.create_boxed();
                                        self.read_fields(
                                            array_type,
                                            instance_guid,
                                            xml_reader,
                                            &mut *boxed,
                                            0,
                                        )
                                        .await?;
                                        vec.push(boxed);

                                        xml_reader.end_node(&node).await?;
                                    }
                                }
                                TypeInfoData::Enum => {
                                    let vec = unsafe { &mut *(target_ptr.0 as *mut Vec<i64>) };

                                    while let Some(node) = xml_reader.next_node().await? {
                                        if node.name == "array" {
                                            eprintln!("Arrays of arrays are not supported!");
                                            break;
                                        }

                                        let value = xml_reader.read_text().await?.parse::<i64>()?;
                                        vec.push(value);

                                        xml_reader.end_node(&node).await?;
                                    }
                                }
                                TypeInfoData::Unknown => todo!(),
                            }
                        }
                        TypeInfoData::Class(_) => {
                            let target =
                                unsafe { &mut *(target_ptr.0 as *mut Option<LockedTypeObject>) }
                                    as *mut _ as *mut u8;
                            self.marshal_reference(instance_guid, &node, SendPtr(target))
                                .await;
                        }
                        TypeInfoData::ValueType(_) => {
                            self.read_fields(
                                field_type,
                                instance_guid,
                                xml_reader,
                                instance,
                                offset + field_info.rust_offset as isize,
                            )
                            .await?;
                        }
                        TypeInfoData::Enum => {
                            let value = xml_reader.read_text().await?.parse::<i64>()?;
                            unsafe {
                                *(target_ptr.0 as *mut i64) = value;
                            };
                        }
                        TypeInfoData::Unknown => todo!(),
                    }

                    xml_reader.end_node(&node).await?;
                } else {
                    eprintln!("Unexpected node: {:?}", node);
                    break;
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    async fn marshal_reference(
        &mut self,
        instance_guid: &Guid,
        node: &XmlNode,
        target: SendPtr<u8>,
    ) {
        let id = node.attributes.get("ref").expect("No ref attribute");
        if id == "null" {
            return;
        }

        let external = node.attributes.contains_key("partitionGuid");
        if external {
            if let Some(importer) = &mut self.importer {
                let partition_guid = Guid::from_str(
                    node.attributes
                        .get("partitionGuid")
                        .expect("No partitionGuid attribute"),
                );

                let guid = Guid::from_str(
                    id.split("/")
                        .last()
                        .expect("Malformed external partition reference"),
                );
                let instance = importer.load_partition(partition_guid, guid).await;

                if let Some(instance) = instance {
                    unsafe {
                        *(target.0 as *mut Option<LockedTypeObject>) = Some(instance);
                    }
                } else {
                    warn!("Failed to load external reference: {}", id);
                }
            }
        } else {
            let guid = Guid::from_str(id);

            if let Some(instance) = self.instances.get(&guid) {
                unsafe {
                    *(target.0 as *mut Option<LockedTypeObject>) = Some(instance.clone());
                }
            } else {
                self.deferred_references.push(DbxDeferredReference {
                    instance_guid: *instance_guid,
                    target_instance_guid: guid,
                    target,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use glacier_reflect::type_registry::TypeRegistry;
    use glacier_reflect_swbf2::register_mod_types;

    use super::*;

    #[tokio::test]
    async fn test_read_partition() {
        let data = include_bytes!("../../tests/data/Pathfinding_Schematic2.dbx");

        let mut registry = TypeRegistry::default();
        register_mod_types(&mut registry);

        let mut reader =
            DbxPartitionReader::new("DroidekaStateMachine".to_owned(), &registry, None);
        reader.read(Cursor::new(data)).await.unwrap();

        let partition = reader.finalize().await;
        let instance = partition.primary_instance().expect("No primary instance");
        dbg!(instance);
    }

    #[tokio::test]
    async fn test_read_partition_2() {
        let data = include_bytes!("../../tests/data/spacearcade_networkregistry_Win32.dbx");

        let mut registry = TypeRegistry::default();
        register_mod_types(&mut registry);

        let mut reader =
            DbxPartitionReader::new("DroidekaStateMachine".to_owned(), &registry, None);
        reader.read(Cursor::new(data)).await.unwrap();

        let partition = reader.finalize().await;
        let instance = partition.primary_instance().expect("No primary instance");
        dbg!(instance);
    }
}
