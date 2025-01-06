use std::sync::Arc;

use glacier_reflect::{
    type_info::{
        BoxedTypeObject, FieldInfoData, LockedTypeObject, TypeInfo, TypeInfoData, TypeObject,
    },
    type_registry::TypeRegistry,
};
use glacier_util::guid::Guid;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

use crate::{
    db::partition::DatabasePartition,
    util::trait_coercion::{data_ptr_from_trait_object, SendPtr},
};

struct XmlNode {
    name: String,
    newline: bool,
}

struct XmlWriter<W: AsyncWrite + Unpin> {
    writer: BufWriter<W>,
    stack: Vec<XmlNode>,
}

impl<W: AsyncWrite + Unpin> XmlWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: BufWriter::new(writer),
            stack: Vec::new(),
        }
    }

    pub async fn flush(&mut self) -> tokio::io::Result<()> {
        self.writer.flush().await
    }

    pub async fn write(&mut self, text: &str) -> tokio::io::Result<()> {
        self.writer.write(text.as_bytes()).await?;
        Ok(())
    }

    pub async fn write_open_tag(
        &mut self,
        tag: &str,
        attributes: Vec<(String, String)>,
        newline: bool,
        close: bool,
    ) -> tokio::io::Result<()> {
        self.write_indent().await?;

        self.writer.write("<".as_bytes()).await?;
        self.writer.write(tag.as_bytes()).await?;

        for (name, value) in attributes {
            self.writer.write(" ".as_bytes()).await?;
            self.writer.write(name.as_bytes()).await?;
            self.writer.write("=\"".as_bytes()).await?;
            self.writer.write(value.as_bytes()).await?;
            self.writer.write("\"".as_bytes()).await?;
        }

        if close {
            self.writer.write(" /".as_bytes()).await?;
        } else {
        }

        self.writer.write(">".as_bytes()).await?;

        let node = XmlNode {
            name: tag.to_string(),
            newline,
        };

        if newline {
            self.writer.write(b"\n").await?;
        }

        if !close {
            self.stack.push(node);
        }

        Ok(())
    }

    pub async fn write_close_tag(&mut self) -> tokio::io::Result<()> {
        let node = self.stack.pop().unwrap();

        if node.newline {
            self.write_indent().await?;
        }

        self.writer.write("</".as_bytes()).await?;
        self.writer.write(node.name.as_bytes()).await?;
        self.writer.write(">".as_bytes()).await?;
        self.writer.write(b"\n").await?;

        Ok(())
    }

    async fn write_indent(&mut self) -> tokio::io::Result<()> {
        for _ in 0..self.stack.len() {
            self.writer.write(b"    ").await?;
        }

        Ok(())
    }
}

#[async_trait::async_trait]
pub trait DbxWriterImportResolver: Sync {
    async fn resolve_import_name(&self, partition_guid: &Guid) -> Option<String>;
}

pub struct DbxPartitionWriter<'a, T: DbxWriterImportResolver> {
    partition: &'a DatabasePartition,
    type_registry: &'a TypeRegistry,
    import_resolver: &'a T,
}

impl<'a, T: DbxWriterImportResolver> DbxPartitionWriter<'a, T> {
    pub fn new(
        partition: &'a DatabasePartition,
        type_registry: &'a TypeRegistry,
        import_resolver: &'a T,
    ) -> Self {
        Self {
            partition,
            type_registry,
            import_resolver,
        }
    }

    pub async fn write<W: AsyncWrite + Unpin + Send>(self, mut writer: W) -> tokio::io::Result<()> {
        writer
            .write("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n".as_bytes())
            .await?;

        let mut xml_writer = XmlWriter::new(writer);

        let primary_instance_id = {
            if let Some(primary_instance) = self.partition.primary_instance() {
                let primary_instance = primary_instance.lock().await;
                let dc_core = primary_instance
                    .data_container_core()
                    .expect("Instance is not a DataContainer");
                dc_core.instance_guid.to_string()
            } else {
                "".to_owned()
            }
        };

        xml_writer
            .write_open_tag(
                "partition",
                vec![
                    ("guid".to_owned(), self.partition.guid().to_string()),
                    ("primaryInstance".to_owned(), primary_instance_id),
                    ("exportMode".to_owned(), "All".to_owned()),
                ],
                true,
                false,
            )
            .await?;

        for instance in self.partition.instances() {
            self.write_instance(&mut xml_writer, instance).await?;
        }

        xml_writer.write_close_tag().await?;
        xml_writer.flush().await?;
        Ok(())
    }

    async fn write_instance<W: AsyncWrite + Unpin + Send>(
        &self,
        writer: &mut XmlWriter<W>,
        instance: &LockedTypeObject,
    ) -> tokio::io::Result<()> {
        let new_instance = instance.clone();
        let mut instance = instance.lock().await;
        let dc_core = instance
            .data_container_core()
            .expect("Instance is not a DataContainer");

        let mut attributes = Vec::new();

        if !dc_core.id.is_empty() {
            attributes.push(("id".to_owned(), dc_core.id.to_owned()));
        }

        let type_info = instance.type_info();
        attributes.append(&mut vec![
            ("guid".to_owned(), dc_core.instance_guid.to_string()),
            ("type".to_owned(), type_info.full_name()),
        ]);

        if dc_core.exported {
            attributes.push(("exported".to_owned(), "True".to_owned()));
        }

        writer.write_open_tag("instance", attributes, true, false).await?;

        self.write_fields(writer, &new_instance, None, &mut *instance, 0, type_info)
            .await?;

        writer.write_close_tag().await?;
        Ok(())
    }

    #[async_recursion::async_recursion]
    async fn write_fields<W: AsyncWrite + Unpin + Send>(
        &self,
        writer: &mut XmlWriter<W>,
        container: &LockedTypeObject,
        mut container_instance: Option<&mut dyn TypeObject>,
        instance: &mut dyn TypeObject,
        offset: usize,
        type_info: &'static TypeInfo,
    ) -> tokio::io::Result<()> {
        if let TypeInfoData::Class(class_info) = &type_info.data {
            if let Some(super_class_info) = &class_info.super_class {
                self.write_fields(
                    writer,
                    container,
                    container_instance.as_deref_mut(),
                    instance,
                    offset,
                    super_class_info,
                )
                .await?;
            }

            for field in class_info.fields {
                self.write_field(
                    writer,
                    container,
                    container_instance.as_deref_mut(),
                    instance,
                    offset,
                    field,
                )
                .await?;
            }
        } else if let TypeInfoData::ValueType(value_type_info) = &type_info.data {
            for field in value_type_info.fields {
                self.write_field(
                    writer,
                    container,
                    container_instance.as_deref_mut(),
                    instance,
                    offset,
                    field,
                )
                .await?;
            }
        }

        Ok(())
    }

    async fn write_field<W: AsyncWrite + Unpin + Send>(
        &self,
        writer: &mut XmlWriter<W>,
        container: &LockedTypeObject,
        mut container_instance: Option<&mut dyn TypeObject>,
        instance: &mut dyn TypeObject,
        offset: usize,
        field_info: &'static FieldInfoData,
    ) -> tokio::io::Result<()> {
        macro_rules! open_tag {
            ($tag: tt, $newline: tt) => {
                writer
                    .write_open_tag(
                        $tag,
                        vec![("name".to_owned(), field_info.name.to_owned())],
                        $newline,
                        false,
                    )
                    .await?;
            };
        }

        macro_rules! open_tag_attr {
            ($tag: tt, $newline: tt, $attr: expr) => {
                writer.write_open_tag($tag, $attr, $newline, false).await?;
            };
        }

        macro_rules! tag_attr {
            ($tag: tt, $newline: tt, $attr: expr) => {
                writer.write_open_tag($tag, $attr, $newline, true).await?;
            };
        }

        let raw = data_ptr_from_trait_object(instance);
        let source_ptr = SendPtr(unsafe {
            raw.data_ptr
                .offset(offset as isize)
                .offset(field_info.rust_offset as isize)
        });

        let field_type_info = field_info.field_type(&self.type_registry);
        match &field_type_info.data {
            TypeInfoData::Uint8 => {
                open_tag!("field", false);
                let value = unsafe { *(source_ptr.0 as *const u8) };
                writer.write(&value.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Int8 => {
                open_tag!("field", false);
                let value = unsafe { *(source_ptr.0 as *const i8) };
                writer.write(&value.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Uint16 => {
                open_tag!("field", false);
                let value = unsafe { *(source_ptr.0 as *const u16) };
                writer.write(&value.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Int16 => {
                open_tag!("field", false);
                let value = unsafe { *(source_ptr.0 as *const i16) };
                writer.write(&value.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Uint32 => {
                open_tag!("field", false);
                let value = unsafe { *(source_ptr.0 as *const u32) };
                writer.write(&value.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Int32 => {
                open_tag!("field", false);
                let value = unsafe { *(source_ptr.0 as *const i32) };
                writer.write(&value.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Uint64 => {
                open_tag!("field", false);
                let value = unsafe { *(source_ptr.0 as *const u64) };
                writer.write(&value.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Int64 => {
                open_tag!("field", false);
                let value = unsafe { *(source_ptr.0 as *const i64) };
                writer.write(&value.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Float32 => {
                open_tag!("field", false);
                let value = unsafe { *(source_ptr.0 as *const f32) };
                writer.write(&value.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Float64 => todo!(),
            TypeInfoData::Boolean => {
                open_tag!("field", false);
                let value = unsafe { *(source_ptr.0 as *const bool) };
                writer.write(&value.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::CString => {
                open_tag!("field", false);
                let str = source_ptr.0 as *mut String;
                let str_ref: &str = unsafe { &*str.as_ref().unwrap() };
                writer.write(str_ref).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::ResourceRef => {
                open_tag!("field", false);
                writer.write("TODO").await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::TypeRef => {
                open_tag!("field", false);
                writer.write("TODO").await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::FileRef => {
                open_tag!("field", false);
                writer.write("TODO").await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::BoxedValueRef => {
                open_tag!("field", false);
                writer.write("TODO").await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::SHA1 => todo!(),
            TypeInfoData::Guid => {
                open_tag!("field", false);
                let guid = unsafe { *(source_ptr.0 as *const Guid) };
                writer.write(&guid.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Array(element_type_name) => {
                let element_type_info = self
                    .type_registry
                    .type_by_name(element_type_name)
                    .expect("Element type not found");

                match &element_type_info.data {
                    TypeInfoData::Uint8 => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<u8>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&value.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Int8 => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<i8>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&value.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Uint16 => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<u16>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&value.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Int16 => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<i16>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&value.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Uint32 => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<u32>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&value.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Int32 => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<i32>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&value.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Uint64 => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<u64>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&value.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Int64 => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<i64>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&value.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Float32 => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<f32>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&value.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Float64 => todo!(),
                    TypeInfoData::Boolean => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<bool>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&value.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::CString => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<String>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(value).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::ResourceRef => todo!(),
                    TypeInfoData::TypeRef => todo!(),
                    TypeInfoData::FileRef => todo!(),
                    TypeInfoData::BoxedValueRef => todo!(),
                    TypeInfoData::SHA1 => todo!(),
                    TypeInfoData::Guid => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<Guid>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for guid in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&guid.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Array(_) => todo!(),
                    TypeInfoData::Class(_) => {
                        let vec =
                            unsafe { &mut *(source_ptr.0 as *mut Vec<Option<LockedTypeObject>>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for item in vec.iter() {
                                let mut reference = self
                                    .collect_reference(
                                        container,
                                        if container_instance.is_some() {
                                            container_instance.as_deref_mut()
                                        } else {
                                            Some(instance)
                                        },
                                        item,
                                    )
                                    .await;
                                if reference.is_empty() {
                                    reference.push(("ref".to_owned(), "null".to_owned()));
                                }

                                // We should use tag_attr! here, but the reader doesn't support
                                // fields without explicit ending tags yet
                                
                                open_tag_attr!("item", false, reference);
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::ValueType(_) => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<BoxedTypeObject>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for item in vec.iter_mut() {
                                open_tag_attr!("complex", true, vec![]);
                                self.write_fields(
                                    writer,
                                    container,
                                    if container_instance.is_some() {
                                        container_instance.as_deref_mut()
                                    } else {
                                        Some(instance)
                                    },
                                    &mut **item,
                                    0,
                                    element_type_info,
                                )
                                .await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Enum => {
                        let vec = unsafe { &mut *(source_ptr.0 as *mut Vec<i64>) };
                        if !vec.is_empty() {
                            open_tag!("array", true);
                            for value in vec.iter() {
                                open_tag_attr!("item", false, vec![]);
                                writer.write(&value.to_string()).await?;
                                writer.write_close_tag().await?;
                            }
                            writer.write_close_tag().await?;
                        }
                    }
                    TypeInfoData::Unknown => todo!(),
                }
            }
            TypeInfoData::Class(_) => {
                let field_instance =
                    unsafe { &mut *(source_ptr.0 as *mut Option<LockedTypeObject>) };
                let mut reference = self
                    .collect_reference(
                        container,
                        if container_instance.is_some() {
                            container_instance.as_deref_mut()
                        } else {
                            Some(instance)
                        },
                        field_instance,
                    )
                    .await;
                if !reference.is_empty() {
                    let mut attr = vec![("name".to_owned(), field_info.name.to_owned())];
                    attr.append(&mut reference);

                    open_tag_attr!("field", false, attr);
                    writer.write_close_tag().await?;
                }
            }
            TypeInfoData::ValueType(_) => {
                open_tag!("complex", true);
                self.write_fields(
                    writer,
                    container,
                    container_instance,
                    instance,
                    offset + field_info.rust_offset,
                    &field_type_info,
                )
                .await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Enum => {
                open_tag!("field", false);
                let value = unsafe { *(source_ptr.0 as *const i64) };
                writer.write(&value.to_string()).await?;
                writer.write_close_tag().await?;
            }
            TypeInfoData::Unknown => todo!(),
        }

        Ok(())
    }

    async fn collect_reference(
        &self,
        container: &LockedTypeObject,
        container_instance: Option<&mut dyn TypeObject>,
        instance: &Option<LockedTypeObject>,
    ) -> Vec<(String, String)> {
        let instance = match instance {
            Some(instance) => instance,
            None => return Vec::new(),
        };

        let dc_core = if Arc::ptr_eq(container, instance) {
            if let Some(container) = container_instance {
                container
                    .data_container_core()
                    .expect("Instance is not a DataContainer")
                    .clone()
            } else {
                panic!("Top-level container instance not provided for reference collection");
            }
        } else {
            let instance = instance.lock().await;
            instance
                .data_container_core()
                .expect("Instance is not a DataContainer")
                .clone()
        };

        let partition_guid = dc_core.partition_guid;
        let instance_guid = dc_core.instance_guid.to_string();

        if &partition_guid == self.partition.guid() {
            vec![("ref".to_owned(), instance_guid)]
        } else {
            let partition_name = self
                .import_resolver
                .resolve_import_name(&partition_guid)
                .await
                .unwrap_or_else(|| "unknown".to_string());

            vec![
                (
                    "ref".to_owned(),
                    format!("{}/{}", partition_name, instance_guid),
                ),
                ("partitionGuid".to_owned(), partition_guid.to_string()),
            ]
        }
    }
}
