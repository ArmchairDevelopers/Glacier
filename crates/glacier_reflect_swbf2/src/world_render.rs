use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_world_render_types(registry: &mut TypeRegistry) {
    registry.register_type(MESHCOMPUTEBUFFERRESOURCE_TYPE_INFO);
    registry.register_type(MESHCOMPUTEBUFFERRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(MESHWORLDSETTINGS_TYPE_INFO);
    registry.register_type(MESHWORLDSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(DEBUGOVERLAYSETTINGS_TYPE_INFO);
    registry.register_type(DEBUGOVERLAYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUSIONSETTINGS_TYPE_INFO);
    registry.register_type(OCCLUSIONSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(EDGEMODELLIGHTMAPDATA_TYPE_INFO);
    registry.register_type(EDGEMODELLIGHTMAPDATA_ARRAY_TYPE_INFO);
    registry.register_type(EDGEMODELSDATA_TYPE_INFO);
    registry.register_type(EDGEMODELSDATA_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMEDATA_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMEASSET_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMEASSET_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMESETTINGS_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(IESRESOURCE_TYPE_INFO);
    registry.register_type(IESRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(OCCLUDERMESH_TYPE_INFO);
    registry.register_type(OCCLUDERMESH_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTESETTINGS_TYPE_INFO);
    registry.register_type(MESHCOMPUTESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTEMESHDEFINITIONRESOURCE_TYPE_INFO);
    registry.register_type(MESHCOMPUTEMESHDEFINITIONRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTEINDEXBUFFERRESOURCE_TYPE_INFO);
    registry.register_type(MESHCOMPUTEINDEXBUFFERRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTEFACEADJACENCYRESOURCE_TYPE_INFO);
    registry.register_type(MESHCOMPUTEFACEADJACENCYRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(MESHCOMPUTEDYNAMICMORPHTARGETSRESOURCE_TYPE_INFO);
    registry.register_type(MESHCOMPUTEDYNAMICMORPHTARGETSRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(VISIBLEAREAOBJECT_TYPE_INFO);
    registry.register_type(VISIBLEAREAOBJECT_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENPROBESET_TYPE_INFO);
    registry.register_type(ENLIGHTENPROBESET_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENSTATICDATABASE_TYPE_INFO);
    registry.register_type(ENLIGHTENSTATICDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENSHADERDATABASERESOURCE_TYPE_INFO);
    registry.register_type(ENLIGHTENSHADERDATABASERESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENDATABASE_TYPE_INFO);
    registry.register_type(ENLIGHTENDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(RENDEROBJECT_TYPE_INFO);
    registry.register_type(RENDEROBJECT_ARRAY_TYPE_INFO);
    registry.register_type(MODELWITHFALLBACKRENDEROBJECT_TYPE_INFO);
    registry.register_type(MODELWITHFALLBACKRENDEROBJECT_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENSTATE_TYPE_INFO);
    registry.register_type(ENLIGHTENSTATE_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENRUNTIMECONFIG_TYPE_INFO);
    registry.register_type(ENLIGHTENRUNTIMECONFIG_ARRAY_TYPE_INFO);
    registry.register_type(ENLIGHTENRUNTIMESETTINGS_TYPE_INFO);
    registry.register_type(ENLIGHTENRUNTIMESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(WORLDRENDERSETTINGS_TYPE_INFO);
    registry.register_type(WORLDRENDERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(WORLDRENDERSETTINGSBASE_TYPE_INFO);
    registry.register_type(WORLDRENDERSETTINGSBASE_ARRAY_TYPE_INFO);
    registry.register_type(LIGHTTILEDEBUGLIGHTCOUNTMODE_TYPE_INFO);
    registry.register_type(LIGHTTILEDEBUGLIGHTCOUNTMODE_ARRAY_TYPE_INFO);
    registry.register_type(FRAMESYNTHESISMODE_TYPE_INFO);
    registry.register_type(FRAMESYNTHESISMODE_ARRAY_TYPE_INFO);
    registry.register_type(SKYRENDERMODE_TYPE_INFO);
    registry.register_type(SKYRENDERMODE_ARRAY_TYPE_INFO);
    registry.register_type(POSTPROCESSDOFMODE_TYPE_INFO);
    registry.register_type(POSTPROCESSDOFMODE_ARRAY_TYPE_INFO);
    registry.register_type(POSTPROCESSAAMODE_TYPE_INFO);
    registry.register_type(POSTPROCESSAAMODE_ARRAY_TYPE_INFO);
    registry.register_type(MIPMAPFILTERMODE_TYPE_INFO);
    registry.register_type(MIPMAPFILTERMODE_ARRAY_TYPE_INFO);
    registry.register_type(LUMINANCEPRESET_TYPE_INFO);
    registry.register_type(LUMINANCEPRESET_ARRAY_TYPE_INFO);
    registry.register_type(ILLUMINANCEPRESET_TYPE_INFO);
    registry.register_type(ILLUMINANCEPRESET_ARRAY_TYPE_INFO);
    registry.register_type(WORLDVIEWMODE_TYPE_INFO);
    registry.register_type(WORLDVIEWMODE_ARRAY_TYPE_INFO);
    registry.register_type(MODELRENDEROBJECT_TYPE_INFO);
    registry.register_type(MODELRENDEROBJECT_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct MeshComputeBufferResource {
}

pub trait MeshComputeBufferResourceTrait: TypeObject {
}

impl MeshComputeBufferResourceTrait for MeshComputeBufferResource {
}

pub static MESHCOMPUTEBUFFERRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeBufferResource",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshComputeBufferResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHCOMPUTEBUFFERRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshComputeBufferResource {
    fn type_info(&self) -> &'static TypeInfo {
        MESHCOMPUTEBUFFERRESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHCOMPUTEBUFFERRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeBufferResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("MeshComputeBufferResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshWorldSettings {
    pub _glacier_base: super::core::DataContainer,
    pub dummy: bool,
}

pub trait MeshWorldSettingsTrait: super::core::DataContainerTrait {
    fn dummy(&self) -> &bool;
}

impl MeshWorldSettingsTrait for MeshWorldSettings {
    fn dummy(&self) -> &bool {
        &self.dummy
    }
}

impl super::core::DataContainerTrait for MeshWorldSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MESHWORLDSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshWorldSettings",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshWorldSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Dummy",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshWorldSettings, dummy),
            },
        ],
    }),
    array_type: Some(MESHWORLDSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshWorldSettings {
    fn type_info(&self) -> &'static TypeInfo {
        MESHWORLDSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHWORLDSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshWorldSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("MeshWorldSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DebugOverlaySettings {
    pub _glacier_base: super::core::DataContainer,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub debug_info_enable: bool,
}

pub trait DebugOverlaySettingsTrait: super::core::DataContainerTrait {
    fn mouse_x(&self) -> &f32;
    fn mouse_y(&self) -> &f32;
    fn debug_info_enable(&self) -> &bool;
}

impl DebugOverlaySettingsTrait for DebugOverlaySettings {
    fn mouse_x(&self) -> &f32 {
        &self.mouse_x
    }
    fn mouse_y(&self) -> &f32 {
        &self.mouse_y
    }
    fn debug_info_enable(&self) -> &bool {
        &self.debug_info_enable
    }
}

impl super::core::DataContainerTrait for DebugOverlaySettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DEBUGOVERLAYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugOverlaySettings",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DebugOverlaySettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MouseX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebugOverlaySettings, mouse_x),
            },
            FieldInfoData {
                name: "MouseY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DebugOverlaySettings, mouse_y),
            },
            FieldInfoData {
                name: "DebugInfoEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DebugOverlaySettings, debug_info_enable),
            },
        ],
    }),
    array_type: Some(DEBUGOVERLAYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DebugOverlaySettings {
    fn type_info(&self) -> &'static TypeInfo {
        DEBUGOVERLAYSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEBUGOVERLAYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DebugOverlaySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("DebugOverlaySettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OcclusionSettings {
    pub _glacier_base: super::core::DataContainer,
    pub enable: bool,
    pub coverage_enable: bool,
    pub draw_split_view: bool,
    pub draw_binary_buffer: bool,
    pub draw_coverage_buffer: bool,
    pub global_cull_screen_area: f32,
    pub normal_view_distance: f32,
    pub terrain_view_distance: f32,
    pub high_priority_view_distance: f32,
}

pub trait OcclusionSettingsTrait: super::core::DataContainerTrait {
    fn enable(&self) -> &bool;
    fn coverage_enable(&self) -> &bool;
    fn draw_split_view(&self) -> &bool;
    fn draw_binary_buffer(&self) -> &bool;
    fn draw_coverage_buffer(&self) -> &bool;
    fn global_cull_screen_area(&self) -> &f32;
    fn normal_view_distance(&self) -> &f32;
    fn terrain_view_distance(&self) -> &f32;
    fn high_priority_view_distance(&self) -> &f32;
}

impl OcclusionSettingsTrait for OcclusionSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn coverage_enable(&self) -> &bool {
        &self.coverage_enable
    }
    fn draw_split_view(&self) -> &bool {
        &self.draw_split_view
    }
    fn draw_binary_buffer(&self) -> &bool {
        &self.draw_binary_buffer
    }
    fn draw_coverage_buffer(&self) -> &bool {
        &self.draw_coverage_buffer
    }
    fn global_cull_screen_area(&self) -> &f32 {
        &self.global_cull_screen_area
    }
    fn normal_view_distance(&self) -> &f32 {
        &self.normal_view_distance
    }
    fn terrain_view_distance(&self) -> &f32 {
        &self.terrain_view_distance
    }
    fn high_priority_view_distance(&self) -> &f32 {
        &self.high_priority_view_distance
    }
}

impl super::core::DataContainerTrait for OcclusionSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static OCCLUSIONSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OcclusionSettings",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OcclusionSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OcclusionSettings, enable),
            },
            FieldInfoData {
                name: "CoverageEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OcclusionSettings, coverage_enable),
            },
            FieldInfoData {
                name: "DrawSplitView",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OcclusionSettings, draw_split_view),
            },
            FieldInfoData {
                name: "DrawBinaryBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OcclusionSettings, draw_binary_buffer),
            },
            FieldInfoData {
                name: "DrawCoverageBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OcclusionSettings, draw_coverage_buffer),
            },
            FieldInfoData {
                name: "GlobalCullScreenArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OcclusionSettings, global_cull_screen_area),
            },
            FieldInfoData {
                name: "NormalViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OcclusionSettings, normal_view_distance),
            },
            FieldInfoData {
                name: "TerrainViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OcclusionSettings, terrain_view_distance),
            },
            FieldInfoData {
                name: "HighPriorityViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(OcclusionSettings, high_priority_view_distance),
            },
        ],
    }),
    array_type: Some(OCCLUSIONSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OcclusionSettings {
    fn type_info(&self) -> &'static TypeInfo {
        OCCLUSIONSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OCCLUSIONSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OcclusionSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("OcclusionSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EdgeModelLightMapData {
    pub _glacier_base: EdgeModelsData,
}

pub trait EdgeModelLightMapDataTrait: EdgeModelsDataTrait {
}

impl EdgeModelLightMapDataTrait for EdgeModelLightMapData {
}

impl EdgeModelsDataTrait for EdgeModelLightMapData {
    fn rigid_meshes(&self) -> &Vec<Option<Arc<Mutex<dyn super::render::RigidMeshAssetTrait>>>> {
        self._glacier_base.rigid_meshes()
    }
    fn light_map_uvs(&self) -> &Vec<super::core::Vec4> {
        self._glacier_base.light_map_uvs()
    }
}

impl super::world_base::EdgeModelsBaseDataTrait for EdgeModelLightMapData {
    fn instance_transforms(&self) -> &Vec<super::core::LinearTransform> {
        self._glacier_base.instance_transforms()
    }
    fn mesh_instance_ranges(&self) -> &Vec<u16> {
        self._glacier_base.mesh_instance_ranges()
    }
    fn connection_instance_lookup_table(&self) -> &Vec<u16> {
        self._glacier_base.connection_instance_lookup_table()
    }
    fn connections(&self) -> &Vec<super::world_base::EdgeModelConnectionInfo> {
        self._glacier_base.connections()
    }
    fn part_connection_ranges(&self) -> &Vec<u16> {
        self._glacier_base.part_connection_ranges()
    }
}

impl super::core::DataContainerTrait for EdgeModelLightMapData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EDGEMODELLIGHTMAPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelLightMapData",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EDGEMODELSDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EdgeModelLightMapData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EDGEMODELLIGHTMAPDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EdgeModelLightMapData {
    fn type_info(&self) -> &'static TypeInfo {
        EDGEMODELLIGHTMAPDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EDGEMODELLIGHTMAPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelLightMapData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("EdgeModelLightMapData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EdgeModelsData {
    pub _glacier_base: super::world_base::EdgeModelsBaseData,
    pub rigid_meshes: Vec<Option<Arc<Mutex<dyn super::render::RigidMeshAssetTrait>>>>,
    pub light_map_uvs: Vec<super::core::Vec4>,
}

pub trait EdgeModelsDataTrait: super::world_base::EdgeModelsBaseDataTrait {
    fn rigid_meshes(&self) -> &Vec<Option<Arc<Mutex<dyn super::render::RigidMeshAssetTrait>>>>;
    fn light_map_uvs(&self) -> &Vec<super::core::Vec4>;
}

impl EdgeModelsDataTrait for EdgeModelsData {
    fn rigid_meshes(&self) -> &Vec<Option<Arc<Mutex<dyn super::render::RigidMeshAssetTrait>>>> {
        &self.rigid_meshes
    }
    fn light_map_uvs(&self) -> &Vec<super::core::Vec4> {
        &self.light_map_uvs
    }
}

impl super::world_base::EdgeModelsBaseDataTrait for EdgeModelsData {
    fn instance_transforms(&self) -> &Vec<super::core::LinearTransform> {
        self._glacier_base.instance_transforms()
    }
    fn mesh_instance_ranges(&self) -> &Vec<u16> {
        self._glacier_base.mesh_instance_ranges()
    }
    fn connection_instance_lookup_table(&self) -> &Vec<u16> {
        self._glacier_base.connection_instance_lookup_table()
    }
    fn connections(&self) -> &Vec<super::world_base::EdgeModelConnectionInfo> {
        self._glacier_base.connections()
    }
    fn part_connection_ranges(&self) -> &Vec<u16> {
        self._glacier_base.part_connection_ranges()
    }
}

impl super::core::DataContainerTrait for EdgeModelsData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static EDGEMODELSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelsData",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::world_base::EDGEMODELSBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EdgeModelsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RigidMeshes",
                flags: MemberInfoFlags::new(144),
                field_type: "RigidMeshAsset-Array",
                rust_offset: offset_of!(EdgeModelsData, rigid_meshes),
            },
            FieldInfoData {
                name: "LightMapUvs",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec4-Array",
                rust_offset: offset_of!(EdgeModelsData, light_map_uvs),
            },
        ],
    }),
    array_type: Some(EDGEMODELSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EdgeModelsData {
    fn type_info(&self) -> &'static TypeInfo {
        EDGEMODELSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EDGEMODELSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("EdgeModelsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionVolumeData {
    pub _glacier_base: super::world_base::DestructionVolumeBaseData,
    pub asset: Option<Arc<Mutex<dyn DestructionVolumeAssetTrait>>>,
    pub bounding_box: super::core::AxisAlignedBox,
    pub impacts: Vec<super::core::Vec4>,
    pub part_to_impact_indices: Vec<u32>,
}

pub trait DestructionVolumeDataTrait: super::world_base::DestructionVolumeBaseDataTrait {
    fn asset(&self) -> &Option<Arc<Mutex<dyn DestructionVolumeAssetTrait>>>;
    fn bounding_box(&self) -> &super::core::AxisAlignedBox;
    fn impacts(&self) -> &Vec<super::core::Vec4>;
    fn part_to_impact_indices(&self) -> &Vec<u32>;
}

impl DestructionVolumeDataTrait for DestructionVolumeData {
    fn asset(&self) -> &Option<Arc<Mutex<dyn DestructionVolumeAssetTrait>>> {
        &self.asset
    }
    fn bounding_box(&self) -> &super::core::AxisAlignedBox {
        &self.bounding_box
    }
    fn impacts(&self) -> &Vec<super::core::Vec4> {
        &self.impacts
    }
    fn part_to_impact_indices(&self) -> &Vec<u32> {
        &self.part_to_impact_indices
    }
}

impl super::world_base::DestructionVolumeBaseDataTrait for DestructionVolumeData {
}

impl super::core::DataContainerTrait for DestructionVolumeData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DESTRUCTIONVOLUMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeData",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::world_base::DESTRUCTIONVOLUMEBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionVolumeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: "DestructionVolumeAsset",
                rust_offset: offset_of!(DestructionVolumeData, asset),
            },
            FieldInfoData {
                name: "BoundingBox",
                flags: MemberInfoFlags::new(0),
                field_type: "AxisAlignedBox",
                rust_offset: offset_of!(DestructionVolumeData, bounding_box),
            },
            FieldInfoData {
                name: "Impacts",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec4-Array",
                rust_offset: offset_of!(DestructionVolumeData, impacts),
            },
            FieldInfoData {
                name: "PartToImpactIndices",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(DestructionVolumeData, part_to_impact_indices),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONVOLUMEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DestructionVolumeData {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONVOLUMEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONVOLUMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("DestructionVolumeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionVolumeAsset {
    pub _glacier_base: super::world_base::DestructionVolumeBaseAsset,
    pub diffuse_atlas: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub normalmap_atlas: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
}

pub trait DestructionVolumeAssetTrait: super::world_base::DestructionVolumeBaseAssetTrait {
    fn diffuse_atlas(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn normalmap_atlas(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
}

impl DestructionVolumeAssetTrait for DestructionVolumeAsset {
    fn diffuse_atlas(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.diffuse_atlas
    }
    fn normalmap_atlas(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.normalmap_atlas
    }
}

impl super::world_base::DestructionVolumeBaseAssetTrait for DestructionVolumeAsset {
}

impl super::core::AssetTrait for DestructionVolumeAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for DestructionVolumeAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DESTRUCTIONVOLUMEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeAsset",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::world_base::DESTRUCTIONVOLUMEBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionVolumeAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DiffuseAtlas",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(DestructionVolumeAsset, diffuse_atlas),
            },
            FieldInfoData {
                name: "NormalmapAtlas",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(DestructionVolumeAsset, normalmap_atlas),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONVOLUMEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionVolumeAsset {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONVOLUMEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONVOLUMEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("DestructionVolumeAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DestructionVolumeSettings {
    pub _glacier_base: super::core::DataContainer,
    pub pixels_per_meter: f32,
    pub distance_field_multiplier: f32,
    pub texture_pool_size: i32,
    pub texture_pool_headroom_size: i32,
    pub texture_pool_defrag_transfer_limit: i32,
    pub jobs_enable: bool,
    pub force_update_enable: bool,
    pub draw_debug_volumes: bool,
    pub draw_debug_impacts: bool,
    pub draw_debug_texture_pool: bool,
}

pub trait DestructionVolumeSettingsTrait: super::core::DataContainerTrait {
    fn pixels_per_meter(&self) -> &f32;
    fn distance_field_multiplier(&self) -> &f32;
    fn texture_pool_size(&self) -> &i32;
    fn texture_pool_headroom_size(&self) -> &i32;
    fn texture_pool_defrag_transfer_limit(&self) -> &i32;
    fn jobs_enable(&self) -> &bool;
    fn force_update_enable(&self) -> &bool;
    fn draw_debug_volumes(&self) -> &bool;
    fn draw_debug_impacts(&self) -> &bool;
    fn draw_debug_texture_pool(&self) -> &bool;
}

impl DestructionVolumeSettingsTrait for DestructionVolumeSettings {
    fn pixels_per_meter(&self) -> &f32 {
        &self.pixels_per_meter
    }
    fn distance_field_multiplier(&self) -> &f32 {
        &self.distance_field_multiplier
    }
    fn texture_pool_size(&self) -> &i32 {
        &self.texture_pool_size
    }
    fn texture_pool_headroom_size(&self) -> &i32 {
        &self.texture_pool_headroom_size
    }
    fn texture_pool_defrag_transfer_limit(&self) -> &i32 {
        &self.texture_pool_defrag_transfer_limit
    }
    fn jobs_enable(&self) -> &bool {
        &self.jobs_enable
    }
    fn force_update_enable(&self) -> &bool {
        &self.force_update_enable
    }
    fn draw_debug_volumes(&self) -> &bool {
        &self.draw_debug_volumes
    }
    fn draw_debug_impacts(&self) -> &bool {
        &self.draw_debug_impacts
    }
    fn draw_debug_texture_pool(&self) -> &bool {
        &self.draw_debug_texture_pool
    }
}

impl super::core::DataContainerTrait for DestructionVolumeSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DESTRUCTIONVOLUMESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeSettings",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionVolumeSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PixelsPerMeter",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DestructionVolumeSettings, pixels_per_meter),
            },
            FieldInfoData {
                name: "DistanceFieldMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DestructionVolumeSettings, distance_field_multiplier),
            },
            FieldInfoData {
                name: "TexturePoolSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DestructionVolumeSettings, texture_pool_size),
            },
            FieldInfoData {
                name: "TexturePoolHeadroomSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DestructionVolumeSettings, texture_pool_headroom_size),
            },
            FieldInfoData {
                name: "TexturePoolDefragTransferLimit",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(DestructionVolumeSettings, texture_pool_defrag_transfer_limit),
            },
            FieldInfoData {
                name: "JobsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DestructionVolumeSettings, jobs_enable),
            },
            FieldInfoData {
                name: "ForceUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DestructionVolumeSettings, force_update_enable),
            },
            FieldInfoData {
                name: "DrawDebugVolumes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DestructionVolumeSettings, draw_debug_volumes),
            },
            FieldInfoData {
                name: "DrawDebugImpacts",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DestructionVolumeSettings, draw_debug_impacts),
            },
            FieldInfoData {
                name: "DrawDebugTexturePool",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DestructionVolumeSettings, draw_debug_texture_pool),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONVOLUMESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DestructionVolumeSettings {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONVOLUMESETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DESTRUCTIONVOLUMESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("DestructionVolumeSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IesResource {
}

pub trait IesResourceTrait: TypeObject {
}

impl IesResourceTrait for IesResource {
}

pub static IESRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IesResource",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IesResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IESRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IesResource {
    fn type_info(&self) -> &'static TypeInfo {
        IESRESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IESRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IesResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("IesResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct OccluderMesh {
}

pub trait OccluderMeshTrait: TypeObject {
}

impl OccluderMeshTrait for OccluderMesh {
}

pub static OCCLUDERMESH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMesh",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OccluderMesh as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(OCCLUDERMESH_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for OccluderMesh {
    fn type_info(&self) -> &'static TypeInfo {
        OCCLUDERMESH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OCCLUDERMESH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OccluderMesh-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("OccluderMesh"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshComputeSettings {
    pub _glacier_base: super::core::DataContainer,
    pub transient_buffer_size: u32,
    pub output_buffer_max_vertex_count: u32,
    pub dispatch_scheduler: i32,
    pub draw_enabled: bool,
    pub draw_debug_enabled: bool,
}

pub trait MeshComputeSettingsTrait: super::core::DataContainerTrait {
    fn transient_buffer_size(&self) -> &u32;
    fn output_buffer_max_vertex_count(&self) -> &u32;
    fn dispatch_scheduler(&self) -> &i32;
    fn draw_enabled(&self) -> &bool;
    fn draw_debug_enabled(&self) -> &bool;
}

impl MeshComputeSettingsTrait for MeshComputeSettings {
    fn transient_buffer_size(&self) -> &u32 {
        &self.transient_buffer_size
    }
    fn output_buffer_max_vertex_count(&self) -> &u32 {
        &self.output_buffer_max_vertex_count
    }
    fn dispatch_scheduler(&self) -> &i32 {
        &self.dispatch_scheduler
    }
    fn draw_enabled(&self) -> &bool {
        &self.draw_enabled
    }
    fn draw_debug_enabled(&self) -> &bool {
        &self.draw_debug_enabled
    }
}

impl super::core::DataContainerTrait for MeshComputeSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MESHCOMPUTESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeSettings",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshComputeSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransientBufferSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MeshComputeSettings, transient_buffer_size),
            },
            FieldInfoData {
                name: "OutputBufferMaxVertexCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MeshComputeSettings, output_buffer_max_vertex_count),
            },
            FieldInfoData {
                name: "DispatchScheduler",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MeshComputeSettings, dispatch_scheduler),
            },
            FieldInfoData {
                name: "DrawEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshComputeSettings, draw_enabled),
            },
            FieldInfoData {
                name: "DrawDebugEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MeshComputeSettings, draw_debug_enabled),
            },
        ],
    }),
    array_type: Some(MESHCOMPUTESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MeshComputeSettings {
    fn type_info(&self) -> &'static TypeInfo {
        MESHCOMPUTESETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHCOMPUTESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("MeshComputeSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshComputeMeshDefinitionResource {
    pub _glacier_base: MeshComputeBufferResource,
}

pub trait MeshComputeMeshDefinitionResourceTrait: MeshComputeBufferResourceTrait {
}

impl MeshComputeMeshDefinitionResourceTrait for MeshComputeMeshDefinitionResource {
}

impl MeshComputeBufferResourceTrait for MeshComputeMeshDefinitionResource {
}

pub static MESHCOMPUTEMESHDEFINITIONRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeMeshDefinitionResource",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHCOMPUTEBUFFERRESOURCE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshComputeMeshDefinitionResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHCOMPUTEMESHDEFINITIONRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshComputeMeshDefinitionResource {
    fn type_info(&self) -> &'static TypeInfo {
        MESHCOMPUTEMESHDEFINITIONRESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHCOMPUTEMESHDEFINITIONRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeMeshDefinitionResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("MeshComputeMeshDefinitionResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshComputeIndexBufferResource {
    pub _glacier_base: MeshComputeBufferResource,
}

pub trait MeshComputeIndexBufferResourceTrait: MeshComputeBufferResourceTrait {
}

impl MeshComputeIndexBufferResourceTrait for MeshComputeIndexBufferResource {
}

impl MeshComputeBufferResourceTrait for MeshComputeIndexBufferResource {
}

pub static MESHCOMPUTEINDEXBUFFERRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeIndexBufferResource",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHCOMPUTEBUFFERRESOURCE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshComputeIndexBufferResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHCOMPUTEINDEXBUFFERRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshComputeIndexBufferResource {
    fn type_info(&self) -> &'static TypeInfo {
        MESHCOMPUTEINDEXBUFFERRESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHCOMPUTEINDEXBUFFERRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeIndexBufferResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("MeshComputeIndexBufferResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshComputeFaceAdjacencyResource {
    pub _glacier_base: MeshComputeBufferResource,
}

pub trait MeshComputeFaceAdjacencyResourceTrait: MeshComputeBufferResourceTrait {
}

impl MeshComputeFaceAdjacencyResourceTrait for MeshComputeFaceAdjacencyResource {
}

impl MeshComputeBufferResourceTrait for MeshComputeFaceAdjacencyResource {
}

pub static MESHCOMPUTEFACEADJACENCYRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeFaceAdjacencyResource",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHCOMPUTEBUFFERRESOURCE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshComputeFaceAdjacencyResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHCOMPUTEFACEADJACENCYRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshComputeFaceAdjacencyResource {
    fn type_info(&self) -> &'static TypeInfo {
        MESHCOMPUTEFACEADJACENCYRESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHCOMPUTEFACEADJACENCYRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeFaceAdjacencyResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("MeshComputeFaceAdjacencyResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MeshComputeDynamicMorphTargetsResource {
    pub _glacier_base: MeshComputeBufferResource,
}

pub trait MeshComputeDynamicMorphTargetsResourceTrait: MeshComputeBufferResourceTrait {
}

impl MeshComputeDynamicMorphTargetsResourceTrait for MeshComputeDynamicMorphTargetsResource {
}

impl MeshComputeBufferResourceTrait for MeshComputeDynamicMorphTargetsResource {
}

pub static MESHCOMPUTEDYNAMICMORPHTARGETSRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeDynamicMorphTargetsResource",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MESHCOMPUTEBUFFERRESOURCE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MeshComputeDynamicMorphTargetsResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MESHCOMPUTEDYNAMICMORPHTARGETSRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MeshComputeDynamicMorphTargetsResource {
    fn type_info(&self) -> &'static TypeInfo {
        MESHCOMPUTEDYNAMICMORPHTARGETSRESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MESHCOMPUTEDYNAMICMORPHTARGETSRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MeshComputeDynamicMorphTargetsResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("MeshComputeDynamicMorphTargetsResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VisibleAreaObject {
    pub _glacier_base: RenderObject,
}

pub trait VisibleAreaObjectTrait: RenderObjectTrait {
}

impl VisibleAreaObjectTrait for VisibleAreaObject {
}

impl RenderObjectTrait for VisibleAreaObject {
}

pub static VISIBLEAREAOBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaObject",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RENDEROBJECT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VisibleAreaObject as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VISIBLEAREAOBJECT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for VisibleAreaObject {
    fn type_info(&self) -> &'static TypeInfo {
        VISIBLEAREAOBJECT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VISIBLEAREAOBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VisibleAreaObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("VisibleAreaObject"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnlightenProbeSet {
}

pub trait EnlightenProbeSetTrait: TypeObject {
}

impl EnlightenProbeSetTrait for EnlightenProbeSet {
}

pub static ENLIGHTENPROBESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenProbeSet",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnlightenProbeSet as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENLIGHTENPROBESET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnlightenProbeSet {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENPROBESET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENLIGHTENPROBESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenProbeSet-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("EnlightenProbeSet"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnlightenStaticDatabase {
}

pub trait EnlightenStaticDatabaseTrait: TypeObject {
}

impl EnlightenStaticDatabaseTrait for EnlightenStaticDatabase {
}

pub static ENLIGHTENSTATICDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenStaticDatabase",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnlightenStaticDatabase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENLIGHTENSTATICDATABASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnlightenStaticDatabase {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENSTATICDATABASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENLIGHTENSTATICDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenStaticDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("EnlightenStaticDatabase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnlightenShaderDatabaseResource {
}

pub trait EnlightenShaderDatabaseResourceTrait: TypeObject {
}

impl EnlightenShaderDatabaseResourceTrait for EnlightenShaderDatabaseResource {
}

pub static ENLIGHTENSHADERDATABASERESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenShaderDatabaseResource",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnlightenShaderDatabaseResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENLIGHTENSHADERDATABASERESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnlightenShaderDatabaseResource {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENSHADERDATABASERESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENLIGHTENSHADERDATABASERESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenShaderDatabaseResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("EnlightenShaderDatabaseResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnlightenDatabase {
}

pub trait EnlightenDatabaseTrait: TypeObject {
}

impl EnlightenDatabaseTrait for EnlightenDatabase {
}

pub static ENLIGHTENDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenDatabase",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnlightenDatabase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENLIGHTENDATABASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EnlightenDatabase {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENDATABASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENLIGHTENDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("EnlightenDatabase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RenderObject {
}

pub trait RenderObjectTrait: TypeObject {
}

impl RenderObjectTrait for RenderObject {
}

pub static RENDEROBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderObject",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RenderObject as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RENDEROBJECT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RenderObject {
    fn type_info(&self) -> &'static TypeInfo {
        RENDEROBJECT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static RENDEROBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RenderObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("RenderObject"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ModelWithFallbackRenderObject {
    pub _glacier_base: ModelRenderObject,
}

pub trait ModelWithFallbackRenderObjectTrait: ModelRenderObjectTrait {
}

impl ModelWithFallbackRenderObjectTrait for ModelWithFallbackRenderObject {
}

impl ModelRenderObjectTrait for ModelWithFallbackRenderObject {
}

impl RenderObjectTrait for ModelWithFallbackRenderObject {
}

pub static MODELWITHFALLBACKRENDEROBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelWithFallbackRenderObject",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MODELRENDEROBJECT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ModelWithFallbackRenderObject as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MODELWITHFALLBACKRENDEROBJECT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ModelWithFallbackRenderObject {
    fn type_info(&self) -> &'static TypeInfo {
        MODELWITHFALLBACKRENDEROBJECT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MODELWITHFALLBACKRENDEROBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelWithFallbackRenderObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("ModelWithFallbackRenderObject"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnlightenState {
    pub _glacier_base: super::core::DataContainer,
    pub bounce_scale: f32,
    pub sun_scale: f32,
    pub output_scale: f32,
    pub sky_box_enable: bool,
    pub sky_box_sky_color: super::core::Vec3,
    pub sky_box_ground_color: super::core::Vec3,
    pub sky_box_sun_light_color: super::core::Vec3,
    pub sky_box_sun_light_color_size: f32,
    pub sky_box_back_light_color: super::core::Vec3,
    pub sky_box_back_light_color_size: f32,
    pub sky_box_back_light_rotation_x: f32,
    pub sky_box_back_light_rotation_y: f32,
}

pub trait EnlightenStateTrait: super::core::DataContainerTrait {
    fn bounce_scale(&self) -> &f32;
    fn sun_scale(&self) -> &f32;
    fn output_scale(&self) -> &f32;
    fn sky_box_enable(&self) -> &bool;
    fn sky_box_sky_color(&self) -> &super::core::Vec3;
    fn sky_box_ground_color(&self) -> &super::core::Vec3;
    fn sky_box_sun_light_color(&self) -> &super::core::Vec3;
    fn sky_box_sun_light_color_size(&self) -> &f32;
    fn sky_box_back_light_color(&self) -> &super::core::Vec3;
    fn sky_box_back_light_color_size(&self) -> &f32;
    fn sky_box_back_light_rotation_x(&self) -> &f32;
    fn sky_box_back_light_rotation_y(&self) -> &f32;
}

impl EnlightenStateTrait for EnlightenState {
    fn bounce_scale(&self) -> &f32 {
        &self.bounce_scale
    }
    fn sun_scale(&self) -> &f32 {
        &self.sun_scale
    }
    fn output_scale(&self) -> &f32 {
        &self.output_scale
    }
    fn sky_box_enable(&self) -> &bool {
        &self.sky_box_enable
    }
    fn sky_box_sky_color(&self) -> &super::core::Vec3 {
        &self.sky_box_sky_color
    }
    fn sky_box_ground_color(&self) -> &super::core::Vec3 {
        &self.sky_box_ground_color
    }
    fn sky_box_sun_light_color(&self) -> &super::core::Vec3 {
        &self.sky_box_sun_light_color
    }
    fn sky_box_sun_light_color_size(&self) -> &f32 {
        &self.sky_box_sun_light_color_size
    }
    fn sky_box_back_light_color(&self) -> &super::core::Vec3 {
        &self.sky_box_back_light_color
    }
    fn sky_box_back_light_color_size(&self) -> &f32 {
        &self.sky_box_back_light_color_size
    }
    fn sky_box_back_light_rotation_x(&self) -> &f32 {
        &self.sky_box_back_light_rotation_x
    }
    fn sky_box_back_light_rotation_y(&self) -> &f32 {
        &self.sky_box_back_light_rotation_y
    }
}

impl super::core::DataContainerTrait for EnlightenState {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ENLIGHTENSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenState",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnlightenState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BounceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenState, bounce_scale),
            },
            FieldInfoData {
                name: "SunScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenState, sun_scale),
            },
            FieldInfoData {
                name: "OutputScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenState, output_scale),
            },
            FieldInfoData {
                name: "SkyBoxEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenState, sky_box_enable),
            },
            FieldInfoData {
                name: "SkyBoxSkyColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnlightenState, sky_box_sky_color),
            },
            FieldInfoData {
                name: "SkyBoxGroundColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnlightenState, sky_box_ground_color),
            },
            FieldInfoData {
                name: "SkyBoxSunLightColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnlightenState, sky_box_sun_light_color),
            },
            FieldInfoData {
                name: "SkyBoxSunLightColorSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenState, sky_box_sun_light_color_size),
            },
            FieldInfoData {
                name: "SkyBoxBackLightColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnlightenState, sky_box_back_light_color),
            },
            FieldInfoData {
                name: "SkyBoxBackLightColorSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenState, sky_box_back_light_color_size),
            },
            FieldInfoData {
                name: "SkyBoxBackLightRotationX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenState, sky_box_back_light_rotation_x),
            },
            FieldInfoData {
                name: "SkyBoxBackLightRotationY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenState, sky_box_back_light_rotation_y),
            },
        ],
    }),
    array_type: Some(ENLIGHTENSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EnlightenState {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENLIGHTENSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenState-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("EnlightenState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnlightenRuntimeConfig {
    pub _glacier_base: super::render_base::EnlightenRuntimeConfigBaseAsset,
    pub job_count: u32,
}

pub trait EnlightenRuntimeConfigTrait: super::render_base::EnlightenRuntimeConfigBaseAssetTrait {
    fn job_count(&self) -> &u32;
}

impl EnlightenRuntimeConfigTrait for EnlightenRuntimeConfig {
    fn job_count(&self) -> &u32 {
        &self.job_count
    }
}

impl super::render_base::EnlightenRuntimeConfigBaseAssetTrait for EnlightenRuntimeConfig {
}

impl super::core::AssetTrait for EnlightenRuntimeConfig {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for EnlightenRuntimeConfig {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ENLIGHTENRUNTIMECONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenRuntimeConfig",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::render_base::ENLIGHTENRUNTIMECONFIGBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnlightenRuntimeConfig as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "JobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EnlightenRuntimeConfig, job_count),
            },
        ],
    }),
    array_type: Some(ENLIGHTENRUNTIMECONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EnlightenRuntimeConfig {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENRUNTIMECONFIG_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENLIGHTENRUNTIMECONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenRuntimeConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("EnlightenRuntimeConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EnlightenRuntimeSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub enable: bool,
    pub force_dynamic: bool,
    pub force_update_static_lighting_buffers_enable: bool,
    pub save_radiosity_textures_enable: bool,
    pub save_radiosity_textures_data_asset_guid: glacier_util::guid::Guid,
    pub temporal_coherence_threshold: f32,
    pub max_per_frame_temporal_solve_time: f32,
    pub distance_priority: f32,
    pub frustum_priority: f32,
    pub sky_box_scale: f32,
    pub max_per_frame_solve_time: f32,
    pub min_system_update_count: u32,
    pub max_system_update_count: i32,
    pub jobs_enable: bool,
    pub job_count: u32,
    pub shadows_enable: bool,
    pub spot_light_shadows_enable: bool,
    pub sun_light_shadows_enable: bool,
    pub compensate_sun_shadow_height_scale: bool,
    pub light_maps_enable: bool,
    pub light_probe_enable: bool,
    pub light_probe_force_update: bool,
    pub light_probe_jobs_enable: bool,
    pub light_probe_max_source_solve_count: u32,
    pub light_probe_max_instance_update_count: u32,
    pub light_probe_min_instance_update_distance: f32,
    pub light_probe_force_min_instance_update_distance: f32,
    pub light_probe_table_cell_size: u32,
    pub light_probe_max_per_frame_time_ms: f32,
    pub local_lights_enable: bool,
    pub local_light_culling_enable: bool,
    pub local_light_custum_falloff: bool,
    pub local_light_force_radius: f32,
    pub draw_debug_mesh_lod: i32,
    pub draw_debug_entities: bool,
    pub draw_debug_static_entities: bool,
    pub draw_debug_systems_enable: bool,
    pub draw_debug_system_dependencies_enable: i32,
    pub draw_debug_system_bounding_box_enable: i32,
    pub draw_debug_static_light_probes: bool,
    pub draw_debug_dynamic_light_probes: bool,
    pub draw_debug_light_probe_grid: bool,
    pub draw_debug_light_probe_occlusion: bool,
    pub draw_debug_light_probe_stats: bool,
    pub draw_debug_light_probe_bounding_boxes: bool,
    pub draw_debug_transparency_light_probes: bool,
    pub draw_debug_light_probe_size: f32,
    pub draw_debug_light_probe_cull_distance: f32,
    pub draw_solve_task_performance_summary: bool,
    pub draw_solve_task_performance_details: bool,
    pub draw_debug_coloring_enable: bool,
    pub draw_debug_textures: bool,
    pub draw_debug_g_p_u_dusters: bool,
    pub draw_debug_g_p_u_sun_visibility: bool,
    pub draw_debug_g_p_u_spot_visibility: bool,
    pub draw_debug_back_faces: bool,
    pub draw_debug_target_meshes: bool,
    pub draw_debug_duster_positions: bool,
    pub draw_debug_clusters: bool,
    pub draw_debug_trans_sample_positions: bool,
    pub draw_warnings_enable: bool,
    pub albedo_force_update_enable: bool,
    pub albedo_force_color_enable: bool,
    pub albedo_default_color: super::core::Vec3,
    pub terrain_map_enable: bool,
    pub emissive_enable: bool,
    pub debug_meshes_enable: bool,
    pub validation_enable: bool,
    pub update_transparency_with_sun_light_change: bool,
    pub transparency_interpolants: i32,
    pub transparency_update_count: i32,
    pub display_dirty_message: bool,
    pub frame_amortized_update_enabled: bool,
}

pub trait EnlightenRuntimeSettingsTrait: super::core::SystemSettingsTrait {
    fn enable(&self) -> &bool;
    fn force_dynamic(&self) -> &bool;
    fn force_update_static_lighting_buffers_enable(&self) -> &bool;
    fn save_radiosity_textures_enable(&self) -> &bool;
    fn save_radiosity_textures_data_asset_guid(&self) -> &glacier_util::guid::Guid;
    fn temporal_coherence_threshold(&self) -> &f32;
    fn max_per_frame_temporal_solve_time(&self) -> &f32;
    fn distance_priority(&self) -> &f32;
    fn frustum_priority(&self) -> &f32;
    fn sky_box_scale(&self) -> &f32;
    fn max_per_frame_solve_time(&self) -> &f32;
    fn min_system_update_count(&self) -> &u32;
    fn max_system_update_count(&self) -> &i32;
    fn jobs_enable(&self) -> &bool;
    fn job_count(&self) -> &u32;
    fn shadows_enable(&self) -> &bool;
    fn spot_light_shadows_enable(&self) -> &bool;
    fn sun_light_shadows_enable(&self) -> &bool;
    fn compensate_sun_shadow_height_scale(&self) -> &bool;
    fn light_maps_enable(&self) -> &bool;
    fn light_probe_enable(&self) -> &bool;
    fn light_probe_force_update(&self) -> &bool;
    fn light_probe_jobs_enable(&self) -> &bool;
    fn light_probe_max_source_solve_count(&self) -> &u32;
    fn light_probe_max_instance_update_count(&self) -> &u32;
    fn light_probe_min_instance_update_distance(&self) -> &f32;
    fn light_probe_force_min_instance_update_distance(&self) -> &f32;
    fn light_probe_table_cell_size(&self) -> &u32;
    fn light_probe_max_per_frame_time_ms(&self) -> &f32;
    fn local_lights_enable(&self) -> &bool;
    fn local_light_culling_enable(&self) -> &bool;
    fn local_light_custum_falloff(&self) -> &bool;
    fn local_light_force_radius(&self) -> &f32;
    fn draw_debug_mesh_lod(&self) -> &i32;
    fn draw_debug_entities(&self) -> &bool;
    fn draw_debug_static_entities(&self) -> &bool;
    fn draw_debug_systems_enable(&self) -> &bool;
    fn draw_debug_system_dependencies_enable(&self) -> &i32;
    fn draw_debug_system_bounding_box_enable(&self) -> &i32;
    fn draw_debug_static_light_probes(&self) -> &bool;
    fn draw_debug_dynamic_light_probes(&self) -> &bool;
    fn draw_debug_light_probe_grid(&self) -> &bool;
    fn draw_debug_light_probe_occlusion(&self) -> &bool;
    fn draw_debug_light_probe_stats(&self) -> &bool;
    fn draw_debug_light_probe_bounding_boxes(&self) -> &bool;
    fn draw_debug_transparency_light_probes(&self) -> &bool;
    fn draw_debug_light_probe_size(&self) -> &f32;
    fn draw_debug_light_probe_cull_distance(&self) -> &f32;
    fn draw_solve_task_performance_summary(&self) -> &bool;
    fn draw_solve_task_performance_details(&self) -> &bool;
    fn draw_debug_coloring_enable(&self) -> &bool;
    fn draw_debug_textures(&self) -> &bool;
    fn draw_debug_g_p_u_dusters(&self) -> &bool;
    fn draw_debug_g_p_u_sun_visibility(&self) -> &bool;
    fn draw_debug_g_p_u_spot_visibility(&self) -> &bool;
    fn draw_debug_back_faces(&self) -> &bool;
    fn draw_debug_target_meshes(&self) -> &bool;
    fn draw_debug_duster_positions(&self) -> &bool;
    fn draw_debug_clusters(&self) -> &bool;
    fn draw_debug_trans_sample_positions(&self) -> &bool;
    fn draw_warnings_enable(&self) -> &bool;
    fn albedo_force_update_enable(&self) -> &bool;
    fn albedo_force_color_enable(&self) -> &bool;
    fn albedo_default_color(&self) -> &super::core::Vec3;
    fn terrain_map_enable(&self) -> &bool;
    fn emissive_enable(&self) -> &bool;
    fn debug_meshes_enable(&self) -> &bool;
    fn validation_enable(&self) -> &bool;
    fn update_transparency_with_sun_light_change(&self) -> &bool;
    fn transparency_interpolants(&self) -> &i32;
    fn transparency_update_count(&self) -> &i32;
    fn display_dirty_message(&self) -> &bool;
    fn frame_amortized_update_enabled(&self) -> &bool;
}

impl EnlightenRuntimeSettingsTrait for EnlightenRuntimeSettings {
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn force_dynamic(&self) -> &bool {
        &self.force_dynamic
    }
    fn force_update_static_lighting_buffers_enable(&self) -> &bool {
        &self.force_update_static_lighting_buffers_enable
    }
    fn save_radiosity_textures_enable(&self) -> &bool {
        &self.save_radiosity_textures_enable
    }
    fn save_radiosity_textures_data_asset_guid(&self) -> &glacier_util::guid::Guid {
        &self.save_radiosity_textures_data_asset_guid
    }
    fn temporal_coherence_threshold(&self) -> &f32 {
        &self.temporal_coherence_threshold
    }
    fn max_per_frame_temporal_solve_time(&self) -> &f32 {
        &self.max_per_frame_temporal_solve_time
    }
    fn distance_priority(&self) -> &f32 {
        &self.distance_priority
    }
    fn frustum_priority(&self) -> &f32 {
        &self.frustum_priority
    }
    fn sky_box_scale(&self) -> &f32 {
        &self.sky_box_scale
    }
    fn max_per_frame_solve_time(&self) -> &f32 {
        &self.max_per_frame_solve_time
    }
    fn min_system_update_count(&self) -> &u32 {
        &self.min_system_update_count
    }
    fn max_system_update_count(&self) -> &i32 {
        &self.max_system_update_count
    }
    fn jobs_enable(&self) -> &bool {
        &self.jobs_enable
    }
    fn job_count(&self) -> &u32 {
        &self.job_count
    }
    fn shadows_enable(&self) -> &bool {
        &self.shadows_enable
    }
    fn spot_light_shadows_enable(&self) -> &bool {
        &self.spot_light_shadows_enable
    }
    fn sun_light_shadows_enable(&self) -> &bool {
        &self.sun_light_shadows_enable
    }
    fn compensate_sun_shadow_height_scale(&self) -> &bool {
        &self.compensate_sun_shadow_height_scale
    }
    fn light_maps_enable(&self) -> &bool {
        &self.light_maps_enable
    }
    fn light_probe_enable(&self) -> &bool {
        &self.light_probe_enable
    }
    fn light_probe_force_update(&self) -> &bool {
        &self.light_probe_force_update
    }
    fn light_probe_jobs_enable(&self) -> &bool {
        &self.light_probe_jobs_enable
    }
    fn light_probe_max_source_solve_count(&self) -> &u32 {
        &self.light_probe_max_source_solve_count
    }
    fn light_probe_max_instance_update_count(&self) -> &u32 {
        &self.light_probe_max_instance_update_count
    }
    fn light_probe_min_instance_update_distance(&self) -> &f32 {
        &self.light_probe_min_instance_update_distance
    }
    fn light_probe_force_min_instance_update_distance(&self) -> &f32 {
        &self.light_probe_force_min_instance_update_distance
    }
    fn light_probe_table_cell_size(&self) -> &u32 {
        &self.light_probe_table_cell_size
    }
    fn light_probe_max_per_frame_time_ms(&self) -> &f32 {
        &self.light_probe_max_per_frame_time_ms
    }
    fn local_lights_enable(&self) -> &bool {
        &self.local_lights_enable
    }
    fn local_light_culling_enable(&self) -> &bool {
        &self.local_light_culling_enable
    }
    fn local_light_custum_falloff(&self) -> &bool {
        &self.local_light_custum_falloff
    }
    fn local_light_force_radius(&self) -> &f32 {
        &self.local_light_force_radius
    }
    fn draw_debug_mesh_lod(&self) -> &i32 {
        &self.draw_debug_mesh_lod
    }
    fn draw_debug_entities(&self) -> &bool {
        &self.draw_debug_entities
    }
    fn draw_debug_static_entities(&self) -> &bool {
        &self.draw_debug_static_entities
    }
    fn draw_debug_systems_enable(&self) -> &bool {
        &self.draw_debug_systems_enable
    }
    fn draw_debug_system_dependencies_enable(&self) -> &i32 {
        &self.draw_debug_system_dependencies_enable
    }
    fn draw_debug_system_bounding_box_enable(&self) -> &i32 {
        &self.draw_debug_system_bounding_box_enable
    }
    fn draw_debug_static_light_probes(&self) -> &bool {
        &self.draw_debug_static_light_probes
    }
    fn draw_debug_dynamic_light_probes(&self) -> &bool {
        &self.draw_debug_dynamic_light_probes
    }
    fn draw_debug_light_probe_grid(&self) -> &bool {
        &self.draw_debug_light_probe_grid
    }
    fn draw_debug_light_probe_occlusion(&self) -> &bool {
        &self.draw_debug_light_probe_occlusion
    }
    fn draw_debug_light_probe_stats(&self) -> &bool {
        &self.draw_debug_light_probe_stats
    }
    fn draw_debug_light_probe_bounding_boxes(&self) -> &bool {
        &self.draw_debug_light_probe_bounding_boxes
    }
    fn draw_debug_transparency_light_probes(&self) -> &bool {
        &self.draw_debug_transparency_light_probes
    }
    fn draw_debug_light_probe_size(&self) -> &f32 {
        &self.draw_debug_light_probe_size
    }
    fn draw_debug_light_probe_cull_distance(&self) -> &f32 {
        &self.draw_debug_light_probe_cull_distance
    }
    fn draw_solve_task_performance_summary(&self) -> &bool {
        &self.draw_solve_task_performance_summary
    }
    fn draw_solve_task_performance_details(&self) -> &bool {
        &self.draw_solve_task_performance_details
    }
    fn draw_debug_coloring_enable(&self) -> &bool {
        &self.draw_debug_coloring_enable
    }
    fn draw_debug_textures(&self) -> &bool {
        &self.draw_debug_textures
    }
    fn draw_debug_g_p_u_dusters(&self) -> &bool {
        &self.draw_debug_g_p_u_dusters
    }
    fn draw_debug_g_p_u_sun_visibility(&self) -> &bool {
        &self.draw_debug_g_p_u_sun_visibility
    }
    fn draw_debug_g_p_u_spot_visibility(&self) -> &bool {
        &self.draw_debug_g_p_u_spot_visibility
    }
    fn draw_debug_back_faces(&self) -> &bool {
        &self.draw_debug_back_faces
    }
    fn draw_debug_target_meshes(&self) -> &bool {
        &self.draw_debug_target_meshes
    }
    fn draw_debug_duster_positions(&self) -> &bool {
        &self.draw_debug_duster_positions
    }
    fn draw_debug_clusters(&self) -> &bool {
        &self.draw_debug_clusters
    }
    fn draw_debug_trans_sample_positions(&self) -> &bool {
        &self.draw_debug_trans_sample_positions
    }
    fn draw_warnings_enable(&self) -> &bool {
        &self.draw_warnings_enable
    }
    fn albedo_force_update_enable(&self) -> &bool {
        &self.albedo_force_update_enable
    }
    fn albedo_force_color_enable(&self) -> &bool {
        &self.albedo_force_color_enable
    }
    fn albedo_default_color(&self) -> &super::core::Vec3 {
        &self.albedo_default_color
    }
    fn terrain_map_enable(&self) -> &bool {
        &self.terrain_map_enable
    }
    fn emissive_enable(&self) -> &bool {
        &self.emissive_enable
    }
    fn debug_meshes_enable(&self) -> &bool {
        &self.debug_meshes_enable
    }
    fn validation_enable(&self) -> &bool {
        &self.validation_enable
    }
    fn update_transparency_with_sun_light_change(&self) -> &bool {
        &self.update_transparency_with_sun_light_change
    }
    fn transparency_interpolants(&self) -> &i32 {
        &self.transparency_interpolants
    }
    fn transparency_update_count(&self) -> &i32 {
        &self.transparency_update_count
    }
    fn display_dirty_message(&self) -> &bool {
        &self.display_dirty_message
    }
    fn frame_amortized_update_enabled(&self) -> &bool {
        &self.frame_amortized_update_enabled
    }
}

impl super::core::SystemSettingsTrait for EnlightenRuntimeSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for EnlightenRuntimeSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static ENLIGHTENRUNTIMESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenRuntimeSettings",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EnlightenRuntimeSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, enable),
            },
            FieldInfoData {
                name: "ForceDynamic",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, force_dynamic),
            },
            FieldInfoData {
                name: "ForceUpdateStaticLightingBuffersEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, force_update_static_lighting_buffers_enable),
            },
            FieldInfoData {
                name: "SaveRadiosityTexturesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, save_radiosity_textures_enable),
            },
            FieldInfoData {
                name: "SaveRadiosityTexturesDataAssetGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(EnlightenRuntimeSettings, save_radiosity_textures_data_asset_guid),
            },
            FieldInfoData {
                name: "TemporalCoherenceThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, temporal_coherence_threshold),
            },
            FieldInfoData {
                name: "MaxPerFrameTemporalSolveTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, max_per_frame_temporal_solve_time),
            },
            FieldInfoData {
                name: "DistancePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, distance_priority),
            },
            FieldInfoData {
                name: "FrustumPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, frustum_priority),
            },
            FieldInfoData {
                name: "SkyBoxScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, sky_box_scale),
            },
            FieldInfoData {
                name: "MaxPerFrameSolveTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, max_per_frame_solve_time),
            },
            FieldInfoData {
                name: "MinSystemUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, min_system_update_count),
            },
            FieldInfoData {
                name: "MaxSystemUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, max_system_update_count),
            },
            FieldInfoData {
                name: "JobsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, jobs_enable),
            },
            FieldInfoData {
                name: "JobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, job_count),
            },
            FieldInfoData {
                name: "ShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, shadows_enable),
            },
            FieldInfoData {
                name: "SpotLightShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, spot_light_shadows_enable),
            },
            FieldInfoData {
                name: "SunLightShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, sun_light_shadows_enable),
            },
            FieldInfoData {
                name: "CompensateSunShadowHeightScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, compensate_sun_shadow_height_scale),
            },
            FieldInfoData {
                name: "LightMapsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, light_maps_enable),
            },
            FieldInfoData {
                name: "LightProbeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, light_probe_enable),
            },
            FieldInfoData {
                name: "LightProbeForceUpdate",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, light_probe_force_update),
            },
            FieldInfoData {
                name: "LightProbeJobsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, light_probe_jobs_enable),
            },
            FieldInfoData {
                name: "LightProbeMaxSourceSolveCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, light_probe_max_source_solve_count),
            },
            FieldInfoData {
                name: "LightProbeMaxInstanceUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, light_probe_max_instance_update_count),
            },
            FieldInfoData {
                name: "LightProbeMinInstanceUpdateDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, light_probe_min_instance_update_distance),
            },
            FieldInfoData {
                name: "LightProbeForceMinInstanceUpdateDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, light_probe_force_min_instance_update_distance),
            },
            FieldInfoData {
                name: "LightProbeTableCellSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, light_probe_table_cell_size),
            },
            FieldInfoData {
                name: "LightProbeMaxPerFrameTimeMs",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, light_probe_max_per_frame_time_ms),
            },
            FieldInfoData {
                name: "LocalLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, local_lights_enable),
            },
            FieldInfoData {
                name: "LocalLightCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, local_light_culling_enable),
            },
            FieldInfoData {
                name: "LocalLightCustumFalloff",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, local_light_custum_falloff),
            },
            FieldInfoData {
                name: "LocalLightForceRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, local_light_force_radius),
            },
            FieldInfoData {
                name: "DrawDebugMeshLod",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_mesh_lod),
            },
            FieldInfoData {
                name: "DrawDebugEntities",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_entities),
            },
            FieldInfoData {
                name: "DrawDebugStaticEntities",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_static_entities),
            },
            FieldInfoData {
                name: "DrawDebugSystemsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_systems_enable),
            },
            FieldInfoData {
                name: "DrawDebugSystemDependenciesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_system_dependencies_enable),
            },
            FieldInfoData {
                name: "DrawDebugSystemBoundingBoxEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_system_bounding_box_enable),
            },
            FieldInfoData {
                name: "DrawDebugStaticLightProbes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_static_light_probes),
            },
            FieldInfoData {
                name: "DrawDebugDynamicLightProbes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_dynamic_light_probes),
            },
            FieldInfoData {
                name: "DrawDebugLightProbeGrid",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_light_probe_grid),
            },
            FieldInfoData {
                name: "DrawDebugLightProbeOcclusion",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_light_probe_occlusion),
            },
            FieldInfoData {
                name: "DrawDebugLightProbeStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_light_probe_stats),
            },
            FieldInfoData {
                name: "DrawDebugLightProbeBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_light_probe_bounding_boxes),
            },
            FieldInfoData {
                name: "DrawDebugTransparencyLightProbes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_transparency_light_probes),
            },
            FieldInfoData {
                name: "DrawDebugLightProbeSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_light_probe_size),
            },
            FieldInfoData {
                name: "DrawDebugLightProbeCullDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_light_probe_cull_distance),
            },
            FieldInfoData {
                name: "DrawSolveTaskPerformanceSummary",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_solve_task_performance_summary),
            },
            FieldInfoData {
                name: "DrawSolveTaskPerformanceDetails",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_solve_task_performance_details),
            },
            FieldInfoData {
                name: "DrawDebugColoringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_coloring_enable),
            },
            FieldInfoData {
                name: "DrawDebugTextures",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_textures),
            },
            FieldInfoData {
                name: "DrawDebugGPUDusters",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_g_p_u_dusters),
            },
            FieldInfoData {
                name: "DrawDebugGPUSunVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_g_p_u_sun_visibility),
            },
            FieldInfoData {
                name: "DrawDebugGPUSpotVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_g_p_u_spot_visibility),
            },
            FieldInfoData {
                name: "DrawDebugBackFaces",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_back_faces),
            },
            FieldInfoData {
                name: "DrawDebugTargetMeshes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_target_meshes),
            },
            FieldInfoData {
                name: "DrawDebugDusterPositions",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_duster_positions),
            },
            FieldInfoData {
                name: "DrawDebugClusters",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_clusters),
            },
            FieldInfoData {
                name: "DrawDebugTransSamplePositions",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_debug_trans_sample_positions),
            },
            FieldInfoData {
                name: "DrawWarningsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, draw_warnings_enable),
            },
            FieldInfoData {
                name: "AlbedoForceUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, albedo_force_update_enable),
            },
            FieldInfoData {
                name: "AlbedoForceColorEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, albedo_force_color_enable),
            },
            FieldInfoData {
                name: "AlbedoDefaultColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EnlightenRuntimeSettings, albedo_default_color),
            },
            FieldInfoData {
                name: "TerrainMapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, terrain_map_enable),
            },
            FieldInfoData {
                name: "EmissiveEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, emissive_enable),
            },
            FieldInfoData {
                name: "DebugMeshesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, debug_meshes_enable),
            },
            FieldInfoData {
                name: "ValidationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, validation_enable),
            },
            FieldInfoData {
                name: "UpdateTransparencyWithSunLightChange",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, update_transparency_with_sun_light_change),
            },
            FieldInfoData {
                name: "TransparencyInterpolants",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, transparency_interpolants),
            },
            FieldInfoData {
                name: "TransparencyUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EnlightenRuntimeSettings, transparency_update_count),
            },
            FieldInfoData {
                name: "DisplayDirtyMessage",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, display_dirty_message),
            },
            FieldInfoData {
                name: "FrameAmortizedUpdateEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EnlightenRuntimeSettings, frame_amortized_update_enabled),
            },
        ],
    }),
    array_type: Some(ENLIGHTENRUNTIMESETTINGS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EnlightenRuntimeSettings {
    fn type_info(&self) -> &'static TypeInfo {
        ENLIGHTENRUNTIMESETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ENLIGHTENRUNTIMESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EnlightenRuntimeSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("EnlightenRuntimeSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WorldRenderSettings {
    pub _glacier_base: WorldRenderSettingsBase,
    pub test_rendering_enable: bool,
    pub generic_entity_renderer_enable: bool,
    pub generic_entity_max_visible_entity_count: u32,
    pub z_buffer_shadow_test_enable: bool,
    pub draw_debug_ground_height: u32,
    pub decal_volume_enable: bool,
    pub decal_volume_scale: f32,
    pub draw_debug_decal_volumes: bool,
    pub environment_decal_volumes_enabled: bool,
    pub environment_decal_volume_quality: super::core::QualityLevel,
    pub environment_decal_volume_max_count: super::core::QualityScalableInt,
    pub emitter_decal_volume_quality: super::core::QualityLevel,
    pub emitter_decal_volume_max_count: super::core::QualityScalableInt,
    pub emitter_decal_volume_max_count_per_set: super::core::QualityScalableInt,
    pub edge_models_enable: bool,
    pub edge_model_cast_shadows_enable: bool,
    pub edge_model_depth_bias_enable: bool,
    pub edge_model_shadow_depth_bias_enable: bool,
    pub edge_model_view_distance: f32,
    pub edge_model_use_main_lod_enable: bool,
    pub edge_model_force_lod: i32,
    pub edge_model_lod_scale: f32,
    pub lens_flares_enable: bool,
    pub draw_debug_lens_flare_occluders: bool,
    pub draw_debug_lens_flares: bool,
    pub lens_flare_occlusion_enable: bool,
    pub max_lens_flares_per_frame: u32,
    pub lens_flares_quality_level: super::core::QualityLevel,
    pub cloud_shadow_enable: bool,
    pub override_dynamic_a_o: bool,
    pub draw_debug_dynamic_a_o: bool,
    pub draw_debug_raytrace_refl: bool,
    pub draw_debug_raytrace_a_o: bool,
    pub draw_debug_raytrace_primary_ray: bool,
    pub filmic_effects_enable: bool,
    pub emissive_enable: bool,
    pub g_buffer_layout: u32,
    pub g_buffer_test_count: u32,
    pub g_buffer_clear_enable: bool,
    pub dx_g_buffer_light16_bit_enable: bool,
    pub dx_g_buffer_normal16_bit_enable: bool,
    pub dx_g_buffer_roughness16_bit_enable: bool,
    pub g_buffer_alpha_test_simple_enable: bool,
    pub g_buffer_alpha_test_simple_smoothness: f32,
    pub g_buffer_force_metal_mask: f32,
    pub g_buffer_force_smoothness: f32,
    pub g_buffer_force_specular_albedo: f32,
    pub alpha_unroll_enable: bool,
    pub gen4a_esram_enable: bool,
    pub specular_lighting_enable: bool,
    pub skin_lighting_enable: bool,
    pub translucency_lighting_enable: bool,
    pub translucency_auto_thickness_enable: bool,
    pub local_light_translucency_enable: bool,
    pub dynamic_envmap_lighting_enable: bool,
    pub outdoor_light_enable: bool,
    pub light_stencil_method_enable: bool,
    pub light_volume_method_enable: bool,
    pub light_volume_depth_test_enable: bool,
    pub outdoor_key_light_enable: bool,
    pub outdoor_sky_light_enable: bool,
    pub outdoor_light_tile_blend_enable: bool,
    pub emitter_sun_transmittance_map_enable: bool,
    pub emitter_sun_transmittance_map_resolution: u32,
    pub max_decal_volume_count: u32,
    pub light_tile_combine_outdoor_light_enable: bool,
    pub light_tile_cs_path_enable: bool,
    pub light_tile_ps_path_enable: bool,
    pub light_tile_async_compute_culling: bool,
    pub light_tile_cs_avg_light_count_per_tile: u32,
    pub light_tile_min_max_use_h_tile: bool,
    pub light_tile_use_culling_hierarchy: bool,
    pub light_tile_use_detailed_gpu_timers: bool,
    pub light_tile_use_cs_indirect_clears: bool,
    pub light_cull_frustum_expand_distance: f32,
    pub light_tile_debug_light_mode: LightTileDebugLightCountMode,
    pub light_tile_debug_color_mode: i32,
    pub draw_debug_light_stats: bool,
    pub draw_debug_light_stats_forward: bool,
    pub draw_debug_light_stats_volumetric: bool,
    pub draw_debug_light_stats_hierarchy: bool,
    pub draw_debug_light_sources: bool,
    pub draw_debug_light_shadow_sources: bool,
    pub draw_debug_light_shadow_stats: bool,
    pub draw_debug_light_cull_stats: bool,
    pub draw_debug_g_buffer: bool,
    pub draw_debug_material_input: bool,
    pub draw_debug_material_output: bool,
    pub draw_debug_light_emissive_surface: bool,
    pub punctual_emissive_light_shape_min_size: f32,
    pub debug_light_stats_light_count_highwatermark: u32,
    pub light_lod_fade_area: f32,
    pub light_lod_min_area: f32,
    pub light_lod_radius_factor: f32,
    pub use_new_light_cull_enable: bool,
    pub light_cull_enable: bool,
    pub light_cull_job_max_job_count: u32,
    pub light_occlusion_cull_enable: bool,
    pub light_cone_cull_enable: bool,
    pub occlusion_culling_width: u32,
    pub occlusion_culling_height: u32,
    pub occlusion_triangle_count: u32,
    pub local_i_b_l_occlusion_culling_enable: bool,
    pub shadow_occlusion_culling_enable: bool,
    pub shadow_occlusion_culling_width: u32,
    pub shadow_occlusion_culling_height: u32,
    pub shadow_occlusion_triangle_count: u32,
    pub frustum_silhouette_culling_enable: bool,
    pub frustum_silhouette_culling_padding: f32,
    pub sub_surface_scattering_enable: bool,
    pub translucency_enable: bool,
    pub sub_surface_scattering_sample_count: i32,
    pub split_lighting_enable: bool,
    pub subsurface_blur_compute_enable: bool,
    pub subsurface_blur_quadtree_tile_gen_enable: bool,
    pub subsurface_blur_pixel_radius_cull_threshold: f32,
    pub opaque_sort_by_solution_enable: bool,
    pub main_opaque_z_pass_enable: bool,
    pub planar_reflection_enable: bool,
    pub planar_reflection_fast_hdr_enable: bool,
    pub planar_reflection_view_scale: f32,
    pub planar_reflection_blur_enable: bool,
    pub planar_reflection_convolution_enable: bool,
    pub planar_reflection_convolution_sample_clamp_threshold: f32,
    pub planar_reflection_convolution_sample_count: u32,
    pub planar_reflection_convolution_random_samples_enable: bool,
    pub planar_reflection_convolution_post_filter_enable: bool,
    pub planar_reflection_cull_f_o_v: f32,
    pub planar_reflection_clipping_enable: bool,
    pub draw_debug_render_texture: bool,
    pub draw_debug_planar_reflection: bool,
    pub draw_debug_planar_reflection_mip_level: u32,
    pub draw_debug_planar_reflection_mode: u32,
    pub draw_debug_planar_reflection_cull_frustum: bool,
    pub local_planar_reflection_convolution_enable: bool,
    pub local_planar_reflection_force_lowest_lod_enable: bool,
    pub reflection_lod_scale: f32,
    pub object_highlight_enable: bool,
    pub object_highlight_mask_first_person_enable: bool,
    pub transparent_depth_z_prepass_enable: bool,
    pub hologram_rendertarget_enable: bool,
    pub sonar_rendertarget_enable: bool,
    pub simple_volumetrics_enable: bool,
    pub simple_volumetrics_half_res_enable: bool,
    pub fog_exclusion_volume_enable: bool,
    pub threat_alert_overlay_enable: bool,
    pub mesh_compute_enabled: bool,
    pub outline_edge_detect_enable: bool,
    pub overlay_blur_enable: bool,
    pub overlay_blur_async_compute_enable: bool,
    pub overlay_blur_kernel_radius: u32,
    pub post_process_antialiasing_mode: PostProcessAAMode,
    pub smaa_predicated_thresholding_enable: bool,
    pub temporal_a_a_jitter_count: u32,
    pub temporal_a_a_disocclusion_rejection_factor: f32,
    pub temporal_a_a_history_sharpening_enable: bool,
    pub temporal_a_a_motion_sharpening_factor: f32,
    pub temporal_a_a_responsiveness: f32,
    pub temporal_a_a_antiflicker_strength: f32,
    pub temporal_a_a_quality: u32,
    pub temporal_a_a_post_sharpening_amount: f32,
    pub draw_debug_temporal_a_a_enable: bool,
    pub draw_debug_temporal_a_a_accumulation_count: u32,
    pub draw_debug_temporal_a_a_debug_mode: u32,
    pub draw_debug_temporal_a_a_max_distance: f32,
    pub temporal_a_a_dof_coc_filter_enable: bool,
    pub d_l_a_a_jitter_count: u32,
    pub d_l_a_a_jitter_scale: f32,
    pub d_l_a_a_draw_enable: bool,
    pub d_l_a_a_capture_enable: bool,
    pub sky_celestial_enable: bool,
    pub sky_celestial_quality: super::core::QualityLevel,
    pub sky_celestial_max_quad_count: super::core::QualityScalableInt,
    pub sky_render_mode: SkyRenderMode,
    pub lens_reflection_enable: bool,
    pub dof_before_motion_blur: bool,
    pub lens_dirt_enable: bool,
    pub parallel_create_draw_view_enable: bool,
    pub draw_hologram_with_temporal_a_a: bool,
    pub interpupillary_distance: f32,
    pub vr_hmd_lens_distortion_enable: bool,
    pub vr_hmd_late_reprojection_enable: bool,
    pub enable_persistent_sink_usage: bool,
    pub raytrace_enable: bool,
    pub raytrace_debug_enable: bool,
    pub raytrace_forward_enable: bool,
    pub raytrace_build_async_compute_enable: bool,
    pub raytrace_ao_enable: bool,
    pub raytrace_refl_enable: bool,
    pub raytrace_refl_ray_to_pixel_ratio: f32,
    pub raytrace_refl_transparent_enable: bool,
    pub raytrace_refl_force_min_smoothness: f32,
    pub raytrace_refl_ssr_compare_enable: bool,
    pub raytrace_refl_ssr_compare_invert_side: bool,
    pub raytrace_refl_ssr_compare_location: f32,
    pub raytrace_primary_ray_enable: bool,
    pub raytrace_cull_mode: u32,
    pub raytrace_cull_frustum_fov_y: f32,
    pub raytrace_cull_radius: f32,
    pub raytrace_cull_radius_large_object_scale: f32,
    pub raytrace_cull_frustum_large_object_scale: f32,
    pub raytrace_cull_radius_emitters: f32,
    pub local_light_shadow_enable: bool,
    pub local_light_shadow16_bit_enable: bool,
    pub local_light_shadow_filter_quality: u32,
    pub local_light_shadow_resolution_low: u32,
    pub local_light_shadow_resolution_medium: u32,
    pub local_light_shadow_resolution_high: u32,
    pub local_light_shadow_resolution_ultra: u32,
    pub local_light_shadow_atlas_slot_count: u32,
    pub local_light_shadow_atlas_slot_resolution: u32,
    pub reflection_local_light_shadow_resolution: u32,
    pub local_light_shadow_cache_enable: bool,
    pub max_shadow_count: u32,
    pub max_punctual_light_count: u32,
    pub max_punctual_shadow_light_count: u32,
    pub max_area_light_count: u32,
    pub max_area_shadow_light_count: u32,
    pub max_local_reflection_volume_count: u32,
    pub max_local_planar_reflection_count: u32,
    pub max_punctual_rectangular_light_count: u32,
    pub p_b_r_support_original_light: bool,
    pub radiosity_shadow_culling_enable: bool,
    pub punctual_lights_enable: bool,
    pub area_lights_enable: bool,
    pub local_reflection_enable: bool,
    pub tile_pass_punctual_lights_enable: bool,
    pub tile_pass_punctual_light_shadow_enable: bool,
    pub tile_pass_area_lights_enable: bool,
    pub tile_pass_area_light_shadow_enable: bool,
    pub tile_pass_local_reflection_volume_enable: bool,
    pub tile_pass_local_planar_reflection_enable: bool,
    pub punctual_light_shadow_level: super::core::QualityLevel,
    pub area_light_shadow_level: super::core::QualityLevel,
    pub sphere_lights_enable: bool,
    pub punctual_sphere_lights_enable: bool,
    pub spot_lights_enable: bool,
    pub punctual_spot_lights_enable: bool,
    pub tube_lights_enable: bool,
    pub punctual_tube_lights_enable: bool,
    pub rectangular_lights_enable: bool,
    pub punctual_rectangular_lights_enable: bool,
    pub local_reflection_volume_sphere_enable: bool,
    pub local_reflection_volume_box_enable: bool,
    pub local_planar_reflection_enable: bool,
    pub local_i_b_l_max_face_capture: u32,
    pub local_i_b_l_update_with_sky_enable: bool,
    pub local_i_b_l_update_with_enlighten_sky_box_change: bool,
    pub local_i_b_l_sun_specular_occlusion_enable: bool,
    pub local_i_b_l_lighting_update_count: u32,
    pub local_i_b_l_refresh_delay_count: u32,
    pub local_i_b_l_box_culling_enable: bool,
    pub local_i_b_l_sun_update_threshold: f32,
    pub local_i_b_l_shadowmap_slice_count: u32,
    pub local_i_b_l_shadowmap_resolution: u32,
    pub local_i_b_l_shadowmap_face_merging: bool,
    pub local_i_b_l_shadowmap_separate_frame: bool,
    pub local_i_b_l_wait_for_enlighten_to_render: bool,
    pub local_i_b_l_exposure: f32,
    pub local_i_b_l_render_transparent: bool,
    pub local_i_b_l_render_emitter_quad: bool,
    pub local_i_b_l_render_emitter_mesh: bool,
    pub p_b_r_local_i_b_l_acquisition_wait_for_mesh_loading: bool,
    pub p_b_r_local_i_b_l_acquisition_wait_frame_count: u32,
    pub p_b_r_draw_distant_i_b_l_diffuse_reference: bool,
    pub p_b_r_draw_distant_i_b_l_specular_reference: bool,
    pub p_b_r_draw_local_i_b_l_reference: bool,
    pub p_b_r_draw_area_light_reference: bool,
    pub p_b_r_specular_convolution_sample_count: u32,
    pub p_b_r_convolution_highest_m_i_p_enable: bool,
    pub local_i_b_l_resolution: u32,
    pub draw_debug_local_i_b_l_enable: bool,
    pub draw_debug_local_i_b_l_stats_enable: bool,
    pub draw_debug_local_i_b_l_volumes_enable: bool,
    pub draw_debug_local_i_b_l_preview_scale: f32,
    pub draw_debug_local_i_b_l_index: u32,
    pub draw_debug_local_i_b_l_mip_level: u32,
    pub draw_debug_local_i_b_l_shadowmaps: bool,
    pub draw_debug_pre_integrated_f_g_texture: bool,
    pub draw_debug_reflection_state: bool,
    pub draw_debug_probe_mirror_enable: bool,
    pub draw_debug_probe_diffuse_enable: bool,
    pub draw_debug_probe_screen_enable: bool,
    pub draw_debug_probe_screen_on_right: bool,
    pub continuous_local_i_b_l_enable: bool,
    pub continuous_local_i_b_l_index: u32,
    pub p_b_r_convolution_precomputed_sample_enable: bool,
    pub p_b_r_convolution_random_rotation_enable: bool,
    pub draw_debug_local_planar_reflections: bool,
    pub emitter_quad_rendering_enable: bool,
    pub emitter_mesh_rendering_enable: bool,
    pub emitter_point_lights_enable: bool,
    pub emitter_spot_lights_enable: bool,
    pub use_s_s_s_profilefor_o_a_t_s: bool,
    pub deterministic_rendering_enable: bool,
    pub hdr_nan_inf_removal_enable: bool,
    pub hdr_nan_inf_removal_force_enable: bool,
    pub p_b_r_max_illuminance_value: f32,
    pub p_b_r_max_luminance_value: f32,
    pub dielectric_range_min_metal_mask: f32,
    pub dielectric_range_max_metal_mask: f32,
    pub dielectric_range_s_r_g_b_min_value: f32,
    pub dielectric_range_s_r_g_b_max_value: f32,
    pub dielectric_range_s_r_g_b_min_color: super::core::Vec3,
    pub dielectric_range_s_r_g_b_color: super::core::Vec3,
    pub dielectric_range_s_r_g_b_max_color: super::core::Vec3,
    pub dielectric_range_overlay: f32,
    pub conductor_range_min_metal_mask: f32,
    pub conductor_range_max_metal_mask: f32,
    pub conductor_range_s_r_g_b_min_value: f32,
    pub conductor_range_s_r_g_b_max_value: f32,
    pub conductor_range_s_r_g_b_min_color: super::core::Vec3,
    pub conductor_range_s_r_g_b_color: super::core::Vec3,
    pub conductor_range_s_r_g_b_max_color: super::core::Vec3,
    pub conductor_range_overlay: f32,
    pub fresnel0_range_min_metal_mask: f32,
    pub fresnel0_range_max_metal_mask: f32,
    pub volumetric_rendering_enable: bool,
    pub volumetric_extinction_cascade_base_voxel_size: f32,
    pub volumetric_extinction_cascade_voxel_size_cascade_factor: f32,
    pub volumetric_extinction_cascade_resolution: u32,
    pub volumetric_extinction_cascade_lock_view: bool,
    pub volumetric_shadowmap_enable: bool,
    pub volumetric_shadowmap_resolution: u32,
    pub volumetric_shadowmap_max_count: u32,
    pub volumetric_shadowmap_accumulate_cs_enable: bool,
    pub punctual_light_cast_volumetric_shadowmap_enable_level: super::core::QualityLevel,
    pub area_light_cast_volumetric_shadowmap_enable_level: super::core::QualityLevel,
    pub local_light_cast_volumetric_level: super::core::QualityLevel,
    pub volumetric_emitter_voxelisation_enable: bool,
    pub volumetric_emitter_voxelisation_mode: u32,
    pub volumetric_participating_media_enable: bool,
    pub volumetric_participating_media_use_light_cull: bool,
    pub volumetric_participating_media_texture_depth: u32,
    pub volumetric_participating_media_camera_depth: f32,
    pub volumetric_participating_media_resolution_scale: u32,
    pub volumetric_participating_media_from_v_e_fog: bool,
    pub volumetric_participating_media_lock_view: bool,
    pub volumetric_participating_media_local_lights: bool,
    pub volumetric_participating_media_ambient_lighting: bool,
    pub volumetric_participating_media_sun: bool,
    pub reflection_volumetric_participating_media_texture_depth: u32,
    pub reflection_volumetric_participating_media_camera_depth: f32,
    pub reflection_volumetric_participating_media_resolution_scale: u32,
    pub volumetric_participating_media_temporal_integration: bool,
    pub volumetric_participating_media_temporal_filter_strght: f32,
    pub volumetric_local_fog_volume_enable: bool,
    pub draw_debug_volumetric_local_fog_volume: bool,
    pub draw_debug_volumetric_cascaded_volumes: bool,
    pub draw_debug_volumetric_shadow_maps: bool,
    pub draw_debug_volumetric_extinction: u32,
    pub draw_debug_volumetric_extinction_scale: f32,
    pub draw_debug_volumetric_voxelised_emitter: bool,
    pub light_shaft_fast_hdr_enable: bool,
    pub draw_gpu_histogram_enable: bool,
    pub draw_gpu_histogram_red: bool,
    pub draw_gpu_histogram_blue: bool,
    pub draw_gpu_histogram_green: bool,
    pub draw_gpu_histogram_luminance: bool,
    pub draw_gpu_histogram_h_d_r_mode: bool,
    pub draw_gpu_histogram_h_d_r_min_e_v: f32,
    pub draw_gpu_histogram_h_d_r_max_e_v: f32,
    pub draw_gpu_histogram_bin_count: u32,
    pub draw_clamped_pixels_enable: bool,
    pub draw_clamped_pixels_clamp_min: f32,
    pub draw_clamped_pixels_clamp_max: f32,
    pub enable_baked_direct_light_in_game_view: bool,
    pub enable_baked_lightmap_in_game_view: bool,
    pub texture_space_render_module_enable: bool,
    pub compute_linearize_depth: bool,
    pub compute_downsample_depth: bool,
    pub draw_debug_disable_explanation: bool,
}

pub trait WorldRenderSettingsTrait: WorldRenderSettingsBaseTrait {
    fn test_rendering_enable(&self) -> &bool;
    fn generic_entity_renderer_enable(&self) -> &bool;
    fn generic_entity_max_visible_entity_count(&self) -> &u32;
    fn z_buffer_shadow_test_enable(&self) -> &bool;
    fn draw_debug_ground_height(&self) -> &u32;
    fn decal_volume_enable(&self) -> &bool;
    fn decal_volume_scale(&self) -> &f32;
    fn draw_debug_decal_volumes(&self) -> &bool;
    fn environment_decal_volumes_enabled(&self) -> &bool;
    fn environment_decal_volume_quality(&self) -> &super::core::QualityLevel;
    fn environment_decal_volume_max_count(&self) -> &super::core::QualityScalableInt;
    fn emitter_decal_volume_quality(&self) -> &super::core::QualityLevel;
    fn emitter_decal_volume_max_count(&self) -> &super::core::QualityScalableInt;
    fn emitter_decal_volume_max_count_per_set(&self) -> &super::core::QualityScalableInt;
    fn edge_models_enable(&self) -> &bool;
    fn edge_model_cast_shadows_enable(&self) -> &bool;
    fn edge_model_depth_bias_enable(&self) -> &bool;
    fn edge_model_shadow_depth_bias_enable(&self) -> &bool;
    fn edge_model_view_distance(&self) -> &f32;
    fn edge_model_use_main_lod_enable(&self) -> &bool;
    fn edge_model_force_lod(&self) -> &i32;
    fn edge_model_lod_scale(&self) -> &f32;
    fn lens_flares_enable(&self) -> &bool;
    fn draw_debug_lens_flare_occluders(&self) -> &bool;
    fn draw_debug_lens_flares(&self) -> &bool;
    fn lens_flare_occlusion_enable(&self) -> &bool;
    fn max_lens_flares_per_frame(&self) -> &u32;
    fn lens_flares_quality_level(&self) -> &super::core::QualityLevel;
    fn cloud_shadow_enable(&self) -> &bool;
    fn override_dynamic_a_o(&self) -> &bool;
    fn draw_debug_dynamic_a_o(&self) -> &bool;
    fn draw_debug_raytrace_refl(&self) -> &bool;
    fn draw_debug_raytrace_a_o(&self) -> &bool;
    fn draw_debug_raytrace_primary_ray(&self) -> &bool;
    fn filmic_effects_enable(&self) -> &bool;
    fn emissive_enable(&self) -> &bool;
    fn g_buffer_layout(&self) -> &u32;
    fn g_buffer_test_count(&self) -> &u32;
    fn g_buffer_clear_enable(&self) -> &bool;
    fn dx_g_buffer_light16_bit_enable(&self) -> &bool;
    fn dx_g_buffer_normal16_bit_enable(&self) -> &bool;
    fn dx_g_buffer_roughness16_bit_enable(&self) -> &bool;
    fn g_buffer_alpha_test_simple_enable(&self) -> &bool;
    fn g_buffer_alpha_test_simple_smoothness(&self) -> &f32;
    fn g_buffer_force_metal_mask(&self) -> &f32;
    fn g_buffer_force_smoothness(&self) -> &f32;
    fn g_buffer_force_specular_albedo(&self) -> &f32;
    fn alpha_unroll_enable(&self) -> &bool;
    fn gen4a_esram_enable(&self) -> &bool;
    fn specular_lighting_enable(&self) -> &bool;
    fn skin_lighting_enable(&self) -> &bool;
    fn translucency_lighting_enable(&self) -> &bool;
    fn translucency_auto_thickness_enable(&self) -> &bool;
    fn local_light_translucency_enable(&self) -> &bool;
    fn dynamic_envmap_lighting_enable(&self) -> &bool;
    fn outdoor_light_enable(&self) -> &bool;
    fn light_stencil_method_enable(&self) -> &bool;
    fn light_volume_method_enable(&self) -> &bool;
    fn light_volume_depth_test_enable(&self) -> &bool;
    fn outdoor_key_light_enable(&self) -> &bool;
    fn outdoor_sky_light_enable(&self) -> &bool;
    fn outdoor_light_tile_blend_enable(&self) -> &bool;
    fn emitter_sun_transmittance_map_enable(&self) -> &bool;
    fn emitter_sun_transmittance_map_resolution(&self) -> &u32;
    fn max_decal_volume_count(&self) -> &u32;
    fn light_tile_combine_outdoor_light_enable(&self) -> &bool;
    fn light_tile_cs_path_enable(&self) -> &bool;
    fn light_tile_ps_path_enable(&self) -> &bool;
    fn light_tile_async_compute_culling(&self) -> &bool;
    fn light_tile_cs_avg_light_count_per_tile(&self) -> &u32;
    fn light_tile_min_max_use_h_tile(&self) -> &bool;
    fn light_tile_use_culling_hierarchy(&self) -> &bool;
    fn light_tile_use_detailed_gpu_timers(&self) -> &bool;
    fn light_tile_use_cs_indirect_clears(&self) -> &bool;
    fn light_cull_frustum_expand_distance(&self) -> &f32;
    fn light_tile_debug_light_mode(&self) -> &LightTileDebugLightCountMode;
    fn light_tile_debug_color_mode(&self) -> &i32;
    fn draw_debug_light_stats(&self) -> &bool;
    fn draw_debug_light_stats_forward(&self) -> &bool;
    fn draw_debug_light_stats_volumetric(&self) -> &bool;
    fn draw_debug_light_stats_hierarchy(&self) -> &bool;
    fn draw_debug_light_sources(&self) -> &bool;
    fn draw_debug_light_shadow_sources(&self) -> &bool;
    fn draw_debug_light_shadow_stats(&self) -> &bool;
    fn draw_debug_light_cull_stats(&self) -> &bool;
    fn draw_debug_g_buffer(&self) -> &bool;
    fn draw_debug_material_input(&self) -> &bool;
    fn draw_debug_material_output(&self) -> &bool;
    fn draw_debug_light_emissive_surface(&self) -> &bool;
    fn punctual_emissive_light_shape_min_size(&self) -> &f32;
    fn debug_light_stats_light_count_highwatermark(&self) -> &u32;
    fn light_lod_fade_area(&self) -> &f32;
    fn light_lod_min_area(&self) -> &f32;
    fn light_lod_radius_factor(&self) -> &f32;
    fn use_new_light_cull_enable(&self) -> &bool;
    fn light_cull_enable(&self) -> &bool;
    fn light_cull_job_max_job_count(&self) -> &u32;
    fn light_occlusion_cull_enable(&self) -> &bool;
    fn light_cone_cull_enable(&self) -> &bool;
    fn occlusion_culling_width(&self) -> &u32;
    fn occlusion_culling_height(&self) -> &u32;
    fn occlusion_triangle_count(&self) -> &u32;
    fn local_i_b_l_occlusion_culling_enable(&self) -> &bool;
    fn shadow_occlusion_culling_enable(&self) -> &bool;
    fn shadow_occlusion_culling_width(&self) -> &u32;
    fn shadow_occlusion_culling_height(&self) -> &u32;
    fn shadow_occlusion_triangle_count(&self) -> &u32;
    fn frustum_silhouette_culling_enable(&self) -> &bool;
    fn frustum_silhouette_culling_padding(&self) -> &f32;
    fn sub_surface_scattering_enable(&self) -> &bool;
    fn translucency_enable(&self) -> &bool;
    fn sub_surface_scattering_sample_count(&self) -> &i32;
    fn split_lighting_enable(&self) -> &bool;
    fn subsurface_blur_compute_enable(&self) -> &bool;
    fn subsurface_blur_quadtree_tile_gen_enable(&self) -> &bool;
    fn subsurface_blur_pixel_radius_cull_threshold(&self) -> &f32;
    fn opaque_sort_by_solution_enable(&self) -> &bool;
    fn main_opaque_z_pass_enable(&self) -> &bool;
    fn planar_reflection_enable(&self) -> &bool;
    fn planar_reflection_fast_hdr_enable(&self) -> &bool;
    fn planar_reflection_view_scale(&self) -> &f32;
    fn planar_reflection_blur_enable(&self) -> &bool;
    fn planar_reflection_convolution_enable(&self) -> &bool;
    fn planar_reflection_convolution_sample_clamp_threshold(&self) -> &f32;
    fn planar_reflection_convolution_sample_count(&self) -> &u32;
    fn planar_reflection_convolution_random_samples_enable(&self) -> &bool;
    fn planar_reflection_convolution_post_filter_enable(&self) -> &bool;
    fn planar_reflection_cull_f_o_v(&self) -> &f32;
    fn planar_reflection_clipping_enable(&self) -> &bool;
    fn draw_debug_render_texture(&self) -> &bool;
    fn draw_debug_planar_reflection(&self) -> &bool;
    fn draw_debug_planar_reflection_mip_level(&self) -> &u32;
    fn draw_debug_planar_reflection_mode(&self) -> &u32;
    fn draw_debug_planar_reflection_cull_frustum(&self) -> &bool;
    fn local_planar_reflection_convolution_enable(&self) -> &bool;
    fn local_planar_reflection_force_lowest_lod_enable(&self) -> &bool;
    fn reflection_lod_scale(&self) -> &f32;
    fn object_highlight_enable(&self) -> &bool;
    fn object_highlight_mask_first_person_enable(&self) -> &bool;
    fn transparent_depth_z_prepass_enable(&self) -> &bool;
    fn hologram_rendertarget_enable(&self) -> &bool;
    fn sonar_rendertarget_enable(&self) -> &bool;
    fn simple_volumetrics_enable(&self) -> &bool;
    fn simple_volumetrics_half_res_enable(&self) -> &bool;
    fn fog_exclusion_volume_enable(&self) -> &bool;
    fn threat_alert_overlay_enable(&self) -> &bool;
    fn mesh_compute_enabled(&self) -> &bool;
    fn outline_edge_detect_enable(&self) -> &bool;
    fn overlay_blur_enable(&self) -> &bool;
    fn overlay_blur_async_compute_enable(&self) -> &bool;
    fn overlay_blur_kernel_radius(&self) -> &u32;
    fn post_process_antialiasing_mode(&self) -> &PostProcessAAMode;
    fn smaa_predicated_thresholding_enable(&self) -> &bool;
    fn temporal_a_a_jitter_count(&self) -> &u32;
    fn temporal_a_a_disocclusion_rejection_factor(&self) -> &f32;
    fn temporal_a_a_history_sharpening_enable(&self) -> &bool;
    fn temporal_a_a_motion_sharpening_factor(&self) -> &f32;
    fn temporal_a_a_responsiveness(&self) -> &f32;
    fn temporal_a_a_antiflicker_strength(&self) -> &f32;
    fn temporal_a_a_quality(&self) -> &u32;
    fn temporal_a_a_post_sharpening_amount(&self) -> &f32;
    fn draw_debug_temporal_a_a_enable(&self) -> &bool;
    fn draw_debug_temporal_a_a_accumulation_count(&self) -> &u32;
    fn draw_debug_temporal_a_a_debug_mode(&self) -> &u32;
    fn draw_debug_temporal_a_a_max_distance(&self) -> &f32;
    fn temporal_a_a_dof_coc_filter_enable(&self) -> &bool;
    fn d_l_a_a_jitter_count(&self) -> &u32;
    fn d_l_a_a_jitter_scale(&self) -> &f32;
    fn d_l_a_a_draw_enable(&self) -> &bool;
    fn d_l_a_a_capture_enable(&self) -> &bool;
    fn sky_celestial_enable(&self) -> &bool;
    fn sky_celestial_quality(&self) -> &super::core::QualityLevel;
    fn sky_celestial_max_quad_count(&self) -> &super::core::QualityScalableInt;
    fn sky_render_mode(&self) -> &SkyRenderMode;
    fn lens_reflection_enable(&self) -> &bool;
    fn dof_before_motion_blur(&self) -> &bool;
    fn lens_dirt_enable(&self) -> &bool;
    fn parallel_create_draw_view_enable(&self) -> &bool;
    fn draw_hologram_with_temporal_a_a(&self) -> &bool;
    fn interpupillary_distance(&self) -> &f32;
    fn vr_hmd_lens_distortion_enable(&self) -> &bool;
    fn vr_hmd_late_reprojection_enable(&self) -> &bool;
    fn enable_persistent_sink_usage(&self) -> &bool;
    fn raytrace_enable(&self) -> &bool;
    fn raytrace_debug_enable(&self) -> &bool;
    fn raytrace_forward_enable(&self) -> &bool;
    fn raytrace_build_async_compute_enable(&self) -> &bool;
    fn raytrace_ao_enable(&self) -> &bool;
    fn raytrace_refl_enable(&self) -> &bool;
    fn raytrace_refl_ray_to_pixel_ratio(&self) -> &f32;
    fn raytrace_refl_transparent_enable(&self) -> &bool;
    fn raytrace_refl_force_min_smoothness(&self) -> &f32;
    fn raytrace_refl_ssr_compare_enable(&self) -> &bool;
    fn raytrace_refl_ssr_compare_invert_side(&self) -> &bool;
    fn raytrace_refl_ssr_compare_location(&self) -> &f32;
    fn raytrace_primary_ray_enable(&self) -> &bool;
    fn raytrace_cull_mode(&self) -> &u32;
    fn raytrace_cull_frustum_fov_y(&self) -> &f32;
    fn raytrace_cull_radius(&self) -> &f32;
    fn raytrace_cull_radius_large_object_scale(&self) -> &f32;
    fn raytrace_cull_frustum_large_object_scale(&self) -> &f32;
    fn raytrace_cull_radius_emitters(&self) -> &f32;
    fn local_light_shadow_enable(&self) -> &bool;
    fn local_light_shadow16_bit_enable(&self) -> &bool;
    fn local_light_shadow_filter_quality(&self) -> &u32;
    fn local_light_shadow_resolution_low(&self) -> &u32;
    fn local_light_shadow_resolution_medium(&self) -> &u32;
    fn local_light_shadow_resolution_high(&self) -> &u32;
    fn local_light_shadow_resolution_ultra(&self) -> &u32;
    fn local_light_shadow_atlas_slot_count(&self) -> &u32;
    fn local_light_shadow_atlas_slot_resolution(&self) -> &u32;
    fn reflection_local_light_shadow_resolution(&self) -> &u32;
    fn local_light_shadow_cache_enable(&self) -> &bool;
    fn max_shadow_count(&self) -> &u32;
    fn max_punctual_light_count(&self) -> &u32;
    fn max_punctual_shadow_light_count(&self) -> &u32;
    fn max_area_light_count(&self) -> &u32;
    fn max_area_shadow_light_count(&self) -> &u32;
    fn max_local_reflection_volume_count(&self) -> &u32;
    fn max_local_planar_reflection_count(&self) -> &u32;
    fn max_punctual_rectangular_light_count(&self) -> &u32;
    fn p_b_r_support_original_light(&self) -> &bool;
    fn radiosity_shadow_culling_enable(&self) -> &bool;
    fn punctual_lights_enable(&self) -> &bool;
    fn area_lights_enable(&self) -> &bool;
    fn local_reflection_enable(&self) -> &bool;
    fn tile_pass_punctual_lights_enable(&self) -> &bool;
    fn tile_pass_punctual_light_shadow_enable(&self) -> &bool;
    fn tile_pass_area_lights_enable(&self) -> &bool;
    fn tile_pass_area_light_shadow_enable(&self) -> &bool;
    fn tile_pass_local_reflection_volume_enable(&self) -> &bool;
    fn tile_pass_local_planar_reflection_enable(&self) -> &bool;
    fn punctual_light_shadow_level(&self) -> &super::core::QualityLevel;
    fn area_light_shadow_level(&self) -> &super::core::QualityLevel;
    fn sphere_lights_enable(&self) -> &bool;
    fn punctual_sphere_lights_enable(&self) -> &bool;
    fn spot_lights_enable(&self) -> &bool;
    fn punctual_spot_lights_enable(&self) -> &bool;
    fn tube_lights_enable(&self) -> &bool;
    fn punctual_tube_lights_enable(&self) -> &bool;
    fn rectangular_lights_enable(&self) -> &bool;
    fn punctual_rectangular_lights_enable(&self) -> &bool;
    fn local_reflection_volume_sphere_enable(&self) -> &bool;
    fn local_reflection_volume_box_enable(&self) -> &bool;
    fn local_planar_reflection_enable(&self) -> &bool;
    fn local_i_b_l_max_face_capture(&self) -> &u32;
    fn local_i_b_l_update_with_sky_enable(&self) -> &bool;
    fn local_i_b_l_update_with_enlighten_sky_box_change(&self) -> &bool;
    fn local_i_b_l_sun_specular_occlusion_enable(&self) -> &bool;
    fn local_i_b_l_lighting_update_count(&self) -> &u32;
    fn local_i_b_l_refresh_delay_count(&self) -> &u32;
    fn local_i_b_l_box_culling_enable(&self) -> &bool;
    fn local_i_b_l_sun_update_threshold(&self) -> &f32;
    fn local_i_b_l_shadowmap_slice_count(&self) -> &u32;
    fn local_i_b_l_shadowmap_resolution(&self) -> &u32;
    fn local_i_b_l_shadowmap_face_merging(&self) -> &bool;
    fn local_i_b_l_shadowmap_separate_frame(&self) -> &bool;
    fn local_i_b_l_wait_for_enlighten_to_render(&self) -> &bool;
    fn local_i_b_l_exposure(&self) -> &f32;
    fn local_i_b_l_render_transparent(&self) -> &bool;
    fn local_i_b_l_render_emitter_quad(&self) -> &bool;
    fn local_i_b_l_render_emitter_mesh(&self) -> &bool;
    fn p_b_r_local_i_b_l_acquisition_wait_for_mesh_loading(&self) -> &bool;
    fn p_b_r_local_i_b_l_acquisition_wait_frame_count(&self) -> &u32;
    fn p_b_r_draw_distant_i_b_l_diffuse_reference(&self) -> &bool;
    fn p_b_r_draw_distant_i_b_l_specular_reference(&self) -> &bool;
    fn p_b_r_draw_local_i_b_l_reference(&self) -> &bool;
    fn p_b_r_draw_area_light_reference(&self) -> &bool;
    fn p_b_r_specular_convolution_sample_count(&self) -> &u32;
    fn p_b_r_convolution_highest_m_i_p_enable(&self) -> &bool;
    fn local_i_b_l_resolution(&self) -> &u32;
    fn draw_debug_local_i_b_l_enable(&self) -> &bool;
    fn draw_debug_local_i_b_l_stats_enable(&self) -> &bool;
    fn draw_debug_local_i_b_l_volumes_enable(&self) -> &bool;
    fn draw_debug_local_i_b_l_preview_scale(&self) -> &f32;
    fn draw_debug_local_i_b_l_index(&self) -> &u32;
    fn draw_debug_local_i_b_l_mip_level(&self) -> &u32;
    fn draw_debug_local_i_b_l_shadowmaps(&self) -> &bool;
    fn draw_debug_pre_integrated_f_g_texture(&self) -> &bool;
    fn draw_debug_reflection_state(&self) -> &bool;
    fn draw_debug_probe_mirror_enable(&self) -> &bool;
    fn draw_debug_probe_diffuse_enable(&self) -> &bool;
    fn draw_debug_probe_screen_enable(&self) -> &bool;
    fn draw_debug_probe_screen_on_right(&self) -> &bool;
    fn continuous_local_i_b_l_enable(&self) -> &bool;
    fn continuous_local_i_b_l_index(&self) -> &u32;
    fn p_b_r_convolution_precomputed_sample_enable(&self) -> &bool;
    fn p_b_r_convolution_random_rotation_enable(&self) -> &bool;
    fn draw_debug_local_planar_reflections(&self) -> &bool;
    fn emitter_quad_rendering_enable(&self) -> &bool;
    fn emitter_mesh_rendering_enable(&self) -> &bool;
    fn emitter_point_lights_enable(&self) -> &bool;
    fn emitter_spot_lights_enable(&self) -> &bool;
    fn use_s_s_s_profilefor_o_a_t_s(&self) -> &bool;
    fn deterministic_rendering_enable(&self) -> &bool;
    fn hdr_nan_inf_removal_enable(&self) -> &bool;
    fn hdr_nan_inf_removal_force_enable(&self) -> &bool;
    fn p_b_r_max_illuminance_value(&self) -> &f32;
    fn p_b_r_max_luminance_value(&self) -> &f32;
    fn dielectric_range_min_metal_mask(&self) -> &f32;
    fn dielectric_range_max_metal_mask(&self) -> &f32;
    fn dielectric_range_s_r_g_b_min_value(&self) -> &f32;
    fn dielectric_range_s_r_g_b_max_value(&self) -> &f32;
    fn dielectric_range_s_r_g_b_min_color(&self) -> &super::core::Vec3;
    fn dielectric_range_s_r_g_b_color(&self) -> &super::core::Vec3;
    fn dielectric_range_s_r_g_b_max_color(&self) -> &super::core::Vec3;
    fn dielectric_range_overlay(&self) -> &f32;
    fn conductor_range_min_metal_mask(&self) -> &f32;
    fn conductor_range_max_metal_mask(&self) -> &f32;
    fn conductor_range_s_r_g_b_min_value(&self) -> &f32;
    fn conductor_range_s_r_g_b_max_value(&self) -> &f32;
    fn conductor_range_s_r_g_b_min_color(&self) -> &super::core::Vec3;
    fn conductor_range_s_r_g_b_color(&self) -> &super::core::Vec3;
    fn conductor_range_s_r_g_b_max_color(&self) -> &super::core::Vec3;
    fn conductor_range_overlay(&self) -> &f32;
    fn fresnel0_range_min_metal_mask(&self) -> &f32;
    fn fresnel0_range_max_metal_mask(&self) -> &f32;
    fn volumetric_rendering_enable(&self) -> &bool;
    fn volumetric_extinction_cascade_base_voxel_size(&self) -> &f32;
    fn volumetric_extinction_cascade_voxel_size_cascade_factor(&self) -> &f32;
    fn volumetric_extinction_cascade_resolution(&self) -> &u32;
    fn volumetric_extinction_cascade_lock_view(&self) -> &bool;
    fn volumetric_shadowmap_enable(&self) -> &bool;
    fn volumetric_shadowmap_resolution(&self) -> &u32;
    fn volumetric_shadowmap_max_count(&self) -> &u32;
    fn volumetric_shadowmap_accumulate_cs_enable(&self) -> &bool;
    fn punctual_light_cast_volumetric_shadowmap_enable_level(&self) -> &super::core::QualityLevel;
    fn area_light_cast_volumetric_shadowmap_enable_level(&self) -> &super::core::QualityLevel;
    fn local_light_cast_volumetric_level(&self) -> &super::core::QualityLevel;
    fn volumetric_emitter_voxelisation_enable(&self) -> &bool;
    fn volumetric_emitter_voxelisation_mode(&self) -> &u32;
    fn volumetric_participating_media_enable(&self) -> &bool;
    fn volumetric_participating_media_use_light_cull(&self) -> &bool;
    fn volumetric_participating_media_texture_depth(&self) -> &u32;
    fn volumetric_participating_media_camera_depth(&self) -> &f32;
    fn volumetric_participating_media_resolution_scale(&self) -> &u32;
    fn volumetric_participating_media_from_v_e_fog(&self) -> &bool;
    fn volumetric_participating_media_lock_view(&self) -> &bool;
    fn volumetric_participating_media_local_lights(&self) -> &bool;
    fn volumetric_participating_media_ambient_lighting(&self) -> &bool;
    fn volumetric_participating_media_sun(&self) -> &bool;
    fn reflection_volumetric_participating_media_texture_depth(&self) -> &u32;
    fn reflection_volumetric_participating_media_camera_depth(&self) -> &f32;
    fn reflection_volumetric_participating_media_resolution_scale(&self) -> &u32;
    fn volumetric_participating_media_temporal_integration(&self) -> &bool;
    fn volumetric_participating_media_temporal_filter_strght(&self) -> &f32;
    fn volumetric_local_fog_volume_enable(&self) -> &bool;
    fn draw_debug_volumetric_local_fog_volume(&self) -> &bool;
    fn draw_debug_volumetric_cascaded_volumes(&self) -> &bool;
    fn draw_debug_volumetric_shadow_maps(&self) -> &bool;
    fn draw_debug_volumetric_extinction(&self) -> &u32;
    fn draw_debug_volumetric_extinction_scale(&self) -> &f32;
    fn draw_debug_volumetric_voxelised_emitter(&self) -> &bool;
    fn light_shaft_fast_hdr_enable(&self) -> &bool;
    fn draw_gpu_histogram_enable(&self) -> &bool;
    fn draw_gpu_histogram_red(&self) -> &bool;
    fn draw_gpu_histogram_blue(&self) -> &bool;
    fn draw_gpu_histogram_green(&self) -> &bool;
    fn draw_gpu_histogram_luminance(&self) -> &bool;
    fn draw_gpu_histogram_h_d_r_mode(&self) -> &bool;
    fn draw_gpu_histogram_h_d_r_min_e_v(&self) -> &f32;
    fn draw_gpu_histogram_h_d_r_max_e_v(&self) -> &f32;
    fn draw_gpu_histogram_bin_count(&self) -> &u32;
    fn draw_clamped_pixels_enable(&self) -> &bool;
    fn draw_clamped_pixels_clamp_min(&self) -> &f32;
    fn draw_clamped_pixels_clamp_max(&self) -> &f32;
    fn enable_baked_direct_light_in_game_view(&self) -> &bool;
    fn enable_baked_lightmap_in_game_view(&self) -> &bool;
    fn texture_space_render_module_enable(&self) -> &bool;
    fn compute_linearize_depth(&self) -> &bool;
    fn compute_downsample_depth(&self) -> &bool;
    fn draw_debug_disable_explanation(&self) -> &bool;
}

impl WorldRenderSettingsTrait for WorldRenderSettings {
    fn test_rendering_enable(&self) -> &bool {
        &self.test_rendering_enable
    }
    fn generic_entity_renderer_enable(&self) -> &bool {
        &self.generic_entity_renderer_enable
    }
    fn generic_entity_max_visible_entity_count(&self) -> &u32 {
        &self.generic_entity_max_visible_entity_count
    }
    fn z_buffer_shadow_test_enable(&self) -> &bool {
        &self.z_buffer_shadow_test_enable
    }
    fn draw_debug_ground_height(&self) -> &u32 {
        &self.draw_debug_ground_height
    }
    fn decal_volume_enable(&self) -> &bool {
        &self.decal_volume_enable
    }
    fn decal_volume_scale(&self) -> &f32 {
        &self.decal_volume_scale
    }
    fn draw_debug_decal_volumes(&self) -> &bool {
        &self.draw_debug_decal_volumes
    }
    fn environment_decal_volumes_enabled(&self) -> &bool {
        &self.environment_decal_volumes_enabled
    }
    fn environment_decal_volume_quality(&self) -> &super::core::QualityLevel {
        &self.environment_decal_volume_quality
    }
    fn environment_decal_volume_max_count(&self) -> &super::core::QualityScalableInt {
        &self.environment_decal_volume_max_count
    }
    fn emitter_decal_volume_quality(&self) -> &super::core::QualityLevel {
        &self.emitter_decal_volume_quality
    }
    fn emitter_decal_volume_max_count(&self) -> &super::core::QualityScalableInt {
        &self.emitter_decal_volume_max_count
    }
    fn emitter_decal_volume_max_count_per_set(&self) -> &super::core::QualityScalableInt {
        &self.emitter_decal_volume_max_count_per_set
    }
    fn edge_models_enable(&self) -> &bool {
        &self.edge_models_enable
    }
    fn edge_model_cast_shadows_enable(&self) -> &bool {
        &self.edge_model_cast_shadows_enable
    }
    fn edge_model_depth_bias_enable(&self) -> &bool {
        &self.edge_model_depth_bias_enable
    }
    fn edge_model_shadow_depth_bias_enable(&self) -> &bool {
        &self.edge_model_shadow_depth_bias_enable
    }
    fn edge_model_view_distance(&self) -> &f32 {
        &self.edge_model_view_distance
    }
    fn edge_model_use_main_lod_enable(&self) -> &bool {
        &self.edge_model_use_main_lod_enable
    }
    fn edge_model_force_lod(&self) -> &i32 {
        &self.edge_model_force_lod
    }
    fn edge_model_lod_scale(&self) -> &f32 {
        &self.edge_model_lod_scale
    }
    fn lens_flares_enable(&self) -> &bool {
        &self.lens_flares_enable
    }
    fn draw_debug_lens_flare_occluders(&self) -> &bool {
        &self.draw_debug_lens_flare_occluders
    }
    fn draw_debug_lens_flares(&self) -> &bool {
        &self.draw_debug_lens_flares
    }
    fn lens_flare_occlusion_enable(&self) -> &bool {
        &self.lens_flare_occlusion_enable
    }
    fn max_lens_flares_per_frame(&self) -> &u32 {
        &self.max_lens_flares_per_frame
    }
    fn lens_flares_quality_level(&self) -> &super::core::QualityLevel {
        &self.lens_flares_quality_level
    }
    fn cloud_shadow_enable(&self) -> &bool {
        &self.cloud_shadow_enable
    }
    fn override_dynamic_a_o(&self) -> &bool {
        &self.override_dynamic_a_o
    }
    fn draw_debug_dynamic_a_o(&self) -> &bool {
        &self.draw_debug_dynamic_a_o
    }
    fn draw_debug_raytrace_refl(&self) -> &bool {
        &self.draw_debug_raytrace_refl
    }
    fn draw_debug_raytrace_a_o(&self) -> &bool {
        &self.draw_debug_raytrace_a_o
    }
    fn draw_debug_raytrace_primary_ray(&self) -> &bool {
        &self.draw_debug_raytrace_primary_ray
    }
    fn filmic_effects_enable(&self) -> &bool {
        &self.filmic_effects_enable
    }
    fn emissive_enable(&self) -> &bool {
        &self.emissive_enable
    }
    fn g_buffer_layout(&self) -> &u32 {
        &self.g_buffer_layout
    }
    fn g_buffer_test_count(&self) -> &u32 {
        &self.g_buffer_test_count
    }
    fn g_buffer_clear_enable(&self) -> &bool {
        &self.g_buffer_clear_enable
    }
    fn dx_g_buffer_light16_bit_enable(&self) -> &bool {
        &self.dx_g_buffer_light16_bit_enable
    }
    fn dx_g_buffer_normal16_bit_enable(&self) -> &bool {
        &self.dx_g_buffer_normal16_bit_enable
    }
    fn dx_g_buffer_roughness16_bit_enable(&self) -> &bool {
        &self.dx_g_buffer_roughness16_bit_enable
    }
    fn g_buffer_alpha_test_simple_enable(&self) -> &bool {
        &self.g_buffer_alpha_test_simple_enable
    }
    fn g_buffer_alpha_test_simple_smoothness(&self) -> &f32 {
        &self.g_buffer_alpha_test_simple_smoothness
    }
    fn g_buffer_force_metal_mask(&self) -> &f32 {
        &self.g_buffer_force_metal_mask
    }
    fn g_buffer_force_smoothness(&self) -> &f32 {
        &self.g_buffer_force_smoothness
    }
    fn g_buffer_force_specular_albedo(&self) -> &f32 {
        &self.g_buffer_force_specular_albedo
    }
    fn alpha_unroll_enable(&self) -> &bool {
        &self.alpha_unroll_enable
    }
    fn gen4a_esram_enable(&self) -> &bool {
        &self.gen4a_esram_enable
    }
    fn specular_lighting_enable(&self) -> &bool {
        &self.specular_lighting_enable
    }
    fn skin_lighting_enable(&self) -> &bool {
        &self.skin_lighting_enable
    }
    fn translucency_lighting_enable(&self) -> &bool {
        &self.translucency_lighting_enable
    }
    fn translucency_auto_thickness_enable(&self) -> &bool {
        &self.translucency_auto_thickness_enable
    }
    fn local_light_translucency_enable(&self) -> &bool {
        &self.local_light_translucency_enable
    }
    fn dynamic_envmap_lighting_enable(&self) -> &bool {
        &self.dynamic_envmap_lighting_enable
    }
    fn outdoor_light_enable(&self) -> &bool {
        &self.outdoor_light_enable
    }
    fn light_stencil_method_enable(&self) -> &bool {
        &self.light_stencil_method_enable
    }
    fn light_volume_method_enable(&self) -> &bool {
        &self.light_volume_method_enable
    }
    fn light_volume_depth_test_enable(&self) -> &bool {
        &self.light_volume_depth_test_enable
    }
    fn outdoor_key_light_enable(&self) -> &bool {
        &self.outdoor_key_light_enable
    }
    fn outdoor_sky_light_enable(&self) -> &bool {
        &self.outdoor_sky_light_enable
    }
    fn outdoor_light_tile_blend_enable(&self) -> &bool {
        &self.outdoor_light_tile_blend_enable
    }
    fn emitter_sun_transmittance_map_enable(&self) -> &bool {
        &self.emitter_sun_transmittance_map_enable
    }
    fn emitter_sun_transmittance_map_resolution(&self) -> &u32 {
        &self.emitter_sun_transmittance_map_resolution
    }
    fn max_decal_volume_count(&self) -> &u32 {
        &self.max_decal_volume_count
    }
    fn light_tile_combine_outdoor_light_enable(&self) -> &bool {
        &self.light_tile_combine_outdoor_light_enable
    }
    fn light_tile_cs_path_enable(&self) -> &bool {
        &self.light_tile_cs_path_enable
    }
    fn light_tile_ps_path_enable(&self) -> &bool {
        &self.light_tile_ps_path_enable
    }
    fn light_tile_async_compute_culling(&self) -> &bool {
        &self.light_tile_async_compute_culling
    }
    fn light_tile_cs_avg_light_count_per_tile(&self) -> &u32 {
        &self.light_tile_cs_avg_light_count_per_tile
    }
    fn light_tile_min_max_use_h_tile(&self) -> &bool {
        &self.light_tile_min_max_use_h_tile
    }
    fn light_tile_use_culling_hierarchy(&self) -> &bool {
        &self.light_tile_use_culling_hierarchy
    }
    fn light_tile_use_detailed_gpu_timers(&self) -> &bool {
        &self.light_tile_use_detailed_gpu_timers
    }
    fn light_tile_use_cs_indirect_clears(&self) -> &bool {
        &self.light_tile_use_cs_indirect_clears
    }
    fn light_cull_frustum_expand_distance(&self) -> &f32 {
        &self.light_cull_frustum_expand_distance
    }
    fn light_tile_debug_light_mode(&self) -> &LightTileDebugLightCountMode {
        &self.light_tile_debug_light_mode
    }
    fn light_tile_debug_color_mode(&self) -> &i32 {
        &self.light_tile_debug_color_mode
    }
    fn draw_debug_light_stats(&self) -> &bool {
        &self.draw_debug_light_stats
    }
    fn draw_debug_light_stats_forward(&self) -> &bool {
        &self.draw_debug_light_stats_forward
    }
    fn draw_debug_light_stats_volumetric(&self) -> &bool {
        &self.draw_debug_light_stats_volumetric
    }
    fn draw_debug_light_stats_hierarchy(&self) -> &bool {
        &self.draw_debug_light_stats_hierarchy
    }
    fn draw_debug_light_sources(&self) -> &bool {
        &self.draw_debug_light_sources
    }
    fn draw_debug_light_shadow_sources(&self) -> &bool {
        &self.draw_debug_light_shadow_sources
    }
    fn draw_debug_light_shadow_stats(&self) -> &bool {
        &self.draw_debug_light_shadow_stats
    }
    fn draw_debug_light_cull_stats(&self) -> &bool {
        &self.draw_debug_light_cull_stats
    }
    fn draw_debug_g_buffer(&self) -> &bool {
        &self.draw_debug_g_buffer
    }
    fn draw_debug_material_input(&self) -> &bool {
        &self.draw_debug_material_input
    }
    fn draw_debug_material_output(&self) -> &bool {
        &self.draw_debug_material_output
    }
    fn draw_debug_light_emissive_surface(&self) -> &bool {
        &self.draw_debug_light_emissive_surface
    }
    fn punctual_emissive_light_shape_min_size(&self) -> &f32 {
        &self.punctual_emissive_light_shape_min_size
    }
    fn debug_light_stats_light_count_highwatermark(&self) -> &u32 {
        &self.debug_light_stats_light_count_highwatermark
    }
    fn light_lod_fade_area(&self) -> &f32 {
        &self.light_lod_fade_area
    }
    fn light_lod_min_area(&self) -> &f32 {
        &self.light_lod_min_area
    }
    fn light_lod_radius_factor(&self) -> &f32 {
        &self.light_lod_radius_factor
    }
    fn use_new_light_cull_enable(&self) -> &bool {
        &self.use_new_light_cull_enable
    }
    fn light_cull_enable(&self) -> &bool {
        &self.light_cull_enable
    }
    fn light_cull_job_max_job_count(&self) -> &u32 {
        &self.light_cull_job_max_job_count
    }
    fn light_occlusion_cull_enable(&self) -> &bool {
        &self.light_occlusion_cull_enable
    }
    fn light_cone_cull_enable(&self) -> &bool {
        &self.light_cone_cull_enable
    }
    fn occlusion_culling_width(&self) -> &u32 {
        &self.occlusion_culling_width
    }
    fn occlusion_culling_height(&self) -> &u32 {
        &self.occlusion_culling_height
    }
    fn occlusion_triangle_count(&self) -> &u32 {
        &self.occlusion_triangle_count
    }
    fn local_i_b_l_occlusion_culling_enable(&self) -> &bool {
        &self.local_i_b_l_occlusion_culling_enable
    }
    fn shadow_occlusion_culling_enable(&self) -> &bool {
        &self.shadow_occlusion_culling_enable
    }
    fn shadow_occlusion_culling_width(&self) -> &u32 {
        &self.shadow_occlusion_culling_width
    }
    fn shadow_occlusion_culling_height(&self) -> &u32 {
        &self.shadow_occlusion_culling_height
    }
    fn shadow_occlusion_triangle_count(&self) -> &u32 {
        &self.shadow_occlusion_triangle_count
    }
    fn frustum_silhouette_culling_enable(&self) -> &bool {
        &self.frustum_silhouette_culling_enable
    }
    fn frustum_silhouette_culling_padding(&self) -> &f32 {
        &self.frustum_silhouette_culling_padding
    }
    fn sub_surface_scattering_enable(&self) -> &bool {
        &self.sub_surface_scattering_enable
    }
    fn translucency_enable(&self) -> &bool {
        &self.translucency_enable
    }
    fn sub_surface_scattering_sample_count(&self) -> &i32 {
        &self.sub_surface_scattering_sample_count
    }
    fn split_lighting_enable(&self) -> &bool {
        &self.split_lighting_enable
    }
    fn subsurface_blur_compute_enable(&self) -> &bool {
        &self.subsurface_blur_compute_enable
    }
    fn subsurface_blur_quadtree_tile_gen_enable(&self) -> &bool {
        &self.subsurface_blur_quadtree_tile_gen_enable
    }
    fn subsurface_blur_pixel_radius_cull_threshold(&self) -> &f32 {
        &self.subsurface_blur_pixel_radius_cull_threshold
    }
    fn opaque_sort_by_solution_enable(&self) -> &bool {
        &self.opaque_sort_by_solution_enable
    }
    fn main_opaque_z_pass_enable(&self) -> &bool {
        &self.main_opaque_z_pass_enable
    }
    fn planar_reflection_enable(&self) -> &bool {
        &self.planar_reflection_enable
    }
    fn planar_reflection_fast_hdr_enable(&self) -> &bool {
        &self.planar_reflection_fast_hdr_enable
    }
    fn planar_reflection_view_scale(&self) -> &f32 {
        &self.planar_reflection_view_scale
    }
    fn planar_reflection_blur_enable(&self) -> &bool {
        &self.planar_reflection_blur_enable
    }
    fn planar_reflection_convolution_enable(&self) -> &bool {
        &self.planar_reflection_convolution_enable
    }
    fn planar_reflection_convolution_sample_clamp_threshold(&self) -> &f32 {
        &self.planar_reflection_convolution_sample_clamp_threshold
    }
    fn planar_reflection_convolution_sample_count(&self) -> &u32 {
        &self.planar_reflection_convolution_sample_count
    }
    fn planar_reflection_convolution_random_samples_enable(&self) -> &bool {
        &self.planar_reflection_convolution_random_samples_enable
    }
    fn planar_reflection_convolution_post_filter_enable(&self) -> &bool {
        &self.planar_reflection_convolution_post_filter_enable
    }
    fn planar_reflection_cull_f_o_v(&self) -> &f32 {
        &self.planar_reflection_cull_f_o_v
    }
    fn planar_reflection_clipping_enable(&self) -> &bool {
        &self.planar_reflection_clipping_enable
    }
    fn draw_debug_render_texture(&self) -> &bool {
        &self.draw_debug_render_texture
    }
    fn draw_debug_planar_reflection(&self) -> &bool {
        &self.draw_debug_planar_reflection
    }
    fn draw_debug_planar_reflection_mip_level(&self) -> &u32 {
        &self.draw_debug_planar_reflection_mip_level
    }
    fn draw_debug_planar_reflection_mode(&self) -> &u32 {
        &self.draw_debug_planar_reflection_mode
    }
    fn draw_debug_planar_reflection_cull_frustum(&self) -> &bool {
        &self.draw_debug_planar_reflection_cull_frustum
    }
    fn local_planar_reflection_convolution_enable(&self) -> &bool {
        &self.local_planar_reflection_convolution_enable
    }
    fn local_planar_reflection_force_lowest_lod_enable(&self) -> &bool {
        &self.local_planar_reflection_force_lowest_lod_enable
    }
    fn reflection_lod_scale(&self) -> &f32 {
        &self.reflection_lod_scale
    }
    fn object_highlight_enable(&self) -> &bool {
        &self.object_highlight_enable
    }
    fn object_highlight_mask_first_person_enable(&self) -> &bool {
        &self.object_highlight_mask_first_person_enable
    }
    fn transparent_depth_z_prepass_enable(&self) -> &bool {
        &self.transparent_depth_z_prepass_enable
    }
    fn hologram_rendertarget_enable(&self) -> &bool {
        &self.hologram_rendertarget_enable
    }
    fn sonar_rendertarget_enable(&self) -> &bool {
        &self.sonar_rendertarget_enable
    }
    fn simple_volumetrics_enable(&self) -> &bool {
        &self.simple_volumetrics_enable
    }
    fn simple_volumetrics_half_res_enable(&self) -> &bool {
        &self.simple_volumetrics_half_res_enable
    }
    fn fog_exclusion_volume_enable(&self) -> &bool {
        &self.fog_exclusion_volume_enable
    }
    fn threat_alert_overlay_enable(&self) -> &bool {
        &self.threat_alert_overlay_enable
    }
    fn mesh_compute_enabled(&self) -> &bool {
        &self.mesh_compute_enabled
    }
    fn outline_edge_detect_enable(&self) -> &bool {
        &self.outline_edge_detect_enable
    }
    fn overlay_blur_enable(&self) -> &bool {
        &self.overlay_blur_enable
    }
    fn overlay_blur_async_compute_enable(&self) -> &bool {
        &self.overlay_blur_async_compute_enable
    }
    fn overlay_blur_kernel_radius(&self) -> &u32 {
        &self.overlay_blur_kernel_radius
    }
    fn post_process_antialiasing_mode(&self) -> &PostProcessAAMode {
        &self.post_process_antialiasing_mode
    }
    fn smaa_predicated_thresholding_enable(&self) -> &bool {
        &self.smaa_predicated_thresholding_enable
    }
    fn temporal_a_a_jitter_count(&self) -> &u32 {
        &self.temporal_a_a_jitter_count
    }
    fn temporal_a_a_disocclusion_rejection_factor(&self) -> &f32 {
        &self.temporal_a_a_disocclusion_rejection_factor
    }
    fn temporal_a_a_history_sharpening_enable(&self) -> &bool {
        &self.temporal_a_a_history_sharpening_enable
    }
    fn temporal_a_a_motion_sharpening_factor(&self) -> &f32 {
        &self.temporal_a_a_motion_sharpening_factor
    }
    fn temporal_a_a_responsiveness(&self) -> &f32 {
        &self.temporal_a_a_responsiveness
    }
    fn temporal_a_a_antiflicker_strength(&self) -> &f32 {
        &self.temporal_a_a_antiflicker_strength
    }
    fn temporal_a_a_quality(&self) -> &u32 {
        &self.temporal_a_a_quality
    }
    fn temporal_a_a_post_sharpening_amount(&self) -> &f32 {
        &self.temporal_a_a_post_sharpening_amount
    }
    fn draw_debug_temporal_a_a_enable(&self) -> &bool {
        &self.draw_debug_temporal_a_a_enable
    }
    fn draw_debug_temporal_a_a_accumulation_count(&self) -> &u32 {
        &self.draw_debug_temporal_a_a_accumulation_count
    }
    fn draw_debug_temporal_a_a_debug_mode(&self) -> &u32 {
        &self.draw_debug_temporal_a_a_debug_mode
    }
    fn draw_debug_temporal_a_a_max_distance(&self) -> &f32 {
        &self.draw_debug_temporal_a_a_max_distance
    }
    fn temporal_a_a_dof_coc_filter_enable(&self) -> &bool {
        &self.temporal_a_a_dof_coc_filter_enable
    }
    fn d_l_a_a_jitter_count(&self) -> &u32 {
        &self.d_l_a_a_jitter_count
    }
    fn d_l_a_a_jitter_scale(&self) -> &f32 {
        &self.d_l_a_a_jitter_scale
    }
    fn d_l_a_a_draw_enable(&self) -> &bool {
        &self.d_l_a_a_draw_enable
    }
    fn d_l_a_a_capture_enable(&self) -> &bool {
        &self.d_l_a_a_capture_enable
    }
    fn sky_celestial_enable(&self) -> &bool {
        &self.sky_celestial_enable
    }
    fn sky_celestial_quality(&self) -> &super::core::QualityLevel {
        &self.sky_celestial_quality
    }
    fn sky_celestial_max_quad_count(&self) -> &super::core::QualityScalableInt {
        &self.sky_celestial_max_quad_count
    }
    fn sky_render_mode(&self) -> &SkyRenderMode {
        &self.sky_render_mode
    }
    fn lens_reflection_enable(&self) -> &bool {
        &self.lens_reflection_enable
    }
    fn dof_before_motion_blur(&self) -> &bool {
        &self.dof_before_motion_blur
    }
    fn lens_dirt_enable(&self) -> &bool {
        &self.lens_dirt_enable
    }
    fn parallel_create_draw_view_enable(&self) -> &bool {
        &self.parallel_create_draw_view_enable
    }
    fn draw_hologram_with_temporal_a_a(&self) -> &bool {
        &self.draw_hologram_with_temporal_a_a
    }
    fn interpupillary_distance(&self) -> &f32 {
        &self.interpupillary_distance
    }
    fn vr_hmd_lens_distortion_enable(&self) -> &bool {
        &self.vr_hmd_lens_distortion_enable
    }
    fn vr_hmd_late_reprojection_enable(&self) -> &bool {
        &self.vr_hmd_late_reprojection_enable
    }
    fn enable_persistent_sink_usage(&self) -> &bool {
        &self.enable_persistent_sink_usage
    }
    fn raytrace_enable(&self) -> &bool {
        &self.raytrace_enable
    }
    fn raytrace_debug_enable(&self) -> &bool {
        &self.raytrace_debug_enable
    }
    fn raytrace_forward_enable(&self) -> &bool {
        &self.raytrace_forward_enable
    }
    fn raytrace_build_async_compute_enable(&self) -> &bool {
        &self.raytrace_build_async_compute_enable
    }
    fn raytrace_ao_enable(&self) -> &bool {
        &self.raytrace_ao_enable
    }
    fn raytrace_refl_enable(&self) -> &bool {
        &self.raytrace_refl_enable
    }
    fn raytrace_refl_ray_to_pixel_ratio(&self) -> &f32 {
        &self.raytrace_refl_ray_to_pixel_ratio
    }
    fn raytrace_refl_transparent_enable(&self) -> &bool {
        &self.raytrace_refl_transparent_enable
    }
    fn raytrace_refl_force_min_smoothness(&self) -> &f32 {
        &self.raytrace_refl_force_min_smoothness
    }
    fn raytrace_refl_ssr_compare_enable(&self) -> &bool {
        &self.raytrace_refl_ssr_compare_enable
    }
    fn raytrace_refl_ssr_compare_invert_side(&self) -> &bool {
        &self.raytrace_refl_ssr_compare_invert_side
    }
    fn raytrace_refl_ssr_compare_location(&self) -> &f32 {
        &self.raytrace_refl_ssr_compare_location
    }
    fn raytrace_primary_ray_enable(&self) -> &bool {
        &self.raytrace_primary_ray_enable
    }
    fn raytrace_cull_mode(&self) -> &u32 {
        &self.raytrace_cull_mode
    }
    fn raytrace_cull_frustum_fov_y(&self) -> &f32 {
        &self.raytrace_cull_frustum_fov_y
    }
    fn raytrace_cull_radius(&self) -> &f32 {
        &self.raytrace_cull_radius
    }
    fn raytrace_cull_radius_large_object_scale(&self) -> &f32 {
        &self.raytrace_cull_radius_large_object_scale
    }
    fn raytrace_cull_frustum_large_object_scale(&self) -> &f32 {
        &self.raytrace_cull_frustum_large_object_scale
    }
    fn raytrace_cull_radius_emitters(&self) -> &f32 {
        &self.raytrace_cull_radius_emitters
    }
    fn local_light_shadow_enable(&self) -> &bool {
        &self.local_light_shadow_enable
    }
    fn local_light_shadow16_bit_enable(&self) -> &bool {
        &self.local_light_shadow16_bit_enable
    }
    fn local_light_shadow_filter_quality(&self) -> &u32 {
        &self.local_light_shadow_filter_quality
    }
    fn local_light_shadow_resolution_low(&self) -> &u32 {
        &self.local_light_shadow_resolution_low
    }
    fn local_light_shadow_resolution_medium(&self) -> &u32 {
        &self.local_light_shadow_resolution_medium
    }
    fn local_light_shadow_resolution_high(&self) -> &u32 {
        &self.local_light_shadow_resolution_high
    }
    fn local_light_shadow_resolution_ultra(&self) -> &u32 {
        &self.local_light_shadow_resolution_ultra
    }
    fn local_light_shadow_atlas_slot_count(&self) -> &u32 {
        &self.local_light_shadow_atlas_slot_count
    }
    fn local_light_shadow_atlas_slot_resolution(&self) -> &u32 {
        &self.local_light_shadow_atlas_slot_resolution
    }
    fn reflection_local_light_shadow_resolution(&self) -> &u32 {
        &self.reflection_local_light_shadow_resolution
    }
    fn local_light_shadow_cache_enable(&self) -> &bool {
        &self.local_light_shadow_cache_enable
    }
    fn max_shadow_count(&self) -> &u32 {
        &self.max_shadow_count
    }
    fn max_punctual_light_count(&self) -> &u32 {
        &self.max_punctual_light_count
    }
    fn max_punctual_shadow_light_count(&self) -> &u32 {
        &self.max_punctual_shadow_light_count
    }
    fn max_area_light_count(&self) -> &u32 {
        &self.max_area_light_count
    }
    fn max_area_shadow_light_count(&self) -> &u32 {
        &self.max_area_shadow_light_count
    }
    fn max_local_reflection_volume_count(&self) -> &u32 {
        &self.max_local_reflection_volume_count
    }
    fn max_local_planar_reflection_count(&self) -> &u32 {
        &self.max_local_planar_reflection_count
    }
    fn max_punctual_rectangular_light_count(&self) -> &u32 {
        &self.max_punctual_rectangular_light_count
    }
    fn p_b_r_support_original_light(&self) -> &bool {
        &self.p_b_r_support_original_light
    }
    fn radiosity_shadow_culling_enable(&self) -> &bool {
        &self.radiosity_shadow_culling_enable
    }
    fn punctual_lights_enable(&self) -> &bool {
        &self.punctual_lights_enable
    }
    fn area_lights_enable(&self) -> &bool {
        &self.area_lights_enable
    }
    fn local_reflection_enable(&self) -> &bool {
        &self.local_reflection_enable
    }
    fn tile_pass_punctual_lights_enable(&self) -> &bool {
        &self.tile_pass_punctual_lights_enable
    }
    fn tile_pass_punctual_light_shadow_enable(&self) -> &bool {
        &self.tile_pass_punctual_light_shadow_enable
    }
    fn tile_pass_area_lights_enable(&self) -> &bool {
        &self.tile_pass_area_lights_enable
    }
    fn tile_pass_area_light_shadow_enable(&self) -> &bool {
        &self.tile_pass_area_light_shadow_enable
    }
    fn tile_pass_local_reflection_volume_enable(&self) -> &bool {
        &self.tile_pass_local_reflection_volume_enable
    }
    fn tile_pass_local_planar_reflection_enable(&self) -> &bool {
        &self.tile_pass_local_planar_reflection_enable
    }
    fn punctual_light_shadow_level(&self) -> &super::core::QualityLevel {
        &self.punctual_light_shadow_level
    }
    fn area_light_shadow_level(&self) -> &super::core::QualityLevel {
        &self.area_light_shadow_level
    }
    fn sphere_lights_enable(&self) -> &bool {
        &self.sphere_lights_enable
    }
    fn punctual_sphere_lights_enable(&self) -> &bool {
        &self.punctual_sphere_lights_enable
    }
    fn spot_lights_enable(&self) -> &bool {
        &self.spot_lights_enable
    }
    fn punctual_spot_lights_enable(&self) -> &bool {
        &self.punctual_spot_lights_enable
    }
    fn tube_lights_enable(&self) -> &bool {
        &self.tube_lights_enable
    }
    fn punctual_tube_lights_enable(&self) -> &bool {
        &self.punctual_tube_lights_enable
    }
    fn rectangular_lights_enable(&self) -> &bool {
        &self.rectangular_lights_enable
    }
    fn punctual_rectangular_lights_enable(&self) -> &bool {
        &self.punctual_rectangular_lights_enable
    }
    fn local_reflection_volume_sphere_enable(&self) -> &bool {
        &self.local_reflection_volume_sphere_enable
    }
    fn local_reflection_volume_box_enable(&self) -> &bool {
        &self.local_reflection_volume_box_enable
    }
    fn local_planar_reflection_enable(&self) -> &bool {
        &self.local_planar_reflection_enable
    }
    fn local_i_b_l_max_face_capture(&self) -> &u32 {
        &self.local_i_b_l_max_face_capture
    }
    fn local_i_b_l_update_with_sky_enable(&self) -> &bool {
        &self.local_i_b_l_update_with_sky_enable
    }
    fn local_i_b_l_update_with_enlighten_sky_box_change(&self) -> &bool {
        &self.local_i_b_l_update_with_enlighten_sky_box_change
    }
    fn local_i_b_l_sun_specular_occlusion_enable(&self) -> &bool {
        &self.local_i_b_l_sun_specular_occlusion_enable
    }
    fn local_i_b_l_lighting_update_count(&self) -> &u32 {
        &self.local_i_b_l_lighting_update_count
    }
    fn local_i_b_l_refresh_delay_count(&self) -> &u32 {
        &self.local_i_b_l_refresh_delay_count
    }
    fn local_i_b_l_box_culling_enable(&self) -> &bool {
        &self.local_i_b_l_box_culling_enable
    }
    fn local_i_b_l_sun_update_threshold(&self) -> &f32 {
        &self.local_i_b_l_sun_update_threshold
    }
    fn local_i_b_l_shadowmap_slice_count(&self) -> &u32 {
        &self.local_i_b_l_shadowmap_slice_count
    }
    fn local_i_b_l_shadowmap_resolution(&self) -> &u32 {
        &self.local_i_b_l_shadowmap_resolution
    }
    fn local_i_b_l_shadowmap_face_merging(&self) -> &bool {
        &self.local_i_b_l_shadowmap_face_merging
    }
    fn local_i_b_l_shadowmap_separate_frame(&self) -> &bool {
        &self.local_i_b_l_shadowmap_separate_frame
    }
    fn local_i_b_l_wait_for_enlighten_to_render(&self) -> &bool {
        &self.local_i_b_l_wait_for_enlighten_to_render
    }
    fn local_i_b_l_exposure(&self) -> &f32 {
        &self.local_i_b_l_exposure
    }
    fn local_i_b_l_render_transparent(&self) -> &bool {
        &self.local_i_b_l_render_transparent
    }
    fn local_i_b_l_render_emitter_quad(&self) -> &bool {
        &self.local_i_b_l_render_emitter_quad
    }
    fn local_i_b_l_render_emitter_mesh(&self) -> &bool {
        &self.local_i_b_l_render_emitter_mesh
    }
    fn p_b_r_local_i_b_l_acquisition_wait_for_mesh_loading(&self) -> &bool {
        &self.p_b_r_local_i_b_l_acquisition_wait_for_mesh_loading
    }
    fn p_b_r_local_i_b_l_acquisition_wait_frame_count(&self) -> &u32 {
        &self.p_b_r_local_i_b_l_acquisition_wait_frame_count
    }
    fn p_b_r_draw_distant_i_b_l_diffuse_reference(&self) -> &bool {
        &self.p_b_r_draw_distant_i_b_l_diffuse_reference
    }
    fn p_b_r_draw_distant_i_b_l_specular_reference(&self) -> &bool {
        &self.p_b_r_draw_distant_i_b_l_specular_reference
    }
    fn p_b_r_draw_local_i_b_l_reference(&self) -> &bool {
        &self.p_b_r_draw_local_i_b_l_reference
    }
    fn p_b_r_draw_area_light_reference(&self) -> &bool {
        &self.p_b_r_draw_area_light_reference
    }
    fn p_b_r_specular_convolution_sample_count(&self) -> &u32 {
        &self.p_b_r_specular_convolution_sample_count
    }
    fn p_b_r_convolution_highest_m_i_p_enable(&self) -> &bool {
        &self.p_b_r_convolution_highest_m_i_p_enable
    }
    fn local_i_b_l_resolution(&self) -> &u32 {
        &self.local_i_b_l_resolution
    }
    fn draw_debug_local_i_b_l_enable(&self) -> &bool {
        &self.draw_debug_local_i_b_l_enable
    }
    fn draw_debug_local_i_b_l_stats_enable(&self) -> &bool {
        &self.draw_debug_local_i_b_l_stats_enable
    }
    fn draw_debug_local_i_b_l_volumes_enable(&self) -> &bool {
        &self.draw_debug_local_i_b_l_volumes_enable
    }
    fn draw_debug_local_i_b_l_preview_scale(&self) -> &f32 {
        &self.draw_debug_local_i_b_l_preview_scale
    }
    fn draw_debug_local_i_b_l_index(&self) -> &u32 {
        &self.draw_debug_local_i_b_l_index
    }
    fn draw_debug_local_i_b_l_mip_level(&self) -> &u32 {
        &self.draw_debug_local_i_b_l_mip_level
    }
    fn draw_debug_local_i_b_l_shadowmaps(&self) -> &bool {
        &self.draw_debug_local_i_b_l_shadowmaps
    }
    fn draw_debug_pre_integrated_f_g_texture(&self) -> &bool {
        &self.draw_debug_pre_integrated_f_g_texture
    }
    fn draw_debug_reflection_state(&self) -> &bool {
        &self.draw_debug_reflection_state
    }
    fn draw_debug_probe_mirror_enable(&self) -> &bool {
        &self.draw_debug_probe_mirror_enable
    }
    fn draw_debug_probe_diffuse_enable(&self) -> &bool {
        &self.draw_debug_probe_diffuse_enable
    }
    fn draw_debug_probe_screen_enable(&self) -> &bool {
        &self.draw_debug_probe_screen_enable
    }
    fn draw_debug_probe_screen_on_right(&self) -> &bool {
        &self.draw_debug_probe_screen_on_right
    }
    fn continuous_local_i_b_l_enable(&self) -> &bool {
        &self.continuous_local_i_b_l_enable
    }
    fn continuous_local_i_b_l_index(&self) -> &u32 {
        &self.continuous_local_i_b_l_index
    }
    fn p_b_r_convolution_precomputed_sample_enable(&self) -> &bool {
        &self.p_b_r_convolution_precomputed_sample_enable
    }
    fn p_b_r_convolution_random_rotation_enable(&self) -> &bool {
        &self.p_b_r_convolution_random_rotation_enable
    }
    fn draw_debug_local_planar_reflections(&self) -> &bool {
        &self.draw_debug_local_planar_reflections
    }
    fn emitter_quad_rendering_enable(&self) -> &bool {
        &self.emitter_quad_rendering_enable
    }
    fn emitter_mesh_rendering_enable(&self) -> &bool {
        &self.emitter_mesh_rendering_enable
    }
    fn emitter_point_lights_enable(&self) -> &bool {
        &self.emitter_point_lights_enable
    }
    fn emitter_spot_lights_enable(&self) -> &bool {
        &self.emitter_spot_lights_enable
    }
    fn use_s_s_s_profilefor_o_a_t_s(&self) -> &bool {
        &self.use_s_s_s_profilefor_o_a_t_s
    }
    fn deterministic_rendering_enable(&self) -> &bool {
        &self.deterministic_rendering_enable
    }
    fn hdr_nan_inf_removal_enable(&self) -> &bool {
        &self.hdr_nan_inf_removal_enable
    }
    fn hdr_nan_inf_removal_force_enable(&self) -> &bool {
        &self.hdr_nan_inf_removal_force_enable
    }
    fn p_b_r_max_illuminance_value(&self) -> &f32 {
        &self.p_b_r_max_illuminance_value
    }
    fn p_b_r_max_luminance_value(&self) -> &f32 {
        &self.p_b_r_max_luminance_value
    }
    fn dielectric_range_min_metal_mask(&self) -> &f32 {
        &self.dielectric_range_min_metal_mask
    }
    fn dielectric_range_max_metal_mask(&self) -> &f32 {
        &self.dielectric_range_max_metal_mask
    }
    fn dielectric_range_s_r_g_b_min_value(&self) -> &f32 {
        &self.dielectric_range_s_r_g_b_min_value
    }
    fn dielectric_range_s_r_g_b_max_value(&self) -> &f32 {
        &self.dielectric_range_s_r_g_b_max_value
    }
    fn dielectric_range_s_r_g_b_min_color(&self) -> &super::core::Vec3 {
        &self.dielectric_range_s_r_g_b_min_color
    }
    fn dielectric_range_s_r_g_b_color(&self) -> &super::core::Vec3 {
        &self.dielectric_range_s_r_g_b_color
    }
    fn dielectric_range_s_r_g_b_max_color(&self) -> &super::core::Vec3 {
        &self.dielectric_range_s_r_g_b_max_color
    }
    fn dielectric_range_overlay(&self) -> &f32 {
        &self.dielectric_range_overlay
    }
    fn conductor_range_min_metal_mask(&self) -> &f32 {
        &self.conductor_range_min_metal_mask
    }
    fn conductor_range_max_metal_mask(&self) -> &f32 {
        &self.conductor_range_max_metal_mask
    }
    fn conductor_range_s_r_g_b_min_value(&self) -> &f32 {
        &self.conductor_range_s_r_g_b_min_value
    }
    fn conductor_range_s_r_g_b_max_value(&self) -> &f32 {
        &self.conductor_range_s_r_g_b_max_value
    }
    fn conductor_range_s_r_g_b_min_color(&self) -> &super::core::Vec3 {
        &self.conductor_range_s_r_g_b_min_color
    }
    fn conductor_range_s_r_g_b_color(&self) -> &super::core::Vec3 {
        &self.conductor_range_s_r_g_b_color
    }
    fn conductor_range_s_r_g_b_max_color(&self) -> &super::core::Vec3 {
        &self.conductor_range_s_r_g_b_max_color
    }
    fn conductor_range_overlay(&self) -> &f32 {
        &self.conductor_range_overlay
    }
    fn fresnel0_range_min_metal_mask(&self) -> &f32 {
        &self.fresnel0_range_min_metal_mask
    }
    fn fresnel0_range_max_metal_mask(&self) -> &f32 {
        &self.fresnel0_range_max_metal_mask
    }
    fn volumetric_rendering_enable(&self) -> &bool {
        &self.volumetric_rendering_enable
    }
    fn volumetric_extinction_cascade_base_voxel_size(&self) -> &f32 {
        &self.volumetric_extinction_cascade_base_voxel_size
    }
    fn volumetric_extinction_cascade_voxel_size_cascade_factor(&self) -> &f32 {
        &self.volumetric_extinction_cascade_voxel_size_cascade_factor
    }
    fn volumetric_extinction_cascade_resolution(&self) -> &u32 {
        &self.volumetric_extinction_cascade_resolution
    }
    fn volumetric_extinction_cascade_lock_view(&self) -> &bool {
        &self.volumetric_extinction_cascade_lock_view
    }
    fn volumetric_shadowmap_enable(&self) -> &bool {
        &self.volumetric_shadowmap_enable
    }
    fn volumetric_shadowmap_resolution(&self) -> &u32 {
        &self.volumetric_shadowmap_resolution
    }
    fn volumetric_shadowmap_max_count(&self) -> &u32 {
        &self.volumetric_shadowmap_max_count
    }
    fn volumetric_shadowmap_accumulate_cs_enable(&self) -> &bool {
        &self.volumetric_shadowmap_accumulate_cs_enable
    }
    fn punctual_light_cast_volumetric_shadowmap_enable_level(&self) -> &super::core::QualityLevel {
        &self.punctual_light_cast_volumetric_shadowmap_enable_level
    }
    fn area_light_cast_volumetric_shadowmap_enable_level(&self) -> &super::core::QualityLevel {
        &self.area_light_cast_volumetric_shadowmap_enable_level
    }
    fn local_light_cast_volumetric_level(&self) -> &super::core::QualityLevel {
        &self.local_light_cast_volumetric_level
    }
    fn volumetric_emitter_voxelisation_enable(&self) -> &bool {
        &self.volumetric_emitter_voxelisation_enable
    }
    fn volumetric_emitter_voxelisation_mode(&self) -> &u32 {
        &self.volumetric_emitter_voxelisation_mode
    }
    fn volumetric_participating_media_enable(&self) -> &bool {
        &self.volumetric_participating_media_enable
    }
    fn volumetric_participating_media_use_light_cull(&self) -> &bool {
        &self.volumetric_participating_media_use_light_cull
    }
    fn volumetric_participating_media_texture_depth(&self) -> &u32 {
        &self.volumetric_participating_media_texture_depth
    }
    fn volumetric_participating_media_camera_depth(&self) -> &f32 {
        &self.volumetric_participating_media_camera_depth
    }
    fn volumetric_participating_media_resolution_scale(&self) -> &u32 {
        &self.volumetric_participating_media_resolution_scale
    }
    fn volumetric_participating_media_from_v_e_fog(&self) -> &bool {
        &self.volumetric_participating_media_from_v_e_fog
    }
    fn volumetric_participating_media_lock_view(&self) -> &bool {
        &self.volumetric_participating_media_lock_view
    }
    fn volumetric_participating_media_local_lights(&self) -> &bool {
        &self.volumetric_participating_media_local_lights
    }
    fn volumetric_participating_media_ambient_lighting(&self) -> &bool {
        &self.volumetric_participating_media_ambient_lighting
    }
    fn volumetric_participating_media_sun(&self) -> &bool {
        &self.volumetric_participating_media_sun
    }
    fn reflection_volumetric_participating_media_texture_depth(&self) -> &u32 {
        &self.reflection_volumetric_participating_media_texture_depth
    }
    fn reflection_volumetric_participating_media_camera_depth(&self) -> &f32 {
        &self.reflection_volumetric_participating_media_camera_depth
    }
    fn reflection_volumetric_participating_media_resolution_scale(&self) -> &u32 {
        &self.reflection_volumetric_participating_media_resolution_scale
    }
    fn volumetric_participating_media_temporal_integration(&self) -> &bool {
        &self.volumetric_participating_media_temporal_integration
    }
    fn volumetric_participating_media_temporal_filter_strght(&self) -> &f32 {
        &self.volumetric_participating_media_temporal_filter_strght
    }
    fn volumetric_local_fog_volume_enable(&self) -> &bool {
        &self.volumetric_local_fog_volume_enable
    }
    fn draw_debug_volumetric_local_fog_volume(&self) -> &bool {
        &self.draw_debug_volumetric_local_fog_volume
    }
    fn draw_debug_volumetric_cascaded_volumes(&self) -> &bool {
        &self.draw_debug_volumetric_cascaded_volumes
    }
    fn draw_debug_volumetric_shadow_maps(&self) -> &bool {
        &self.draw_debug_volumetric_shadow_maps
    }
    fn draw_debug_volumetric_extinction(&self) -> &u32 {
        &self.draw_debug_volumetric_extinction
    }
    fn draw_debug_volumetric_extinction_scale(&self) -> &f32 {
        &self.draw_debug_volumetric_extinction_scale
    }
    fn draw_debug_volumetric_voxelised_emitter(&self) -> &bool {
        &self.draw_debug_volumetric_voxelised_emitter
    }
    fn light_shaft_fast_hdr_enable(&self) -> &bool {
        &self.light_shaft_fast_hdr_enable
    }
    fn draw_gpu_histogram_enable(&self) -> &bool {
        &self.draw_gpu_histogram_enable
    }
    fn draw_gpu_histogram_red(&self) -> &bool {
        &self.draw_gpu_histogram_red
    }
    fn draw_gpu_histogram_blue(&self) -> &bool {
        &self.draw_gpu_histogram_blue
    }
    fn draw_gpu_histogram_green(&self) -> &bool {
        &self.draw_gpu_histogram_green
    }
    fn draw_gpu_histogram_luminance(&self) -> &bool {
        &self.draw_gpu_histogram_luminance
    }
    fn draw_gpu_histogram_h_d_r_mode(&self) -> &bool {
        &self.draw_gpu_histogram_h_d_r_mode
    }
    fn draw_gpu_histogram_h_d_r_min_e_v(&self) -> &f32 {
        &self.draw_gpu_histogram_h_d_r_min_e_v
    }
    fn draw_gpu_histogram_h_d_r_max_e_v(&self) -> &f32 {
        &self.draw_gpu_histogram_h_d_r_max_e_v
    }
    fn draw_gpu_histogram_bin_count(&self) -> &u32 {
        &self.draw_gpu_histogram_bin_count
    }
    fn draw_clamped_pixels_enable(&self) -> &bool {
        &self.draw_clamped_pixels_enable
    }
    fn draw_clamped_pixels_clamp_min(&self) -> &f32 {
        &self.draw_clamped_pixels_clamp_min
    }
    fn draw_clamped_pixels_clamp_max(&self) -> &f32 {
        &self.draw_clamped_pixels_clamp_max
    }
    fn enable_baked_direct_light_in_game_view(&self) -> &bool {
        &self.enable_baked_direct_light_in_game_view
    }
    fn enable_baked_lightmap_in_game_view(&self) -> &bool {
        &self.enable_baked_lightmap_in_game_view
    }
    fn texture_space_render_module_enable(&self) -> &bool {
        &self.texture_space_render_module_enable
    }
    fn compute_linearize_depth(&self) -> &bool {
        &self.compute_linearize_depth
    }
    fn compute_downsample_depth(&self) -> &bool {
        &self.compute_downsample_depth
    }
    fn draw_debug_disable_explanation(&self) -> &bool {
        &self.draw_debug_disable_explanation
    }
}

impl WorldRenderSettingsBaseTrait for WorldRenderSettings {
    fn cull_screen_area_scale(&self) -> &f32 {
        self._glacier_base.cull_screen_area_scale()
    }
    fn deferred_shading_enable(&self) -> &bool {
        self._glacier_base.deferred_shading_enable()
    }
    fn forward_opaque_enable(&self) -> &bool {
        self._glacier_base.forward_opaque_enable()
    }
    fn full_z_pass_enable(&self) -> &bool {
        self._glacier_base.full_z_pass_enable()
    }
    fn tile_material_classification_enable(&self) -> &bool {
        self._glacier_base.tile_material_classification_enable()
    }
    fn shadowmaps_enable(&self) -> &bool {
        self._glacier_base.shadowmaps_enable()
    }
    fn shadowmap_array_enable(&self) -> &bool {
        self._glacier_base.shadowmap_array_enable()
    }
    fn transparency_shadowmaps_enable(&self) -> &bool {
        self._glacier_base.transparency_shadowmaps_enable()
    }
    fn n_v_i_d_i_a_shadows_p_c_s_s_enable(&self) -> &bool {
        self._glacier_base.n_v_i_d_i_a_shadows_p_c_s_s_enable()
    }
    fn n_v_i_d_i_a_shadows_h_f_t_s_enable(&self) -> &bool {
        self._glacier_base.n_v_i_d_i_a_shadows_h_f_t_s_enable()
    }
    fn transparency_shadowmaps_half_res(&self) -> &bool {
        self._glacier_base.transparency_shadowmaps_half_res()
    }
    fn sun_shadowmap_level(&self) -> &super::core::QualityLevel {
        self._glacier_base.sun_shadowmap_level()
    }
    fn shadowmap_min_fov(&self) -> &f32 {
        self._glacier_base.shadowmap_min_fov()
    }
    fn shadowmap_fixed_movement_enable(&self) -> &bool {
        self._glacier_base.shadowmap_fixed_movement_enable()
    }
    fn shadowmap_fixed_depth_enable(&self) -> &bool {
        self._glacier_base.shadowmap_fixed_depth_enable()
    }
    fn shadowmap_size_z_scale(&self) -> &f32 {
        self._glacier_base.shadowmap_size_z_scale()
    }
    fn shadowmap_resolution(&self) -> &u32 {
        self._glacier_base.shadowmap_resolution()
    }
    fn adjusted_shadowmap_resolution(&self) -> &u32 {
        self._glacier_base.adjusted_shadowmap_resolution()
    }
    fn shadowmap_quality(&self) -> &u32 {
        self._glacier_base.shadowmap_quality()
    }
    fn shadowmap_poisson_filter_scale(&self) -> &f32 {
        self._glacier_base.shadowmap_poisson_filter_scale()
    }
    fn shadowmap_slice_count(&self) -> &u32 {
        self._glacier_base.shadowmap_slice_count()
    }
    fn adjusted_shadowmap_slice_count(&self) -> &u32 {
        self._glacier_base.adjusted_shadowmap_slice_count()
    }
    fn shadowmap_slice_scheme_weight(&self) -> &f32 {
        self._glacier_base.shadowmap_slice_scheme_weight()
    }
    fn shadowmap_first_slice_scale(&self) -> &f32 {
        self._glacier_base.shadowmap_first_slice_scale()
    }
    fn shadowmap_view_distance(&self) -> &f32 {
        self._glacier_base.shadowmap_view_distance()
    }
    fn shadowmap_view_distance_scale_enable(&self) -> &bool {
        self._glacier_base.shadowmap_view_distance_scale_enable()
    }
    fn shadowmap_extrusion_length(&self) -> &f32 {
        self._glacier_base.shadowmap_extrusion_length()
    }
    fn shadowmap_first_slice_extrusion_length(&self) -> &f32 {
        self._glacier_base.shadowmap_first_slice_extrusion_length()
    }
    fn shadowmap_adjust_far_plane(&self) -> &bool {
        self._glacier_base.shadowmap_adjust_far_plane()
    }
    fn draw_debug_shadowmap_cascades(&self) -> &bool {
        self._glacier_base.draw_debug_shadowmap_cascades()
    }
    fn shadowmap_accum_enable(&self) -> &bool {
        self._glacier_base.shadowmap_accum_enable()
    }
    fn shadowmap_accum_reuse_enable(&self) -> &bool {
        self._glacier_base.shadowmap_accum_reuse_enable()
    }
    fn shadowmap_accum_stencil_enable(&self) -> &bool {
        self._glacier_base.shadowmap_accum_stencil_enable()
    }
    fn shadowmap_accum_stencil2_enable(&self) -> &bool {
        self._glacier_base.shadowmap_accum_stencil2_enable()
    }
    fn shadowmap_transition_blend_enable(&self) -> &bool {
        self._glacier_base.shadowmap_transition_blend_enable()
    }
    fn shadowmap_transition_blend_amount(&self) -> &f32 {
        self._glacier_base.shadowmap_transition_blend_amount()
    }
    fn shadowmap_foreground_enable(&self) -> &bool {
        self._glacier_base.shadowmap_foreground_enable()
    }
    fn shadowmap_foreground_use_first_person_view_transform(&self) -> &bool {
        self._glacier_base.shadowmap_foreground_use_first_person_view_transform()
    }
    fn shadowmap_foreground_extrusion_length(&self) -> &f32 {
        self._glacier_base.shadowmap_foreground_extrusion_length()
    }
    fn shadowmap_foreground_split_distance(&self) -> &f32 {
        self._glacier_base.shadowmap_foreground_split_distance()
    }
    fn shadowmap_foreground_size_z_scale(&self) -> &f32 {
        self._glacier_base.shadowmap_foreground_size_z_scale()
    }
    fn shadowmap_adjust_shadow_distance_with_fov(&self) -> &bool {
        self._glacier_base.shadowmap_adjust_shadow_distance_with_fov()
    }
    fn shadowmap_draw_batched_enable(&self) -> &bool {
        self._glacier_base.shadowmap_draw_batched_enable()
    }
    fn distant_shadow_cache_draw_frustum(&self) -> &bool {
        self._glacier_base.distant_shadow_cache_draw_frustum()
    }
    fn distant_shadow_cache_enable(&self) -> &bool {
        self._glacier_base.distant_shadow_cache_enable()
    }
    fn distant_shadow_cache_resolution(&self) -> &u32 {
        self._glacier_base.distant_shadow_cache_resolution()
    }
    fn distant_shadow_cache_force_resolution(&self) -> &i32 {
        self._glacier_base.distant_shadow_cache_force_resolution()
    }
    fn distant_shadow_cache_resolution_scale(&self) -> &f32 {
        self._glacier_base.distant_shadow_cache_resolution_scale()
    }
    fn distant_shadow_cache_max_resolution(&self) -> &u32 {
        self._glacier_base.distant_shadow_cache_max_resolution()
    }
    fn distant_shadow_cache_draw_debug(&self) -> &bool {
        self._glacier_base.distant_shadow_cache_draw_debug()
    }
    fn distant_shadow_cache_force_mode(&self) -> &u32 {
        self._glacier_base.distant_shadow_cache_force_mode()
    }
    fn distant_shadow_cache_use_quadtree(&self) -> &bool {
        self._glacier_base.distant_shadow_cache_use_quadtree()
    }
    fn distant_shadow_cache_batch_group_entity_bake(&self) -> &bool {
        self._glacier_base.distant_shadow_cache_batch_group_entity_bake()
    }
    fn distant_shadow_cache_rebake_on_light_change(&self) -> &bool {
        self._glacier_base.distant_shadow_cache_rebake_on_light_change()
    }
    fn distant_shadow_cache_rebake_on_add_remove(&self) -> &bool {
        self._glacier_base.distant_shadow_cache_rebake_on_add_remove()
    }
    fn distant_shadow_cache_rebake_on_move(&self) -> &bool {
        self._glacier_base.distant_shadow_cache_rebake_on_move()
    }
    fn distant_shadow_cache_rebake_on_part_visibility(&self) -> &bool {
        self._glacier_base.distant_shadow_cache_rebake_on_part_visibility()
    }
    fn distant_shadow_cache_coalesce_time(&self) -> &f32 {
        self._glacier_base.distant_shadow_cache_coalesce_time()
    }
    fn distant_shadow_cache_max_bake_events(&self) -> &u32 {
        self._glacier_base.distant_shadow_cache_max_bake_events()
    }
    fn sun_pcss_max_sample_count(&self) -> &i32 {
        self._glacier_base.sun_pcss_max_sample_count()
    }
    fn sun_pcss_adaptive_sample_increment(&self) -> &i32 {
        self._glacier_base.sun_pcss_adaptive_sample_increment()
    }
    fn dx_shadowmap16_bit_enable(&self) -> &bool {
        self._glacier_base.dx_shadowmap16_bit_enable()
    }
    fn dx_dynamic_envmap_shadowmap16_bit_enable(&self) -> &bool {
        self._glacier_base.dx_dynamic_envmap_shadowmap16_bit_enable()
    }
    fn apply_shadowmaps_enable(&self) -> &bool {
        self._glacier_base.apply_shadowmaps_enable()
    }
    fn simple_shadowmaps_enable(&self) -> &bool {
        self._glacier_base.simple_shadowmaps_enable()
    }
    fn emitter_shadowing_blend_toggle(&self) -> &bool {
        self._glacier_base.emitter_shadowing_blend_toggle()
    }
    fn emitter_shadowing_many_samples_toggle(&self) -> &bool {
        self._glacier_base.emitter_shadowing_many_samples_toggle()
    }
    fn dx_linear_depth32_bit_format_enable(&self) -> &bool {
        self._glacier_base.dx_linear_depth32_bit_format_enable()
    }
    fn motion_blur_enable(&self) -> &bool {
        self._glacier_base.motion_blur_enable()
    }
    fn motion_blur_force_on(&self) -> &bool {
        self._glacier_base.motion_blur_force_on()
    }
    fn motion_blur_scale(&self) -> &f32 {
        self._glacier_base.motion_blur_scale()
    }
    fn motion_blur_fixed_shutter_time(&self) -> &f32 {
        self._glacier_base.motion_blur_fixed_shutter_time()
    }
    fn motion_blur_max(&self) -> &f32 {
        self._glacier_base.motion_blur_max()
    }
    fn motion_blur_radial_blur_max(&self) -> &f32 {
        self._glacier_base.motion_blur_radial_blur_max()
    }
    fn motion_blur_noise_scale(&self) -> &f32 {
        self._glacier_base.motion_blur_noise_scale()
    }
    fn motion_blur_quality(&self) -> &u32 {
        self._glacier_base.motion_blur_quality()
    }
    fn motion_blur_debug_mode(&self) -> &u32 {
        self._glacier_base.motion_blur_debug_mode()
    }
    fn motion_blur_perceptual_space_enable(&self) -> &bool {
        self._glacier_base.motion_blur_perceptual_space_enable()
    }
    fn motion_blur_stencil_pass_enable(&self) -> &bool {
        self._glacier_base.motion_blur_stencil_pass_enable()
    }
    fn motion_blur_centered_enable(&self) -> &bool {
        self._glacier_base.motion_blur_centered_enable()
    }
    fn motion_blur_max_sample_count(&self) -> &u32 {
        self._glacier_base.motion_blur_max_sample_count()
    }
    fn motion_blur_depth_check_threshold(&self) -> &f32 {
        self._glacier_base.motion_blur_depth_check_threshold()
    }
    fn motion_blur_depth_check_max_distance(&self) -> &f32 {
        self._glacier_base.motion_blur_depth_check_max_distance()
    }
    fn tiled_motion_blur_variance_threshold_scale(&self) -> &f32 {
        self._glacier_base.tiled_motion_blur_variance_threshold_scale()
    }
    fn tiled_motion_blur_vel_mag_depth_downsample(&self) -> &u32 {
        self._glacier_base.tiled_motion_blur_vel_mag_depth_downsample()
    }
    fn tiled_motion_blur_separable_enable(&self) -> &bool {
        self._glacier_base.tiled_motion_blur_separable_enable()
    }
    fn tiled_motion_blur_enable(&self) -> &bool {
        self._glacier_base.tiled_motion_blur_enable()
    }
    fn tiled_motion_blur_force20_px_tile(&self) -> &bool {
        self._glacier_base.tiled_motion_blur_force20_px_tile()
    }
    fn motion_blur_use_detailed_gpu_timers(&self) -> &bool {
        self._glacier_base.motion_blur_use_detailed_gpu_timers()
    }
    fn velocity_vectors_derive_from_depth_enable(&self) -> &bool {
        self._glacier_base.velocity_vectors_derive_from_depth_enable()
    }
    fn velocity_vectors_derive_from_dynamic_objects_enable(&self) -> &bool {
        self._glacier_base.velocity_vectors_derive_from_dynamic_objects_enable()
    }
    fn velocity_vectors_clear_value(&self) -> &super::core::Vec3 {
        self._glacier_base.velocity_vectors_clear_value()
    }
    fn multisample_count(&self) -> &u32 {
        self._glacier_base.multisample_count()
    }
    fn multisample_quality(&self) -> &u32 {
        self._glacier_base.multisample_quality()
    }
    fn draw_transparent(&self) -> &bool {
        self._glacier_base.draw_transparent()
    }
    fn draw_half_res_transparent(&self) -> &bool {
        self._glacier_base.draw_half_res_transparent()
    }
    fn draw_transparent_decal(&self) -> &bool {
        self._glacier_base.draw_transparent_decal()
    }
    fn transparent_dof_enable(&self) -> &bool {
        self._glacier_base.transparent_dof_enable()
    }
    fn transparent_dof_half_res_enable(&self) -> &bool {
        self._glacier_base.transparent_dof_half_res_enable()
    }
    fn transparent_dof_lerp_coc_enable(&self) -> &bool {
        self._glacier_base.transparent_dof_lerp_coc_enable()
    }
    fn only_shadowmap_slice(&self) -> &i32 {
        self._glacier_base.only_shadowmap_slice()
    }
    fn view_mode(&self) -> &WorldViewMode {
        self._glacier_base.view_mode()
    }
    fn enable(&self) -> &bool {
        self._glacier_base.enable()
    }
    fn console_render_target_pool_sharing_enable(&self) -> &bool {
        self._glacier_base.console_render_target_pool_sharing_enable()
    }
    fn fast_hdr_enable(&self) -> &bool {
        self._glacier_base.fast_hdr_enable()
    }
    fn additional_hdr_target_in_e_s_r_a_m(&self) -> &u32 {
        self._glacier_base.additional_hdr_target_in_e_s_r_a_m()
    }
    fn linear_depth_in_e_s_r_a_m(&self) -> &bool {
        self._glacier_base.linear_depth_in_e_s_r_a_m()
    }
    fn half_res_depth_resolve_enable(&self) -> &bool {
        self._glacier_base.half_res_depth_resolve_enable()
    }
    fn depth_buffer_collision_enable(&self) -> &bool {
        self._glacier_base.depth_buffer_collision_enable()
    }
    fn final_post_enable(&self) -> &bool {
        self._glacier_base.final_post_enable()
    }
    fn output_gamma_correction_enable(&self) -> &bool {
        self._glacier_base.output_gamma_correction_enable()
    }
    fn screen_effect_enable(&self) -> &bool {
        self._glacier_base.screen_effect_enable()
    }
    fn draw_solid_bounding_boxes(&self) -> &bool {
        self._glacier_base.draw_solid_bounding_boxes()
    }
    fn draw_line_bounding_boxes(&self) -> &bool {
        self._glacier_base.draw_line_bounding_boxes()
    }
    fn draw_bounding_spheres(&self) -> &bool {
        self._glacier_base.draw_bounding_spheres()
    }
    fn draw_frustums(&self) -> &bool {
        self._glacier_base.draw_frustums()
    }
    fn draw_local_i_b_l_frustums(&self) -> &bool {
        self._glacier_base.draw_local_i_b_l_frustums()
    }
    fn draw_debug_shadowmaps(&self) -> &bool {
        self._glacier_base.draw_debug_shadowmaps()
    }
    fn draw_debug_local_light_shadows(&self) -> &bool {
        self._glacier_base.draw_debug_local_light_shadows()
    }
    fn draw_debug_sky_envmap(&self) -> &bool {
        self._glacier_base.draw_debug_sky_envmap()
    }
    fn draw_debug_velocity_buffer(&self) -> &bool {
        self._glacier_base.draw_debug_velocity_buffer()
    }
    fn draw_debug_half_res_environment(&self) -> &bool {
        self._glacier_base.draw_debug_half_res_environment()
    }
    fn draw_debug_distortion(&self) -> &bool {
        self._glacier_base.draw_debug_distortion()
    }
    fn draw_debug_visible_entity_types(&self) -> &bool {
        self._glacier_base.draw_debug_visible_entity_types()
    }
    fn draw_debug_sky_textures(&self) -> &bool {
        self._glacier_base.draw_debug_sky_textures()
    }
    fn draw_debug_dof(&self) -> &bool {
        self._glacier_base.draw_debug_dof()
    }
    fn draw_debug_half_res_hdr_targets(&self) -> &bool {
        self._glacier_base.draw_debug_half_res_hdr_targets()
    }
    fn draw_debug_hi_z_min_max_buffer_enable(&self) -> &bool {
        self._glacier_base.draw_debug_hi_z_min_max_buffer_enable()
    }
    fn draw_debug_screen_space_raytrace_buckets_enable(&self) -> &bool {
        self._glacier_base.draw_debug_screen_space_raytrace_buckets_enable()
    }
    fn draw_debug_emitter_sun_transmittance_maps(&self) -> &bool {
        self._glacier_base.draw_debug_emitter_sun_transmittance_maps()
    }
    fn draw_debug_blur_pyramid(&self) -> &bool {
        self._glacier_base.draw_debug_blur_pyramid()
    }
    fn draw_debug_occlusion_z_buffer(&self) -> &bool {
        self._glacier_base.draw_debug_occlusion_z_buffer()
    }
    fn draw_debug_local_i_b_l_occlusion_z_buffer(&self) -> &bool {
        self._glacier_base.draw_debug_local_i_b_l_occlusion_z_buffer()
    }
    fn draw_debug_buffers(&self) -> &u32 {
        self._glacier_base.draw_debug_buffers()
    }
    fn wireframe_enable(&self) -> &bool {
        self._glacier_base.wireframe_enable()
    }
    fn z_pass_enable(&self) -> &bool {
        self._glacier_base.z_pass_enable()
    }
    fn occluder_mesh_z_prepass_enable(&self) -> &bool {
        self._glacier_base.occluder_mesh_z_prepass_enable()
    }
    fn occluder_mesh_z_prepass_draw_enable(&self) -> &bool {
        self._glacier_base.occluder_mesh_z_prepass_draw_enable()
    }
    fn occluder_mesh_z_prepass_debug_enable(&self) -> &bool {
        self._glacier_base.occluder_mesh_z_prepass_debug_enable()
    }
    fn frame_synthesis_mode(&self) -> &FrameSynthesisMode {
        self._glacier_base.frame_synthesis_mode()
    }
    fn half_res_enable(&self) -> &bool {
        self._glacier_base.half_res_enable()
    }
    fn force_full_res_enable(&self) -> &bool {
        self._glacier_base.force_full_res_enable()
    }
    fn half_res_lens_flares_enable(&self) -> &bool {
        self._glacier_base.half_res_lens_flares_enable()
    }
    fn foreground_enable(&self) -> &bool {
        self._glacier_base.foreground_enable()
    }
    fn foreground_z_pass_enable(&self) -> &bool {
        self._glacier_base.foreground_z_pass_enable()
    }
    fn foreground_transparent_enable(&self) -> &bool {
        self._glacier_base.foreground_transparent_enable()
    }
    fn bilateral_half_res_composite_enable(&self) -> &bool {
        self._glacier_base.bilateral_half_res_composite_enable()
    }
    fn half_res_depth_min_max_dither_enable(&self) -> &bool {
        self._glacier_base.half_res_depth_min_max_dither_enable()
    }
    fn half_res_depth_min_max_dither_threshold(&self) -> &f32 {
        self._glacier_base.half_res_depth_min_max_dither_threshold()
    }
    fn sky_lighting_enable(&self) -> &bool {
        self._glacier_base.sky_lighting_enable()
    }
    fn sky_render_enable(&self) -> &bool {
        self._glacier_base.sky_render_enable()
    }
    fn sky_depth_fog_enable(&self) -> &bool {
        self._glacier_base.sky_depth_fog_enable()
    }
    fn sky_height_fog_enable(&self) -> &bool {
        self._glacier_base.sky_height_fog_enable()
    }
    fn sky_forward_scattering_enable(&self) -> &bool {
        self._glacier_base.sky_forward_scattering_enable()
    }
    fn procedural_sky_receive_height_fog(&self) -> &bool {
        self._glacier_base.procedural_sky_receive_height_fog()
    }
    fn physical_sky_enabled(&self) -> &bool {
        self._glacier_base.physical_sky_enabled()
    }
    fn physical_sky_precision_height(&self) -> &u32 {
        self._glacier_base.physical_sky_precision_height()
    }
    fn physical_sky_precision_view(&self) -> &u32 {
        self._glacier_base.physical_sky_precision_view()
    }
    fn physical_sky_precision_sun(&self) -> &u32 {
        self._glacier_base.physical_sky_precision_sun()
    }
    fn physical_sky_scattering_orders(&self) -> &u32 {
        self._glacier_base.physical_sky_scattering_orders()
    }
    fn physical_sky_aerial_perspective_texture_width(&self) -> &u32 {
        self._glacier_base.physical_sky_aerial_perspective_texture_width()
    }
    fn physical_sky_aerial_perspective_texture_height(&self) -> &u32 {
        self._glacier_base.physical_sky_aerial_perspective_texture_height()
    }
    fn physical_sky_aerial_perspective_texture_depth(&self) -> &u32 {
        self._glacier_base.physical_sky_aerial_perspective_texture_depth()
    }
    fn physical_sky_scattering_eval_frame_count(&self) -> &u32 {
        self._glacier_base.physical_sky_scattering_eval_frame_count()
    }
    fn physical_sky_aerial_perspective_max_distance(&self) -> &f32 {
        self._glacier_base.physical_sky_aerial_perspective_max_distance()
    }
    fn physical_sky_force_precompute(&self) -> &bool {
        self._glacier_base.physical_sky_force_precompute()
    }
    fn volumetric_clouds_enabled(&self) -> &bool {
        self._glacier_base.volumetric_clouds_enabled()
    }
    fn volumetric_clouds_quality(&self) -> &super::core::QualityLevel {
        self._glacier_base.volumetric_clouds_quality()
    }
    fn volumetric_clouds_cast_shadow(&self) -> &bool {
        self._glacier_base.volumetric_clouds_cast_shadow()
    }
    fn volumetric_clouds_cast_shadow_in_forward_render(&self) -> &bool {
        self._glacier_base.volumetric_clouds_cast_shadow_in_forward_render()
    }
    fn volumetric_clouds_affect_aerial_perspective(&self) -> &bool {
        self._glacier_base.volumetric_clouds_affect_aerial_perspective()
    }
    fn volumetric_clouds_receive_aerial_perspective(&self) -> &bool {
        self._glacier_base.volumetric_clouds_receive_aerial_perspective()
    }
    fn volumetric_clouds_occlude_lens_flare(&self) -> &bool {
        self._glacier_base.volumetric_clouds_occlude_lens_flare()
    }
    fn volumetric_clouds_render_target_resolution_divider(&self) -> &u32 {
        self._glacier_base.volumetric_clouds_render_target_resolution_divider()
    }
    fn volumetric_clouds_reflection_render_target_resolution_divider(&self) -> &u32 {
        self._glacier_base.volumetric_clouds_reflection_render_target_resolution_divider()
    }
    fn volumetric_clouds_shadow_iteration_count(&self) -> &u32 {
        self._glacier_base.volumetric_clouds_shadow_iteration_count()
    }
    fn volumetric_clouds_shadowmap_resolution(&self) -> &u32 {
        self._glacier_base.volumetric_clouds_shadowmap_resolution()
    }
    fn volumetric_clouds_shadowmap_blur_samples(&self) -> &u32 {
        self._glacier_base.volumetric_clouds_shadowmap_blur_samples()
    }
    fn volumetric_clouds_sample_count(&self) -> &super::core::QualityScalableInt {
        self._glacier_base.volumetric_clouds_sample_count()
    }
    fn volumetric_clouds_reflection_sample_count(&self) -> &u32 {
        self._glacier_base.volumetric_clouds_reflection_sample_count()
    }
    fn volumetric_clouds_i_b_l_sample_count(&self) -> &u32 {
        self._glacier_base.volumetric_clouds_i_b_l_sample_count()
    }
    fn volumetric_clouds_temporal_coefficient(&self) -> &f32 {
        self._glacier_base.volumetric_clouds_temporal_coefficient()
    }
    fn volumetric_clouds_env_color_temporal_coefficient(&self) -> &f32 {
        self._glacier_base.volumetric_clouds_env_color_temporal_coefficient()
    }
    fn transparent_fogging_enable(&self) -> &bool {
        self._glacier_base.transparent_fogging_enable()
    }
    fn distortion_enable(&self) -> &bool {
        self._glacier_base.distortion_enable()
    }
    fn distortion_half_res_enable(&self) -> &bool {
        self._glacier_base.distortion_half_res_enable()
    }
    fn distortion8_bit_enable(&self) -> &bool {
        self._glacier_base.distortion8_bit_enable()
    }
    fn distortion_tiling_enable(&self) -> &bool {
        self._glacier_base.distortion_tiling_enable()
    }
    fn static_envmap_enable(&self) -> &bool {
        self._glacier_base.static_envmap_enable()
    }
    fn custom_envmap_enable(&self) -> &bool {
        self._glacier_base.custom_envmap_enable()
    }
    fn sky_envmap_enable(&self) -> &bool {
        self._glacier_base.sky_envmap_enable()
    }
    fn sky_envmap_filter_width(&self) -> &f32 {
        self._glacier_base.sky_envmap_filter_width()
    }
    fn sky_envmap_resolution(&self) -> &u32 {
        self._glacier_base.sky_envmap_resolution()
    }
    fn sky_envmap_mipmap_gen_enable(&self) -> &bool {
        self._glacier_base.sky_envmap_mipmap_gen_enable()
    }
    fn draw_debug_sky_envmap_mip_level(&self) -> &i32 {
        self._glacier_base.draw_debug_sky_envmap_mip_level()
    }
    fn sky_envmap_filter_mode(&self) -> &MipmapFilterMode {
        self._glacier_base.sky_envmap_filter_mode()
    }
    fn sky_envmap_sides_per_frame_count(&self) -> &u32 {
        self._glacier_base.sky_envmap_sides_per_frame_count()
    }
    fn sky_envmap_force_update_enable(&self) -> &bool {
        self._glacier_base.sky_envmap_force_update_enable()
    }
    fn sky_envmap_use_fast_h_d_r(&self) -> &bool {
        self._glacier_base.sky_envmap_use_fast_h_d_r()
    }
    fn sky_envmap_debug_color_enable(&self) -> &bool {
        self._glacier_base.sky_envmap_debug_color_enable()
    }
    fn sky_envmap_update_count_threshold(&self) -> &f32 {
        self._glacier_base.sky_envmap_update_count_threshold()
    }
    fn sky_envmap_update_value_threshold(&self) -> &f32 {
        self._glacier_base.sky_envmap_update_value_threshold()
    }
    fn sky_envmap_cloud_fog_enable(&self) -> &bool {
        self._glacier_base.sky_envmap_cloud_fog_enable()
    }
    fn sky_envmap_generate_no_backdrop_enable(&self) -> &bool {
        self._glacier_base.sky_envmap_generate_no_backdrop_enable()
    }
    fn dynamic_envmap_enable(&self) -> &bool {
        self._glacier_base.dynamic_envmap_enable()
    }
    fn draw_debug_dynamic_envmap_mip_level(&self) -> &i32 {
        self._glacier_base.draw_debug_dynamic_envmap_mip_level()
    }
    fn dynamic_envmap_mipmap_gen_enable(&self) -> &bool {
        self._glacier_base.dynamic_envmap_mipmap_gen_enable()
    }
    fn draw_debug_dynamic_envmap(&self) -> &bool {
        self._glacier_base.draw_debug_dynamic_envmap()
    }
    fn dynamic_envmap_shadowmap_enable(&self) -> &bool {
        self._glacier_base.dynamic_envmap_shadowmap_enable()
    }
    fn dynamic_envmap_shadowmap_resolution(&self) -> &u32 {
        self._glacier_base.dynamic_envmap_shadowmap_resolution()
    }
    fn dynamic_envmap_shadowmap_far_plane_override(&self) -> &bool {
        self._glacier_base.dynamic_envmap_shadowmap_far_plane_override()
    }
    fn dynamic_envmap_shadowmap_far_plane(&self) -> &i32 {
        self._glacier_base.dynamic_envmap_shadowmap_far_plane()
    }
    fn dynamic_envmap_shadowmap_shadow_extrusion_override(&self) -> &bool {
        self._glacier_base.dynamic_envmap_shadowmap_shadow_extrusion_override()
    }
    fn dynamic_envmap_shadowmap_shadow_extrusion(&self) -> &i32 {
        self._glacier_base.dynamic_envmap_shadowmap_shadow_extrusion()
    }
    fn draw_debug_dynamic_envmap_shadowmap(&self) -> &bool {
        self._glacier_base.draw_debug_dynamic_envmap_shadowmap()
    }
    fn draw_dynamic_envmap_frustums(&self) -> &bool {
        self._glacier_base.draw_dynamic_envmap_frustums()
    }
    fn setup_job_enable(&self) -> &bool {
        self._glacier_base.setup_job_enable()
    }
    fn setup_jobs_create_view_job(&self) -> &bool {
        self._glacier_base.setup_jobs_create_view_job()
    }
    fn prepare_dispatch_list_job_enable(&self) -> &bool {
        self._glacier_base.prepare_dispatch_list_job_enable()
    }
    fn indirect_specular_intensity(&self) -> &f32 {
        self._glacier_base.indirect_specular_intensity()
    }
    fn indirect_specular_reflectance_scale(&self) -> &f32 {
        self._glacier_base.indirect_specular_reflectance_scale()
    }
    fn indirect_specular_probes_intensity(&self) -> &f32 {
        self._glacier_base.indirect_specular_probes_intensity()
    }
    fn indirect_specular_probes_reflectance_scale(&self) -> &f32 {
        self._glacier_base.indirect_specular_probes_reflectance_scale()
    }
}

impl super::core::DataContainerTrait for WorldRenderSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WORLDRENDERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldRenderSettings",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WORLDRENDERSETTINGSBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WorldRenderSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TestRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, test_rendering_enable),
            },
            FieldInfoData {
                name: "GenericEntityRendererEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, generic_entity_renderer_enable),
            },
            FieldInfoData {
                name: "GenericEntityMaxVisibleEntityCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, generic_entity_max_visible_entity_count),
            },
            FieldInfoData {
                name: "ZBufferShadowTestEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, z_buffer_shadow_test_enable),
            },
            FieldInfoData {
                name: "DrawDebugGroundHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_ground_height),
            },
            FieldInfoData {
                name: "DecalVolumeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, decal_volume_enable),
            },
            FieldInfoData {
                name: "DecalVolumeScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, decal_volume_scale),
            },
            FieldInfoData {
                name: "DrawDebugDecalVolumes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_decal_volumes),
            },
            FieldInfoData {
                name: "EnvironmentDecalVolumesEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, environment_decal_volumes_enabled),
            },
            FieldInfoData {
                name: "EnvironmentDecalVolumeQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WorldRenderSettings, environment_decal_volume_quality),
            },
            FieldInfoData {
                name: "EnvironmentDecalVolumeMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(WorldRenderSettings, environment_decal_volume_max_count),
            },
            FieldInfoData {
                name: "EmitterDecalVolumeQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WorldRenderSettings, emitter_decal_volume_quality),
            },
            FieldInfoData {
                name: "EmitterDecalVolumeMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(WorldRenderSettings, emitter_decal_volume_max_count),
            },
            FieldInfoData {
                name: "EmitterDecalVolumeMaxCountPerSet",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(WorldRenderSettings, emitter_decal_volume_max_count_per_set),
            },
            FieldInfoData {
                name: "EdgeModelsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, edge_models_enable),
            },
            FieldInfoData {
                name: "EdgeModelCastShadowsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, edge_model_cast_shadows_enable),
            },
            FieldInfoData {
                name: "EdgeModelDepthBiasEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, edge_model_depth_bias_enable),
            },
            FieldInfoData {
                name: "EdgeModelShadowDepthBiasEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, edge_model_shadow_depth_bias_enable),
            },
            FieldInfoData {
                name: "EdgeModelViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, edge_model_view_distance),
            },
            FieldInfoData {
                name: "EdgeModelUseMainLodEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, edge_model_use_main_lod_enable),
            },
            FieldInfoData {
                name: "EdgeModelForceLod",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldRenderSettings, edge_model_force_lod),
            },
            FieldInfoData {
                name: "EdgeModelLodScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, edge_model_lod_scale),
            },
            FieldInfoData {
                name: "LensFlaresEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, lens_flares_enable),
            },
            FieldInfoData {
                name: "DrawDebugLensFlareOccluders",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_lens_flare_occluders),
            },
            FieldInfoData {
                name: "DrawDebugLensFlares",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_lens_flares),
            },
            FieldInfoData {
                name: "LensFlareOcclusionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, lens_flare_occlusion_enable),
            },
            FieldInfoData {
                name: "MaxLensFlaresPerFrame",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, max_lens_flares_per_frame),
            },
            FieldInfoData {
                name: "LensFlaresQualityLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WorldRenderSettings, lens_flares_quality_level),
            },
            FieldInfoData {
                name: "CloudShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, cloud_shadow_enable),
            },
            FieldInfoData {
                name: "OverrideDynamicAO",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, override_dynamic_a_o),
            },
            FieldInfoData {
                name: "DrawDebugDynamicAO",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_dynamic_a_o),
            },
            FieldInfoData {
                name: "DrawDebugRaytraceRefl",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_raytrace_refl),
            },
            FieldInfoData {
                name: "DrawDebugRaytraceAO",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_raytrace_a_o),
            },
            FieldInfoData {
                name: "DrawDebugRaytracePrimaryRay",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_raytrace_primary_ray),
            },
            FieldInfoData {
                name: "FilmicEffectsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, filmic_effects_enable),
            },
            FieldInfoData {
                name: "EmissiveEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, emissive_enable),
            },
            FieldInfoData {
                name: "GBufferLayout",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, g_buffer_layout),
            },
            FieldInfoData {
                name: "GBufferTestCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, g_buffer_test_count),
            },
            FieldInfoData {
                name: "GBufferClearEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, g_buffer_clear_enable),
            },
            FieldInfoData {
                name: "DxGBufferLight16BitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, dx_g_buffer_light16_bit_enable),
            },
            FieldInfoData {
                name: "DxGBufferNormal16BitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, dx_g_buffer_normal16_bit_enable),
            },
            FieldInfoData {
                name: "DxGBufferRoughness16BitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, dx_g_buffer_roughness16_bit_enable),
            },
            FieldInfoData {
                name: "GBufferAlphaTestSimpleEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, g_buffer_alpha_test_simple_enable),
            },
            FieldInfoData {
                name: "GBufferAlphaTestSimpleSmoothness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, g_buffer_alpha_test_simple_smoothness),
            },
            FieldInfoData {
                name: "GBufferForceMetalMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, g_buffer_force_metal_mask),
            },
            FieldInfoData {
                name: "GBufferForceSmoothness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, g_buffer_force_smoothness),
            },
            FieldInfoData {
                name: "GBufferForceSpecularAlbedo",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, g_buffer_force_specular_albedo),
            },
            FieldInfoData {
                name: "AlphaUnrollEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, alpha_unroll_enable),
            },
            FieldInfoData {
                name: "Gen4aEsramEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, gen4a_esram_enable),
            },
            FieldInfoData {
                name: "SpecularLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, specular_lighting_enable),
            },
            FieldInfoData {
                name: "SkinLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, skin_lighting_enable),
            },
            FieldInfoData {
                name: "TranslucencyLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, translucency_lighting_enable),
            },
            FieldInfoData {
                name: "TranslucencyAutoThicknessEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, translucency_auto_thickness_enable),
            },
            FieldInfoData {
                name: "LocalLightTranslucencyEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_light_translucency_enable),
            },
            FieldInfoData {
                name: "DynamicEnvmapLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, dynamic_envmap_lighting_enable),
            },
            FieldInfoData {
                name: "OutdoorLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, outdoor_light_enable),
            },
            FieldInfoData {
                name: "LightStencilMethodEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_stencil_method_enable),
            },
            FieldInfoData {
                name: "LightVolumeMethodEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_volume_method_enable),
            },
            FieldInfoData {
                name: "LightVolumeDepthTestEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_volume_depth_test_enable),
            },
            FieldInfoData {
                name: "OutdoorKeyLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, outdoor_key_light_enable),
            },
            FieldInfoData {
                name: "OutdoorSkyLightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, outdoor_sky_light_enable),
            },
            FieldInfoData {
                name: "OutdoorLightTileBlendEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, outdoor_light_tile_blend_enable),
            },
            FieldInfoData {
                name: "EmitterSunTransmittanceMapEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, emitter_sun_transmittance_map_enable),
            },
            FieldInfoData {
                name: "EmitterSunTransmittanceMapResolution",
                flags: MemberInfoFlags::new(8192),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, emitter_sun_transmittance_map_resolution),
            },
            FieldInfoData {
                name: "MaxDecalVolumeCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, max_decal_volume_count),
            },
            FieldInfoData {
                name: "LightTileCombineOutdoorLightEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_tile_combine_outdoor_light_enable),
            },
            FieldInfoData {
                name: "LightTileCsPathEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_tile_cs_path_enable),
            },
            FieldInfoData {
                name: "LightTilePsPathEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_tile_ps_path_enable),
            },
            FieldInfoData {
                name: "LightTileAsyncComputeCulling",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_tile_async_compute_culling),
            },
            FieldInfoData {
                name: "LightTileCsAvgLightCountPerTile",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, light_tile_cs_avg_light_count_per_tile),
            },
            FieldInfoData {
                name: "LightTileMinMaxUseHTile",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_tile_min_max_use_h_tile),
            },
            FieldInfoData {
                name: "LightTileUseCullingHierarchy",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_tile_use_culling_hierarchy),
            },
            FieldInfoData {
                name: "LightTileUseDetailedGpuTimers",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_tile_use_detailed_gpu_timers),
            },
            FieldInfoData {
                name: "LightTileUseCsIndirectClears",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_tile_use_cs_indirect_clears),
            },
            FieldInfoData {
                name: "LightCullFrustumExpandDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, light_cull_frustum_expand_distance),
            },
            FieldInfoData {
                name: "LightTileDebugLightMode",
                flags: MemberInfoFlags::new(0),
                field_type: "LightTileDebugLightCountMode",
                rust_offset: offset_of!(WorldRenderSettings, light_tile_debug_light_mode),
            },
            FieldInfoData {
                name: "LightTileDebugColorMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldRenderSettings, light_tile_debug_color_mode),
            },
            FieldInfoData {
                name: "DrawDebugLightStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_light_stats),
            },
            FieldInfoData {
                name: "DrawDebugLightStatsForward",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_light_stats_forward),
            },
            FieldInfoData {
                name: "DrawDebugLightStatsVolumetric",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_light_stats_volumetric),
            },
            FieldInfoData {
                name: "DrawDebugLightStatsHierarchy",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_light_stats_hierarchy),
            },
            FieldInfoData {
                name: "DrawDebugLightSources",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_light_sources),
            },
            FieldInfoData {
                name: "DrawDebugLightShadowSources",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_light_shadow_sources),
            },
            FieldInfoData {
                name: "DrawDebugLightShadowStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_light_shadow_stats),
            },
            FieldInfoData {
                name: "DrawDebugLightCullStats",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_light_cull_stats),
            },
            FieldInfoData {
                name: "DrawDebugGBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_g_buffer),
            },
            FieldInfoData {
                name: "DrawDebugMaterialInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_material_input),
            },
            FieldInfoData {
                name: "DrawDebugMaterialOutput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_material_output),
            },
            FieldInfoData {
                name: "DrawDebugLightEmissiveSurface",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_light_emissive_surface),
            },
            FieldInfoData {
                name: "PunctualEmissiveLightShapeMinSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, punctual_emissive_light_shape_min_size),
            },
            FieldInfoData {
                name: "DebugLightStatsLightCountHighwatermark",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, debug_light_stats_light_count_highwatermark),
            },
            FieldInfoData {
                name: "LightLodFadeArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, light_lod_fade_area),
            },
            FieldInfoData {
                name: "LightLodMinArea",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, light_lod_min_area),
            },
            FieldInfoData {
                name: "LightLodRadiusFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, light_lod_radius_factor),
            },
            FieldInfoData {
                name: "UseNewLightCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, use_new_light_cull_enable),
            },
            FieldInfoData {
                name: "LightCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_cull_enable),
            },
            FieldInfoData {
                name: "LightCullJobMaxJobCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, light_cull_job_max_job_count),
            },
            FieldInfoData {
                name: "LightOcclusionCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_occlusion_cull_enable),
            },
            FieldInfoData {
                name: "LightConeCullEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_cone_cull_enable),
            },
            FieldInfoData {
                name: "OcclusionCullingWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, occlusion_culling_width),
            },
            FieldInfoData {
                name: "OcclusionCullingHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, occlusion_culling_height),
            },
            FieldInfoData {
                name: "OcclusionTriangleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, occlusion_triangle_count),
            },
            FieldInfoData {
                name: "LocalIBLOcclusionCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_occlusion_culling_enable),
            },
            FieldInfoData {
                name: "ShadowOcclusionCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, shadow_occlusion_culling_enable),
            },
            FieldInfoData {
                name: "ShadowOcclusionCullingWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, shadow_occlusion_culling_width),
            },
            FieldInfoData {
                name: "ShadowOcclusionCullingHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, shadow_occlusion_culling_height),
            },
            FieldInfoData {
                name: "ShadowOcclusionTriangleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, shadow_occlusion_triangle_count),
            },
            FieldInfoData {
                name: "FrustumSilhouetteCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, frustum_silhouette_culling_enable),
            },
            FieldInfoData {
                name: "FrustumSilhouetteCullingPadding",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, frustum_silhouette_culling_padding),
            },
            FieldInfoData {
                name: "SubSurfaceScatteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, sub_surface_scattering_enable),
            },
            FieldInfoData {
                name: "TranslucencyEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, translucency_enable),
            },
            FieldInfoData {
                name: "SubSurfaceScatteringSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldRenderSettings, sub_surface_scattering_sample_count),
            },
            FieldInfoData {
                name: "SplitLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, split_lighting_enable),
            },
            FieldInfoData {
                name: "SubsurfaceBlurComputeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, subsurface_blur_compute_enable),
            },
            FieldInfoData {
                name: "SubsurfaceBlurQuadtreeTileGenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, subsurface_blur_quadtree_tile_gen_enable),
            },
            FieldInfoData {
                name: "SubsurfaceBlurPixelRadiusCullThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, subsurface_blur_pixel_radius_cull_threshold),
            },
            FieldInfoData {
                name: "OpaqueSortBySolutionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, opaque_sort_by_solution_enable),
            },
            FieldInfoData {
                name: "MainOpaqueZPassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, main_opaque_z_pass_enable),
            },
            FieldInfoData {
                name: "PlanarReflectionEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, planar_reflection_enable),
            },
            FieldInfoData {
                name: "PlanarReflectionFastHdrEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, planar_reflection_fast_hdr_enable),
            },
            FieldInfoData {
                name: "PlanarReflectionViewScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, planar_reflection_view_scale),
            },
            FieldInfoData {
                name: "PlanarReflectionBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, planar_reflection_blur_enable),
            },
            FieldInfoData {
                name: "PlanarReflectionConvolutionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, planar_reflection_convolution_enable),
            },
            FieldInfoData {
                name: "PlanarReflectionConvolutionSampleClampThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, planar_reflection_convolution_sample_clamp_threshold),
            },
            FieldInfoData {
                name: "PlanarReflectionConvolutionSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, planar_reflection_convolution_sample_count),
            },
            FieldInfoData {
                name: "PlanarReflectionConvolutionRandomSamplesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, planar_reflection_convolution_random_samples_enable),
            },
            FieldInfoData {
                name: "PlanarReflectionConvolutionPostFilterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, planar_reflection_convolution_post_filter_enable),
            },
            FieldInfoData {
                name: "PlanarReflectionCullFOV",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, planar_reflection_cull_f_o_v),
            },
            FieldInfoData {
                name: "PlanarReflectionClippingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, planar_reflection_clipping_enable),
            },
            FieldInfoData {
                name: "DrawDebugRenderTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_render_texture),
            },
            FieldInfoData {
                name: "DrawDebugPlanarReflection",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_planar_reflection),
            },
            FieldInfoData {
                name: "DrawDebugPlanarReflectionMipLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_planar_reflection_mip_level),
            },
            FieldInfoData {
                name: "DrawDebugPlanarReflectionMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_planar_reflection_mode),
            },
            FieldInfoData {
                name: "DrawDebugPlanarReflectionCullFrustum",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_planar_reflection_cull_frustum),
            },
            FieldInfoData {
                name: "LocalPlanarReflectionConvolutionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_planar_reflection_convolution_enable),
            },
            FieldInfoData {
                name: "LocalPlanarReflectionForceLowestLodEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_planar_reflection_force_lowest_lod_enable),
            },
            FieldInfoData {
                name: "ReflectionLodScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, reflection_lod_scale),
            },
            FieldInfoData {
                name: "ObjectHighlightEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, object_highlight_enable),
            },
            FieldInfoData {
                name: "ObjectHighlightMaskFirstPersonEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, object_highlight_mask_first_person_enable),
            },
            FieldInfoData {
                name: "TransparentDepthZPrepassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, transparent_depth_z_prepass_enable),
            },
            FieldInfoData {
                name: "HologramRendertargetEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, hologram_rendertarget_enable),
            },
            FieldInfoData {
                name: "SonarRendertargetEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, sonar_rendertarget_enable),
            },
            FieldInfoData {
                name: "SimpleVolumetricsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, simple_volumetrics_enable),
            },
            FieldInfoData {
                name: "SimpleVolumetricsHalfResEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, simple_volumetrics_half_res_enable),
            },
            FieldInfoData {
                name: "FogExclusionVolumeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, fog_exclusion_volume_enable),
            },
            FieldInfoData {
                name: "ThreatAlertOverlayEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, threat_alert_overlay_enable),
            },
            FieldInfoData {
                name: "MeshComputeEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, mesh_compute_enabled),
            },
            FieldInfoData {
                name: "OutlineEdgeDetectEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, outline_edge_detect_enable),
            },
            FieldInfoData {
                name: "OverlayBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, overlay_blur_enable),
            },
            FieldInfoData {
                name: "OverlayBlurAsyncComputeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, overlay_blur_async_compute_enable),
            },
            FieldInfoData {
                name: "OverlayBlurKernelRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, overlay_blur_kernel_radius),
            },
            FieldInfoData {
                name: "PostProcessAntialiasingMode",
                flags: MemberInfoFlags::new(0),
                field_type: "PostProcessAAMode",
                rust_offset: offset_of!(WorldRenderSettings, post_process_antialiasing_mode),
            },
            FieldInfoData {
                name: "SmaaPredicatedThresholdingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, smaa_predicated_thresholding_enable),
            },
            FieldInfoData {
                name: "TemporalAAJitterCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, temporal_a_a_jitter_count),
            },
            FieldInfoData {
                name: "TemporalAADisocclusionRejectionFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, temporal_a_a_disocclusion_rejection_factor),
            },
            FieldInfoData {
                name: "TemporalAAHistorySharpeningEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, temporal_a_a_history_sharpening_enable),
            },
            FieldInfoData {
                name: "TemporalAAMotionSharpeningFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, temporal_a_a_motion_sharpening_factor),
            },
            FieldInfoData {
                name: "TemporalAAResponsiveness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, temporal_a_a_responsiveness),
            },
            FieldInfoData {
                name: "TemporalAAAntiflickerStrength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, temporal_a_a_antiflicker_strength),
            },
            FieldInfoData {
                name: "TemporalAAQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, temporal_a_a_quality),
            },
            FieldInfoData {
                name: "TemporalAAPostSharpeningAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, temporal_a_a_post_sharpening_amount),
            },
            FieldInfoData {
                name: "DrawDebugTemporalAAEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_temporal_a_a_enable),
            },
            FieldInfoData {
                name: "DrawDebugTemporalAAAccumulationCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_temporal_a_a_accumulation_count),
            },
            FieldInfoData {
                name: "DrawDebugTemporalAADebugMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_temporal_a_a_debug_mode),
            },
            FieldInfoData {
                name: "DrawDebugTemporalAAMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_temporal_a_a_max_distance),
            },
            FieldInfoData {
                name: "TemporalAADofCocFilterEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, temporal_a_a_dof_coc_filter_enable),
            },
            FieldInfoData {
                name: "DLAAJitterCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, d_l_a_a_jitter_count),
            },
            FieldInfoData {
                name: "DLAAJitterScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, d_l_a_a_jitter_scale),
            },
            FieldInfoData {
                name: "DLAADrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, d_l_a_a_draw_enable),
            },
            FieldInfoData {
                name: "DLAACaptureEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, d_l_a_a_capture_enable),
            },
            FieldInfoData {
                name: "SkyCelestialEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, sky_celestial_enable),
            },
            FieldInfoData {
                name: "SkyCelestialQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WorldRenderSettings, sky_celestial_quality),
            },
            FieldInfoData {
                name: "SkyCelestialMaxQuadCount",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(WorldRenderSettings, sky_celestial_max_quad_count),
            },
            FieldInfoData {
                name: "SkyRenderMode",
                flags: MemberInfoFlags::new(0),
                field_type: "SkyRenderMode",
                rust_offset: offset_of!(WorldRenderSettings, sky_render_mode),
            },
            FieldInfoData {
                name: "LensReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, lens_reflection_enable),
            },
            FieldInfoData {
                name: "DofBeforeMotionBlur",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, dof_before_motion_blur),
            },
            FieldInfoData {
                name: "LensDirtEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, lens_dirt_enable),
            },
            FieldInfoData {
                name: "ParallelCreateDrawViewEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, parallel_create_draw_view_enable),
            },
            FieldInfoData {
                name: "DrawHologramWithTemporalAA",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_hologram_with_temporal_a_a),
            },
            FieldInfoData {
                name: "InterpupillaryDistance",
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, interpupillary_distance),
            },
            FieldInfoData {
                name: "VrHmdLensDistortionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, vr_hmd_lens_distortion_enable),
            },
            FieldInfoData {
                name: "VrHmdLateReprojectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, vr_hmd_late_reprojection_enable),
            },
            FieldInfoData {
                name: "EnablePersistentSinkUsage",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, enable_persistent_sink_usage),
            },
            FieldInfoData {
                name: "RaytraceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_enable),
            },
            FieldInfoData {
                name: "RaytraceDebugEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_debug_enable),
            },
            FieldInfoData {
                name: "RaytraceForwardEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_forward_enable),
            },
            FieldInfoData {
                name: "RaytraceBuildAsyncComputeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_build_async_compute_enable),
            },
            FieldInfoData {
                name: "RaytraceAoEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_ao_enable),
            },
            FieldInfoData {
                name: "RaytraceReflEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_refl_enable),
            },
            FieldInfoData {
                name: "RaytraceReflRayToPixelRatio",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_refl_ray_to_pixel_ratio),
            },
            FieldInfoData {
                name: "RaytraceReflTransparentEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_refl_transparent_enable),
            },
            FieldInfoData {
                name: "RaytraceReflForceMinSmoothness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_refl_force_min_smoothness),
            },
            FieldInfoData {
                name: "RaytraceReflSsrCompareEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_refl_ssr_compare_enable),
            },
            FieldInfoData {
                name: "RaytraceReflSsrCompareInvertSide",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_refl_ssr_compare_invert_side),
            },
            FieldInfoData {
                name: "RaytraceReflSsrCompareLocation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_refl_ssr_compare_location),
            },
            FieldInfoData {
                name: "RaytracePrimaryRayEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_primary_ray_enable),
            },
            FieldInfoData {
                name: "RaytraceCullMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_cull_mode),
            },
            FieldInfoData {
                name: "RaytraceCullFrustumFovY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_cull_frustum_fov_y),
            },
            FieldInfoData {
                name: "RaytraceCullRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_cull_radius),
            },
            FieldInfoData {
                name: "RaytraceCullRadiusLargeObjectScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_cull_radius_large_object_scale),
            },
            FieldInfoData {
                name: "RaytraceCullFrustumLargeObjectScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_cull_frustum_large_object_scale),
            },
            FieldInfoData {
                name: "RaytraceCullRadiusEmitters",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, raytrace_cull_radius_emitters),
            },
            FieldInfoData {
                name: "LocalLightShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_light_shadow_enable),
            },
            FieldInfoData {
                name: "LocalLightShadow16BitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_light_shadow16_bit_enable),
            },
            FieldInfoData {
                name: "LocalLightShadowFilterQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_light_shadow_filter_quality),
            },
            FieldInfoData {
                name: "LocalLightShadowResolutionLow",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_light_shadow_resolution_low),
            },
            FieldInfoData {
                name: "LocalLightShadowResolutionMedium",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_light_shadow_resolution_medium),
            },
            FieldInfoData {
                name: "LocalLightShadowResolutionHigh",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_light_shadow_resolution_high),
            },
            FieldInfoData {
                name: "LocalLightShadowResolutionUltra",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_light_shadow_resolution_ultra),
            },
            FieldInfoData {
                name: "LocalLightShadowAtlasSlotCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_light_shadow_atlas_slot_count),
            },
            FieldInfoData {
                name: "LocalLightShadowAtlasSlotResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_light_shadow_atlas_slot_resolution),
            },
            FieldInfoData {
                name: "ReflectionLocalLightShadowResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, reflection_local_light_shadow_resolution),
            },
            FieldInfoData {
                name: "LocalLightShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_light_shadow_cache_enable),
            },
            FieldInfoData {
                name: "MaxShadowCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, max_shadow_count),
            },
            FieldInfoData {
                name: "MaxPunctualLightCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, max_punctual_light_count),
            },
            FieldInfoData {
                name: "MaxPunctualShadowLightCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, max_punctual_shadow_light_count),
            },
            FieldInfoData {
                name: "MaxAreaLightCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, max_area_light_count),
            },
            FieldInfoData {
                name: "MaxAreaShadowLightCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, max_area_shadow_light_count),
            },
            FieldInfoData {
                name: "MaxLocalReflectionVolumeCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, max_local_reflection_volume_count),
            },
            FieldInfoData {
                name: "MaxLocalPlanarReflectionCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, max_local_planar_reflection_count),
            },
            FieldInfoData {
                name: "MaxPunctualRectangularLightCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, max_punctual_rectangular_light_count),
            },
            FieldInfoData {
                name: "PBRSupportOriginalLight",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_support_original_light),
            },
            FieldInfoData {
                name: "RadiosityShadowCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, radiosity_shadow_culling_enable),
            },
            FieldInfoData {
                name: "PunctualLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, punctual_lights_enable),
            },
            FieldInfoData {
                name: "AreaLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, area_lights_enable),
            },
            FieldInfoData {
                name: "LocalReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_reflection_enable),
            },
            FieldInfoData {
                name: "TilePassPunctualLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, tile_pass_punctual_lights_enable),
            },
            FieldInfoData {
                name: "TilePassPunctualLightShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, tile_pass_punctual_light_shadow_enable),
            },
            FieldInfoData {
                name: "TilePassAreaLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, tile_pass_area_lights_enable),
            },
            FieldInfoData {
                name: "TilePassAreaLightShadowEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, tile_pass_area_light_shadow_enable),
            },
            FieldInfoData {
                name: "TilePassLocalReflectionVolumeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, tile_pass_local_reflection_volume_enable),
            },
            FieldInfoData {
                name: "TilePassLocalPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, tile_pass_local_planar_reflection_enable),
            },
            FieldInfoData {
                name: "PunctualLightShadowLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WorldRenderSettings, punctual_light_shadow_level),
            },
            FieldInfoData {
                name: "AreaLightShadowLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WorldRenderSettings, area_light_shadow_level),
            },
            FieldInfoData {
                name: "SphereLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, sphere_lights_enable),
            },
            FieldInfoData {
                name: "PunctualSphereLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, punctual_sphere_lights_enable),
            },
            FieldInfoData {
                name: "SpotLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, spot_lights_enable),
            },
            FieldInfoData {
                name: "PunctualSpotLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, punctual_spot_lights_enable),
            },
            FieldInfoData {
                name: "TubeLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, tube_lights_enable),
            },
            FieldInfoData {
                name: "PunctualTubeLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, punctual_tube_lights_enable),
            },
            FieldInfoData {
                name: "RectangularLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, rectangular_lights_enable),
            },
            FieldInfoData {
                name: "PunctualRectangularLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, punctual_rectangular_lights_enable),
            },
            FieldInfoData {
                name: "LocalReflectionVolumeSphereEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_reflection_volume_sphere_enable),
            },
            FieldInfoData {
                name: "LocalReflectionVolumeBoxEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_reflection_volume_box_enable),
            },
            FieldInfoData {
                name: "LocalPlanarReflectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_planar_reflection_enable),
            },
            FieldInfoData {
                name: "LocalIBLMaxFaceCapture",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_max_face_capture),
            },
            FieldInfoData {
                name: "LocalIBLUpdateWithSkyEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_update_with_sky_enable),
            },
            FieldInfoData {
                name: "LocalIBLUpdateWithEnlightenSkyBoxChange",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_update_with_enlighten_sky_box_change),
            },
            FieldInfoData {
                name: "LocalIBLSunSpecularOcclusionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_sun_specular_occlusion_enable),
            },
            FieldInfoData {
                name: "LocalIBLLightingUpdateCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_lighting_update_count),
            },
            FieldInfoData {
                name: "LocalIBLRefreshDelayCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_refresh_delay_count),
            },
            FieldInfoData {
                name: "LocalIBLBoxCullingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_box_culling_enable),
            },
            FieldInfoData {
                name: "LocalIBLSunUpdateThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_sun_update_threshold),
            },
            FieldInfoData {
                name: "LocalIBLShadowmapSliceCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_shadowmap_slice_count),
            },
            FieldInfoData {
                name: "LocalIBLShadowmapResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_shadowmap_resolution),
            },
            FieldInfoData {
                name: "LocalIBLShadowmapFaceMerging",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_shadowmap_face_merging),
            },
            FieldInfoData {
                name: "LocalIBLShadowmapSeparateFrame",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_shadowmap_separate_frame),
            },
            FieldInfoData {
                name: "LocalIBLWaitForEnlightenToRender",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_wait_for_enlighten_to_render),
            },
            FieldInfoData {
                name: "LocalIBLExposure",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_exposure),
            },
            FieldInfoData {
                name: "LocalIBLRenderTransparent",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_render_transparent),
            },
            FieldInfoData {
                name: "LocalIBLRenderEmitterQuad",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_render_emitter_quad),
            },
            FieldInfoData {
                name: "LocalIBLRenderEmitterMesh",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_render_emitter_mesh),
            },
            FieldInfoData {
                name: "PBRLocalIBLAcquisitionWaitForMeshLoading",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_local_i_b_l_acquisition_wait_for_mesh_loading),
            },
            FieldInfoData {
                name: "PBRLocalIBLAcquisitionWaitFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_local_i_b_l_acquisition_wait_frame_count),
            },
            FieldInfoData {
                name: "PBRDrawDistantIBLDiffuseReference",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_draw_distant_i_b_l_diffuse_reference),
            },
            FieldInfoData {
                name: "PBRDrawDistantIBLSpecularReference",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_draw_distant_i_b_l_specular_reference),
            },
            FieldInfoData {
                name: "PBRDrawLocalIBLReference",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_draw_local_i_b_l_reference),
            },
            FieldInfoData {
                name: "PBRDrawAreaLightReference",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_draw_area_light_reference),
            },
            FieldInfoData {
                name: "PBRSpecularConvolutionSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_specular_convolution_sample_count),
            },
            FieldInfoData {
                name: "PBRConvolutionHighestMIPEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_convolution_highest_m_i_p_enable),
            },
            FieldInfoData {
                name: "LocalIBLResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, local_i_b_l_resolution),
            },
            FieldInfoData {
                name: "DrawDebugLocalIBLEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_local_i_b_l_enable),
            },
            FieldInfoData {
                name: "DrawDebugLocalIBLStatsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_local_i_b_l_stats_enable),
            },
            FieldInfoData {
                name: "DrawDebugLocalIBLVolumesEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_local_i_b_l_volumes_enable),
            },
            FieldInfoData {
                name: "DrawDebugLocalIBLPreviewScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_local_i_b_l_preview_scale),
            },
            FieldInfoData {
                name: "DrawDebugLocalIBLIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_local_i_b_l_index),
            },
            FieldInfoData {
                name: "DrawDebugLocalIBLMipLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_local_i_b_l_mip_level),
            },
            FieldInfoData {
                name: "DrawDebugLocalIBLShadowmaps",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_local_i_b_l_shadowmaps),
            },
            FieldInfoData {
                name: "DrawDebugPreIntegratedFGTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_pre_integrated_f_g_texture),
            },
            FieldInfoData {
                name: "DrawDebugReflectionState",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_reflection_state),
            },
            FieldInfoData {
                name: "DrawDebugProbeMirrorEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_probe_mirror_enable),
            },
            FieldInfoData {
                name: "DrawDebugProbeDiffuseEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_probe_diffuse_enable),
            },
            FieldInfoData {
                name: "DrawDebugProbeScreenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_probe_screen_enable),
            },
            FieldInfoData {
                name: "DrawDebugProbeScreenOnRight",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_probe_screen_on_right),
            },
            FieldInfoData {
                name: "ContinuousLocalIBLEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, continuous_local_i_b_l_enable),
            },
            FieldInfoData {
                name: "ContinuousLocalIBLIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, continuous_local_i_b_l_index),
            },
            FieldInfoData {
                name: "PBRConvolutionPrecomputedSampleEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_convolution_precomputed_sample_enable),
            },
            FieldInfoData {
                name: "PBRConvolutionRandomRotationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_convolution_random_rotation_enable),
            },
            FieldInfoData {
                name: "DrawDebugLocalPlanarReflections",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_local_planar_reflections),
            },
            FieldInfoData {
                name: "EmitterQuadRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, emitter_quad_rendering_enable),
            },
            FieldInfoData {
                name: "EmitterMeshRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, emitter_mesh_rendering_enable),
            },
            FieldInfoData {
                name: "EmitterPointLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, emitter_point_lights_enable),
            },
            FieldInfoData {
                name: "EmitterSpotLightsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, emitter_spot_lights_enable),
            },
            FieldInfoData {
                name: "UseSSSProfileforOATS",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, use_s_s_s_profilefor_o_a_t_s),
            },
            FieldInfoData {
                name: "DeterministicRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, deterministic_rendering_enable),
            },
            FieldInfoData {
                name: "HdrNanInfRemovalEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, hdr_nan_inf_removal_enable),
            },
            FieldInfoData {
                name: "HdrNanInfRemovalForceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, hdr_nan_inf_removal_force_enable),
            },
            FieldInfoData {
                name: "PBRMaxIlluminanceValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_max_illuminance_value),
            },
            FieldInfoData {
                name: "PBRMaxLuminanceValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, p_b_r_max_luminance_value),
            },
            FieldInfoData {
                name: "DielectricRangeMinMetalMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, dielectric_range_min_metal_mask),
            },
            FieldInfoData {
                name: "DielectricRangeMaxMetalMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, dielectric_range_max_metal_mask),
            },
            FieldInfoData {
                name: "DielectricRangeSRGBMinValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, dielectric_range_s_r_g_b_min_value),
            },
            FieldInfoData {
                name: "DielectricRangeSRGBMaxValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, dielectric_range_s_r_g_b_max_value),
            },
            FieldInfoData {
                name: "DielectricRangeSRGBMinColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WorldRenderSettings, dielectric_range_s_r_g_b_min_color),
            },
            FieldInfoData {
                name: "DielectricRangeSRGBColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WorldRenderSettings, dielectric_range_s_r_g_b_color),
            },
            FieldInfoData {
                name: "DielectricRangeSRGBMaxColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WorldRenderSettings, dielectric_range_s_r_g_b_max_color),
            },
            FieldInfoData {
                name: "DielectricRangeOverlay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, dielectric_range_overlay),
            },
            FieldInfoData {
                name: "ConductorRangeMinMetalMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, conductor_range_min_metal_mask),
            },
            FieldInfoData {
                name: "ConductorRangeMaxMetalMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, conductor_range_max_metal_mask),
            },
            FieldInfoData {
                name: "ConductorRangeSRGBMinValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, conductor_range_s_r_g_b_min_value),
            },
            FieldInfoData {
                name: "ConductorRangeSRGBMaxValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, conductor_range_s_r_g_b_max_value),
            },
            FieldInfoData {
                name: "ConductorRangeSRGBMinColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WorldRenderSettings, conductor_range_s_r_g_b_min_color),
            },
            FieldInfoData {
                name: "ConductorRangeSRGBColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WorldRenderSettings, conductor_range_s_r_g_b_color),
            },
            FieldInfoData {
                name: "ConductorRangeSRGBMaxColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WorldRenderSettings, conductor_range_s_r_g_b_max_color),
            },
            FieldInfoData {
                name: "ConductorRangeOverlay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, conductor_range_overlay),
            },
            FieldInfoData {
                name: "Fresnel0RangeMinMetalMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, fresnel0_range_min_metal_mask),
            },
            FieldInfoData {
                name: "Fresnel0RangeMaxMetalMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, fresnel0_range_max_metal_mask),
            },
            FieldInfoData {
                name: "VolumetricRenderingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_rendering_enable),
            },
            FieldInfoData {
                name: "VolumetricExtinctionCascadeBaseVoxelSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_extinction_cascade_base_voxel_size),
            },
            FieldInfoData {
                name: "VolumetricExtinctionCascadeVoxelSizeCascadeFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_extinction_cascade_voxel_size_cascade_factor),
            },
            FieldInfoData {
                name: "VolumetricExtinctionCascadeResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_extinction_cascade_resolution),
            },
            FieldInfoData {
                name: "VolumetricExtinctionCascadeLockView",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_extinction_cascade_lock_view),
            },
            FieldInfoData {
                name: "VolumetricShadowmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_shadowmap_enable),
            },
            FieldInfoData {
                name: "VolumetricShadowmapResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_shadowmap_resolution),
            },
            FieldInfoData {
                name: "VolumetricShadowmapMaxCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_shadowmap_max_count),
            },
            FieldInfoData {
                name: "VolumetricShadowmapAccumulateCsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_shadowmap_accumulate_cs_enable),
            },
            FieldInfoData {
                name: "PunctualLightCastVolumetricShadowmapEnableLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WorldRenderSettings, punctual_light_cast_volumetric_shadowmap_enable_level),
            },
            FieldInfoData {
                name: "AreaLightCastVolumetricShadowmapEnableLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WorldRenderSettings, area_light_cast_volumetric_shadowmap_enable_level),
            },
            FieldInfoData {
                name: "LocalLightCastVolumetricLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WorldRenderSettings, local_light_cast_volumetric_level),
            },
            FieldInfoData {
                name: "VolumetricEmitterVoxelisationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_emitter_voxelisation_enable),
            },
            FieldInfoData {
                name: "VolumetricEmitterVoxelisationMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_emitter_voxelisation_mode),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_enable),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaUseLightCull",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_use_light_cull),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaTextureDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_texture_depth),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaCameraDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_camera_depth),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_resolution_scale),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaFromVEFog",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_from_v_e_fog),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaLockView",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_lock_view),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaLocalLights",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_local_lights),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaAmbientLighting",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_ambient_lighting),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaSun",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_sun),
            },
            FieldInfoData {
                name: "ReflectionVolumetricParticipatingMediaTextureDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, reflection_volumetric_participating_media_texture_depth),
            },
            FieldInfoData {
                name: "ReflectionVolumetricParticipatingMediaCameraDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, reflection_volumetric_participating_media_camera_depth),
            },
            FieldInfoData {
                name: "ReflectionVolumetricParticipatingMediaResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, reflection_volumetric_participating_media_resolution_scale),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaTemporalIntegration",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_temporal_integration),
            },
            FieldInfoData {
                name: "VolumetricParticipatingMediaTemporalFilterStrght",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_participating_media_temporal_filter_strght),
            },
            FieldInfoData {
                name: "VolumetricLocalFogVolumeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, volumetric_local_fog_volume_enable),
            },
            FieldInfoData {
                name: "DrawDebugVolumetricLocalFogVolume",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_volumetric_local_fog_volume),
            },
            FieldInfoData {
                name: "DrawDebugVolumetricCascadedVolumes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_volumetric_cascaded_volumes),
            },
            FieldInfoData {
                name: "DrawDebugVolumetricShadowMaps",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_volumetric_shadow_maps),
            },
            FieldInfoData {
                name: "DrawDebugVolumetricExtinction",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_volumetric_extinction),
            },
            FieldInfoData {
                name: "DrawDebugVolumetricExtinctionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_volumetric_extinction_scale),
            },
            FieldInfoData {
                name: "DrawDebugVolumetricVoxelisedEmitter",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_volumetric_voxelised_emitter),
            },
            FieldInfoData {
                name: "LightShaftFastHdrEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, light_shaft_fast_hdr_enable),
            },
            FieldInfoData {
                name: "DrawGpuHistogramEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_gpu_histogram_enable),
            },
            FieldInfoData {
                name: "DrawGpuHistogramRed",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_gpu_histogram_red),
            },
            FieldInfoData {
                name: "DrawGpuHistogramBlue",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_gpu_histogram_blue),
            },
            FieldInfoData {
                name: "DrawGpuHistogramGreen",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_gpu_histogram_green),
            },
            FieldInfoData {
                name: "DrawGpuHistogramLuminance",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_gpu_histogram_luminance),
            },
            FieldInfoData {
                name: "DrawGpuHistogramHDRMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_gpu_histogram_h_d_r_mode),
            },
            FieldInfoData {
                name: "DrawGpuHistogramHDRMinEV",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, draw_gpu_histogram_h_d_r_min_e_v),
            },
            FieldInfoData {
                name: "DrawGpuHistogramHDRMaxEV",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, draw_gpu_histogram_h_d_r_max_e_v),
            },
            FieldInfoData {
                name: "DrawGpuHistogramBinCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettings, draw_gpu_histogram_bin_count),
            },
            FieldInfoData {
                name: "DrawClampedPixelsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_clamped_pixels_enable),
            },
            FieldInfoData {
                name: "DrawClampedPixelsClampMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, draw_clamped_pixels_clamp_min),
            },
            FieldInfoData {
                name: "DrawClampedPixelsClampMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettings, draw_clamped_pixels_clamp_max),
            },
            FieldInfoData {
                name: "EnableBakedDirectLightInGameView",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, enable_baked_direct_light_in_game_view),
            },
            FieldInfoData {
                name: "EnableBakedLightmapInGameView",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, enable_baked_lightmap_in_game_view),
            },
            FieldInfoData {
                name: "TextureSpaceRenderModuleEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, texture_space_render_module_enable),
            },
            FieldInfoData {
                name: "ComputeLinearizeDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, compute_linearize_depth),
            },
            FieldInfoData {
                name: "ComputeDownsampleDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, compute_downsample_depth),
            },
            FieldInfoData {
                name: "DrawDebugDisableExplanation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettings, draw_debug_disable_explanation),
            },
        ],
    }),
    array_type: Some(WORLDRENDERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WorldRenderSettings {
    fn type_info(&self) -> &'static TypeInfo {
        WORLDRENDERSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WORLDRENDERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldRenderSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("WorldRenderSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WorldRenderSettingsBase {
    pub _glacier_base: super::core::DataContainer,
    pub cull_screen_area_scale: f32,
    pub deferred_shading_enable: bool,
    pub forward_opaque_enable: bool,
    pub full_z_pass_enable: bool,
    pub tile_material_classification_enable: bool,
    pub shadowmaps_enable: bool,
    pub shadowmap_array_enable: bool,
    pub transparency_shadowmaps_enable: bool,
    pub n_v_i_d_i_a_shadows_p_c_s_s_enable: bool,
    pub n_v_i_d_i_a_shadows_h_f_t_s_enable: bool,
    pub transparency_shadowmaps_half_res: bool,
    pub sun_shadowmap_level: super::core::QualityLevel,
    pub shadowmap_min_fov: f32,
    pub shadowmap_fixed_movement_enable: bool,
    pub shadowmap_fixed_depth_enable: bool,
    pub shadowmap_size_z_scale: f32,
    pub shadowmap_resolution: u32,
    pub adjusted_shadowmap_resolution: u32,
    pub shadowmap_quality: u32,
    pub shadowmap_poisson_filter_scale: f32,
    pub shadowmap_slice_count: u32,
    pub adjusted_shadowmap_slice_count: u32,
    pub shadowmap_slice_scheme_weight: f32,
    pub shadowmap_first_slice_scale: f32,
    pub shadowmap_view_distance: f32,
    pub shadowmap_view_distance_scale_enable: bool,
    pub shadowmap_extrusion_length: f32,
    pub shadowmap_first_slice_extrusion_length: f32,
    pub shadowmap_adjust_far_plane: bool,
    pub draw_debug_shadowmap_cascades: bool,
    pub shadowmap_accum_enable: bool,
    pub shadowmap_accum_reuse_enable: bool,
    pub shadowmap_accum_stencil_enable: bool,
    pub shadowmap_accum_stencil2_enable: bool,
    pub shadowmap_transition_blend_enable: bool,
    pub shadowmap_transition_blend_amount: f32,
    pub shadowmap_foreground_enable: bool,
    pub shadowmap_foreground_use_first_person_view_transform: bool,
    pub shadowmap_foreground_extrusion_length: f32,
    pub shadowmap_foreground_split_distance: f32,
    pub shadowmap_foreground_size_z_scale: f32,
    pub shadowmap_adjust_shadow_distance_with_fov: bool,
    pub shadowmap_draw_batched_enable: bool,
    pub distant_shadow_cache_draw_frustum: bool,
    pub distant_shadow_cache_enable: bool,
    pub distant_shadow_cache_resolution: u32,
    pub distant_shadow_cache_force_resolution: i32,
    pub distant_shadow_cache_resolution_scale: f32,
    pub distant_shadow_cache_max_resolution: u32,
    pub distant_shadow_cache_draw_debug: bool,
    pub distant_shadow_cache_force_mode: u32,
    pub distant_shadow_cache_use_quadtree: bool,
    pub distant_shadow_cache_batch_group_entity_bake: bool,
    pub distant_shadow_cache_rebake_on_light_change: bool,
    pub distant_shadow_cache_rebake_on_add_remove: bool,
    pub distant_shadow_cache_rebake_on_move: bool,
    pub distant_shadow_cache_rebake_on_part_visibility: bool,
    pub distant_shadow_cache_coalesce_time: f32,
    pub distant_shadow_cache_max_bake_events: u32,
    pub sun_pcss_max_sample_count: i32,
    pub sun_pcss_adaptive_sample_increment: i32,
    pub dx_shadowmap16_bit_enable: bool,
    pub dx_dynamic_envmap_shadowmap16_bit_enable: bool,
    pub apply_shadowmaps_enable: bool,
    pub simple_shadowmaps_enable: bool,
    pub emitter_shadowing_blend_toggle: bool,
    pub emitter_shadowing_many_samples_toggle: bool,
    pub dx_linear_depth32_bit_format_enable: bool,
    pub motion_blur_enable: bool,
    pub motion_blur_force_on: bool,
    pub motion_blur_scale: f32,
    pub motion_blur_fixed_shutter_time: f32,
    pub motion_blur_max: f32,
    pub motion_blur_radial_blur_max: f32,
    pub motion_blur_noise_scale: f32,
    pub motion_blur_quality: u32,
    pub motion_blur_debug_mode: u32,
    pub motion_blur_perceptual_space_enable: bool,
    pub motion_blur_stencil_pass_enable: bool,
    pub motion_blur_centered_enable: bool,
    pub motion_blur_max_sample_count: u32,
    pub motion_blur_depth_check_threshold: f32,
    pub motion_blur_depth_check_max_distance: f32,
    pub tiled_motion_blur_variance_threshold_scale: f32,
    pub tiled_motion_blur_vel_mag_depth_downsample: u32,
    pub tiled_motion_blur_separable_enable: bool,
    pub tiled_motion_blur_enable: bool,
    pub tiled_motion_blur_force20_px_tile: bool,
    pub motion_blur_use_detailed_gpu_timers: bool,
    pub velocity_vectors_derive_from_depth_enable: bool,
    pub velocity_vectors_derive_from_dynamic_objects_enable: bool,
    pub velocity_vectors_clear_value: super::core::Vec3,
    pub multisample_count: u32,
    pub multisample_quality: u32,
    pub draw_transparent: bool,
    pub draw_half_res_transparent: bool,
    pub draw_transparent_decal: bool,
    pub transparent_dof_enable: bool,
    pub transparent_dof_half_res_enable: bool,
    pub transparent_dof_lerp_coc_enable: bool,
    pub only_shadowmap_slice: i32,
    pub view_mode: WorldViewMode,
    pub enable: bool,
    pub console_render_target_pool_sharing_enable: bool,
    pub fast_hdr_enable: bool,
    pub additional_hdr_target_in_e_s_r_a_m: u32,
    pub linear_depth_in_e_s_r_a_m: bool,
    pub half_res_depth_resolve_enable: bool,
    pub depth_buffer_collision_enable: bool,
    pub final_post_enable: bool,
    pub output_gamma_correction_enable: bool,
    pub screen_effect_enable: bool,
    pub draw_solid_bounding_boxes: bool,
    pub draw_line_bounding_boxes: bool,
    pub draw_bounding_spheres: bool,
    pub draw_frustums: bool,
    pub draw_local_i_b_l_frustums: bool,
    pub draw_debug_shadowmaps: bool,
    pub draw_debug_local_light_shadows: bool,
    pub draw_debug_sky_envmap: bool,
    pub draw_debug_velocity_buffer: bool,
    pub draw_debug_half_res_environment: bool,
    pub draw_debug_distortion: bool,
    pub draw_debug_visible_entity_types: bool,
    pub draw_debug_sky_textures: bool,
    pub draw_debug_dof: bool,
    pub draw_debug_half_res_hdr_targets: bool,
    pub draw_debug_hi_z_min_max_buffer_enable: bool,
    pub draw_debug_screen_space_raytrace_buckets_enable: bool,
    pub draw_debug_emitter_sun_transmittance_maps: bool,
    pub draw_debug_blur_pyramid: bool,
    pub draw_debug_occlusion_z_buffer: bool,
    pub draw_debug_local_i_b_l_occlusion_z_buffer: bool,
    pub draw_debug_buffers: u32,
    pub wireframe_enable: bool,
    pub z_pass_enable: bool,
    pub occluder_mesh_z_prepass_enable: bool,
    pub occluder_mesh_z_prepass_draw_enable: bool,
    pub occluder_mesh_z_prepass_debug_enable: bool,
    pub frame_synthesis_mode: FrameSynthesisMode,
    pub half_res_enable: bool,
    pub force_full_res_enable: bool,
    pub half_res_lens_flares_enable: bool,
    pub foreground_enable: bool,
    pub foreground_z_pass_enable: bool,
    pub foreground_transparent_enable: bool,
    pub bilateral_half_res_composite_enable: bool,
    pub half_res_depth_min_max_dither_enable: bool,
    pub half_res_depth_min_max_dither_threshold: f32,
    pub sky_lighting_enable: bool,
    pub sky_render_enable: bool,
    pub sky_depth_fog_enable: bool,
    pub sky_height_fog_enable: bool,
    pub sky_forward_scattering_enable: bool,
    pub procedural_sky_receive_height_fog: bool,
    pub physical_sky_enabled: bool,
    pub physical_sky_precision_height: u32,
    pub physical_sky_precision_view: u32,
    pub physical_sky_precision_sun: u32,
    pub physical_sky_scattering_orders: u32,
    pub physical_sky_aerial_perspective_texture_width: u32,
    pub physical_sky_aerial_perspective_texture_height: u32,
    pub physical_sky_aerial_perspective_texture_depth: u32,
    pub physical_sky_scattering_eval_frame_count: u32,
    pub physical_sky_aerial_perspective_max_distance: f32,
    pub physical_sky_force_precompute: bool,
    pub volumetric_clouds_enabled: bool,
    pub volumetric_clouds_quality: super::core::QualityLevel,
    pub volumetric_clouds_cast_shadow: bool,
    pub volumetric_clouds_cast_shadow_in_forward_render: bool,
    pub volumetric_clouds_affect_aerial_perspective: bool,
    pub volumetric_clouds_receive_aerial_perspective: bool,
    pub volumetric_clouds_occlude_lens_flare: bool,
    pub volumetric_clouds_render_target_resolution_divider: u32,
    pub volumetric_clouds_reflection_render_target_resolution_divider: u32,
    pub volumetric_clouds_shadow_iteration_count: u32,
    pub volumetric_clouds_shadowmap_resolution: u32,
    pub volumetric_clouds_shadowmap_blur_samples: u32,
    pub volumetric_clouds_sample_count: super::core::QualityScalableInt,
    pub volumetric_clouds_reflection_sample_count: u32,
    pub volumetric_clouds_i_b_l_sample_count: u32,
    pub volumetric_clouds_temporal_coefficient: f32,
    pub volumetric_clouds_env_color_temporal_coefficient: f32,
    pub transparent_fogging_enable: bool,
    pub distortion_enable: bool,
    pub distortion_half_res_enable: bool,
    pub distortion8_bit_enable: bool,
    pub distortion_tiling_enable: bool,
    pub static_envmap_enable: bool,
    pub custom_envmap_enable: bool,
    pub sky_envmap_enable: bool,
    pub sky_envmap_filter_width: f32,
    pub sky_envmap_resolution: u32,
    pub sky_envmap_mipmap_gen_enable: bool,
    pub draw_debug_sky_envmap_mip_level: i32,
    pub sky_envmap_filter_mode: MipmapFilterMode,
    pub sky_envmap_sides_per_frame_count: u32,
    pub sky_envmap_force_update_enable: bool,
    pub sky_envmap_use_fast_h_d_r: bool,
    pub sky_envmap_debug_color_enable: bool,
    pub sky_envmap_update_count_threshold: f32,
    pub sky_envmap_update_value_threshold: f32,
    pub sky_envmap_cloud_fog_enable: bool,
    pub sky_envmap_generate_no_backdrop_enable: bool,
    pub dynamic_envmap_enable: bool,
    pub draw_debug_dynamic_envmap_mip_level: i32,
    pub dynamic_envmap_mipmap_gen_enable: bool,
    pub draw_debug_dynamic_envmap: bool,
    pub dynamic_envmap_shadowmap_enable: bool,
    pub dynamic_envmap_shadowmap_resolution: u32,
    pub dynamic_envmap_shadowmap_far_plane_override: bool,
    pub dynamic_envmap_shadowmap_far_plane: i32,
    pub dynamic_envmap_shadowmap_shadow_extrusion_override: bool,
    pub dynamic_envmap_shadowmap_shadow_extrusion: i32,
    pub draw_debug_dynamic_envmap_shadowmap: bool,
    pub draw_dynamic_envmap_frustums: bool,
    pub setup_job_enable: bool,
    pub setup_jobs_create_view_job: bool,
    pub prepare_dispatch_list_job_enable: bool,
    pub indirect_specular_intensity: f32,
    pub indirect_specular_reflectance_scale: f32,
    pub indirect_specular_probes_intensity: f32,
    pub indirect_specular_probes_reflectance_scale: f32,
}

pub trait WorldRenderSettingsBaseTrait: super::core::DataContainerTrait {
    fn cull_screen_area_scale(&self) -> &f32;
    fn deferred_shading_enable(&self) -> &bool;
    fn forward_opaque_enable(&self) -> &bool;
    fn full_z_pass_enable(&self) -> &bool;
    fn tile_material_classification_enable(&self) -> &bool;
    fn shadowmaps_enable(&self) -> &bool;
    fn shadowmap_array_enable(&self) -> &bool;
    fn transparency_shadowmaps_enable(&self) -> &bool;
    fn n_v_i_d_i_a_shadows_p_c_s_s_enable(&self) -> &bool;
    fn n_v_i_d_i_a_shadows_h_f_t_s_enable(&self) -> &bool;
    fn transparency_shadowmaps_half_res(&self) -> &bool;
    fn sun_shadowmap_level(&self) -> &super::core::QualityLevel;
    fn shadowmap_min_fov(&self) -> &f32;
    fn shadowmap_fixed_movement_enable(&self) -> &bool;
    fn shadowmap_fixed_depth_enable(&self) -> &bool;
    fn shadowmap_size_z_scale(&self) -> &f32;
    fn shadowmap_resolution(&self) -> &u32;
    fn adjusted_shadowmap_resolution(&self) -> &u32;
    fn shadowmap_quality(&self) -> &u32;
    fn shadowmap_poisson_filter_scale(&self) -> &f32;
    fn shadowmap_slice_count(&self) -> &u32;
    fn adjusted_shadowmap_slice_count(&self) -> &u32;
    fn shadowmap_slice_scheme_weight(&self) -> &f32;
    fn shadowmap_first_slice_scale(&self) -> &f32;
    fn shadowmap_view_distance(&self) -> &f32;
    fn shadowmap_view_distance_scale_enable(&self) -> &bool;
    fn shadowmap_extrusion_length(&self) -> &f32;
    fn shadowmap_first_slice_extrusion_length(&self) -> &f32;
    fn shadowmap_adjust_far_plane(&self) -> &bool;
    fn draw_debug_shadowmap_cascades(&self) -> &bool;
    fn shadowmap_accum_enable(&self) -> &bool;
    fn shadowmap_accum_reuse_enable(&self) -> &bool;
    fn shadowmap_accum_stencil_enable(&self) -> &bool;
    fn shadowmap_accum_stencil2_enable(&self) -> &bool;
    fn shadowmap_transition_blend_enable(&self) -> &bool;
    fn shadowmap_transition_blend_amount(&self) -> &f32;
    fn shadowmap_foreground_enable(&self) -> &bool;
    fn shadowmap_foreground_use_first_person_view_transform(&self) -> &bool;
    fn shadowmap_foreground_extrusion_length(&self) -> &f32;
    fn shadowmap_foreground_split_distance(&self) -> &f32;
    fn shadowmap_foreground_size_z_scale(&self) -> &f32;
    fn shadowmap_adjust_shadow_distance_with_fov(&self) -> &bool;
    fn shadowmap_draw_batched_enable(&self) -> &bool;
    fn distant_shadow_cache_draw_frustum(&self) -> &bool;
    fn distant_shadow_cache_enable(&self) -> &bool;
    fn distant_shadow_cache_resolution(&self) -> &u32;
    fn distant_shadow_cache_force_resolution(&self) -> &i32;
    fn distant_shadow_cache_resolution_scale(&self) -> &f32;
    fn distant_shadow_cache_max_resolution(&self) -> &u32;
    fn distant_shadow_cache_draw_debug(&self) -> &bool;
    fn distant_shadow_cache_force_mode(&self) -> &u32;
    fn distant_shadow_cache_use_quadtree(&self) -> &bool;
    fn distant_shadow_cache_batch_group_entity_bake(&self) -> &bool;
    fn distant_shadow_cache_rebake_on_light_change(&self) -> &bool;
    fn distant_shadow_cache_rebake_on_add_remove(&self) -> &bool;
    fn distant_shadow_cache_rebake_on_move(&self) -> &bool;
    fn distant_shadow_cache_rebake_on_part_visibility(&self) -> &bool;
    fn distant_shadow_cache_coalesce_time(&self) -> &f32;
    fn distant_shadow_cache_max_bake_events(&self) -> &u32;
    fn sun_pcss_max_sample_count(&self) -> &i32;
    fn sun_pcss_adaptive_sample_increment(&self) -> &i32;
    fn dx_shadowmap16_bit_enable(&self) -> &bool;
    fn dx_dynamic_envmap_shadowmap16_bit_enable(&self) -> &bool;
    fn apply_shadowmaps_enable(&self) -> &bool;
    fn simple_shadowmaps_enable(&self) -> &bool;
    fn emitter_shadowing_blend_toggle(&self) -> &bool;
    fn emitter_shadowing_many_samples_toggle(&self) -> &bool;
    fn dx_linear_depth32_bit_format_enable(&self) -> &bool;
    fn motion_blur_enable(&self) -> &bool;
    fn motion_blur_force_on(&self) -> &bool;
    fn motion_blur_scale(&self) -> &f32;
    fn motion_blur_fixed_shutter_time(&self) -> &f32;
    fn motion_blur_max(&self) -> &f32;
    fn motion_blur_radial_blur_max(&self) -> &f32;
    fn motion_blur_noise_scale(&self) -> &f32;
    fn motion_blur_quality(&self) -> &u32;
    fn motion_blur_debug_mode(&self) -> &u32;
    fn motion_blur_perceptual_space_enable(&self) -> &bool;
    fn motion_blur_stencil_pass_enable(&self) -> &bool;
    fn motion_blur_centered_enable(&self) -> &bool;
    fn motion_blur_max_sample_count(&self) -> &u32;
    fn motion_blur_depth_check_threshold(&self) -> &f32;
    fn motion_blur_depth_check_max_distance(&self) -> &f32;
    fn tiled_motion_blur_variance_threshold_scale(&self) -> &f32;
    fn tiled_motion_blur_vel_mag_depth_downsample(&self) -> &u32;
    fn tiled_motion_blur_separable_enable(&self) -> &bool;
    fn tiled_motion_blur_enable(&self) -> &bool;
    fn tiled_motion_blur_force20_px_tile(&self) -> &bool;
    fn motion_blur_use_detailed_gpu_timers(&self) -> &bool;
    fn velocity_vectors_derive_from_depth_enable(&self) -> &bool;
    fn velocity_vectors_derive_from_dynamic_objects_enable(&self) -> &bool;
    fn velocity_vectors_clear_value(&self) -> &super::core::Vec3;
    fn multisample_count(&self) -> &u32;
    fn multisample_quality(&self) -> &u32;
    fn draw_transparent(&self) -> &bool;
    fn draw_half_res_transparent(&self) -> &bool;
    fn draw_transparent_decal(&self) -> &bool;
    fn transparent_dof_enable(&self) -> &bool;
    fn transparent_dof_half_res_enable(&self) -> &bool;
    fn transparent_dof_lerp_coc_enable(&self) -> &bool;
    fn only_shadowmap_slice(&self) -> &i32;
    fn view_mode(&self) -> &WorldViewMode;
    fn enable(&self) -> &bool;
    fn console_render_target_pool_sharing_enable(&self) -> &bool;
    fn fast_hdr_enable(&self) -> &bool;
    fn additional_hdr_target_in_e_s_r_a_m(&self) -> &u32;
    fn linear_depth_in_e_s_r_a_m(&self) -> &bool;
    fn half_res_depth_resolve_enable(&self) -> &bool;
    fn depth_buffer_collision_enable(&self) -> &bool;
    fn final_post_enable(&self) -> &bool;
    fn output_gamma_correction_enable(&self) -> &bool;
    fn screen_effect_enable(&self) -> &bool;
    fn draw_solid_bounding_boxes(&self) -> &bool;
    fn draw_line_bounding_boxes(&self) -> &bool;
    fn draw_bounding_spheres(&self) -> &bool;
    fn draw_frustums(&self) -> &bool;
    fn draw_local_i_b_l_frustums(&self) -> &bool;
    fn draw_debug_shadowmaps(&self) -> &bool;
    fn draw_debug_local_light_shadows(&self) -> &bool;
    fn draw_debug_sky_envmap(&self) -> &bool;
    fn draw_debug_velocity_buffer(&self) -> &bool;
    fn draw_debug_half_res_environment(&self) -> &bool;
    fn draw_debug_distortion(&self) -> &bool;
    fn draw_debug_visible_entity_types(&self) -> &bool;
    fn draw_debug_sky_textures(&self) -> &bool;
    fn draw_debug_dof(&self) -> &bool;
    fn draw_debug_half_res_hdr_targets(&self) -> &bool;
    fn draw_debug_hi_z_min_max_buffer_enable(&self) -> &bool;
    fn draw_debug_screen_space_raytrace_buckets_enable(&self) -> &bool;
    fn draw_debug_emitter_sun_transmittance_maps(&self) -> &bool;
    fn draw_debug_blur_pyramid(&self) -> &bool;
    fn draw_debug_occlusion_z_buffer(&self) -> &bool;
    fn draw_debug_local_i_b_l_occlusion_z_buffer(&self) -> &bool;
    fn draw_debug_buffers(&self) -> &u32;
    fn wireframe_enable(&self) -> &bool;
    fn z_pass_enable(&self) -> &bool;
    fn occluder_mesh_z_prepass_enable(&self) -> &bool;
    fn occluder_mesh_z_prepass_draw_enable(&self) -> &bool;
    fn occluder_mesh_z_prepass_debug_enable(&self) -> &bool;
    fn frame_synthesis_mode(&self) -> &FrameSynthesisMode;
    fn half_res_enable(&self) -> &bool;
    fn force_full_res_enable(&self) -> &bool;
    fn half_res_lens_flares_enable(&self) -> &bool;
    fn foreground_enable(&self) -> &bool;
    fn foreground_z_pass_enable(&self) -> &bool;
    fn foreground_transparent_enable(&self) -> &bool;
    fn bilateral_half_res_composite_enable(&self) -> &bool;
    fn half_res_depth_min_max_dither_enable(&self) -> &bool;
    fn half_res_depth_min_max_dither_threshold(&self) -> &f32;
    fn sky_lighting_enable(&self) -> &bool;
    fn sky_render_enable(&self) -> &bool;
    fn sky_depth_fog_enable(&self) -> &bool;
    fn sky_height_fog_enable(&self) -> &bool;
    fn sky_forward_scattering_enable(&self) -> &bool;
    fn procedural_sky_receive_height_fog(&self) -> &bool;
    fn physical_sky_enabled(&self) -> &bool;
    fn physical_sky_precision_height(&self) -> &u32;
    fn physical_sky_precision_view(&self) -> &u32;
    fn physical_sky_precision_sun(&self) -> &u32;
    fn physical_sky_scattering_orders(&self) -> &u32;
    fn physical_sky_aerial_perspective_texture_width(&self) -> &u32;
    fn physical_sky_aerial_perspective_texture_height(&self) -> &u32;
    fn physical_sky_aerial_perspective_texture_depth(&self) -> &u32;
    fn physical_sky_scattering_eval_frame_count(&self) -> &u32;
    fn physical_sky_aerial_perspective_max_distance(&self) -> &f32;
    fn physical_sky_force_precompute(&self) -> &bool;
    fn volumetric_clouds_enabled(&self) -> &bool;
    fn volumetric_clouds_quality(&self) -> &super::core::QualityLevel;
    fn volumetric_clouds_cast_shadow(&self) -> &bool;
    fn volumetric_clouds_cast_shadow_in_forward_render(&self) -> &bool;
    fn volumetric_clouds_affect_aerial_perspective(&self) -> &bool;
    fn volumetric_clouds_receive_aerial_perspective(&self) -> &bool;
    fn volumetric_clouds_occlude_lens_flare(&self) -> &bool;
    fn volumetric_clouds_render_target_resolution_divider(&self) -> &u32;
    fn volumetric_clouds_reflection_render_target_resolution_divider(&self) -> &u32;
    fn volumetric_clouds_shadow_iteration_count(&self) -> &u32;
    fn volumetric_clouds_shadowmap_resolution(&self) -> &u32;
    fn volumetric_clouds_shadowmap_blur_samples(&self) -> &u32;
    fn volumetric_clouds_sample_count(&self) -> &super::core::QualityScalableInt;
    fn volumetric_clouds_reflection_sample_count(&self) -> &u32;
    fn volumetric_clouds_i_b_l_sample_count(&self) -> &u32;
    fn volumetric_clouds_temporal_coefficient(&self) -> &f32;
    fn volumetric_clouds_env_color_temporal_coefficient(&self) -> &f32;
    fn transparent_fogging_enable(&self) -> &bool;
    fn distortion_enable(&self) -> &bool;
    fn distortion_half_res_enable(&self) -> &bool;
    fn distortion8_bit_enable(&self) -> &bool;
    fn distortion_tiling_enable(&self) -> &bool;
    fn static_envmap_enable(&self) -> &bool;
    fn custom_envmap_enable(&self) -> &bool;
    fn sky_envmap_enable(&self) -> &bool;
    fn sky_envmap_filter_width(&self) -> &f32;
    fn sky_envmap_resolution(&self) -> &u32;
    fn sky_envmap_mipmap_gen_enable(&self) -> &bool;
    fn draw_debug_sky_envmap_mip_level(&self) -> &i32;
    fn sky_envmap_filter_mode(&self) -> &MipmapFilterMode;
    fn sky_envmap_sides_per_frame_count(&self) -> &u32;
    fn sky_envmap_force_update_enable(&self) -> &bool;
    fn sky_envmap_use_fast_h_d_r(&self) -> &bool;
    fn sky_envmap_debug_color_enable(&self) -> &bool;
    fn sky_envmap_update_count_threshold(&self) -> &f32;
    fn sky_envmap_update_value_threshold(&self) -> &f32;
    fn sky_envmap_cloud_fog_enable(&self) -> &bool;
    fn sky_envmap_generate_no_backdrop_enable(&self) -> &bool;
    fn dynamic_envmap_enable(&self) -> &bool;
    fn draw_debug_dynamic_envmap_mip_level(&self) -> &i32;
    fn dynamic_envmap_mipmap_gen_enable(&self) -> &bool;
    fn draw_debug_dynamic_envmap(&self) -> &bool;
    fn dynamic_envmap_shadowmap_enable(&self) -> &bool;
    fn dynamic_envmap_shadowmap_resolution(&self) -> &u32;
    fn dynamic_envmap_shadowmap_far_plane_override(&self) -> &bool;
    fn dynamic_envmap_shadowmap_far_plane(&self) -> &i32;
    fn dynamic_envmap_shadowmap_shadow_extrusion_override(&self) -> &bool;
    fn dynamic_envmap_shadowmap_shadow_extrusion(&self) -> &i32;
    fn draw_debug_dynamic_envmap_shadowmap(&self) -> &bool;
    fn draw_dynamic_envmap_frustums(&self) -> &bool;
    fn setup_job_enable(&self) -> &bool;
    fn setup_jobs_create_view_job(&self) -> &bool;
    fn prepare_dispatch_list_job_enable(&self) -> &bool;
    fn indirect_specular_intensity(&self) -> &f32;
    fn indirect_specular_reflectance_scale(&self) -> &f32;
    fn indirect_specular_probes_intensity(&self) -> &f32;
    fn indirect_specular_probes_reflectance_scale(&self) -> &f32;
}

impl WorldRenderSettingsBaseTrait for WorldRenderSettingsBase {
    fn cull_screen_area_scale(&self) -> &f32 {
        &self.cull_screen_area_scale
    }
    fn deferred_shading_enable(&self) -> &bool {
        &self.deferred_shading_enable
    }
    fn forward_opaque_enable(&self) -> &bool {
        &self.forward_opaque_enable
    }
    fn full_z_pass_enable(&self) -> &bool {
        &self.full_z_pass_enable
    }
    fn tile_material_classification_enable(&self) -> &bool {
        &self.tile_material_classification_enable
    }
    fn shadowmaps_enable(&self) -> &bool {
        &self.shadowmaps_enable
    }
    fn shadowmap_array_enable(&self) -> &bool {
        &self.shadowmap_array_enable
    }
    fn transparency_shadowmaps_enable(&self) -> &bool {
        &self.transparency_shadowmaps_enable
    }
    fn n_v_i_d_i_a_shadows_p_c_s_s_enable(&self) -> &bool {
        &self.n_v_i_d_i_a_shadows_p_c_s_s_enable
    }
    fn n_v_i_d_i_a_shadows_h_f_t_s_enable(&self) -> &bool {
        &self.n_v_i_d_i_a_shadows_h_f_t_s_enable
    }
    fn transparency_shadowmaps_half_res(&self) -> &bool {
        &self.transparency_shadowmaps_half_res
    }
    fn sun_shadowmap_level(&self) -> &super::core::QualityLevel {
        &self.sun_shadowmap_level
    }
    fn shadowmap_min_fov(&self) -> &f32 {
        &self.shadowmap_min_fov
    }
    fn shadowmap_fixed_movement_enable(&self) -> &bool {
        &self.shadowmap_fixed_movement_enable
    }
    fn shadowmap_fixed_depth_enable(&self) -> &bool {
        &self.shadowmap_fixed_depth_enable
    }
    fn shadowmap_size_z_scale(&self) -> &f32 {
        &self.shadowmap_size_z_scale
    }
    fn shadowmap_resolution(&self) -> &u32 {
        &self.shadowmap_resolution
    }
    fn adjusted_shadowmap_resolution(&self) -> &u32 {
        &self.adjusted_shadowmap_resolution
    }
    fn shadowmap_quality(&self) -> &u32 {
        &self.shadowmap_quality
    }
    fn shadowmap_poisson_filter_scale(&self) -> &f32 {
        &self.shadowmap_poisson_filter_scale
    }
    fn shadowmap_slice_count(&self) -> &u32 {
        &self.shadowmap_slice_count
    }
    fn adjusted_shadowmap_slice_count(&self) -> &u32 {
        &self.adjusted_shadowmap_slice_count
    }
    fn shadowmap_slice_scheme_weight(&self) -> &f32 {
        &self.shadowmap_slice_scheme_weight
    }
    fn shadowmap_first_slice_scale(&self) -> &f32 {
        &self.shadowmap_first_slice_scale
    }
    fn shadowmap_view_distance(&self) -> &f32 {
        &self.shadowmap_view_distance
    }
    fn shadowmap_view_distance_scale_enable(&self) -> &bool {
        &self.shadowmap_view_distance_scale_enable
    }
    fn shadowmap_extrusion_length(&self) -> &f32 {
        &self.shadowmap_extrusion_length
    }
    fn shadowmap_first_slice_extrusion_length(&self) -> &f32 {
        &self.shadowmap_first_slice_extrusion_length
    }
    fn shadowmap_adjust_far_plane(&self) -> &bool {
        &self.shadowmap_adjust_far_plane
    }
    fn draw_debug_shadowmap_cascades(&self) -> &bool {
        &self.draw_debug_shadowmap_cascades
    }
    fn shadowmap_accum_enable(&self) -> &bool {
        &self.shadowmap_accum_enable
    }
    fn shadowmap_accum_reuse_enable(&self) -> &bool {
        &self.shadowmap_accum_reuse_enable
    }
    fn shadowmap_accum_stencil_enable(&self) -> &bool {
        &self.shadowmap_accum_stencil_enable
    }
    fn shadowmap_accum_stencil2_enable(&self) -> &bool {
        &self.shadowmap_accum_stencil2_enable
    }
    fn shadowmap_transition_blend_enable(&self) -> &bool {
        &self.shadowmap_transition_blend_enable
    }
    fn shadowmap_transition_blend_amount(&self) -> &f32 {
        &self.shadowmap_transition_blend_amount
    }
    fn shadowmap_foreground_enable(&self) -> &bool {
        &self.shadowmap_foreground_enable
    }
    fn shadowmap_foreground_use_first_person_view_transform(&self) -> &bool {
        &self.shadowmap_foreground_use_first_person_view_transform
    }
    fn shadowmap_foreground_extrusion_length(&self) -> &f32 {
        &self.shadowmap_foreground_extrusion_length
    }
    fn shadowmap_foreground_split_distance(&self) -> &f32 {
        &self.shadowmap_foreground_split_distance
    }
    fn shadowmap_foreground_size_z_scale(&self) -> &f32 {
        &self.shadowmap_foreground_size_z_scale
    }
    fn shadowmap_adjust_shadow_distance_with_fov(&self) -> &bool {
        &self.shadowmap_adjust_shadow_distance_with_fov
    }
    fn shadowmap_draw_batched_enable(&self) -> &bool {
        &self.shadowmap_draw_batched_enable
    }
    fn distant_shadow_cache_draw_frustum(&self) -> &bool {
        &self.distant_shadow_cache_draw_frustum
    }
    fn distant_shadow_cache_enable(&self) -> &bool {
        &self.distant_shadow_cache_enable
    }
    fn distant_shadow_cache_resolution(&self) -> &u32 {
        &self.distant_shadow_cache_resolution
    }
    fn distant_shadow_cache_force_resolution(&self) -> &i32 {
        &self.distant_shadow_cache_force_resolution
    }
    fn distant_shadow_cache_resolution_scale(&self) -> &f32 {
        &self.distant_shadow_cache_resolution_scale
    }
    fn distant_shadow_cache_max_resolution(&self) -> &u32 {
        &self.distant_shadow_cache_max_resolution
    }
    fn distant_shadow_cache_draw_debug(&self) -> &bool {
        &self.distant_shadow_cache_draw_debug
    }
    fn distant_shadow_cache_force_mode(&self) -> &u32 {
        &self.distant_shadow_cache_force_mode
    }
    fn distant_shadow_cache_use_quadtree(&self) -> &bool {
        &self.distant_shadow_cache_use_quadtree
    }
    fn distant_shadow_cache_batch_group_entity_bake(&self) -> &bool {
        &self.distant_shadow_cache_batch_group_entity_bake
    }
    fn distant_shadow_cache_rebake_on_light_change(&self) -> &bool {
        &self.distant_shadow_cache_rebake_on_light_change
    }
    fn distant_shadow_cache_rebake_on_add_remove(&self) -> &bool {
        &self.distant_shadow_cache_rebake_on_add_remove
    }
    fn distant_shadow_cache_rebake_on_move(&self) -> &bool {
        &self.distant_shadow_cache_rebake_on_move
    }
    fn distant_shadow_cache_rebake_on_part_visibility(&self) -> &bool {
        &self.distant_shadow_cache_rebake_on_part_visibility
    }
    fn distant_shadow_cache_coalesce_time(&self) -> &f32 {
        &self.distant_shadow_cache_coalesce_time
    }
    fn distant_shadow_cache_max_bake_events(&self) -> &u32 {
        &self.distant_shadow_cache_max_bake_events
    }
    fn sun_pcss_max_sample_count(&self) -> &i32 {
        &self.sun_pcss_max_sample_count
    }
    fn sun_pcss_adaptive_sample_increment(&self) -> &i32 {
        &self.sun_pcss_adaptive_sample_increment
    }
    fn dx_shadowmap16_bit_enable(&self) -> &bool {
        &self.dx_shadowmap16_bit_enable
    }
    fn dx_dynamic_envmap_shadowmap16_bit_enable(&self) -> &bool {
        &self.dx_dynamic_envmap_shadowmap16_bit_enable
    }
    fn apply_shadowmaps_enable(&self) -> &bool {
        &self.apply_shadowmaps_enable
    }
    fn simple_shadowmaps_enable(&self) -> &bool {
        &self.simple_shadowmaps_enable
    }
    fn emitter_shadowing_blend_toggle(&self) -> &bool {
        &self.emitter_shadowing_blend_toggle
    }
    fn emitter_shadowing_many_samples_toggle(&self) -> &bool {
        &self.emitter_shadowing_many_samples_toggle
    }
    fn dx_linear_depth32_bit_format_enable(&self) -> &bool {
        &self.dx_linear_depth32_bit_format_enable
    }
    fn motion_blur_enable(&self) -> &bool {
        &self.motion_blur_enable
    }
    fn motion_blur_force_on(&self) -> &bool {
        &self.motion_blur_force_on
    }
    fn motion_blur_scale(&self) -> &f32 {
        &self.motion_blur_scale
    }
    fn motion_blur_fixed_shutter_time(&self) -> &f32 {
        &self.motion_blur_fixed_shutter_time
    }
    fn motion_blur_max(&self) -> &f32 {
        &self.motion_blur_max
    }
    fn motion_blur_radial_blur_max(&self) -> &f32 {
        &self.motion_blur_radial_blur_max
    }
    fn motion_blur_noise_scale(&self) -> &f32 {
        &self.motion_blur_noise_scale
    }
    fn motion_blur_quality(&self) -> &u32 {
        &self.motion_blur_quality
    }
    fn motion_blur_debug_mode(&self) -> &u32 {
        &self.motion_blur_debug_mode
    }
    fn motion_blur_perceptual_space_enable(&self) -> &bool {
        &self.motion_blur_perceptual_space_enable
    }
    fn motion_blur_stencil_pass_enable(&self) -> &bool {
        &self.motion_blur_stencil_pass_enable
    }
    fn motion_blur_centered_enable(&self) -> &bool {
        &self.motion_blur_centered_enable
    }
    fn motion_blur_max_sample_count(&self) -> &u32 {
        &self.motion_blur_max_sample_count
    }
    fn motion_blur_depth_check_threshold(&self) -> &f32 {
        &self.motion_blur_depth_check_threshold
    }
    fn motion_blur_depth_check_max_distance(&self) -> &f32 {
        &self.motion_blur_depth_check_max_distance
    }
    fn tiled_motion_blur_variance_threshold_scale(&self) -> &f32 {
        &self.tiled_motion_blur_variance_threshold_scale
    }
    fn tiled_motion_blur_vel_mag_depth_downsample(&self) -> &u32 {
        &self.tiled_motion_blur_vel_mag_depth_downsample
    }
    fn tiled_motion_blur_separable_enable(&self) -> &bool {
        &self.tiled_motion_blur_separable_enable
    }
    fn tiled_motion_blur_enable(&self) -> &bool {
        &self.tiled_motion_blur_enable
    }
    fn tiled_motion_blur_force20_px_tile(&self) -> &bool {
        &self.tiled_motion_blur_force20_px_tile
    }
    fn motion_blur_use_detailed_gpu_timers(&self) -> &bool {
        &self.motion_blur_use_detailed_gpu_timers
    }
    fn velocity_vectors_derive_from_depth_enable(&self) -> &bool {
        &self.velocity_vectors_derive_from_depth_enable
    }
    fn velocity_vectors_derive_from_dynamic_objects_enable(&self) -> &bool {
        &self.velocity_vectors_derive_from_dynamic_objects_enable
    }
    fn velocity_vectors_clear_value(&self) -> &super::core::Vec3 {
        &self.velocity_vectors_clear_value
    }
    fn multisample_count(&self) -> &u32 {
        &self.multisample_count
    }
    fn multisample_quality(&self) -> &u32 {
        &self.multisample_quality
    }
    fn draw_transparent(&self) -> &bool {
        &self.draw_transparent
    }
    fn draw_half_res_transparent(&self) -> &bool {
        &self.draw_half_res_transparent
    }
    fn draw_transparent_decal(&self) -> &bool {
        &self.draw_transparent_decal
    }
    fn transparent_dof_enable(&self) -> &bool {
        &self.transparent_dof_enable
    }
    fn transparent_dof_half_res_enable(&self) -> &bool {
        &self.transparent_dof_half_res_enable
    }
    fn transparent_dof_lerp_coc_enable(&self) -> &bool {
        &self.transparent_dof_lerp_coc_enable
    }
    fn only_shadowmap_slice(&self) -> &i32 {
        &self.only_shadowmap_slice
    }
    fn view_mode(&self) -> &WorldViewMode {
        &self.view_mode
    }
    fn enable(&self) -> &bool {
        &self.enable
    }
    fn console_render_target_pool_sharing_enable(&self) -> &bool {
        &self.console_render_target_pool_sharing_enable
    }
    fn fast_hdr_enable(&self) -> &bool {
        &self.fast_hdr_enable
    }
    fn additional_hdr_target_in_e_s_r_a_m(&self) -> &u32 {
        &self.additional_hdr_target_in_e_s_r_a_m
    }
    fn linear_depth_in_e_s_r_a_m(&self) -> &bool {
        &self.linear_depth_in_e_s_r_a_m
    }
    fn half_res_depth_resolve_enable(&self) -> &bool {
        &self.half_res_depth_resolve_enable
    }
    fn depth_buffer_collision_enable(&self) -> &bool {
        &self.depth_buffer_collision_enable
    }
    fn final_post_enable(&self) -> &bool {
        &self.final_post_enable
    }
    fn output_gamma_correction_enable(&self) -> &bool {
        &self.output_gamma_correction_enable
    }
    fn screen_effect_enable(&self) -> &bool {
        &self.screen_effect_enable
    }
    fn draw_solid_bounding_boxes(&self) -> &bool {
        &self.draw_solid_bounding_boxes
    }
    fn draw_line_bounding_boxes(&self) -> &bool {
        &self.draw_line_bounding_boxes
    }
    fn draw_bounding_spheres(&self) -> &bool {
        &self.draw_bounding_spheres
    }
    fn draw_frustums(&self) -> &bool {
        &self.draw_frustums
    }
    fn draw_local_i_b_l_frustums(&self) -> &bool {
        &self.draw_local_i_b_l_frustums
    }
    fn draw_debug_shadowmaps(&self) -> &bool {
        &self.draw_debug_shadowmaps
    }
    fn draw_debug_local_light_shadows(&self) -> &bool {
        &self.draw_debug_local_light_shadows
    }
    fn draw_debug_sky_envmap(&self) -> &bool {
        &self.draw_debug_sky_envmap
    }
    fn draw_debug_velocity_buffer(&self) -> &bool {
        &self.draw_debug_velocity_buffer
    }
    fn draw_debug_half_res_environment(&self) -> &bool {
        &self.draw_debug_half_res_environment
    }
    fn draw_debug_distortion(&self) -> &bool {
        &self.draw_debug_distortion
    }
    fn draw_debug_visible_entity_types(&self) -> &bool {
        &self.draw_debug_visible_entity_types
    }
    fn draw_debug_sky_textures(&self) -> &bool {
        &self.draw_debug_sky_textures
    }
    fn draw_debug_dof(&self) -> &bool {
        &self.draw_debug_dof
    }
    fn draw_debug_half_res_hdr_targets(&self) -> &bool {
        &self.draw_debug_half_res_hdr_targets
    }
    fn draw_debug_hi_z_min_max_buffer_enable(&self) -> &bool {
        &self.draw_debug_hi_z_min_max_buffer_enable
    }
    fn draw_debug_screen_space_raytrace_buckets_enable(&self) -> &bool {
        &self.draw_debug_screen_space_raytrace_buckets_enable
    }
    fn draw_debug_emitter_sun_transmittance_maps(&self) -> &bool {
        &self.draw_debug_emitter_sun_transmittance_maps
    }
    fn draw_debug_blur_pyramid(&self) -> &bool {
        &self.draw_debug_blur_pyramid
    }
    fn draw_debug_occlusion_z_buffer(&self) -> &bool {
        &self.draw_debug_occlusion_z_buffer
    }
    fn draw_debug_local_i_b_l_occlusion_z_buffer(&self) -> &bool {
        &self.draw_debug_local_i_b_l_occlusion_z_buffer
    }
    fn draw_debug_buffers(&self) -> &u32 {
        &self.draw_debug_buffers
    }
    fn wireframe_enable(&self) -> &bool {
        &self.wireframe_enable
    }
    fn z_pass_enable(&self) -> &bool {
        &self.z_pass_enable
    }
    fn occluder_mesh_z_prepass_enable(&self) -> &bool {
        &self.occluder_mesh_z_prepass_enable
    }
    fn occluder_mesh_z_prepass_draw_enable(&self) -> &bool {
        &self.occluder_mesh_z_prepass_draw_enable
    }
    fn occluder_mesh_z_prepass_debug_enable(&self) -> &bool {
        &self.occluder_mesh_z_prepass_debug_enable
    }
    fn frame_synthesis_mode(&self) -> &FrameSynthesisMode {
        &self.frame_synthesis_mode
    }
    fn half_res_enable(&self) -> &bool {
        &self.half_res_enable
    }
    fn force_full_res_enable(&self) -> &bool {
        &self.force_full_res_enable
    }
    fn half_res_lens_flares_enable(&self) -> &bool {
        &self.half_res_lens_flares_enable
    }
    fn foreground_enable(&self) -> &bool {
        &self.foreground_enable
    }
    fn foreground_z_pass_enable(&self) -> &bool {
        &self.foreground_z_pass_enable
    }
    fn foreground_transparent_enable(&self) -> &bool {
        &self.foreground_transparent_enable
    }
    fn bilateral_half_res_composite_enable(&self) -> &bool {
        &self.bilateral_half_res_composite_enable
    }
    fn half_res_depth_min_max_dither_enable(&self) -> &bool {
        &self.half_res_depth_min_max_dither_enable
    }
    fn half_res_depth_min_max_dither_threshold(&self) -> &f32 {
        &self.half_res_depth_min_max_dither_threshold
    }
    fn sky_lighting_enable(&self) -> &bool {
        &self.sky_lighting_enable
    }
    fn sky_render_enable(&self) -> &bool {
        &self.sky_render_enable
    }
    fn sky_depth_fog_enable(&self) -> &bool {
        &self.sky_depth_fog_enable
    }
    fn sky_height_fog_enable(&self) -> &bool {
        &self.sky_height_fog_enable
    }
    fn sky_forward_scattering_enable(&self) -> &bool {
        &self.sky_forward_scattering_enable
    }
    fn procedural_sky_receive_height_fog(&self) -> &bool {
        &self.procedural_sky_receive_height_fog
    }
    fn physical_sky_enabled(&self) -> &bool {
        &self.physical_sky_enabled
    }
    fn physical_sky_precision_height(&self) -> &u32 {
        &self.physical_sky_precision_height
    }
    fn physical_sky_precision_view(&self) -> &u32 {
        &self.physical_sky_precision_view
    }
    fn physical_sky_precision_sun(&self) -> &u32 {
        &self.physical_sky_precision_sun
    }
    fn physical_sky_scattering_orders(&self) -> &u32 {
        &self.physical_sky_scattering_orders
    }
    fn physical_sky_aerial_perspective_texture_width(&self) -> &u32 {
        &self.physical_sky_aerial_perspective_texture_width
    }
    fn physical_sky_aerial_perspective_texture_height(&self) -> &u32 {
        &self.physical_sky_aerial_perspective_texture_height
    }
    fn physical_sky_aerial_perspective_texture_depth(&self) -> &u32 {
        &self.physical_sky_aerial_perspective_texture_depth
    }
    fn physical_sky_scattering_eval_frame_count(&self) -> &u32 {
        &self.physical_sky_scattering_eval_frame_count
    }
    fn physical_sky_aerial_perspective_max_distance(&self) -> &f32 {
        &self.physical_sky_aerial_perspective_max_distance
    }
    fn physical_sky_force_precompute(&self) -> &bool {
        &self.physical_sky_force_precompute
    }
    fn volumetric_clouds_enabled(&self) -> &bool {
        &self.volumetric_clouds_enabled
    }
    fn volumetric_clouds_quality(&self) -> &super::core::QualityLevel {
        &self.volumetric_clouds_quality
    }
    fn volumetric_clouds_cast_shadow(&self) -> &bool {
        &self.volumetric_clouds_cast_shadow
    }
    fn volumetric_clouds_cast_shadow_in_forward_render(&self) -> &bool {
        &self.volumetric_clouds_cast_shadow_in_forward_render
    }
    fn volumetric_clouds_affect_aerial_perspective(&self) -> &bool {
        &self.volumetric_clouds_affect_aerial_perspective
    }
    fn volumetric_clouds_receive_aerial_perspective(&self) -> &bool {
        &self.volumetric_clouds_receive_aerial_perspective
    }
    fn volumetric_clouds_occlude_lens_flare(&self) -> &bool {
        &self.volumetric_clouds_occlude_lens_flare
    }
    fn volumetric_clouds_render_target_resolution_divider(&self) -> &u32 {
        &self.volumetric_clouds_render_target_resolution_divider
    }
    fn volumetric_clouds_reflection_render_target_resolution_divider(&self) -> &u32 {
        &self.volumetric_clouds_reflection_render_target_resolution_divider
    }
    fn volumetric_clouds_shadow_iteration_count(&self) -> &u32 {
        &self.volumetric_clouds_shadow_iteration_count
    }
    fn volumetric_clouds_shadowmap_resolution(&self) -> &u32 {
        &self.volumetric_clouds_shadowmap_resolution
    }
    fn volumetric_clouds_shadowmap_blur_samples(&self) -> &u32 {
        &self.volumetric_clouds_shadowmap_blur_samples
    }
    fn volumetric_clouds_sample_count(&self) -> &super::core::QualityScalableInt {
        &self.volumetric_clouds_sample_count
    }
    fn volumetric_clouds_reflection_sample_count(&self) -> &u32 {
        &self.volumetric_clouds_reflection_sample_count
    }
    fn volumetric_clouds_i_b_l_sample_count(&self) -> &u32 {
        &self.volumetric_clouds_i_b_l_sample_count
    }
    fn volumetric_clouds_temporal_coefficient(&self) -> &f32 {
        &self.volumetric_clouds_temporal_coefficient
    }
    fn volumetric_clouds_env_color_temporal_coefficient(&self) -> &f32 {
        &self.volumetric_clouds_env_color_temporal_coefficient
    }
    fn transparent_fogging_enable(&self) -> &bool {
        &self.transparent_fogging_enable
    }
    fn distortion_enable(&self) -> &bool {
        &self.distortion_enable
    }
    fn distortion_half_res_enable(&self) -> &bool {
        &self.distortion_half_res_enable
    }
    fn distortion8_bit_enable(&self) -> &bool {
        &self.distortion8_bit_enable
    }
    fn distortion_tiling_enable(&self) -> &bool {
        &self.distortion_tiling_enable
    }
    fn static_envmap_enable(&self) -> &bool {
        &self.static_envmap_enable
    }
    fn custom_envmap_enable(&self) -> &bool {
        &self.custom_envmap_enable
    }
    fn sky_envmap_enable(&self) -> &bool {
        &self.sky_envmap_enable
    }
    fn sky_envmap_filter_width(&self) -> &f32 {
        &self.sky_envmap_filter_width
    }
    fn sky_envmap_resolution(&self) -> &u32 {
        &self.sky_envmap_resolution
    }
    fn sky_envmap_mipmap_gen_enable(&self) -> &bool {
        &self.sky_envmap_mipmap_gen_enable
    }
    fn draw_debug_sky_envmap_mip_level(&self) -> &i32 {
        &self.draw_debug_sky_envmap_mip_level
    }
    fn sky_envmap_filter_mode(&self) -> &MipmapFilterMode {
        &self.sky_envmap_filter_mode
    }
    fn sky_envmap_sides_per_frame_count(&self) -> &u32 {
        &self.sky_envmap_sides_per_frame_count
    }
    fn sky_envmap_force_update_enable(&self) -> &bool {
        &self.sky_envmap_force_update_enable
    }
    fn sky_envmap_use_fast_h_d_r(&self) -> &bool {
        &self.sky_envmap_use_fast_h_d_r
    }
    fn sky_envmap_debug_color_enable(&self) -> &bool {
        &self.sky_envmap_debug_color_enable
    }
    fn sky_envmap_update_count_threshold(&self) -> &f32 {
        &self.sky_envmap_update_count_threshold
    }
    fn sky_envmap_update_value_threshold(&self) -> &f32 {
        &self.sky_envmap_update_value_threshold
    }
    fn sky_envmap_cloud_fog_enable(&self) -> &bool {
        &self.sky_envmap_cloud_fog_enable
    }
    fn sky_envmap_generate_no_backdrop_enable(&self) -> &bool {
        &self.sky_envmap_generate_no_backdrop_enable
    }
    fn dynamic_envmap_enable(&self) -> &bool {
        &self.dynamic_envmap_enable
    }
    fn draw_debug_dynamic_envmap_mip_level(&self) -> &i32 {
        &self.draw_debug_dynamic_envmap_mip_level
    }
    fn dynamic_envmap_mipmap_gen_enable(&self) -> &bool {
        &self.dynamic_envmap_mipmap_gen_enable
    }
    fn draw_debug_dynamic_envmap(&self) -> &bool {
        &self.draw_debug_dynamic_envmap
    }
    fn dynamic_envmap_shadowmap_enable(&self) -> &bool {
        &self.dynamic_envmap_shadowmap_enable
    }
    fn dynamic_envmap_shadowmap_resolution(&self) -> &u32 {
        &self.dynamic_envmap_shadowmap_resolution
    }
    fn dynamic_envmap_shadowmap_far_plane_override(&self) -> &bool {
        &self.dynamic_envmap_shadowmap_far_plane_override
    }
    fn dynamic_envmap_shadowmap_far_plane(&self) -> &i32 {
        &self.dynamic_envmap_shadowmap_far_plane
    }
    fn dynamic_envmap_shadowmap_shadow_extrusion_override(&self) -> &bool {
        &self.dynamic_envmap_shadowmap_shadow_extrusion_override
    }
    fn dynamic_envmap_shadowmap_shadow_extrusion(&self) -> &i32 {
        &self.dynamic_envmap_shadowmap_shadow_extrusion
    }
    fn draw_debug_dynamic_envmap_shadowmap(&self) -> &bool {
        &self.draw_debug_dynamic_envmap_shadowmap
    }
    fn draw_dynamic_envmap_frustums(&self) -> &bool {
        &self.draw_dynamic_envmap_frustums
    }
    fn setup_job_enable(&self) -> &bool {
        &self.setup_job_enable
    }
    fn setup_jobs_create_view_job(&self) -> &bool {
        &self.setup_jobs_create_view_job
    }
    fn prepare_dispatch_list_job_enable(&self) -> &bool {
        &self.prepare_dispatch_list_job_enable
    }
    fn indirect_specular_intensity(&self) -> &f32 {
        &self.indirect_specular_intensity
    }
    fn indirect_specular_reflectance_scale(&self) -> &f32 {
        &self.indirect_specular_reflectance_scale
    }
    fn indirect_specular_probes_intensity(&self) -> &f32 {
        &self.indirect_specular_probes_intensity
    }
    fn indirect_specular_probes_reflectance_scale(&self) -> &f32 {
        &self.indirect_specular_probes_reflectance_scale
    }
}

impl super::core::DataContainerTrait for WorldRenderSettingsBase {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WORLDRENDERSETTINGSBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldRenderSettingsBase",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WorldRenderSettingsBase as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CullScreenAreaScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, cull_screen_area_scale),
            },
            FieldInfoData {
                name: "DeferredShadingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, deferred_shading_enable),
            },
            FieldInfoData {
                name: "ForwardOpaqueEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, forward_opaque_enable),
            },
            FieldInfoData {
                name: "FullZPassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, full_z_pass_enable),
            },
            FieldInfoData {
                name: "TileMaterialClassificationEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, tile_material_classification_enable),
            },
            FieldInfoData {
                name: "ShadowmapsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmaps_enable),
            },
            FieldInfoData {
                name: "ShadowmapArrayEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_array_enable),
            },
            FieldInfoData {
                name: "TransparencyShadowmapsEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, transparency_shadowmaps_enable),
            },
            FieldInfoData {
                name: "NVIDIAShadowsPCSSEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, n_v_i_d_i_a_shadows_p_c_s_s_enable),
            },
            FieldInfoData {
                name: "NVIDIAShadowsHFTSEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, n_v_i_d_i_a_shadows_h_f_t_s_enable),
            },
            FieldInfoData {
                name: "TransparencyShadowmapsHalfRes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, transparency_shadowmaps_half_res),
            },
            FieldInfoData {
                name: "SunShadowmapLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WorldRenderSettingsBase, sun_shadowmap_level),
            },
            FieldInfoData {
                name: "ShadowmapMinFov",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_min_fov),
            },
            FieldInfoData {
                name: "ShadowmapFixedMovementEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_fixed_movement_enable),
            },
            FieldInfoData {
                name: "ShadowmapFixedDepthEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_fixed_depth_enable),
            },
            FieldInfoData {
                name: "ShadowmapSizeZScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_size_z_scale),
            },
            FieldInfoData {
                name: "ShadowmapResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_resolution),
            },
            FieldInfoData {
                name: "AdjustedShadowmapResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, adjusted_shadowmap_resolution),
            },
            FieldInfoData {
                name: "ShadowmapQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_quality),
            },
            FieldInfoData {
                name: "ShadowmapPoissonFilterScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_poisson_filter_scale),
            },
            FieldInfoData {
                name: "ShadowmapSliceCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_slice_count),
            },
            FieldInfoData {
                name: "AdjustedShadowmapSliceCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, adjusted_shadowmap_slice_count),
            },
            FieldInfoData {
                name: "ShadowmapSliceSchemeWeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_slice_scheme_weight),
            },
            FieldInfoData {
                name: "ShadowmapFirstSliceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_first_slice_scale),
            },
            FieldInfoData {
                name: "ShadowmapViewDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_view_distance),
            },
            FieldInfoData {
                name: "ShadowmapViewDistanceScaleEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_view_distance_scale_enable),
            },
            FieldInfoData {
                name: "ShadowmapExtrusionLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_extrusion_length),
            },
            FieldInfoData {
                name: "ShadowmapFirstSliceExtrusionLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_first_slice_extrusion_length),
            },
            FieldInfoData {
                name: "ShadowmapAdjustFarPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_adjust_far_plane),
            },
            FieldInfoData {
                name: "DrawDebugShadowmapCascades",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_shadowmap_cascades),
            },
            FieldInfoData {
                name: "ShadowmapAccumEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_accum_enable),
            },
            FieldInfoData {
                name: "ShadowmapAccumReuseEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_accum_reuse_enable),
            },
            FieldInfoData {
                name: "ShadowmapAccumStencilEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_accum_stencil_enable),
            },
            FieldInfoData {
                name: "ShadowmapAccumStencil2Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_accum_stencil2_enable),
            },
            FieldInfoData {
                name: "ShadowmapTransitionBlendEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_transition_blend_enable),
            },
            FieldInfoData {
                name: "ShadowmapTransitionBlendAmount",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_transition_blend_amount),
            },
            FieldInfoData {
                name: "ShadowmapForegroundEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_foreground_enable),
            },
            FieldInfoData {
                name: "ShadowmapForegroundUseFirstPersonViewTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_foreground_use_first_person_view_transform),
            },
            FieldInfoData {
                name: "ShadowmapForegroundExtrusionLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_foreground_extrusion_length),
            },
            FieldInfoData {
                name: "ShadowmapForegroundSplitDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_foreground_split_distance),
            },
            FieldInfoData {
                name: "ShadowmapForegroundSizeZScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_foreground_size_z_scale),
            },
            FieldInfoData {
                name: "ShadowmapAdjustShadowDistanceWithFov",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_adjust_shadow_distance_with_fov),
            },
            FieldInfoData {
                name: "ShadowmapDrawBatchedEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, shadowmap_draw_batched_enable),
            },
            FieldInfoData {
                name: "DistantShadowCacheDrawFrustum",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_draw_frustum),
            },
            FieldInfoData {
                name: "DistantShadowCacheEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_enable),
            },
            FieldInfoData {
                name: "DistantShadowCacheResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_resolution),
            },
            FieldInfoData {
                name: "DistantShadowCacheForceResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_force_resolution),
            },
            FieldInfoData {
                name: "DistantShadowCacheResolutionScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_resolution_scale),
            },
            FieldInfoData {
                name: "DistantShadowCacheMaxResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_max_resolution),
            },
            FieldInfoData {
                name: "DistantShadowCacheDrawDebug",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_draw_debug),
            },
            FieldInfoData {
                name: "DistantShadowCacheForceMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_force_mode),
            },
            FieldInfoData {
                name: "DistantShadowCacheUseQuadtree",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_use_quadtree),
            },
            FieldInfoData {
                name: "DistantShadowCacheBatchGroupEntityBake",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_batch_group_entity_bake),
            },
            FieldInfoData {
                name: "DistantShadowCacheRebakeOnLightChange",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_rebake_on_light_change),
            },
            FieldInfoData {
                name: "DistantShadowCacheRebakeOnAddRemove",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_rebake_on_add_remove),
            },
            FieldInfoData {
                name: "DistantShadowCacheRebakeOnMove",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_rebake_on_move),
            },
            FieldInfoData {
                name: "DistantShadowCacheRebakeOnPartVisibility",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_rebake_on_part_visibility),
            },
            FieldInfoData {
                name: "DistantShadowCacheCoalesceTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_coalesce_time),
            },
            FieldInfoData {
                name: "DistantShadowCacheMaxBakeEvents",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, distant_shadow_cache_max_bake_events),
            },
            FieldInfoData {
                name: "SunPcssMaxSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldRenderSettingsBase, sun_pcss_max_sample_count),
            },
            FieldInfoData {
                name: "SunPcssAdaptiveSampleIncrement",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldRenderSettingsBase, sun_pcss_adaptive_sample_increment),
            },
            FieldInfoData {
                name: "DxShadowmap16BitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, dx_shadowmap16_bit_enable),
            },
            FieldInfoData {
                name: "DxDynamicEnvmapShadowmap16BitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, dx_dynamic_envmap_shadowmap16_bit_enable),
            },
            FieldInfoData {
                name: "ApplyShadowmapsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, apply_shadowmaps_enable),
            },
            FieldInfoData {
                name: "SimpleShadowmapsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, simple_shadowmaps_enable),
            },
            FieldInfoData {
                name: "EmitterShadowingBlendToggle",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, emitter_shadowing_blend_toggle),
            },
            FieldInfoData {
                name: "EmitterShadowingManySamplesToggle",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, emitter_shadowing_many_samples_toggle),
            },
            FieldInfoData {
                name: "DxLinearDepth32BitFormatEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, dx_linear_depth32_bit_format_enable),
            },
            FieldInfoData {
                name: "MotionBlurEnable",
                flags: MemberInfoFlags::new(8192),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_enable),
            },
            FieldInfoData {
                name: "MotionBlurForceOn",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_force_on),
            },
            FieldInfoData {
                name: "MotionBlurScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_scale),
            },
            FieldInfoData {
                name: "MotionBlurFixedShutterTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_fixed_shutter_time),
            },
            FieldInfoData {
                name: "MotionBlurMax",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_max),
            },
            FieldInfoData {
                name: "MotionBlurRadialBlurMax",
                flags: MemberInfoFlags::new(8192),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_radial_blur_max),
            },
            FieldInfoData {
                name: "MotionBlurNoiseScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_noise_scale),
            },
            FieldInfoData {
                name: "MotionBlurQuality",
                flags: MemberInfoFlags::new(8192),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_quality),
            },
            FieldInfoData {
                name: "MotionBlurDebugMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_debug_mode),
            },
            FieldInfoData {
                name: "MotionBlurPerceptualSpaceEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_perceptual_space_enable),
            },
            FieldInfoData {
                name: "MotionBlurStencilPassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_stencil_pass_enable),
            },
            FieldInfoData {
                name: "MotionBlurCenteredEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_centered_enable),
            },
            FieldInfoData {
                name: "MotionBlurMaxSampleCount",
                flags: MemberInfoFlags::new(8192),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_max_sample_count),
            },
            FieldInfoData {
                name: "MotionBlurDepthCheckThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_depth_check_threshold),
            },
            FieldInfoData {
                name: "MotionBlurDepthCheckMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_depth_check_max_distance),
            },
            FieldInfoData {
                name: "TiledMotionBlurVarianceThresholdScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, tiled_motion_blur_variance_threshold_scale),
            },
            FieldInfoData {
                name: "TiledMotionBlurVelMagDepthDownsample",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, tiled_motion_blur_vel_mag_depth_downsample),
            },
            FieldInfoData {
                name: "TiledMotionBlurSeparableEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, tiled_motion_blur_separable_enable),
            },
            FieldInfoData {
                name: "TiledMotionBlurEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, tiled_motion_blur_enable),
            },
            FieldInfoData {
                name: "TiledMotionBlurForce20PxTile",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, tiled_motion_blur_force20_px_tile),
            },
            FieldInfoData {
                name: "MotionBlurUseDetailedGpuTimers",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, motion_blur_use_detailed_gpu_timers),
            },
            FieldInfoData {
                name: "VelocityVectorsDeriveFromDepthEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, velocity_vectors_derive_from_depth_enable),
            },
            FieldInfoData {
                name: "VelocityVectorsDeriveFromDynamicObjectsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, velocity_vectors_derive_from_dynamic_objects_enable),
            },
            FieldInfoData {
                name: "VelocityVectorsClearValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(WorldRenderSettingsBase, velocity_vectors_clear_value),
            },
            FieldInfoData {
                name: "MultisampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, multisample_count),
            },
            FieldInfoData {
                name: "MultisampleQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, multisample_quality),
            },
            FieldInfoData {
                name: "DrawTransparent",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_transparent),
            },
            FieldInfoData {
                name: "DrawHalfResTransparent",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_half_res_transparent),
            },
            FieldInfoData {
                name: "DrawTransparentDecal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_transparent_decal),
            },
            FieldInfoData {
                name: "TransparentDofEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, transparent_dof_enable),
            },
            FieldInfoData {
                name: "TransparentDofHalfResEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, transparent_dof_half_res_enable),
            },
            FieldInfoData {
                name: "TransparentDofLerpCocEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, transparent_dof_lerp_coc_enable),
            },
            FieldInfoData {
                name: "OnlyShadowmapSlice",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldRenderSettingsBase, only_shadowmap_slice),
            },
            FieldInfoData {
                name: "ViewMode",
                flags: MemberInfoFlags::new(0),
                field_type: "WorldViewMode",
                rust_offset: offset_of!(WorldRenderSettingsBase, view_mode),
            },
            FieldInfoData {
                name: "Enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, enable),
            },
            FieldInfoData {
                name: "ConsoleRenderTargetPoolSharingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, console_render_target_pool_sharing_enable),
            },
            FieldInfoData {
                name: "FastHdrEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, fast_hdr_enable),
            },
            FieldInfoData {
                name: "AdditionalHdrTargetInESRAM",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, additional_hdr_target_in_e_s_r_a_m),
            },
            FieldInfoData {
                name: "LinearDepthInESRAM",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, linear_depth_in_e_s_r_a_m),
            },
            FieldInfoData {
                name: "HalfResDepthResolveEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, half_res_depth_resolve_enable),
            },
            FieldInfoData {
                name: "DepthBufferCollisionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, depth_buffer_collision_enable),
            },
            FieldInfoData {
                name: "FinalPostEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, final_post_enable),
            },
            FieldInfoData {
                name: "OutputGammaCorrectionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, output_gamma_correction_enable),
            },
            FieldInfoData {
                name: "ScreenEffectEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, screen_effect_enable),
            },
            FieldInfoData {
                name: "DrawSolidBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_solid_bounding_boxes),
            },
            FieldInfoData {
                name: "DrawLineBoundingBoxes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_line_bounding_boxes),
            },
            FieldInfoData {
                name: "DrawBoundingSpheres",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_bounding_spheres),
            },
            FieldInfoData {
                name: "DrawFrustums",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_frustums),
            },
            FieldInfoData {
                name: "DrawLocalIBLFrustums",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_local_i_b_l_frustums),
            },
            FieldInfoData {
                name: "DrawDebugShadowmaps",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_shadowmaps),
            },
            FieldInfoData {
                name: "DrawDebugLocalLightShadows",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_local_light_shadows),
            },
            FieldInfoData {
                name: "DrawDebugSkyEnvmap",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_sky_envmap),
            },
            FieldInfoData {
                name: "DrawDebugVelocityBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_velocity_buffer),
            },
            FieldInfoData {
                name: "DrawDebugHalfResEnvironment",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_half_res_environment),
            },
            FieldInfoData {
                name: "DrawDebugDistortion",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_distortion),
            },
            FieldInfoData {
                name: "DrawDebugVisibleEntityTypes",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_visible_entity_types),
            },
            FieldInfoData {
                name: "DrawDebugSkyTextures",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_sky_textures),
            },
            FieldInfoData {
                name: "DrawDebugDof",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_dof),
            },
            FieldInfoData {
                name: "DrawDebugHalfResHdrTargets",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_half_res_hdr_targets),
            },
            FieldInfoData {
                name: "DrawDebugHiZMinMaxBufferEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_hi_z_min_max_buffer_enable),
            },
            FieldInfoData {
                name: "DrawDebugScreenSpaceRaytraceBucketsEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_screen_space_raytrace_buckets_enable),
            },
            FieldInfoData {
                name: "DrawDebugEmitterSunTransmittanceMaps",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_emitter_sun_transmittance_maps),
            },
            FieldInfoData {
                name: "DrawDebugBlurPyramid",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_blur_pyramid),
            },
            FieldInfoData {
                name: "DrawDebugOcclusionZBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_occlusion_z_buffer),
            },
            FieldInfoData {
                name: "DrawDebugLocalIBLOcclusionZBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_local_i_b_l_occlusion_z_buffer),
            },
            FieldInfoData {
                name: "DrawDebugBuffers",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_buffers),
            },
            FieldInfoData {
                name: "WireframeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, wireframe_enable),
            },
            FieldInfoData {
                name: "ZPassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, z_pass_enable),
            },
            FieldInfoData {
                name: "OccluderMeshZPrepassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, occluder_mesh_z_prepass_enable),
            },
            FieldInfoData {
                name: "OccluderMeshZPrepassDrawEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, occluder_mesh_z_prepass_draw_enable),
            },
            FieldInfoData {
                name: "OccluderMeshZPrepassDebugEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, occluder_mesh_z_prepass_debug_enable),
            },
            FieldInfoData {
                name: "FrameSynthesisMode",
                flags: MemberInfoFlags::new(8192),
                field_type: "FrameSynthesisMode",
                rust_offset: offset_of!(WorldRenderSettingsBase, frame_synthesis_mode),
            },
            FieldInfoData {
                name: "HalfResEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, half_res_enable),
            },
            FieldInfoData {
                name: "ForceFullResEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, force_full_res_enable),
            },
            FieldInfoData {
                name: "HalfResLensFlaresEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, half_res_lens_flares_enable),
            },
            FieldInfoData {
                name: "ForegroundEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, foreground_enable),
            },
            FieldInfoData {
                name: "ForegroundZPassEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, foreground_z_pass_enable),
            },
            FieldInfoData {
                name: "ForegroundTransparentEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, foreground_transparent_enable),
            },
            FieldInfoData {
                name: "BilateralHalfResCompositeEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, bilateral_half_res_composite_enable),
            },
            FieldInfoData {
                name: "HalfResDepthMinMaxDitherEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, half_res_depth_min_max_dither_enable),
            },
            FieldInfoData {
                name: "HalfResDepthMinMaxDitherThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, half_res_depth_min_max_dither_threshold),
            },
            FieldInfoData {
                name: "SkyLightingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_lighting_enable),
            },
            FieldInfoData {
                name: "SkyRenderEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_render_enable),
            },
            FieldInfoData {
                name: "SkyDepthFogEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_depth_fog_enable),
            },
            FieldInfoData {
                name: "SkyHeightFogEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_height_fog_enable),
            },
            FieldInfoData {
                name: "SkyForwardScatteringEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_forward_scattering_enable),
            },
            FieldInfoData {
                name: "ProceduralSkyReceiveHeightFog",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, procedural_sky_receive_height_fog),
            },
            FieldInfoData {
                name: "PhysicalSkyEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, physical_sky_enabled),
            },
            FieldInfoData {
                name: "PhysicalSkyPrecisionHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, physical_sky_precision_height),
            },
            FieldInfoData {
                name: "PhysicalSkyPrecisionView",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, physical_sky_precision_view),
            },
            FieldInfoData {
                name: "PhysicalSkyPrecisionSun",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, physical_sky_precision_sun),
            },
            FieldInfoData {
                name: "PhysicalSkyScatteringOrders",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, physical_sky_scattering_orders),
            },
            FieldInfoData {
                name: "PhysicalSkyAerialPerspectiveTextureWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, physical_sky_aerial_perspective_texture_width),
            },
            FieldInfoData {
                name: "PhysicalSkyAerialPerspectiveTextureHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, physical_sky_aerial_perspective_texture_height),
            },
            FieldInfoData {
                name: "PhysicalSkyAerialPerspectiveTextureDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, physical_sky_aerial_perspective_texture_depth),
            },
            FieldInfoData {
                name: "PhysicalSkyScatteringEvalFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, physical_sky_scattering_eval_frame_count),
            },
            FieldInfoData {
                name: "PhysicalSkyAerialPerspectiveMaxDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, physical_sky_aerial_perspective_max_distance),
            },
            FieldInfoData {
                name: "PhysicalSkyForcePrecompute",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, physical_sky_force_precompute),
            },
            FieldInfoData {
                name: "VolumetricCloudsEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_enabled),
            },
            FieldInfoData {
                name: "VolumetricCloudsQuality",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityLevel",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_quality),
            },
            FieldInfoData {
                name: "VolumetricCloudsCastShadow",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_cast_shadow),
            },
            FieldInfoData {
                name: "VolumetricCloudsCastShadowInForwardRender",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_cast_shadow_in_forward_render),
            },
            FieldInfoData {
                name: "VolumetricCloudsAffectAerialPerspective",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_affect_aerial_perspective),
            },
            FieldInfoData {
                name: "VolumetricCloudsReceiveAerialPerspective",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_receive_aerial_perspective),
            },
            FieldInfoData {
                name: "VolumetricCloudsOccludeLensFlare",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_occlude_lens_flare),
            },
            FieldInfoData {
                name: "VolumetricCloudsRenderTargetResolutionDivider",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_render_target_resolution_divider),
            },
            FieldInfoData {
                name: "VolumetricCloudsReflectionRenderTargetResolutionDivider",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_reflection_render_target_resolution_divider),
            },
            FieldInfoData {
                name: "VolumetricCloudsShadowIterationCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_shadow_iteration_count),
            },
            FieldInfoData {
                name: "VolumetricCloudsShadowmapResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_shadowmap_resolution),
            },
            FieldInfoData {
                name: "VolumetricCloudsShadowmapBlurSamples",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_shadowmap_blur_samples),
            },
            FieldInfoData {
                name: "VolumetricCloudsSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "QualityScalableInt",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_sample_count),
            },
            FieldInfoData {
                name: "VolumetricCloudsReflectionSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_reflection_sample_count),
            },
            FieldInfoData {
                name: "VolumetricCloudsIBLSampleCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_i_b_l_sample_count),
            },
            FieldInfoData {
                name: "VolumetricCloudsTemporalCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_temporal_coefficient),
            },
            FieldInfoData {
                name: "VolumetricCloudsEnvColorTemporalCoefficient",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, volumetric_clouds_env_color_temporal_coefficient),
            },
            FieldInfoData {
                name: "TransparentFoggingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, transparent_fogging_enable),
            },
            FieldInfoData {
                name: "DistortionEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distortion_enable),
            },
            FieldInfoData {
                name: "DistortionHalfResEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distortion_half_res_enable),
            },
            FieldInfoData {
                name: "Distortion8BitEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distortion8_bit_enable),
            },
            FieldInfoData {
                name: "DistortionTilingEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, distortion_tiling_enable),
            },
            FieldInfoData {
                name: "StaticEnvmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, static_envmap_enable),
            },
            FieldInfoData {
                name: "CustomEnvmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, custom_envmap_enable),
            },
            FieldInfoData {
                name: "SkyEnvmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_enable),
            },
            FieldInfoData {
                name: "SkyEnvmapFilterWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_filter_width),
            },
            FieldInfoData {
                name: "SkyEnvmapResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_resolution),
            },
            FieldInfoData {
                name: "SkyEnvmapMipmapGenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_mipmap_gen_enable),
            },
            FieldInfoData {
                name: "DrawDebugSkyEnvmapMipLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_sky_envmap_mip_level),
            },
            FieldInfoData {
                name: "SkyEnvmapFilterMode",
                flags: MemberInfoFlags::new(0),
                field_type: "MipmapFilterMode",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_filter_mode),
            },
            FieldInfoData {
                name: "SkyEnvmapSidesPerFrameCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_sides_per_frame_count),
            },
            FieldInfoData {
                name: "SkyEnvmapForceUpdateEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_force_update_enable),
            },
            FieldInfoData {
                name: "SkyEnvmapUseFastHDR",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_use_fast_h_d_r),
            },
            FieldInfoData {
                name: "SkyEnvmapDebugColorEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_debug_color_enable),
            },
            FieldInfoData {
                name: "SkyEnvmapUpdateCountThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_update_count_threshold),
            },
            FieldInfoData {
                name: "SkyEnvmapUpdateValueThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_update_value_threshold),
            },
            FieldInfoData {
                name: "SkyEnvmapCloudFogEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_cloud_fog_enable),
            },
            FieldInfoData {
                name: "SkyEnvmapGenerateNoBackdropEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, sky_envmap_generate_no_backdrop_enable),
            },
            FieldInfoData {
                name: "DynamicEnvmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, dynamic_envmap_enable),
            },
            FieldInfoData {
                name: "DrawDebugDynamicEnvmapMipLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_dynamic_envmap_mip_level),
            },
            FieldInfoData {
                name: "DynamicEnvmapMipmapGenEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, dynamic_envmap_mipmap_gen_enable),
            },
            FieldInfoData {
                name: "DrawDebugDynamicEnvmap",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_dynamic_envmap),
            },
            FieldInfoData {
                name: "DynamicEnvmapShadowmapEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, dynamic_envmap_shadowmap_enable),
            },
            FieldInfoData {
                name: "DynamicEnvmapShadowmapResolution",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(WorldRenderSettingsBase, dynamic_envmap_shadowmap_resolution),
            },
            FieldInfoData {
                name: "DynamicEnvmapShadowmapFarPlaneOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, dynamic_envmap_shadowmap_far_plane_override),
            },
            FieldInfoData {
                name: "DynamicEnvmapShadowmapFarPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldRenderSettingsBase, dynamic_envmap_shadowmap_far_plane),
            },
            FieldInfoData {
                name: "DynamicEnvmapShadowmapShadowExtrusionOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, dynamic_envmap_shadowmap_shadow_extrusion_override),
            },
            FieldInfoData {
                name: "DynamicEnvmapShadowmapShadowExtrusion",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WorldRenderSettingsBase, dynamic_envmap_shadowmap_shadow_extrusion),
            },
            FieldInfoData {
                name: "DrawDebugDynamicEnvmapShadowmap",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_debug_dynamic_envmap_shadowmap),
            },
            FieldInfoData {
                name: "DrawDynamicEnvmapFrustums",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, draw_dynamic_envmap_frustums),
            },
            FieldInfoData {
                name: "SetupJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, setup_job_enable),
            },
            FieldInfoData {
                name: "SetupJobsCreateViewJob",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, setup_jobs_create_view_job),
            },
            FieldInfoData {
                name: "PrepareDispatchListJobEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WorldRenderSettingsBase, prepare_dispatch_list_job_enable),
            },
            FieldInfoData {
                name: "IndirectSpecularIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, indirect_specular_intensity),
            },
            FieldInfoData {
                name: "IndirectSpecularReflectanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, indirect_specular_reflectance_scale),
            },
            FieldInfoData {
                name: "IndirectSpecularProbesIntensity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, indirect_specular_probes_intensity),
            },
            FieldInfoData {
                name: "IndirectSpecularProbesReflectanceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(WorldRenderSettingsBase, indirect_specular_probes_reflectance_scale),
            },
        ],
    }),
    array_type: Some(WORLDRENDERSETTINGSBASE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WorldRenderSettingsBase {
    fn type_info(&self) -> &'static TypeInfo {
        WORLDRENDERSETTINGSBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WORLDRENDERSETTINGSBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldRenderSettingsBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("WorldRenderSettingsBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LightTileDebugLightCountMode {
    #[default]
    LightTileDebugLightCountMode_Total = 0,
    LightTileDebugLightCountMode_Punctual = 1,
    LightTileDebugLightCountMode_PunctualShadow = 2,
    LightTileDebugLightCountMode_Area = 3,
    LightTileDebugLightCountMode_AreaShadow = 4,
    LightTileDebugLightCountMode_LocalIBL = 5,
    LightTileDebugLightCountMode_LocalPR = 6,
}

pub static LIGHTTILEDEBUGLIGHTCOUNTMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTileDebugLightCountMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldRender",
    data: TypeInfoData::Enum,
    array_type: Some(LIGHTTILEDEBUGLIGHTCOUNTMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LightTileDebugLightCountMode {
    fn type_info(&self) -> &'static TypeInfo {
        LIGHTTILEDEBUGLIGHTCOUNTMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LIGHTTILEDEBUGLIGHTCOUNTMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LightTileDebugLightCountMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("LightTileDebugLightCountMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum FrameSynthesisMode {
    #[default]
    FrameSynthesisMode_None = 0,
    FrameSynthesisMode_Checkerboard = 1,
}

pub static FRAMESYNTHESISMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FrameSynthesisMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldRender",
    data: TypeInfoData::Enum,
    array_type: Some(FRAMESYNTHESISMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FrameSynthesisMode {
    fn type_info(&self) -> &'static TypeInfo {
        FRAMESYNTHESISMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FRAMESYNTHESISMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FrameSynthesisMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("FrameSynthesisMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum SkyRenderMode {
    #[default]
    SkyRenderMode_SkyBox = 0,
    SkyRenderMode_PhysicallyBased = 1,
}

pub static SKYRENDERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyRenderMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldRender",
    data: TypeInfoData::Enum,
    array_type: Some(SKYRENDERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SkyRenderMode {
    fn type_info(&self) -> &'static TypeInfo {
        SKYRENDERMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SKYRENDERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SkyRenderMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("SkyRenderMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PostProcessDofMode {
    #[default]
    PostProcessDofMode_Gaussian = 0,
    PostProcessDofMode_Sprite = 1,
    PostProcessDofMode_Circular = 2,
}

pub static POSTPROCESSDOFMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PostProcessDofMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldRender",
    data: TypeInfoData::Enum,
    array_type: Some(POSTPROCESSDOFMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PostProcessDofMode {
    fn type_info(&self) -> &'static TypeInfo {
        POSTPROCESSDOFMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static POSTPROCESSDOFMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PostProcessDofMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("PostProcessDofMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PostProcessAAMode {
    #[default]
    PostProcessAAMode_None = 0,
    PostProcessAAMode_FxaaLow = 1,
    PostProcessAAMode_FxaaMedium = 2,
    PostProcessAAMode_FxaaHigh = 3,
    PostProcessAAMode_FxaaCompute = 4,
    PostProcessAAMode_FxaaComputeExtreme = 5,
    PostProcessAAMode_Smaa1x = 6,
    PostProcessAAMode_DLAA = 7,
    PostProcessAAMode_TemporalAA = 8,
}

pub static POSTPROCESSAAMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PostProcessAAMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldRender",
    data: TypeInfoData::Enum,
    array_type: Some(POSTPROCESSAAMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PostProcessAAMode {
    fn type_info(&self) -> &'static TypeInfo {
        POSTPROCESSAAMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static POSTPROCESSAAMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PostProcessAAMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("PostProcessAAMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MipmapFilterMode {
    #[default]
    MipmapFilterMode_Box = 0,
    MipmapFilterMode_Renormalize = 1,
    MipmapFilterMode_Poisson13 = 2,
    MipmapFilterMode_Poisson13Clamped = 3,
    MipmapFilterMode_BoxAverageEdges = 4,
}

pub static MIPMAPFILTERMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MipmapFilterMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldRender",
    data: TypeInfoData::Enum,
    array_type: Some(MIPMAPFILTERMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MipmapFilterMode {
    fn type_info(&self) -> &'static TypeInfo {
        MIPMAPFILTERMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MIPMAPFILTERMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MipmapFilterMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("MipmapFilterMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LuminancePreset {
    #[default]
    LuminancePreset_OutdoorSunAtHorizon = 600000,
    LuminancePreset_OutdoorClearSky = 8000,
    LuminancePreset_OutdoorOvercastDay = 3000,
}

pub static LUMINANCEPRESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuminancePreset",
    flags: MemberInfoFlags::new(49429),
    module: "WorldRender",
    data: TypeInfoData::Enum,
    array_type: Some(LUMINANCEPRESET_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LuminancePreset {
    fn type_info(&self) -> &'static TypeInfo {
        LUMINANCEPRESET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LUMINANCEPRESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuminancePreset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("LuminancePreset"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum IlluminancePreset {
    #[default]
    IlluminancePreset_OutdoorSunnyDay = 120000,
    IlluminancePreset_OutdoorOvercastDay = 2000,
    IlluminancePreset_OutdoorStreetNight = 15,
    IlluminancePreset_OutdoorCountrySideNight = 1,
    IlluminancePreset_IndoorOffice = 1000,
}

pub static ILLUMINANCEPRESET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IlluminancePreset",
    flags: MemberInfoFlags::new(49429),
    module: "WorldRender",
    data: TypeInfoData::Enum,
    array_type: Some(ILLUMINANCEPRESET_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for IlluminancePreset {
    fn type_info(&self) -> &'static TypeInfo {
        ILLUMINANCEPRESET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ILLUMINANCEPRESET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IlluminancePreset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("IlluminancePreset"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum WorldViewMode {
    #[default]
    WorldViewMode_Default = 0,
    WorldViewMode_Diffuse = 1,
    WorldViewMode_BaseColor = 2,
    WorldViewMode_MetalMask = 3,
    WorldViewMode_Reflectance = 4,
    WorldViewMode_Specular = 5,
    WorldViewMode_Fresnel0 = 6,
    WorldViewMode_Fresnel90 = 7,
    WorldViewMode_Normal = 8,
    WorldViewMode_Smoothness = 9,
    WorldViewMode_Roughness = 10,
    WorldViewMode_LinearRoughness = 11,
    WorldViewMode_MaterialId = 12,
    WorldViewMode_MaterialIdTileMask = 13,
    WorldViewMode_SubSurfaceProfileId = 14,
    WorldViewMode_SubSurfaceRadius = 15,
    WorldViewMode_SubSurfaceTranslucency = 16,
    WorldViewMode_Thickness = 17,
    WorldViewMode_LargeThickness = 18,
    WorldViewMode_CustomEnvmapId = 19,
    WorldViewMode_CoatCoverage = 20,
    WorldViewMode_MaterialData = 21,
    WorldViewMode_RawLinear = 22,
    WorldViewMode_RawLinearAlpha = 23,
    WorldViewMode_Light = 24,
    WorldViewMode_LightDiffuse = 25,
    WorldViewMode_LightColoredDiffuse = 26,
    WorldViewMode_LightSpecular = 27,
    WorldViewMode_LightIndirectDiffuse = 28,
    WorldViewMode_LightIndirectDiffuseOnly = 29,
    WorldViewMode_LightColoredIndirectDiffuse = 30,
    WorldViewMode_LightTranslucency = 31,
    WorldViewMode_LightReflectionOnly = 32,
    WorldViewMode_LightMirrorReflectionOnly = 33,
    WorldViewMode_ShadowMask = 34,
    WorldViewMode_Transmittance = 35,
    WorldViewMode_SkyVisibility = 36,
    WorldViewMode_Emissive = 37,
    WorldViewMode_DynamicAO = 38,
    WorldViewMode_Depth = 39,
    WorldViewMode_RadiosityLightMaps = 40,
    WorldViewMode_RadiosityDiffuseColor = 41,
    WorldViewMode_RadiosityTargetUV = 42,
    WorldViewMode_RadiosityNormal = 43,
    WorldViewMode_Overdraw = 44,
    WorldViewMode_OverdrawDepthTest = 45,
    WorldViewMode_LightOverdraw = 46,
    WorldViewMode_ShaderCost = 47,
    WorldViewMode_Occluders = 48,
    WorldViewMode_SssTiles = 49,
    WorldViewMode_DielectricRange = 50,
    WorldViewMode_ConductorRange = 51,
    WorldViewMode_Fresnel0Range = 52,
    WorldViewMode_IlluminanceRange = 53,
    WorldViewMode_LuminanceRange = 54,
    WorldViewMode_FilmicEffects = 55,
    WorldViewMode_CoC = 56,
    WorldViewMode_VelocityVector = 57,
    WorldViewMode_DistortionVector = 58,
    WorldViewMode_StaticIBL = 59,
    WorldViewMode_ScreenSpaceRaytraceReflections = 60,
    WorldViewMode_ScreenSpaceRaytraceCoverage = 61,
    WorldViewMode_ScreenSpaceRaytraceImportons = 62,
    WorldViewMode_NanDetection = 63,
    WorldViewMode_Count = 64,
}

pub static WORLDVIEWMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldViewMode",
    flags: MemberInfoFlags::new(49429),
    module: "WorldRender",
    data: TypeInfoData::Enum,
    array_type: Some(WORLDVIEWMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WorldViewMode {
    fn type_info(&self) -> &'static TypeInfo {
        WORLDVIEWMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WORLDVIEWMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WorldViewMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("WorldViewMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ModelRenderObject {
    pub _glacier_base: RenderObject,
}

pub trait ModelRenderObjectTrait: RenderObjectTrait {
}

impl ModelRenderObjectTrait for ModelRenderObject {
}

impl RenderObjectTrait for ModelRenderObject {
}

pub static MODELRENDEROBJECT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelRenderObject",
    flags: MemberInfoFlags::new(101),
    module: "WorldRender",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RENDEROBJECT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ModelRenderObject as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MODELRENDEROBJECT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ModelRenderObject {
    fn type_info(&self) -> &'static TypeInfo {
        MODELRENDEROBJECT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MODELRENDEROBJECT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ModelRenderObject-Array",
    flags: MemberInfoFlags::new(145),
    module: "WorldRender",
    data: TypeInfoData::Array("ModelRenderObject"),
    array_type: None,
    alignment: 8,
};


