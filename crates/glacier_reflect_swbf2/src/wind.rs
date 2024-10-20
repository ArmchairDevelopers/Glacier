use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_wind_types(registry: &mut TypeRegistry) {
    registry.register_type(BAKED3DAS2X2DTEXWINDFORCEBASE_TYPE_INFO);
    registry.register_type(BAKED3DAS2X2DTEXWINDFORCEBASE_ARRAY_TYPE_INFO);
    registry.register_type(CONEWINDFORCEBASE_TYPE_INFO);
    registry.register_type(CONEWINDFORCEBASE_ARRAY_TYPE_INFO);
    registry.register_type(SPHEREWINDFORCEBASE_TYPE_INFO);
    registry.register_type(SPHEREWINDFORCEBASE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCE_TYPE_INFO);
    registry.register_type(LOCALWINDFORCE_ARRAY_TYPE_INFO);
    registry.register_type(DIRECTIONWINDFORCEBASE_TYPE_INFO);
    registry.register_type(DIRECTIONWINDFORCEBASE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCEGROUP_TYPE_INFO);
    registry.register_type(LOCALWINDFORCEGROUP_ARRAY_TYPE_INFO);
    registry.register_type(LOCALWINDFORCETYPE_TYPE_INFO);
    registry.register_type(LOCALWINDFORCETYPE_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct Baked3DAs2x2DTexWindForceBase {
    pub position: super::core::Vec3,
    pub direction_x: super::core::Vec3,
    pub direction_y: super::core::Vec3,
    pub direction_z: super::core::Vec3,
    pub scaled_direction_x: super::core::Vec3,
    pub scaled_direction_y: super::core::Vec3,
    pub scaled_direction_z: super::core::Vec3,
    pub texture_velocity_z_x_scale: super::core::Vec3,
    pub texture_velocity_z_y_scale: super::core::Vec3,
    pub texture_velocity_z_x: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub texture_velocity_z_y: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
    pub aabb_min_x: f32,
    pub aabb_max_x: f32,
    pub aabb_min_y: f32,
    pub aabb_max_y: f32,
    pub aabb_min_z: f32,
    pub aabb_max_z: f32,
    pub inv_attenuation: f32,
    pub id: u32,
    pub groups: u32,
    pub strength: f32,
    pub variation: f32,
    pub variation_rate: f32,
    pub micro_variation: f32,
    pub hardness: f32,
    pub force_as_instant_velocity: f32,
    pub field_flag_changed0: u32,
}

pub trait Baked3DAs2x2DTexWindForceBaseTrait: TypeObject {
    fn position(&self) -> &super::core::Vec3;
    fn direction_x(&self) -> &super::core::Vec3;
    fn direction_y(&self) -> &super::core::Vec3;
    fn direction_z(&self) -> &super::core::Vec3;
    fn scaled_direction_x(&self) -> &super::core::Vec3;
    fn scaled_direction_y(&self) -> &super::core::Vec3;
    fn scaled_direction_z(&self) -> &super::core::Vec3;
    fn texture_velocity_z_x_scale(&self) -> &super::core::Vec3;
    fn texture_velocity_z_y_scale(&self) -> &super::core::Vec3;
    fn texture_velocity_z_x(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn texture_velocity_z_y(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn aabb_min_x(&self) -> &f32;
    fn aabb_max_x(&self) -> &f32;
    fn aabb_min_y(&self) -> &f32;
    fn aabb_max_y(&self) -> &f32;
    fn aabb_min_z(&self) -> &f32;
    fn aabb_max_z(&self) -> &f32;
    fn inv_attenuation(&self) -> &f32;
    fn id(&self) -> &u32;
    fn groups(&self) -> &u32;
    fn strength(&self) -> &f32;
    fn variation(&self) -> &f32;
    fn variation_rate(&self) -> &f32;
    fn micro_variation(&self) -> &f32;
    fn hardness(&self) -> &f32;
    fn force_as_instant_velocity(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u32;
}

impl Baked3DAs2x2DTexWindForceBaseTrait for Baked3DAs2x2DTexWindForceBase {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn direction_x(&self) -> &super::core::Vec3 {
        &self.direction_x
    }
    fn direction_y(&self) -> &super::core::Vec3 {
        &self.direction_y
    }
    fn direction_z(&self) -> &super::core::Vec3 {
        &self.direction_z
    }
    fn scaled_direction_x(&self) -> &super::core::Vec3 {
        &self.scaled_direction_x
    }
    fn scaled_direction_y(&self) -> &super::core::Vec3 {
        &self.scaled_direction_y
    }
    fn scaled_direction_z(&self) -> &super::core::Vec3 {
        &self.scaled_direction_z
    }
    fn texture_velocity_z_x_scale(&self) -> &super::core::Vec3 {
        &self.texture_velocity_z_x_scale
    }
    fn texture_velocity_z_y_scale(&self) -> &super::core::Vec3 {
        &self.texture_velocity_z_y_scale
    }
    fn texture_velocity_z_x(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture_velocity_z_x
    }
    fn texture_velocity_z_y(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture_velocity_z_y
    }
    fn aabb_min_x(&self) -> &f32 {
        &self.aabb_min_x
    }
    fn aabb_max_x(&self) -> &f32 {
        &self.aabb_max_x
    }
    fn aabb_min_y(&self) -> &f32 {
        &self.aabb_min_y
    }
    fn aabb_max_y(&self) -> &f32 {
        &self.aabb_max_y
    }
    fn aabb_min_z(&self) -> &f32 {
        &self.aabb_min_z
    }
    fn aabb_max_z(&self) -> &f32 {
        &self.aabb_max_z
    }
    fn inv_attenuation(&self) -> &f32 {
        &self.inv_attenuation
    }
    fn id(&self) -> &u32 {
        &self.id
    }
    fn groups(&self) -> &u32 {
        &self.groups
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
    fn variation(&self) -> &f32 {
        &self.variation
    }
    fn variation_rate(&self) -> &f32 {
        &self.variation_rate
    }
    fn micro_variation(&self) -> &f32 {
        &self.micro_variation
    }
    fn hardness(&self) -> &f32 {
        &self.hardness
    }
    fn force_as_instant_velocity(&self) -> &f32 {
        &self.force_as_instant_velocity
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
}

pub static BAKED3DAS2X2DTEXWINDFORCEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Baked3DAs2x2DTexWindForceBase",
    flags: MemberInfoFlags::new(73),
    module: "Wind",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Baked3DAs2x2DTexWindForceBase as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, position),
            },
            FieldInfoData {
                name: "DirectionX",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, direction_x),
            },
            FieldInfoData {
                name: "DirectionY",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, direction_y),
            },
            FieldInfoData {
                name: "DirectionZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, direction_z),
            },
            FieldInfoData {
                name: "ScaledDirectionX",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, scaled_direction_x),
            },
            FieldInfoData {
                name: "ScaledDirectionY",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, scaled_direction_y),
            },
            FieldInfoData {
                name: "ScaledDirectionZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, scaled_direction_z),
            },
            FieldInfoData {
                name: "TextureVelocityZXScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, texture_velocity_z_x_scale),
            },
            FieldInfoData {
                name: "TextureVelocityZYScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, texture_velocity_z_y_scale),
            },
            FieldInfoData {
                name: "TextureVelocityZX",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, texture_velocity_z_x),
            },
            FieldInfoData {
                name: "TextureVelocityZY",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, texture_velocity_z_y),
            },
            FieldInfoData {
                name: "AabbMinX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, aabb_min_x),
            },
            FieldInfoData {
                name: "AabbMaxX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, aabb_max_x),
            },
            FieldInfoData {
                name: "AabbMinY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, aabb_min_y),
            },
            FieldInfoData {
                name: "AabbMaxY",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, aabb_max_y),
            },
            FieldInfoData {
                name: "AabbMinZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, aabb_min_z),
            },
            FieldInfoData {
                name: "AabbMaxZ",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, aabb_max_z),
            },
            FieldInfoData {
                name: "InvAttenuation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, inv_attenuation),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, id),
            },
            FieldInfoData {
                name: "Groups",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, groups),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, strength),
            },
            FieldInfoData {
                name: "Variation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, variation),
            },
            FieldInfoData {
                name: "VariationRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, variation_rate),
            },
            FieldInfoData {
                name: "MicroVariation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, micro_variation),
            },
            FieldInfoData {
                name: "Hardness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, hardness),
            },
            FieldInfoData {
                name: "ForceAsInstantVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, force_as_instant_velocity),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(Baked3DAs2x2DTexWindForceBase, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(BAKED3DAS2X2DTEXWINDFORCEBASE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Baked3DAs2x2DTexWindForceBase {
    fn type_info(&self) -> &'static TypeInfo {
        BAKED3DAS2X2DTEXWINDFORCEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BAKED3DAS2X2DTEXWINDFORCEBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Baked3DAs2x2DTexWindForceBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Wind",
    data: TypeInfoData::Array("Baked3DAs2x2DTexWindForceBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConeWindForceBase {
    pub position: super::core::Vec3,
    pub direction: super::core::Vec3,
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub cos_inner_angle: f32,
    pub cos_outer_angle: f32,
    pub id: u32,
    pub groups: u32,
    pub strength: f32,
    pub variation: f32,
    pub variation_rate: f32,
    pub micro_variation: f32,
    pub hardness: f32,
    pub force_as_instant_velocity: f32,
    pub field_flag_changed0: u16,
}

pub trait ConeWindForceBaseTrait: TypeObject {
    fn position(&self) -> &super::core::Vec3;
    fn direction(&self) -> &super::core::Vec3;
    fn inner_radius(&self) -> &f32;
    fn outer_radius(&self) -> &f32;
    fn cos_inner_angle(&self) -> &f32;
    fn cos_outer_angle(&self) -> &f32;
    fn id(&self) -> &u32;
    fn groups(&self) -> &u32;
    fn strength(&self) -> &f32;
    fn variation(&self) -> &f32;
    fn variation_rate(&self) -> &f32;
    fn micro_variation(&self) -> &f32;
    fn hardness(&self) -> &f32;
    fn force_as_instant_velocity(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u16;
}

impl ConeWindForceBaseTrait for ConeWindForceBase {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn direction(&self) -> &super::core::Vec3 {
        &self.direction
    }
    fn inner_radius(&self) -> &f32 {
        &self.inner_radius
    }
    fn outer_radius(&self) -> &f32 {
        &self.outer_radius
    }
    fn cos_inner_angle(&self) -> &f32 {
        &self.cos_inner_angle
    }
    fn cos_outer_angle(&self) -> &f32 {
        &self.cos_outer_angle
    }
    fn id(&self) -> &u32 {
        &self.id
    }
    fn groups(&self) -> &u32 {
        &self.groups
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
    fn variation(&self) -> &f32 {
        &self.variation
    }
    fn variation_rate(&self) -> &f32 {
        &self.variation_rate
    }
    fn micro_variation(&self) -> &f32 {
        &self.micro_variation
    }
    fn hardness(&self) -> &f32 {
        &self.hardness
    }
    fn force_as_instant_velocity(&self) -> &f32 {
        &self.force_as_instant_velocity
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static CONEWINDFORCEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConeWindForceBase",
    flags: MemberInfoFlags::new(36937),
    module: "Wind",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConeWindForceBase as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ConeWindForceBase, position),
            },
            FieldInfoData {
                name: "Direction",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ConeWindForceBase, direction),
            },
            FieldInfoData {
                name: "InnerRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConeWindForceBase, inner_radius),
            },
            FieldInfoData {
                name: "OuterRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConeWindForceBase, outer_radius),
            },
            FieldInfoData {
                name: "CosInnerAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConeWindForceBase, cos_inner_angle),
            },
            FieldInfoData {
                name: "CosOuterAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConeWindForceBase, cos_outer_angle),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ConeWindForceBase, id),
            },
            FieldInfoData {
                name: "Groups",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ConeWindForceBase, groups),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConeWindForceBase, strength),
            },
            FieldInfoData {
                name: "Variation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConeWindForceBase, variation),
            },
            FieldInfoData {
                name: "VariationRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConeWindForceBase, variation_rate),
            },
            FieldInfoData {
                name: "MicroVariation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConeWindForceBase, micro_variation),
            },
            FieldInfoData {
                name: "Hardness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConeWindForceBase, hardness),
            },
            FieldInfoData {
                name: "ForceAsInstantVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConeWindForceBase, force_as_instant_velocity),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(ConeWindForceBase, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(CONEWINDFORCEBASE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ConeWindForceBase {
    fn type_info(&self) -> &'static TypeInfo {
        CONEWINDFORCEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONEWINDFORCEBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConeWindForceBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Wind",
    data: TypeInfoData::Array("ConeWindForceBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SphereWindForceBase {
    pub position: super::core::Vec3,
    pub radius: f32,
    pub id: u32,
    pub groups: u32,
    pub strength: f32,
    pub variation: f32,
    pub variation_rate: f32,
    pub micro_variation: f32,
    pub hardness: f32,
    pub force_as_instant_velocity: f32,
    pub field_flag_changed0: u16,
}

pub trait SphereWindForceBaseTrait: TypeObject {
    fn position(&self) -> &super::core::Vec3;
    fn radius(&self) -> &f32;
    fn id(&self) -> &u32;
    fn groups(&self) -> &u32;
    fn strength(&self) -> &f32;
    fn variation(&self) -> &f32;
    fn variation_rate(&self) -> &f32;
    fn micro_variation(&self) -> &f32;
    fn hardness(&self) -> &f32;
    fn force_as_instant_velocity(&self) -> &f32;
    fn field_flag_changed0(&self) -> &u16;
}

impl SphereWindForceBaseTrait for SphereWindForceBase {
    fn position(&self) -> &super::core::Vec3 {
        &self.position
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn id(&self) -> &u32 {
        &self.id
    }
    fn groups(&self) -> &u32 {
        &self.groups
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
    fn variation(&self) -> &f32 {
        &self.variation
    }
    fn variation_rate(&self) -> &f32 {
        &self.variation_rate
    }
    fn micro_variation(&self) -> &f32 {
        &self.micro_variation
    }
    fn hardness(&self) -> &f32 {
        &self.hardness
    }
    fn force_as_instant_velocity(&self) -> &f32 {
        &self.force_as_instant_velocity
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static SPHEREWINDFORCEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereWindForceBase",
    flags: MemberInfoFlags::new(36937),
    module: "Wind",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SphereWindForceBase as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Position",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(SphereWindForceBase, position),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereWindForceBase, radius),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SphereWindForceBase, id),
            },
            FieldInfoData {
                name: "Groups",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SphereWindForceBase, groups),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereWindForceBase, strength),
            },
            FieldInfoData {
                name: "Variation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereWindForceBase, variation),
            },
            FieldInfoData {
                name: "VariationRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereWindForceBase, variation_rate),
            },
            FieldInfoData {
                name: "MicroVariation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereWindForceBase, micro_variation),
            },
            FieldInfoData {
                name: "Hardness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereWindForceBase, hardness),
            },
            FieldInfoData {
                name: "ForceAsInstantVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereWindForceBase, force_as_instant_velocity),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(SphereWindForceBase, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(SPHEREWINDFORCEBASE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SphereWindForceBase {
    fn type_info(&self) -> &'static TypeInfo {
        SPHEREWINDFORCEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPHEREWINDFORCEBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereWindForceBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Wind",
    data: TypeInfoData::Array("SphereWindForceBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalWindForce {
    pub id: u32,
    pub groups: u32,
    pub strength: f32,
    pub variation: f32,
    pub variation_rate: f32,
    pub micro_variation: f32,
    pub hardness: f32,
    pub force_as_instant_velocity: f32,
}

pub trait LocalWindForceTrait: TypeObject {
    fn id(&self) -> &u32;
    fn groups(&self) -> &u32;
    fn strength(&self) -> &f32;
    fn variation(&self) -> &f32;
    fn variation_rate(&self) -> &f32;
    fn micro_variation(&self) -> &f32;
    fn hardness(&self) -> &f32;
    fn force_as_instant_velocity(&self) -> &f32;
}

impl LocalWindForceTrait for LocalWindForce {
    fn id(&self) -> &u32 {
        &self.id
    }
    fn groups(&self) -> &u32 {
        &self.groups
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
    fn variation(&self) -> &f32 {
        &self.variation
    }
    fn variation_rate(&self) -> &f32 {
        &self.variation_rate
    }
    fn micro_variation(&self) -> &f32 {
        &self.micro_variation
    }
    fn hardness(&self) -> &f32 {
        &self.hardness
    }
    fn force_as_instant_velocity(&self) -> &f32 {
        &self.force_as_instant_velocity
    }
}

pub static LOCALWINDFORCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForce",
    flags: MemberInfoFlags::new(36937),
    module: "Wind",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalWindForce as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LocalWindForce, id),
            },
            FieldInfoData {
                name: "Groups",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(LocalWindForce, groups),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalWindForce, strength),
            },
            FieldInfoData {
                name: "Variation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalWindForce, variation),
            },
            FieldInfoData {
                name: "VariationRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalWindForce, variation_rate),
            },
            FieldInfoData {
                name: "MicroVariation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalWindForce, micro_variation),
            },
            FieldInfoData {
                name: "Hardness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalWindForce, hardness),
            },
            FieldInfoData {
                name: "ForceAsInstantVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LocalWindForce, force_as_instant_velocity),
            },
        ],
    }),
    array_type: Some(LOCALWINDFORCE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for LocalWindForce {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALWINDFORCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALWINDFORCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForce-Array",
    flags: MemberInfoFlags::new(145),
    module: "Wind",
    data: TypeInfoData::Array("LocalWindForce"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DirectionWindForceBase {
    pub direction: super::core::Vec3,
    pub strength: f32,
    pub variation: f32,
    pub variation_rate: f32,
    pub micro_variation: f32,
    pub turbulence_multiplier: f32,
    pub turbulence_scale: f32,
    pub id: u32,
    pub field_flag_changed0: u8,
}

pub trait DirectionWindForceBaseTrait: TypeObject {
    fn direction(&self) -> &super::core::Vec3;
    fn strength(&self) -> &f32;
    fn variation(&self) -> &f32;
    fn variation_rate(&self) -> &f32;
    fn micro_variation(&self) -> &f32;
    fn turbulence_multiplier(&self) -> &f32;
    fn turbulence_scale(&self) -> &f32;
    fn id(&self) -> &u32;
    fn field_flag_changed0(&self) -> &u8;
}

impl DirectionWindForceBaseTrait for DirectionWindForceBase {
    fn direction(&self) -> &super::core::Vec3 {
        &self.direction
    }
    fn strength(&self) -> &f32 {
        &self.strength
    }
    fn variation(&self) -> &f32 {
        &self.variation
    }
    fn variation_rate(&self) -> &f32 {
        &self.variation_rate
    }
    fn micro_variation(&self) -> &f32 {
        &self.micro_variation
    }
    fn turbulence_multiplier(&self) -> &f32 {
        &self.turbulence_multiplier
    }
    fn turbulence_scale(&self) -> &f32 {
        &self.turbulence_scale
    }
    fn id(&self) -> &u32 {
        &self.id
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static DIRECTIONWINDFORCEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DirectionWindForceBase",
    flags: MemberInfoFlags::new(36937),
    module: "Wind",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DirectionWindForceBase as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Direction",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(DirectionWindForceBase, direction),
            },
            FieldInfoData {
                name: "Strength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DirectionWindForceBase, strength),
            },
            FieldInfoData {
                name: "Variation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DirectionWindForceBase, variation),
            },
            FieldInfoData {
                name: "VariationRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DirectionWindForceBase, variation_rate),
            },
            FieldInfoData {
                name: "MicroVariation",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DirectionWindForceBase, micro_variation),
            },
            FieldInfoData {
                name: "TurbulenceMultiplier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DirectionWindForceBase, turbulence_multiplier),
            },
            FieldInfoData {
                name: "TurbulenceScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DirectionWindForceBase, turbulence_scale),
            },
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DirectionWindForceBase, id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(DirectionWindForceBase, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DIRECTIONWINDFORCEBASE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DirectionWindForceBase {
    fn type_info(&self) -> &'static TypeInfo {
        DIRECTIONWINDFORCEBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DIRECTIONWINDFORCEBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DirectionWindForceBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Wind",
    data: TypeInfoData::Array("DirectionWindForceBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LocalWindForceGroup {
    #[default]
    LocalWindForceGroup_Vegetation = 0,
    LocalWindForceGroup_MeshScattering = 1,
    LocalWindForceGroup_Effects = 2,
    LocalWindForceGroup_Cloth = 3,
    LocalWindForceGroup_Physics = 4,
    LocalWindForceGroup_Count = 5,
}

pub static LOCALWINDFORCEGROUP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceGroup",
    flags: MemberInfoFlags::new(49429),
    module: "Wind",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALWINDFORCEGROUP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalWindForceGroup {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALWINDFORCEGROUP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALWINDFORCEGROUP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceGroup-Array",
    flags: MemberInfoFlags::new(145),
    module: "Wind",
    data: TypeInfoData::Array("LocalWindForceGroup"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LocalWindForceType {
    #[default]
    LocalWindForce_Sphere = 0,
    LocalWindForce_Cone = 1,
    LocalWindForce_Baked3dAs2x2DTexture = 2,
}

pub static LOCALWINDFORCETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceType",
    flags: MemberInfoFlags::new(49429),
    module: "Wind",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALWINDFORCETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalWindForceType {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALWINDFORCETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALWINDFORCETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalWindForceType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Wind",
    data: TypeInfoData::Array("LocalWindForceType"),
    array_type: None,
    alignment: 8,
};


