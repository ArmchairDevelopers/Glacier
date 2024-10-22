use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_input_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(INPUTSETTINGS_TYPE_INFO);
    registry.register_type(INPUTSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTINPUTREMOTEPADCHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTINPUTSETTINGSREFRESHMESSAGE_TYPE_INFO);
    registry.register_type(CLIENTINPUTUNCHANGEDINPUTMESSAGE_TYPE_INFO);
    registry.register_type(INPUTMESSAGESRESETJOYSTICKSIMVALUESMESSAGE_TYPE_INFO);
    registry.register_type(INPUTMESSAGESKEYBOARDLAYOUTCHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(INPUTMESSAGESSINGLEINPUTEVENTMESSAGE_TYPE_INFO);
    registry.register_type(INPUTGRAPH_TYPE_INFO);
    registry.register_type(INPUTGRAPH_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYINPUTACTIONNODEPORTS_TYPE_INFO);
    registry.register_type(ENTRYINPUTACTIONNODEPORTS_ARRAY_TYPE_INFO);
    registry.register_type(INPUTCONCEPTNODEPORTS_TYPE_INFO);
    registry.register_type(INPUTCONCEPTNODEPORTS_ARRAY_TYPE_INFO);
    registry.register_type(INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGS_TYPE_INFO);
    registry.register_type(INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGS_ARRAY_TYPE_INFO);
    registry.register_type(INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGSTRUCT_TYPE_INFO);
    registry.register_type(INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGSTRUCT_ARRAY_TYPE_INFO);
    registry.register_type(INPUTDEVICEKEYS_TYPE_INFO);
    registry.register_type(INPUTDEVICEKEYS_ARRAY_TYPE_INFO);
    registry.register_type(CURSORSTYLE_TYPE_INFO);
    registry.register_type(CURSORSTYLE_ARRAY_TYPE_INFO);
    registry.register_type(INPUTDEVICEMOUSEBUTTONS_TYPE_INFO);
    registry.register_type(INPUTDEVICEMOUSEBUTTONS_ARRAY_TYPE_INFO);
    registry.register_type(INPUTDEVICEMOTIONCONTROLLERBUTTONS_TYPE_INFO);
    registry.register_type(INPUTDEVICEMOTIONCONTROLLERBUTTONS_ARRAY_TYPE_INFO);
    registry.register_type(INPUTDEVICEWHEELBUTTONS_TYPE_INFO);
    registry.register_type(INPUTDEVICEWHEELBUTTONS_ARRAY_TYPE_INFO);
    registry.register_type(INPUTDEVICEPADBUTTONS_TYPE_INFO);
    registry.register_type(INPUTDEVICEPADBUTTONS_ARRAY_TYPE_INFO);
    registry.register_type(INPUTDEVICEAXES_TYPE_INFO);
    registry.register_type(INPUTDEVICEAXES_ARRAY_TYPE_INFO);
    registry.register_type(INPUTCONFIGURATIONASSET_TYPE_INFO);
    registry.register_type(INPUTCONFIGURATIONASSET_ARRAY_TYPE_INFO);
    registry.register_type(EDITABLEACTIONMAP_TYPE_INFO);
    registry.register_type(EDITABLEACTIONMAP_ARRAY_TYPE_INFO);
    registry.register_type(INPUTACTIONMAPSDATA_TYPE_INFO);
    registry.register_type(INPUTACTIONMAPSDATA_ARRAY_TYPE_INFO);
    registry.register_type(INPUTACTIONMAPDATA_TYPE_INFO);
    registry.register_type(INPUTACTIONMAPDATA_ARRAY_TYPE_INFO);
    registry.register_type(INPUTACTIONSDATA_TYPE_INFO);
    registry.register_type(INPUTACTIONSDATA_ARRAY_TYPE_INFO);
    registry.register_type(MESSAGEINPUTACTIONDATA_TYPE_INFO);
    registry.register_type(MESSAGEINPUTACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOUSEINPUTACTIONDATA_TYPE_INFO);
    registry.register_type(MOUSEINPUTACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(KEYBOARDINPUTACTIONDATA_TYPE_INFO);
    registry.register_type(KEYBOARDINPUTACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(WHEELINPUTACTIONDATA_TYPE_INFO);
    registry.register_type(WHEELINPUTACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(VRINPUTACTIONDATA_TYPE_INFO);
    registry.register_type(VRINPUTACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOTIONCONTROLLERINPUTACTIONDATA_TYPE_INFO);
    registry.register_type(MOTIONCONTROLLERINPUTACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(JOYSTICKINPUTACTIONDATA_TYPE_INFO);
    registry.register_type(JOYSTICKINPUTACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(PADINPUTACTIONDATA_TYPE_INFO);
    registry.register_type(PADINPUTACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(AXESINPUTACTIONDATA_TYPE_INFO);
    registry.register_type(AXESINPUTACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(INPUTACTIONDATA_TYPE_INFO);
    registry.register_type(INPUTACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(INPUTACTIONMAPSLOT_TYPE_INFO);
    registry.register_type(INPUTACTIONMAPSLOT_ARRAY_TYPE_INFO);
    registry.register_type(INPUTACTIONMAPPLATFORM_TYPE_INFO);
    registry.register_type(INPUTACTIONMAPPLATFORM_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYINPUTACTIONBINDINGSDATA_TYPE_INFO);
    registry.register_type(ENTRYINPUTACTIONBINDINGSDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYINPUTACTIONBINDING_TYPE_INFO);
    registry.register_type(ENTRYINPUTACTIONBINDING_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYINPUTACTIONTYPE_TYPE_INFO);
    registry.register_type(ENTRYINPUTACTIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(ENTRYINPUTACTIONINDEXPAIR_TYPE_INFO);
    registry.register_type(ENTRYINPUTACTIONINDEXPAIR_ARRAY_TYPE_INFO);
    registry.register_type(BASEINPUTSETTINGS_TYPE_INFO);
    registry.register_type(BASEINPUTSETTINGS_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct InputSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub input_configuration_asset: Option<Arc<Mutex<dyn InputConfigurationAssetTrait>>>,
    pub input_enable: bool,
    pub use_raw_gamepad_input: bool,
    pub vr_device_type: u32,
    pub invert_pitch: bool,
    pub invert_pad_pc_right_stick: bool,
    pub invert_yaw: bool,
    pub console_input_emulation: bool,
    pub pads_rumble_enabled: Vec<bool>,
    pub pads_index: Vec<u32>,
    pub xenon_gamepad_dead_zone_center: f32,
    pub xenon_gamepad_dead_zone_axis: f32,
    pub xenon_gamepad_dead_zone_offset_axis: f32,
    pub p_s3_gamepad_dead_zone_center: f32,
    pub p_s3_gamepad_dead_zone_axis: f32,
    pub p_s3_gamepad_dead_zone_offset_axis: f32,
    pub p_c_gamepad_dead_zone_center: f32,
    pub p_c_gamepad_dead_zone_axis: f32,
    pub p_c_gamepad_dead_zone_offset_axis: f32,
    pub gen4a_gamepad_dead_zone_center: f32,
    pub gen4a_gamepad_dead_zone_axis: f32,
    pub gen4a_gamepad_dead_zone_offset_axis: f32,
    pub gen4b_gamepad_dead_zone_center: f32,
    pub gen4b_gamepad_dead_zone_axis: f32,
    pub gen4b_gamepad_dead_zone_offset_axis: f32,
    pub use_mouse_and_keyboard_system: bool,
    pub use_global_game_pad_input: bool,
    pub use_pc_game_pad_input: bool,
    pub gamepad_guid: String,
    pub mouse_sensitivity_min: f32,
    pub mouse_sensitivity_slider_range: f32,
    pub mouse_sensitivity_factor: f32,
    pub mouse_sensitivity_power: f32,
    pub invert_free_camera: bool,
    pub thread_enable: bool,
    pub thread_poll_frequency: u32,
}

pub trait InputSettingsTrait: super::core::SystemSettingsTrait {
    fn input_configuration_asset(&self) -> &Option<Arc<Mutex<dyn InputConfigurationAssetTrait>>>;
    fn input_configuration_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputConfigurationAssetTrait>>>;
    fn input_enable(&self) -> &bool;
    fn input_enable_mut(&mut self) -> &mut bool;
    fn use_raw_gamepad_input(&self) -> &bool;
    fn use_raw_gamepad_input_mut(&mut self) -> &mut bool;
    fn vr_device_type(&self) -> &u32;
    fn vr_device_type_mut(&mut self) -> &mut u32;
    fn invert_pitch(&self) -> &bool;
    fn invert_pitch_mut(&mut self) -> &mut bool;
    fn invert_pad_pc_right_stick(&self) -> &bool;
    fn invert_pad_pc_right_stick_mut(&mut self) -> &mut bool;
    fn invert_yaw(&self) -> &bool;
    fn invert_yaw_mut(&mut self) -> &mut bool;
    fn console_input_emulation(&self) -> &bool;
    fn console_input_emulation_mut(&mut self) -> &mut bool;
    fn pads_rumble_enabled(&self) -> &Vec<bool>;
    fn pads_rumble_enabled_mut(&mut self) -> &mut Vec<bool>;
    fn pads_index(&self) -> &Vec<u32>;
    fn pads_index_mut(&mut self) -> &mut Vec<u32>;
    fn xenon_gamepad_dead_zone_center(&self) -> &f32;
    fn xenon_gamepad_dead_zone_center_mut(&mut self) -> &mut f32;
    fn xenon_gamepad_dead_zone_axis(&self) -> &f32;
    fn xenon_gamepad_dead_zone_axis_mut(&mut self) -> &mut f32;
    fn xenon_gamepad_dead_zone_offset_axis(&self) -> &f32;
    fn xenon_gamepad_dead_zone_offset_axis_mut(&mut self) -> &mut f32;
    fn p_s3_gamepad_dead_zone_center(&self) -> &f32;
    fn p_s3_gamepad_dead_zone_center_mut(&mut self) -> &mut f32;
    fn p_s3_gamepad_dead_zone_axis(&self) -> &f32;
    fn p_s3_gamepad_dead_zone_axis_mut(&mut self) -> &mut f32;
    fn p_s3_gamepad_dead_zone_offset_axis(&self) -> &f32;
    fn p_s3_gamepad_dead_zone_offset_axis_mut(&mut self) -> &mut f32;
    fn p_c_gamepad_dead_zone_center(&self) -> &f32;
    fn p_c_gamepad_dead_zone_center_mut(&mut self) -> &mut f32;
    fn p_c_gamepad_dead_zone_axis(&self) -> &f32;
    fn p_c_gamepad_dead_zone_axis_mut(&mut self) -> &mut f32;
    fn p_c_gamepad_dead_zone_offset_axis(&self) -> &f32;
    fn p_c_gamepad_dead_zone_offset_axis_mut(&mut self) -> &mut f32;
    fn gen4a_gamepad_dead_zone_center(&self) -> &f32;
    fn gen4a_gamepad_dead_zone_center_mut(&mut self) -> &mut f32;
    fn gen4a_gamepad_dead_zone_axis(&self) -> &f32;
    fn gen4a_gamepad_dead_zone_axis_mut(&mut self) -> &mut f32;
    fn gen4a_gamepad_dead_zone_offset_axis(&self) -> &f32;
    fn gen4a_gamepad_dead_zone_offset_axis_mut(&mut self) -> &mut f32;
    fn gen4b_gamepad_dead_zone_center(&self) -> &f32;
    fn gen4b_gamepad_dead_zone_center_mut(&mut self) -> &mut f32;
    fn gen4b_gamepad_dead_zone_axis(&self) -> &f32;
    fn gen4b_gamepad_dead_zone_axis_mut(&mut self) -> &mut f32;
    fn gen4b_gamepad_dead_zone_offset_axis(&self) -> &f32;
    fn gen4b_gamepad_dead_zone_offset_axis_mut(&mut self) -> &mut f32;
    fn use_mouse_and_keyboard_system(&self) -> &bool;
    fn use_mouse_and_keyboard_system_mut(&mut self) -> &mut bool;
    fn use_global_game_pad_input(&self) -> &bool;
    fn use_global_game_pad_input_mut(&mut self) -> &mut bool;
    fn use_pc_game_pad_input(&self) -> &bool;
    fn use_pc_game_pad_input_mut(&mut self) -> &mut bool;
    fn gamepad_guid(&self) -> &String;
    fn gamepad_guid_mut(&mut self) -> &mut String;
    fn mouse_sensitivity_min(&self) -> &f32;
    fn mouse_sensitivity_min_mut(&mut self) -> &mut f32;
    fn mouse_sensitivity_slider_range(&self) -> &f32;
    fn mouse_sensitivity_slider_range_mut(&mut self) -> &mut f32;
    fn mouse_sensitivity_factor(&self) -> &f32;
    fn mouse_sensitivity_factor_mut(&mut self) -> &mut f32;
    fn mouse_sensitivity_power(&self) -> &f32;
    fn mouse_sensitivity_power_mut(&mut self) -> &mut f32;
    fn invert_free_camera(&self) -> &bool;
    fn invert_free_camera_mut(&mut self) -> &mut bool;
    fn thread_enable(&self) -> &bool;
    fn thread_enable_mut(&mut self) -> &mut bool;
    fn thread_poll_frequency(&self) -> &u32;
    fn thread_poll_frequency_mut(&mut self) -> &mut u32;
}

impl InputSettingsTrait for InputSettings {
    fn input_configuration_asset(&self) -> &Option<Arc<Mutex<dyn InputConfigurationAssetTrait>>> {
        &self.input_configuration_asset
    }
    fn input_configuration_asset_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputConfigurationAssetTrait>>> {
        &mut self.input_configuration_asset
    }
    fn input_enable(&self) -> &bool {
        &self.input_enable
    }
    fn input_enable_mut(&mut self) -> &mut bool {
        &mut self.input_enable
    }
    fn use_raw_gamepad_input(&self) -> &bool {
        &self.use_raw_gamepad_input
    }
    fn use_raw_gamepad_input_mut(&mut self) -> &mut bool {
        &mut self.use_raw_gamepad_input
    }
    fn vr_device_type(&self) -> &u32 {
        &self.vr_device_type
    }
    fn vr_device_type_mut(&mut self) -> &mut u32 {
        &mut self.vr_device_type
    }
    fn invert_pitch(&self) -> &bool {
        &self.invert_pitch
    }
    fn invert_pitch_mut(&mut self) -> &mut bool {
        &mut self.invert_pitch
    }
    fn invert_pad_pc_right_stick(&self) -> &bool {
        &self.invert_pad_pc_right_stick
    }
    fn invert_pad_pc_right_stick_mut(&mut self) -> &mut bool {
        &mut self.invert_pad_pc_right_stick
    }
    fn invert_yaw(&self) -> &bool {
        &self.invert_yaw
    }
    fn invert_yaw_mut(&mut self) -> &mut bool {
        &mut self.invert_yaw
    }
    fn console_input_emulation(&self) -> &bool {
        &self.console_input_emulation
    }
    fn console_input_emulation_mut(&mut self) -> &mut bool {
        &mut self.console_input_emulation
    }
    fn pads_rumble_enabled(&self) -> &Vec<bool> {
        &self.pads_rumble_enabled
    }
    fn pads_rumble_enabled_mut(&mut self) -> &mut Vec<bool> {
        &mut self.pads_rumble_enabled
    }
    fn pads_index(&self) -> &Vec<u32> {
        &self.pads_index
    }
    fn pads_index_mut(&mut self) -> &mut Vec<u32> {
        &mut self.pads_index
    }
    fn xenon_gamepad_dead_zone_center(&self) -> &f32 {
        &self.xenon_gamepad_dead_zone_center
    }
    fn xenon_gamepad_dead_zone_center_mut(&mut self) -> &mut f32 {
        &mut self.xenon_gamepad_dead_zone_center
    }
    fn xenon_gamepad_dead_zone_axis(&self) -> &f32 {
        &self.xenon_gamepad_dead_zone_axis
    }
    fn xenon_gamepad_dead_zone_axis_mut(&mut self) -> &mut f32 {
        &mut self.xenon_gamepad_dead_zone_axis
    }
    fn xenon_gamepad_dead_zone_offset_axis(&self) -> &f32 {
        &self.xenon_gamepad_dead_zone_offset_axis
    }
    fn xenon_gamepad_dead_zone_offset_axis_mut(&mut self) -> &mut f32 {
        &mut self.xenon_gamepad_dead_zone_offset_axis
    }
    fn p_s3_gamepad_dead_zone_center(&self) -> &f32 {
        &self.p_s3_gamepad_dead_zone_center
    }
    fn p_s3_gamepad_dead_zone_center_mut(&mut self) -> &mut f32 {
        &mut self.p_s3_gamepad_dead_zone_center
    }
    fn p_s3_gamepad_dead_zone_axis(&self) -> &f32 {
        &self.p_s3_gamepad_dead_zone_axis
    }
    fn p_s3_gamepad_dead_zone_axis_mut(&mut self) -> &mut f32 {
        &mut self.p_s3_gamepad_dead_zone_axis
    }
    fn p_s3_gamepad_dead_zone_offset_axis(&self) -> &f32 {
        &self.p_s3_gamepad_dead_zone_offset_axis
    }
    fn p_s3_gamepad_dead_zone_offset_axis_mut(&mut self) -> &mut f32 {
        &mut self.p_s3_gamepad_dead_zone_offset_axis
    }
    fn p_c_gamepad_dead_zone_center(&self) -> &f32 {
        &self.p_c_gamepad_dead_zone_center
    }
    fn p_c_gamepad_dead_zone_center_mut(&mut self) -> &mut f32 {
        &mut self.p_c_gamepad_dead_zone_center
    }
    fn p_c_gamepad_dead_zone_axis(&self) -> &f32 {
        &self.p_c_gamepad_dead_zone_axis
    }
    fn p_c_gamepad_dead_zone_axis_mut(&mut self) -> &mut f32 {
        &mut self.p_c_gamepad_dead_zone_axis
    }
    fn p_c_gamepad_dead_zone_offset_axis(&self) -> &f32 {
        &self.p_c_gamepad_dead_zone_offset_axis
    }
    fn p_c_gamepad_dead_zone_offset_axis_mut(&mut self) -> &mut f32 {
        &mut self.p_c_gamepad_dead_zone_offset_axis
    }
    fn gen4a_gamepad_dead_zone_center(&self) -> &f32 {
        &self.gen4a_gamepad_dead_zone_center
    }
    fn gen4a_gamepad_dead_zone_center_mut(&mut self) -> &mut f32 {
        &mut self.gen4a_gamepad_dead_zone_center
    }
    fn gen4a_gamepad_dead_zone_axis(&self) -> &f32 {
        &self.gen4a_gamepad_dead_zone_axis
    }
    fn gen4a_gamepad_dead_zone_axis_mut(&mut self) -> &mut f32 {
        &mut self.gen4a_gamepad_dead_zone_axis
    }
    fn gen4a_gamepad_dead_zone_offset_axis(&self) -> &f32 {
        &self.gen4a_gamepad_dead_zone_offset_axis
    }
    fn gen4a_gamepad_dead_zone_offset_axis_mut(&mut self) -> &mut f32 {
        &mut self.gen4a_gamepad_dead_zone_offset_axis
    }
    fn gen4b_gamepad_dead_zone_center(&self) -> &f32 {
        &self.gen4b_gamepad_dead_zone_center
    }
    fn gen4b_gamepad_dead_zone_center_mut(&mut self) -> &mut f32 {
        &mut self.gen4b_gamepad_dead_zone_center
    }
    fn gen4b_gamepad_dead_zone_axis(&self) -> &f32 {
        &self.gen4b_gamepad_dead_zone_axis
    }
    fn gen4b_gamepad_dead_zone_axis_mut(&mut self) -> &mut f32 {
        &mut self.gen4b_gamepad_dead_zone_axis
    }
    fn gen4b_gamepad_dead_zone_offset_axis(&self) -> &f32 {
        &self.gen4b_gamepad_dead_zone_offset_axis
    }
    fn gen4b_gamepad_dead_zone_offset_axis_mut(&mut self) -> &mut f32 {
        &mut self.gen4b_gamepad_dead_zone_offset_axis
    }
    fn use_mouse_and_keyboard_system(&self) -> &bool {
        &self.use_mouse_and_keyboard_system
    }
    fn use_mouse_and_keyboard_system_mut(&mut self) -> &mut bool {
        &mut self.use_mouse_and_keyboard_system
    }
    fn use_global_game_pad_input(&self) -> &bool {
        &self.use_global_game_pad_input
    }
    fn use_global_game_pad_input_mut(&mut self) -> &mut bool {
        &mut self.use_global_game_pad_input
    }
    fn use_pc_game_pad_input(&self) -> &bool {
        &self.use_pc_game_pad_input
    }
    fn use_pc_game_pad_input_mut(&mut self) -> &mut bool {
        &mut self.use_pc_game_pad_input
    }
    fn gamepad_guid(&self) -> &String {
        &self.gamepad_guid
    }
    fn gamepad_guid_mut(&mut self) -> &mut String {
        &mut self.gamepad_guid
    }
    fn mouse_sensitivity_min(&self) -> &f32 {
        &self.mouse_sensitivity_min
    }
    fn mouse_sensitivity_min_mut(&mut self) -> &mut f32 {
        &mut self.mouse_sensitivity_min
    }
    fn mouse_sensitivity_slider_range(&self) -> &f32 {
        &self.mouse_sensitivity_slider_range
    }
    fn mouse_sensitivity_slider_range_mut(&mut self) -> &mut f32 {
        &mut self.mouse_sensitivity_slider_range
    }
    fn mouse_sensitivity_factor(&self) -> &f32 {
        &self.mouse_sensitivity_factor
    }
    fn mouse_sensitivity_factor_mut(&mut self) -> &mut f32 {
        &mut self.mouse_sensitivity_factor
    }
    fn mouse_sensitivity_power(&self) -> &f32 {
        &self.mouse_sensitivity_power
    }
    fn mouse_sensitivity_power_mut(&mut self) -> &mut f32 {
        &mut self.mouse_sensitivity_power
    }
    fn invert_free_camera(&self) -> &bool {
        &self.invert_free_camera
    }
    fn invert_free_camera_mut(&mut self) -> &mut bool {
        &mut self.invert_free_camera
    }
    fn thread_enable(&self) -> &bool {
        &self.thread_enable
    }
    fn thread_enable_mut(&mut self) -> &mut bool {
        &mut self.thread_enable
    }
    fn thread_poll_frequency(&self) -> &u32 {
        &self.thread_poll_frequency
    }
    fn thread_poll_frequency_mut(&mut self) -> &mut u32 {
        &mut self.thread_poll_frequency
    }
}

impl super::core::SystemSettingsTrait for InputSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for InputSettings {
}

pub static INPUTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSettings",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InputConfigurationAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "InputConfigurationAsset",
                rust_offset: offset_of!(InputSettings, input_configuration_asset),
            },
            FieldInfoData {
                name: "InputEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputSettings, input_enable),
            },
            FieldInfoData {
                name: "UseRawGamepadInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputSettings, use_raw_gamepad_input),
            },
            FieldInfoData {
                name: "VrDeviceType",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(InputSettings, vr_device_type),
            },
            FieldInfoData {
                name: "InvertPitch",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputSettings, invert_pitch),
            },
            FieldInfoData {
                name: "InvertPadPcRightStick",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputSettings, invert_pad_pc_right_stick),
            },
            FieldInfoData {
                name: "InvertYaw",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputSettings, invert_yaw),
            },
            FieldInfoData {
                name: "ConsoleInputEmulation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputSettings, console_input_emulation),
            },
            FieldInfoData {
                name: "PadsRumbleEnabled",
                flags: MemberInfoFlags::new(144),
                field_type: "Boolean-Array",
                rust_offset: offset_of!(InputSettings, pads_rumble_enabled),
            },
            FieldInfoData {
                name: "PadsIndex",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(InputSettings, pads_index),
            },
            FieldInfoData {
                name: "XenonGamepadDeadZoneCenter",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, xenon_gamepad_dead_zone_center),
            },
            FieldInfoData {
                name: "XenonGamepadDeadZoneAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, xenon_gamepad_dead_zone_axis),
            },
            FieldInfoData {
                name: "XenonGamepadDeadZoneOffsetAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, xenon_gamepad_dead_zone_offset_axis),
            },
            FieldInfoData {
                name: "PS3GamepadDeadZoneCenter",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, p_s3_gamepad_dead_zone_center),
            },
            FieldInfoData {
                name: "PS3GamepadDeadZoneAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, p_s3_gamepad_dead_zone_axis),
            },
            FieldInfoData {
                name: "PS3GamepadDeadZoneOffsetAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, p_s3_gamepad_dead_zone_offset_axis),
            },
            FieldInfoData {
                name: "PCGamepadDeadZoneCenter",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, p_c_gamepad_dead_zone_center),
            },
            FieldInfoData {
                name: "PCGamepadDeadZoneAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, p_c_gamepad_dead_zone_axis),
            },
            FieldInfoData {
                name: "PCGamepadDeadZoneOffsetAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, p_c_gamepad_dead_zone_offset_axis),
            },
            FieldInfoData {
                name: "Gen4aGamepadDeadZoneCenter",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, gen4a_gamepad_dead_zone_center),
            },
            FieldInfoData {
                name: "Gen4aGamepadDeadZoneAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, gen4a_gamepad_dead_zone_axis),
            },
            FieldInfoData {
                name: "Gen4aGamepadDeadZoneOffsetAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, gen4a_gamepad_dead_zone_offset_axis),
            },
            FieldInfoData {
                name: "Gen4bGamepadDeadZoneCenter",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, gen4b_gamepad_dead_zone_center),
            },
            FieldInfoData {
                name: "Gen4bGamepadDeadZoneAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, gen4b_gamepad_dead_zone_axis),
            },
            FieldInfoData {
                name: "Gen4bGamepadDeadZoneOffsetAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, gen4b_gamepad_dead_zone_offset_axis),
            },
            FieldInfoData {
                name: "UseMouseAndKeyboardSystem",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputSettings, use_mouse_and_keyboard_system),
            },
            FieldInfoData {
                name: "UseGlobalGamePadInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputSettings, use_global_game_pad_input),
            },
            FieldInfoData {
                name: "UsePcGamePadInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputSettings, use_pc_game_pad_input),
            },
            FieldInfoData {
                name: "GamepadGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(InputSettings, gamepad_guid),
            },
            FieldInfoData {
                name: "MouseSensitivityMin",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, mouse_sensitivity_min),
            },
            FieldInfoData {
                name: "MouseSensitivitySliderRange",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, mouse_sensitivity_slider_range),
            },
            FieldInfoData {
                name: "MouseSensitivityFactor",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, mouse_sensitivity_factor),
            },
            FieldInfoData {
                name: "MouseSensitivityPower",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(InputSettings, mouse_sensitivity_power),
            },
            FieldInfoData {
                name: "InvertFreeCamera",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputSettings, invert_free_camera),
            },
            FieldInfoData {
                name: "ThreadEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputSettings, thread_enable),
            },
            FieldInfoData {
                name: "ThreadPollFrequency",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(InputSettings, thread_poll_frequency),
            },
        ],
    }),
    array_type: Some(INPUTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputSettings {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INPUTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientInputRemotePadChangedMessage {
}

pub trait ClientInputRemotePadChangedMessageTrait: TypeObject {
}

impl ClientInputRemotePadChangedMessageTrait for ClientInputRemotePadChangedMessage {
}

pub static CLIENTINPUTREMOTEPADCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientInputRemotePadChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientInputRemotePadChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientInputRemotePadChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTINPUTREMOTEPADCHANGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientInputSettingsRefreshMessage {
}

pub trait ClientInputSettingsRefreshMessageTrait: TypeObject {
}

impl ClientInputSettingsRefreshMessageTrait for ClientInputSettingsRefreshMessage {
}

pub static CLIENTINPUTSETTINGSREFRESHMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientInputSettingsRefreshMessage",
    flags: MemberInfoFlags::new(36937),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientInputSettingsRefreshMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientInputSettingsRefreshMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTINPUTSETTINGSREFRESHMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClientInputUnchangedInputMessage {
}

pub trait ClientInputUnchangedInputMessageTrait: TypeObject {
}

impl ClientInputUnchangedInputMessageTrait for ClientInputUnchangedInputMessage {
}

pub static CLIENTINPUTUNCHANGEDINPUTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientInputUnchangedInputMessage",
    flags: MemberInfoFlags::new(36937),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientInputUnchangedInputMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ClientInputUnchangedInputMessage {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTINPUTUNCHANGEDINPUTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct InputMessagesResetJoystickSimValuesMessage {
}

pub trait InputMessagesResetJoystickSimValuesMessageTrait: TypeObject {
}

impl InputMessagesResetJoystickSimValuesMessageTrait for InputMessagesResetJoystickSimValuesMessage {
}

pub static INPUTMESSAGESRESETJOYSTICKSIMVALUESMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputMessagesResetJoystickSimValuesMessage",
    flags: MemberInfoFlags::new(36937),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputMessagesResetJoystickSimValuesMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for InputMessagesResetJoystickSimValuesMessage {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTMESSAGESRESETJOYSTICKSIMVALUESMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct InputMessagesKeyboardLayoutChangedMessage {
}

pub trait InputMessagesKeyboardLayoutChangedMessageTrait: TypeObject {
}

impl InputMessagesKeyboardLayoutChangedMessageTrait for InputMessagesKeyboardLayoutChangedMessage {
}

pub static INPUTMESSAGESKEYBOARDLAYOUTCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputMessagesKeyboardLayoutChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputMessagesKeyboardLayoutChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for InputMessagesKeyboardLayoutChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTMESSAGESKEYBOARDLAYOUTCHANGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct InputMessagesSingleInputEventMessage {
}

pub trait InputMessagesSingleInputEventMessageTrait: TypeObject {
}

impl InputMessagesSingleInputEventMessageTrait for InputMessagesSingleInputEventMessage {
}

pub static INPUTMESSAGESSINGLEINPUTEVENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputMessagesSingleInputEventMessage",
    flags: MemberInfoFlags::new(36937),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputMessagesSingleInputEventMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for InputMessagesSingleInputEventMessage {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTMESSAGESSINGLEINPUTEVENTMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct InputGraph {
    pub _glacier_base: super::expression::ExpressionFunctionTypeInfoAsset,
    pub input_action_maps: Option<Arc<Mutex<dyn InputActionMapsDataTrait>>>,
    pub combined_input_concept_to_entry_input_action_mappings: Option<Arc<Mutex<dyn InputConceptToEntryInputActionMappingsTrait>>>,
    pub concept_ports: Vec<InputConceptNodePorts>,
    pub action_ports: Vec<EntryInputActionNodePorts>,
    pub player_id_port: u32,
}

pub trait InputGraphTrait: super::expression::ExpressionFunctionTypeInfoAssetTrait {
    fn input_action_maps(&self) -> &Option<Arc<Mutex<dyn InputActionMapsDataTrait>>>;
    fn input_action_maps_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputActionMapsDataTrait>>>;
    fn combined_input_concept_to_entry_input_action_mappings(&self) -> &Option<Arc<Mutex<dyn InputConceptToEntryInputActionMappingsTrait>>>;
    fn combined_input_concept_to_entry_input_action_mappings_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputConceptToEntryInputActionMappingsTrait>>>;
    fn concept_ports(&self) -> &Vec<InputConceptNodePorts>;
    fn concept_ports_mut(&mut self) -> &mut Vec<InputConceptNodePorts>;
    fn action_ports(&self) -> &Vec<EntryInputActionNodePorts>;
    fn action_ports_mut(&mut self) -> &mut Vec<EntryInputActionNodePorts>;
    fn player_id_port(&self) -> &u32;
    fn player_id_port_mut(&mut self) -> &mut u32;
}

impl InputGraphTrait for InputGraph {
    fn input_action_maps(&self) -> &Option<Arc<Mutex<dyn InputActionMapsDataTrait>>> {
        &self.input_action_maps
    }
    fn input_action_maps_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputActionMapsDataTrait>>> {
        &mut self.input_action_maps
    }
    fn combined_input_concept_to_entry_input_action_mappings(&self) -> &Option<Arc<Mutex<dyn InputConceptToEntryInputActionMappingsTrait>>> {
        &self.combined_input_concept_to_entry_input_action_mappings
    }
    fn combined_input_concept_to_entry_input_action_mappings_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputConceptToEntryInputActionMappingsTrait>>> {
        &mut self.combined_input_concept_to_entry_input_action_mappings
    }
    fn concept_ports(&self) -> &Vec<InputConceptNodePorts> {
        &self.concept_ports
    }
    fn concept_ports_mut(&mut self) -> &mut Vec<InputConceptNodePorts> {
        &mut self.concept_ports
    }
    fn action_ports(&self) -> &Vec<EntryInputActionNodePorts> {
        &self.action_ports
    }
    fn action_ports_mut(&mut self) -> &mut Vec<EntryInputActionNodePorts> {
        &mut self.action_ports
    }
    fn player_id_port(&self) -> &u32 {
        &self.player_id_port
    }
    fn player_id_port_mut(&mut self) -> &mut u32 {
        &mut self.player_id_port
    }
}

impl super::expression::ExpressionFunctionTypeInfoAssetTrait for InputGraph {
    fn graph_data(&self) -> &Option<Arc<Mutex<dyn super::expression::ExpressionNodeGraphDataTrait>>> {
        self._glacier_base.graph_data()
    }
    fn graph_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::expression::ExpressionNodeGraphDataTrait>>> {
        self._glacier_base.graph_data_mut()
    }
}

impl super::core::FunctionTypeInfoAssetTrait for InputGraph {
    fn parameters(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::TypeInfoParameterDataContainerTrait>>>> {
        self._glacier_base.parameters()
    }
    fn parameters_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::core::TypeInfoParameterDataContainerTrait>>>> {
        self._glacier_base.parameters_mut()
    }
    fn owner(&self) -> &Option<Arc<Mutex<dyn super::core::ClassInfoAssetTrait>>> {
        self._glacier_base.owner()
    }
    fn owner_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::ClassInfoAssetTrait>>> {
        self._glacier_base.owner_mut()
    }
}

impl super::core::TypeInfoAssetTrait for InputGraph {
    fn module_name(&self) -> &String {
        self._glacier_base.module_name()
    }
    fn module_name_mut(&mut self) -> &mut String {
        self._glacier_base.module_name_mut()
    }
    fn type_name(&self) -> &String {
        self._glacier_base.type_name()
    }
    fn type_name_mut(&mut self) -> &mut String {
        self._glacier_base.type_name_mut()
    }
    fn is_meta(&self) -> &bool {
        self._glacier_base.is_meta()
    }
    fn is_meta_mut(&mut self) -> &mut bool {
        self._glacier_base.is_meta_mut()
    }
    fn attributes(&self) -> &Vec<Option<Arc<Mutex<dyn super::core::TypeInfoAttributeTrait>>>> {
        self._glacier_base.attributes()
    }
    fn attributes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::core::TypeInfoAttributeTrait>>>> {
        self._glacier_base.attributes_mut()
    }
    fn is_native(&self) -> &bool {
        self._glacier_base.is_native()
    }
    fn is_native_mut(&mut self) -> &mut bool {
        self._glacier_base.is_native_mut()
    }
}

impl super::core::AssetTrait for InputGraph {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for InputGraph {
}

pub static INPUTGRAPH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputGraph",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::expression::EXPRESSIONFUNCTIONTYPEINFOASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputGraph as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InputActionMaps",
                flags: MemberInfoFlags::new(0),
                field_type: "InputActionMapsData",
                rust_offset: offset_of!(InputGraph, input_action_maps),
            },
            FieldInfoData {
                name: "CombinedInputConceptToEntryInputActionMappings",
                flags: MemberInfoFlags::new(0),
                field_type: "InputConceptToEntryInputActionMappings",
                rust_offset: offset_of!(InputGraph, combined_input_concept_to_entry_input_action_mappings),
            },
            FieldInfoData {
                name: "ConceptPorts",
                flags: MemberInfoFlags::new(144),
                field_type: "InputConceptNodePorts-Array",
                rust_offset: offset_of!(InputGraph, concept_ports),
            },
            FieldInfoData {
                name: "ActionPorts",
                flags: MemberInfoFlags::new(144),
                field_type: "EntryInputActionNodePorts-Array",
                rust_offset: offset_of!(InputGraph, action_ports),
            },
            FieldInfoData {
                name: "PlayerIdPort",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(InputGraph, player_id_port),
            },
        ],
    }),
    array_type: Some(INPUTGRAPH_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputGraph {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTGRAPH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INPUTGRAPH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputGraph-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputGraph"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EntryInputActionNodePorts {
    pub action: i32,
    pub level_port: u32,
    pub is_down_port: u32,
}

pub trait EntryInputActionNodePortsTrait: TypeObject {
    fn action(&self) -> &i32;
    fn action_mut(&mut self) -> &mut i32;
    fn level_port(&self) -> &u32;
    fn level_port_mut(&mut self) -> &mut u32;
    fn is_down_port(&self) -> &u32;
    fn is_down_port_mut(&mut self) -> &mut u32;
}

impl EntryInputActionNodePortsTrait for EntryInputActionNodePorts {
    fn action(&self) -> &i32 {
        &self.action
    }
    fn action_mut(&mut self) -> &mut i32 {
        &mut self.action
    }
    fn level_port(&self) -> &u32 {
        &self.level_port
    }
    fn level_port_mut(&mut self) -> &mut u32 {
        &mut self.level_port
    }
    fn is_down_port(&self) -> &u32 {
        &self.is_down_port
    }
    fn is_down_port_mut(&mut self) -> &mut u32 {
        &mut self.is_down_port
    }
}

pub static ENTRYINPUTACTIONNODEPORTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryInputActionNodePorts",
    flags: MemberInfoFlags::new(36937),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntryInputActionNodePorts as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Action",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EntryInputActionNodePorts, action),
            },
            FieldInfoData {
                name: "LevelPort",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EntryInputActionNodePorts, level_port),
            },
            FieldInfoData {
                name: "IsDownPort",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EntryInputActionNodePorts, is_down_port),
            },
        ],
    }),
    array_type: Some(ENTRYINPUTACTIONNODEPORTS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EntryInputActionNodePorts {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYINPUTACTIONNODEPORTS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static ENTRYINPUTACTIONNODEPORTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryInputActionNodePorts-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("EntryInputActionNodePorts"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputConceptNodePorts {
    pub identifier: i32,
    pub level_port: u32,
    pub went_down_port: u32,
    pub is_down_port: u32,
    pub went_up_port: u32,
}

pub trait InputConceptNodePortsTrait: TypeObject {
    fn identifier(&self) -> &i32;
    fn identifier_mut(&mut self) -> &mut i32;
    fn level_port(&self) -> &u32;
    fn level_port_mut(&mut self) -> &mut u32;
    fn went_down_port(&self) -> &u32;
    fn went_down_port_mut(&mut self) -> &mut u32;
    fn is_down_port(&self) -> &u32;
    fn is_down_port_mut(&mut self) -> &mut u32;
    fn went_up_port(&self) -> &u32;
    fn went_up_port_mut(&mut self) -> &mut u32;
}

impl InputConceptNodePortsTrait for InputConceptNodePorts {
    fn identifier(&self) -> &i32 {
        &self.identifier
    }
    fn identifier_mut(&mut self) -> &mut i32 {
        &mut self.identifier
    }
    fn level_port(&self) -> &u32 {
        &self.level_port
    }
    fn level_port_mut(&mut self) -> &mut u32 {
        &mut self.level_port
    }
    fn went_down_port(&self) -> &u32 {
        &self.went_down_port
    }
    fn went_down_port_mut(&mut self) -> &mut u32 {
        &mut self.went_down_port
    }
    fn is_down_port(&self) -> &u32 {
        &self.is_down_port
    }
    fn is_down_port_mut(&mut self) -> &mut u32 {
        &mut self.is_down_port
    }
    fn went_up_port(&self) -> &u32 {
        &self.went_up_port
    }
    fn went_up_port_mut(&mut self) -> &mut u32 {
        &mut self.went_up_port
    }
}

pub static INPUTCONCEPTNODEPORTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConceptNodePorts",
    flags: MemberInfoFlags::new(36937),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputConceptNodePorts as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Identifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InputConceptNodePorts, identifier),
            },
            FieldInfoData {
                name: "LevelPort",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(InputConceptNodePorts, level_port),
            },
            FieldInfoData {
                name: "WentDownPort",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(InputConceptNodePorts, went_down_port),
            },
            FieldInfoData {
                name: "IsDownPort",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(InputConceptNodePorts, is_down_port),
            },
            FieldInfoData {
                name: "WentUpPort",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(InputConceptNodePorts, went_up_port),
            },
        ],
    }),
    array_type: Some(INPUTCONCEPTNODEPORTS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for InputConceptNodePorts {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTCONCEPTNODEPORTS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INPUTCONCEPTNODEPORTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConceptNodePorts-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputConceptNodePorts"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputConceptToEntryInputActionMappings {
    pub _glacier_base: super::core::DataContainer,
    pub mappings: Vec<InputConceptToEntryInputActionMappingStruct>,
}

pub trait InputConceptToEntryInputActionMappingsTrait: super::core::DataContainerTrait {
    fn mappings(&self) -> &Vec<InputConceptToEntryInputActionMappingStruct>;
    fn mappings_mut(&mut self) -> &mut Vec<InputConceptToEntryInputActionMappingStruct>;
}

impl InputConceptToEntryInputActionMappingsTrait for InputConceptToEntryInputActionMappings {
    fn mappings(&self) -> &Vec<InputConceptToEntryInputActionMappingStruct> {
        &self.mappings
    }
    fn mappings_mut(&mut self) -> &mut Vec<InputConceptToEntryInputActionMappingStruct> {
        &mut self.mappings
    }
}

impl super::core::DataContainerTrait for InputConceptToEntryInputActionMappings {
}

pub static INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConceptToEntryInputActionMappings",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputConceptToEntryInputActionMappings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Mappings",
                flags: MemberInfoFlags::new(144),
                field_type: "InputConceptToEntryInputActionMappingStruct-Array",
                rust_offset: offset_of!(InputConceptToEntryInputActionMappings, mappings),
            },
        ],
    }),
    array_type: Some(INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputConceptToEntryInputActionMappings {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConceptToEntryInputActionMappings-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputConceptToEntryInputActionMappings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputConceptToEntryInputActionMappingStruct {
    pub concept_identifier: i32,
    pub action_identifier: i32,
}

pub trait InputConceptToEntryInputActionMappingStructTrait: TypeObject {
    fn concept_identifier(&self) -> &i32;
    fn concept_identifier_mut(&mut self) -> &mut i32;
    fn action_identifier(&self) -> &i32;
    fn action_identifier_mut(&mut self) -> &mut i32;
}

impl InputConceptToEntryInputActionMappingStructTrait for InputConceptToEntryInputActionMappingStruct {
    fn concept_identifier(&self) -> &i32 {
        &self.concept_identifier
    }
    fn concept_identifier_mut(&mut self) -> &mut i32 {
        &mut self.concept_identifier
    }
    fn action_identifier(&self) -> &i32 {
        &self.action_identifier
    }
    fn action_identifier_mut(&mut self) -> &mut i32 {
        &mut self.action_identifier
    }
}

pub static INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGSTRUCT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConceptToEntryInputActionMappingStruct",
    flags: MemberInfoFlags::new(32841),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputConceptToEntryInputActionMappingStruct as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ConceptIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InputConceptToEntryInputActionMappingStruct, concept_identifier),
            },
            FieldInfoData {
                name: "ActionIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InputConceptToEntryInputActionMappingStruct, action_identifier),
            },
        ],
    }),
    array_type: Some(INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGSTRUCT_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for InputConceptToEntryInputActionMappingStruct {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGSTRUCT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INPUTCONCEPTTOENTRYINPUTACTIONMAPPINGSTRUCT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConceptToEntryInputActionMappingStruct-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputConceptToEntryInputActionMappingStruct"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InputDeviceKeys {
    #[default]
    IDK_None = 0,
    IDK_Escape = 1,
    IDK_1 = 2,
    IDK_2 = 3,
    IDK_3 = 4,
    IDK_4 = 5,
    IDK_5 = 6,
    IDK_6 = 7,
    IDK_7 = 8,
    IDK_8 = 9,
    IDK_9 = 10,
    IDK_0 = 11,
    IDK_Minus = 12,
    IDK_Equals = 13,
    IDK_Backspace = 14,
    IDK_Tab = 15,
    IDK_Q = 16,
    IDK_W = 17,
    IDK_E = 18,
    IDK_R = 19,
    IDK_T = 20,
    IDK_Y = 21,
    IDK_U = 22,
    IDK_I = 23,
    IDK_O = 24,
    IDK_P = 25,
    IDK_LeftBracket = 26,
    IDK_RightBracket = 27,
    IDK_Enter = 28,
    IDK_LeftCtrl = 29,
    IDK_A = 30,
    IDK_S = 31,
    IDK_D = 32,
    IDK_F = 33,
    IDK_G = 34,
    IDK_H = 35,
    IDK_J = 36,
    IDK_K = 37,
    IDK_L = 38,
    IDK_Semicolon = 39,
    IDK_Apostrophe = 40,
    IDK_Grave = 41,
    IDK_LeftShift = 42,
    IDK_Backslash = 43,
    IDK_Z = 44,
    IDK_X = 45,
    IDK_C = 46,
    IDK_V = 47,
    IDK_B = 48,
    IDK_N = 49,
    IDK_M = 50,
    IDK_Comma = 51,
    IDK_Period = 52,
    IDK_Slash = 53,
    IDK_RightShift = 54,
    IDK_Multiply = 55,
    IDK_LeftAlt = 56,
    IDK_Space = 57,
    IDK_Capital = 58,
    IDK_F1 = 59,
    IDK_F2 = 60,
    IDK_F3 = 61,
    IDK_F4 = 62,
    IDK_F5 = 63,
    IDK_F6 = 64,
    IDK_F7 = 65,
    IDK_F8 = 66,
    IDK_F9 = 67,
    IDK_F10 = 68,
    IDK_Numlock = 69,
    IDK_ScrollLock = 70,
    IDK_Numpad7 = 71,
    IDK_Numpad8 = 72,
    IDK_Numpad9 = 73,
    IDK_Subtract = 74,
    IDK_Numpad4 = 75,
    IDK_Numpad5 = 76,
    IDK_Numpad6 = 77,
    IDK_Add = 78,
    IDK_Numpad1 = 79,
    IDK_Numpad2 = 80,
    IDK_Numpad3 = 81,
    IDK_Numpad0 = 82,
    IDK_Decimal = 83,
    IDK_OEM_102 = 86,
    IDK_F11 = 87,
    IDK_F12 = 88,
    IDK_F13 = 100,
    IDK_F14 = 101,
    IDK_F15 = 102,
    IDK_Kana = 112,
    IDK_PTBRSlash = 115,
    IDK_Convert = 121,
    IDK_NoConvert = 123,
    IDK_Yen = 125,
    IDK_PTBRNUMPADPOINT = 126,
    IDK_NumpadEquals = 141,
    IDK_PrevTrack = 144,
    IDK_At = 145,
    IDK_Colon = 146,
    IDK_Underline = 147,
    IDK_Kanji = 148,
    IDK_Stop = 149,
    IDK_Ax = 150,
    IDK_Unlabeled = 151,
    IDK_NextTrack = 153,
    IDK_NumpadEnter = 156,
    IDK_RightCtrl = 157,
    IDK_Mute = 160,
    IDK_Calculator = 161,
    IDK_PlayPause = 162,
    IDK_MediaStop = 164,
    IDK_VolumeDown = 174,
    IDK_VolumeUp = 176,
    IDK_WebHome = 178,
    IDK_NumpadComma = 179,
    IDK_Divide = 181,
    IDK_PrintScreen = 183,
    IDK_RightAlt = 184,
    IDK_Home = 199,
    IDK_ArrowUp = 200,
    IDK_PageUp = 201,
    IDK_ArrowLeft = 203,
    IDK_ArrowRight = 205,
    IDK_End = 207,
    IDK_ArrowDown = 208,
    IDK_PageDown = 209,
    IDK_Insert = 210,
    IDK_Delete = 211,
    IDK_LeftWin = 91,
    IDK_RightWin = 92,
    IDK_AppMenu = 221,
    IDK_Power = 222,
    IDK_Sleep = 223,
    IDK_Wake = 227,
    IDK_WebSearch = 229,
    IDK_WebFavorites = 230,
    IDK_WebRefresh = 231,
    IDK_WebStop = 232,
    IDK_WebForward = 233,
    IDK_WebBack = 234,
    IDK_MyComputer = 235,
    IDK_Mail = 236,
    IDK_MediaSelect = 237,
    IDK_Pause = 197,
    IDK_Undefined = 255,
}

pub static INPUTDEVICEKEYS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDeviceKeys",
    flags: MemberInfoFlags::new(49429),
    module: "InputShared",
    data: TypeInfoData::Enum,
    array_type: Some(INPUTDEVICEKEYS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputDeviceKeys {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTDEVICEKEYS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INPUTDEVICEKEYS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDeviceKeys-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputDeviceKeys"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CursorStyle {
    #[default]
    CursorStyle_Hidden = 0,
    CursorStyle_Arrow = 1,
    CursorStyle_Wait = 2,
    CursorStyle_Current = 3,
}

pub static CURSORSTYLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CursorStyle",
    flags: MemberInfoFlags::new(49429),
    module: "InputShared",
    data: TypeInfoData::Enum,
    array_type: Some(CURSORSTYLE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CursorStyle {
    fn type_info(&self) -> &'static TypeInfo {
        CURSORSTYLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static CURSORSTYLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CursorStyle-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("CursorStyle"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InputDeviceMouseButtons {
    #[default]
    IDB_Button_0 = 0,
    IDB_Button_1 = 1,
    IDB_Button_2 = 2,
    IDB_Button_3 = 3,
    IDB_Button_4 = 4,
    IDB_Button_5 = 5,
    IDB_Button_6 = 6,
    IDB_Button_7 = 7,
    IDB_Button_Undefined = 8,
}

pub static INPUTDEVICEMOUSEBUTTONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDeviceMouseButtons",
    flags: MemberInfoFlags::new(49429),
    module: "InputShared",
    data: TypeInfoData::Enum,
    array_type: Some(INPUTDEVICEMOUSEBUTTONS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputDeviceMouseButtons {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTDEVICEMOUSEBUTTONS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INPUTDEVICEMOUSEBUTTONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDeviceMouseButtons-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputDeviceMouseButtons"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InputDeviceMotionControllerButtons {
    #[default]
    IDMCB_Rup = 0,
    IDMCB_Rdown = 1,
    IDMCB_Rleft = 2,
    IDMCB_Rright = 3,
    IDMCB_start = 4,
    IDMCB_alt = 5,
    IDMCB_center = 6,
    IDMCB_trigger = 7,
    IDMCB_reload = 8,
    IDMCB_pumpaction = 9,
    IDMCB_Gstab = 10,
    IDMCB_Grotleft = 11,
    IDMCB_Grotright = 12,
    IDMCB_Undefined = 13,
}

pub static INPUTDEVICEMOTIONCONTROLLERBUTTONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDeviceMotionControllerButtons",
    flags: MemberInfoFlags::new(49429),
    module: "InputShared",
    data: TypeInfoData::Enum,
    array_type: Some(INPUTDEVICEMOTIONCONTROLLERBUTTONS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputDeviceMotionControllerButtons {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTDEVICEMOTIONCONTROLLERBUTTONS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INPUTDEVICEMOTIONCONTROLLERBUTTONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDeviceMotionControllerButtons-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputDeviceMotionControllerButtons"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InputDeviceWheelButtons {
    #[default]
    IDWB_Lup = 0,
    IDWB_Ldown = 1,
    IDWB_Lleft = 2,
    IDWB_Lright = 3,
    IDWB_Rup = 4,
    IDWB_Rdown = 5,
    IDWB_Rleft = 6,
    IDWB_Rright = 7,
    IDWB_Rtopleft = 8,
    IDWB_Rtopright = 9,
    IDWB_Lthumb = 10,
    IDWB_Rthumb = 11,
    IDWB_start = 12,
    IDWB_alt = 13,
    IDWB_Ltrigger = 14,
    IDWB_Rtrigger = 15,
    IDWB_Ltrigger2 = 16,
    IDWB_Rtrigger2 = 17,
    IDWB_Ltrigger3 = 18,
    IDWB_Rtrigger3 = 19,
    IDWB_Ltrigger4 = 20,
    IDWB_Rtrigger4 = 21,
    IDWB_Ltrigger5 = 22,
    IDWB_Rtrigger5 = 23,
    IDWB_Throttle = 24,
    IDWB_Brake = 25,
    IDWB_Clutch = 26,
    IDWB_Horn = 27,
    IDWB_ShiftUp = 28,
    IDWB_ShiftDown = 29,
    IDWB_Enter = 30,
    IDWB_FirstGear = 31,
    IDWB_SecondGear = 32,
    IDWB_ThirdGear = 33,
    IDWB_FourthGear = 34,
    IDWB_FifthGear = 35,
    IDWB_SixthGear = 36,
    IDWB_ReverseGear = 37,
    IDWB_Undefined = 38,
}

pub static INPUTDEVICEWHEELBUTTONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDeviceWheelButtons",
    flags: MemberInfoFlags::new(49429),
    module: "InputShared",
    data: TypeInfoData::Enum,
    array_type: Some(INPUTDEVICEWHEELBUTTONS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputDeviceWheelButtons {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTDEVICEWHEELBUTTONS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INPUTDEVICEWHEELBUTTONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDeviceWheelButtons-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputDeviceWheelButtons"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InputDevicePadButtons {
    #[default]
    IDB_Lup = 0,
    IDB_Ldown = 1,
    IDB_Lleft = 2,
    IDB_Lright = 3,
    IDB_Rup = 4,
    IDB_Rdown = 5,
    IDB_Rleft = 6,
    IDB_Rright = 7,
    IDB_Rtopleft = 8,
    IDB_Rtopright = 9,
    IDB_Lthumb = 10,
    IDB_Rthumb = 11,
    IDB_start = 12,
    IDB_alt = 13,
    IDB_Ltrigger = 14,
    IDB_Rtrigger = 15,
    IDB_Ltrigger2 = 16,
    IDB_Rtrigger2 = 17,
    IDB_TSwipeLeft = 18,
    IDB_TSwipeUp = 19,
    IDB_TSwipeRight = 20,
    IDB_TSwipeDown = 21,
    IDB_XButton1 = 22,
    IDB_XButton2 = 23,
    IDB_XButton3 = 24,
    IDB_XButton4 = 25,
    IDB_XButton5 = 26,
    IDB_XButton6 = 27,
    IDB_XButton7 = 28,
    IDB_XButton8 = 29,
    IDB_XButton9 = 30,
    IDB_XButton10 = 31,
    IDB_XButton11 = 32,
    IDB_XButton12 = 33,
    IDB_XButton13 = 34,
    IDB_XButton14 = 35,
    IDB_XButton15 = 36,
    IDB_XButton16 = 37,
    IDB_XButton17 = 38,
    IDB_XButton18 = 39,
    IDB_PosZAxis = 40,
    IDB_NegZAxis = 41,
    IDB_XRotationPos = 42,
    IDB_XRotationNeg = 43,
    IDB_YRotationPos = 44,
    IDB_YRotationNeg = 45,
    IDB_ZRotationPos = 46,
    IDB_ZRotationNeg = 47,
    IDB_Pov1North = 48,
    IDB_Pov1South = 49,
    IDB_Pov1West = 50,
    IDB_Pov1East = 51,
    IDB_Pov2North = 52,
    IDB_Pov2South = 53,
    IDB_Pov2West = 54,
    IDB_Pov2East = 55,
    IDB_Pov3North = 56,
    IDB_Pov3South = 57,
    IDB_Pov3West = 58,
    IDB_Pov3East = 59,
    IDB_Pov4North = 60,
    IDB_Pov4South = 61,
    IDB_Pov4West = 62,
    IDB_Pov4East = 63,
    IDB_Undefined = 64,
}

pub static INPUTDEVICEPADBUTTONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDevicePadButtons",
    flags: MemberInfoFlags::new(49429),
    module: "InputShared",
    data: TypeInfoData::Enum,
    array_type: Some(INPUTDEVICEPADBUTTONS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputDevicePadButtons {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTDEVICEPADBUTTONS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INPUTDEVICEPADBUTTONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDevicePadButtons-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputDevicePadButtons"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InputDeviceAxes {
    #[default]
    IDA_Axis0X = 0,
    IDA_Axis0Y = 1,
    IDA_Axis0XPos = 2,
    IDA_Axis0YPos = 3,
    IDA_Axis0XNeg = 4,
    IDA_Axis0YNeg = 5,
    IDA_Axis1X = 6,
    IDA_Axis1Y = 7,
    IDA_Axis1XPos = 8,
    IDA_Axis1YPos = 9,
    IDA_Axis1XNeg = 10,
    IDA_Axis1YNeg = 11,
    IDA_Axis2X = 12,
    IDA_Axis2Y = 13,
    IDA_Axis3X = 14,
    IDA_Axis3Y = 15,
    IDA_Axis4X = 16,
    IDA_Axis4Y = 17,
    IDA_Axis5X = 18,
    IDA_Axis5Y = 19,
    IDA_Axis6X = 20,
    IDA_Axis6Y = 21,
    IDA_Axis7X = 22,
    IDA_Axis7Y = 23,
    IDA_Undefined = 24,
}

pub static INPUTDEVICEAXES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDeviceAxes",
    flags: MemberInfoFlags::new(49429),
    module: "InputShared",
    data: TypeInfoData::Enum,
    array_type: Some(INPUTDEVICEAXES_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputDeviceAxes {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTDEVICEAXES_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INPUTDEVICEAXES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputDeviceAxes-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputDeviceAxes"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputConfigurationAsset {
    pub _glacier_base: super::core::Asset,
    pub default_input_graph: Option<Arc<Mutex<dyn InputGraphTrait>>>,
    pub default_input_action_maps: Option<Arc<Mutex<dyn InputActionMapsDataTrait>>>,
    pub default_input_concept_to_entry_input_mappings: Option<Arc<Mutex<dyn InputConceptToEntryInputActionMappingsTrait>>>,
    pub entry_input_action_bindings: Option<Arc<Mutex<dyn EntryInputActionBindingsDataTrait>>>,
    pub user_configurable_action_maps: Vec<EditableActionMap>,
    pub default_exclusive_input_concepts: Vec<i32>,
    pub reverse_input_concept_exclusion: bool,
    pub input_curves_enabled: bool,
    pub input_settings: Vec<Option<Arc<Mutex<dyn BaseInputSettingsTrait>>>>,
}

pub trait InputConfigurationAssetTrait: super::core::AssetTrait {
    fn default_input_graph(&self) -> &Option<Arc<Mutex<dyn InputGraphTrait>>>;
    fn default_input_graph_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputGraphTrait>>>;
    fn default_input_action_maps(&self) -> &Option<Arc<Mutex<dyn InputActionMapsDataTrait>>>;
    fn default_input_action_maps_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputActionMapsDataTrait>>>;
    fn default_input_concept_to_entry_input_mappings(&self) -> &Option<Arc<Mutex<dyn InputConceptToEntryInputActionMappingsTrait>>>;
    fn default_input_concept_to_entry_input_mappings_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputConceptToEntryInputActionMappingsTrait>>>;
    fn entry_input_action_bindings(&self) -> &Option<Arc<Mutex<dyn EntryInputActionBindingsDataTrait>>>;
    fn entry_input_action_bindings_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EntryInputActionBindingsDataTrait>>>;
    fn user_configurable_action_maps(&self) -> &Vec<EditableActionMap>;
    fn user_configurable_action_maps_mut(&mut self) -> &mut Vec<EditableActionMap>;
    fn default_exclusive_input_concepts(&self) -> &Vec<i32>;
    fn default_exclusive_input_concepts_mut(&mut self) -> &mut Vec<i32>;
    fn reverse_input_concept_exclusion(&self) -> &bool;
    fn reverse_input_concept_exclusion_mut(&mut self) -> &mut bool;
    fn input_curves_enabled(&self) -> &bool;
    fn input_curves_enabled_mut(&mut self) -> &mut bool;
    fn input_settings(&self) -> &Vec<Option<Arc<Mutex<dyn BaseInputSettingsTrait>>>>;
    fn input_settings_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn BaseInputSettingsTrait>>>>;
}

impl InputConfigurationAssetTrait for InputConfigurationAsset {
    fn default_input_graph(&self) -> &Option<Arc<Mutex<dyn InputGraphTrait>>> {
        &self.default_input_graph
    }
    fn default_input_graph_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputGraphTrait>>> {
        &mut self.default_input_graph
    }
    fn default_input_action_maps(&self) -> &Option<Arc<Mutex<dyn InputActionMapsDataTrait>>> {
        &self.default_input_action_maps
    }
    fn default_input_action_maps_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputActionMapsDataTrait>>> {
        &mut self.default_input_action_maps
    }
    fn default_input_concept_to_entry_input_mappings(&self) -> &Option<Arc<Mutex<dyn InputConceptToEntryInputActionMappingsTrait>>> {
        &self.default_input_concept_to_entry_input_mappings
    }
    fn default_input_concept_to_entry_input_mappings_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputConceptToEntryInputActionMappingsTrait>>> {
        &mut self.default_input_concept_to_entry_input_mappings
    }
    fn entry_input_action_bindings(&self) -> &Option<Arc<Mutex<dyn EntryInputActionBindingsDataTrait>>> {
        &self.entry_input_action_bindings
    }
    fn entry_input_action_bindings_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EntryInputActionBindingsDataTrait>>> {
        &mut self.entry_input_action_bindings
    }
    fn user_configurable_action_maps(&self) -> &Vec<EditableActionMap> {
        &self.user_configurable_action_maps
    }
    fn user_configurable_action_maps_mut(&mut self) -> &mut Vec<EditableActionMap> {
        &mut self.user_configurable_action_maps
    }
    fn default_exclusive_input_concepts(&self) -> &Vec<i32> {
        &self.default_exclusive_input_concepts
    }
    fn default_exclusive_input_concepts_mut(&mut self) -> &mut Vec<i32> {
        &mut self.default_exclusive_input_concepts
    }
    fn reverse_input_concept_exclusion(&self) -> &bool {
        &self.reverse_input_concept_exclusion
    }
    fn reverse_input_concept_exclusion_mut(&mut self) -> &mut bool {
        &mut self.reverse_input_concept_exclusion
    }
    fn input_curves_enabled(&self) -> &bool {
        &self.input_curves_enabled
    }
    fn input_curves_enabled_mut(&mut self) -> &mut bool {
        &mut self.input_curves_enabled
    }
    fn input_settings(&self) -> &Vec<Option<Arc<Mutex<dyn BaseInputSettingsTrait>>>> {
        &self.input_settings
    }
    fn input_settings_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn BaseInputSettingsTrait>>>> {
        &mut self.input_settings
    }
}

impl super::core::AssetTrait for InputConfigurationAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for InputConfigurationAsset {
}

pub static INPUTCONFIGURATIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConfigurationAsset",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputConfigurationAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DefaultInputGraph",
                flags: MemberInfoFlags::new(0),
                field_type: "InputGraph",
                rust_offset: offset_of!(InputConfigurationAsset, default_input_graph),
            },
            FieldInfoData {
                name: "DefaultInputActionMaps",
                flags: MemberInfoFlags::new(0),
                field_type: "InputActionMapsData",
                rust_offset: offset_of!(InputConfigurationAsset, default_input_action_maps),
            },
            FieldInfoData {
                name: "DefaultInputConceptToEntryInputMappings",
                flags: MemberInfoFlags::new(0),
                field_type: "InputConceptToEntryInputActionMappings",
                rust_offset: offset_of!(InputConfigurationAsset, default_input_concept_to_entry_input_mappings),
            },
            FieldInfoData {
                name: "EntryInputActionBindings",
                flags: MemberInfoFlags::new(0),
                field_type: "EntryInputActionBindingsData",
                rust_offset: offset_of!(InputConfigurationAsset, entry_input_action_bindings),
            },
            FieldInfoData {
                name: "UserConfigurableActionMaps",
                flags: MemberInfoFlags::new(144),
                field_type: "EditableActionMap-Array",
                rust_offset: offset_of!(InputConfigurationAsset, user_configurable_action_maps),
            },
            FieldInfoData {
                name: "DefaultExclusiveInputConcepts",
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(InputConfigurationAsset, default_exclusive_input_concepts),
            },
            FieldInfoData {
                name: "ReverseInputConceptExclusion",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputConfigurationAsset, reverse_input_concept_exclusion),
            },
            FieldInfoData {
                name: "InputCurvesEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputConfigurationAsset, input_curves_enabled),
            },
            FieldInfoData {
                name: "InputSettings",
                flags: MemberInfoFlags::new(144),
                field_type: "BaseInputSettings-Array",
                rust_offset: offset_of!(InputConfigurationAsset, input_settings),
            },
        ],
    }),
    array_type: Some(INPUTCONFIGURATIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputConfigurationAsset {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTCONFIGURATIONASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INPUTCONFIGURATIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConfigurationAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputConfigurationAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EditableActionMap {
    pub id: String,
    pub action_map: Option<Arc<Mutex<dyn InputActionMapsDataTrait>>>,
}

pub trait EditableActionMapTrait: TypeObject {
    fn id(&self) -> &String;
    fn id_mut(&mut self) -> &mut String;
    fn action_map(&self) -> &Option<Arc<Mutex<dyn InputActionMapsDataTrait>>>;
    fn action_map_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputActionMapsDataTrait>>>;
}

impl EditableActionMapTrait for EditableActionMap {
    fn id(&self) -> &String {
        &self.id
    }
    fn id_mut(&mut self) -> &mut String {
        &mut self.id
    }
    fn action_map(&self) -> &Option<Arc<Mutex<dyn InputActionMapsDataTrait>>> {
        &self.action_map
    }
    fn action_map_mut(&mut self) -> &mut Option<Arc<Mutex<dyn InputActionMapsDataTrait>>> {
        &mut self.action_map
    }
}

pub static EDITABLEACTIONMAP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EditableActionMap",
    flags: MemberInfoFlags::new(73),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EditableActionMap as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(EditableActionMap, id),
            },
            FieldInfoData {
                name: "ActionMap",
                flags: MemberInfoFlags::new(0),
                field_type: "InputActionMapsData",
                rust_offset: offset_of!(EditableActionMap, action_map),
            },
        ],
    }),
    array_type: Some(EDITABLEACTIONMAP_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EditableActionMap {
    fn type_info(&self) -> &'static TypeInfo {
        EDITABLEACTIONMAP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static EDITABLEACTIONMAP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EditableActionMap-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("EditableActionMap"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputActionMapsData {
    pub _glacier_base: super::core::Asset,
    pub action_map_settings_scheme: i32,
    pub default_input_action_map: InputActionMapSlot,
    pub input_action_maps: Vec<Option<Arc<Mutex<dyn InputActionMapDataTrait>>>>,
    pub identifier: u32,
}

pub trait InputActionMapsDataTrait: super::core::AssetTrait {
    fn action_map_settings_scheme(&self) -> &i32;
    fn action_map_settings_scheme_mut(&mut self) -> &mut i32;
    fn default_input_action_map(&self) -> &InputActionMapSlot;
    fn default_input_action_map_mut(&mut self) -> &mut InputActionMapSlot;
    fn input_action_maps(&self) -> &Vec<Option<Arc<Mutex<dyn InputActionMapDataTrait>>>>;
    fn input_action_maps_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn InputActionMapDataTrait>>>>;
    fn identifier(&self) -> &u32;
    fn identifier_mut(&mut self) -> &mut u32;
}

impl InputActionMapsDataTrait for InputActionMapsData {
    fn action_map_settings_scheme(&self) -> &i32 {
        &self.action_map_settings_scheme
    }
    fn action_map_settings_scheme_mut(&mut self) -> &mut i32 {
        &mut self.action_map_settings_scheme
    }
    fn default_input_action_map(&self) -> &InputActionMapSlot {
        &self.default_input_action_map
    }
    fn default_input_action_map_mut(&mut self) -> &mut InputActionMapSlot {
        &mut self.default_input_action_map
    }
    fn input_action_maps(&self) -> &Vec<Option<Arc<Mutex<dyn InputActionMapDataTrait>>>> {
        &self.input_action_maps
    }
    fn input_action_maps_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn InputActionMapDataTrait>>>> {
        &mut self.input_action_maps
    }
    fn identifier(&self) -> &u32 {
        &self.identifier
    }
    fn identifier_mut(&mut self) -> &mut u32 {
        &mut self.identifier
    }
}

impl super::core::AssetTrait for InputActionMapsData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for InputActionMapsData {
}

pub static INPUTACTIONMAPSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionMapsData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputActionMapsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ActionMapSettingsScheme",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InputActionMapsData, action_map_settings_scheme),
            },
            FieldInfoData {
                name: "DefaultInputActionMap",
                flags: MemberInfoFlags::new(0),
                field_type: "InputActionMapSlot",
                rust_offset: offset_of!(InputActionMapsData, default_input_action_map),
            },
            FieldInfoData {
                name: "InputActionMaps",
                flags: MemberInfoFlags::new(144),
                field_type: "InputActionMapData-Array",
                rust_offset: offset_of!(InputActionMapsData, input_action_maps),
            },
            FieldInfoData {
                name: "Identifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(InputActionMapsData, identifier),
            },
        ],
    }),
    array_type: Some(INPUTACTIONMAPSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputActionMapsData {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTACTIONMAPSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INPUTACTIONMAPSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionMapsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputActionMapsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputActionMapData {
    pub _glacier_base: super::core::DataContainer,
    pub actions: Vec<Option<Arc<Mutex<dyn InputActionsDataTrait>>>>,
    pub platform_specific: InputActionMapPlatform,
    pub slot: InputActionMapSlot,
    pub copy_key_bindings_from: String,
}

pub trait InputActionMapDataTrait: super::core::DataContainerTrait {
    fn actions(&self) -> &Vec<Option<Arc<Mutex<dyn InputActionsDataTrait>>>>;
    fn actions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn InputActionsDataTrait>>>>;
    fn platform_specific(&self) -> &InputActionMapPlatform;
    fn platform_specific_mut(&mut self) -> &mut InputActionMapPlatform;
    fn slot(&self) -> &InputActionMapSlot;
    fn slot_mut(&mut self) -> &mut InputActionMapSlot;
    fn copy_key_bindings_from(&self) -> &String;
    fn copy_key_bindings_from_mut(&mut self) -> &mut String;
}

impl InputActionMapDataTrait for InputActionMapData {
    fn actions(&self) -> &Vec<Option<Arc<Mutex<dyn InputActionsDataTrait>>>> {
        &self.actions
    }
    fn actions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn InputActionsDataTrait>>>> {
        &mut self.actions
    }
    fn platform_specific(&self) -> &InputActionMapPlatform {
        &self.platform_specific
    }
    fn platform_specific_mut(&mut self) -> &mut InputActionMapPlatform {
        &mut self.platform_specific
    }
    fn slot(&self) -> &InputActionMapSlot {
        &self.slot
    }
    fn slot_mut(&mut self) -> &mut InputActionMapSlot {
        &mut self.slot
    }
    fn copy_key_bindings_from(&self) -> &String {
        &self.copy_key_bindings_from
    }
    fn copy_key_bindings_from_mut(&mut self) -> &mut String {
        &mut self.copy_key_bindings_from
    }
}

impl super::core::DataContainerTrait for InputActionMapData {
}

pub static INPUTACTIONMAPDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionMapData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputActionMapData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Actions",
                flags: MemberInfoFlags::new(144),
                field_type: "InputActionsData-Array",
                rust_offset: offset_of!(InputActionMapData, actions),
            },
            FieldInfoData {
                name: "PlatformSpecific",
                flags: MemberInfoFlags::new(0),
                field_type: "InputActionMapPlatform",
                rust_offset: offset_of!(InputActionMapData, platform_specific),
            },
            FieldInfoData {
                name: "Slot",
                flags: MemberInfoFlags::new(0),
                field_type: "InputActionMapSlot",
                rust_offset: offset_of!(InputActionMapData, slot),
            },
            FieldInfoData {
                name: "CopyKeyBindingsFrom",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(InputActionMapData, copy_key_bindings_from),
            },
        ],
    }),
    array_type: Some(INPUTACTIONMAPDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputActionMapData {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTACTIONMAPDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INPUTACTIONMAPDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionMapData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputActionMapData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputActionsData {
    pub _glacier_base: super::core::DataContainer,
    pub name_sid: String,
    pub concept_identifier: i32,
    pub copy_key_binding_from: i32,
    pub hide_in_key_bindings: bool,
    pub input_actions: Vec<Option<Arc<Mutex<dyn InputActionDataTrait>>>>,
}

pub trait InputActionsDataTrait: super::core::DataContainerTrait {
    fn name_sid(&self) -> &String;
    fn name_sid_mut(&mut self) -> &mut String;
    fn concept_identifier(&self) -> &i32;
    fn concept_identifier_mut(&mut self) -> &mut i32;
    fn copy_key_binding_from(&self) -> &i32;
    fn copy_key_binding_from_mut(&mut self) -> &mut i32;
    fn hide_in_key_bindings(&self) -> &bool;
    fn hide_in_key_bindings_mut(&mut self) -> &mut bool;
    fn input_actions(&self) -> &Vec<Option<Arc<Mutex<dyn InputActionDataTrait>>>>;
    fn input_actions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn InputActionDataTrait>>>>;
}

impl InputActionsDataTrait for InputActionsData {
    fn name_sid(&self) -> &String {
        &self.name_sid
    }
    fn name_sid_mut(&mut self) -> &mut String {
        &mut self.name_sid
    }
    fn concept_identifier(&self) -> &i32 {
        &self.concept_identifier
    }
    fn concept_identifier_mut(&mut self) -> &mut i32 {
        &mut self.concept_identifier
    }
    fn copy_key_binding_from(&self) -> &i32 {
        &self.copy_key_binding_from
    }
    fn copy_key_binding_from_mut(&mut self) -> &mut i32 {
        &mut self.copy_key_binding_from
    }
    fn hide_in_key_bindings(&self) -> &bool {
        &self.hide_in_key_bindings
    }
    fn hide_in_key_bindings_mut(&mut self) -> &mut bool {
        &mut self.hide_in_key_bindings
    }
    fn input_actions(&self) -> &Vec<Option<Arc<Mutex<dyn InputActionDataTrait>>>> {
        &self.input_actions
    }
    fn input_actions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn InputActionDataTrait>>>> {
        &mut self.input_actions
    }
}

impl super::core::DataContainerTrait for InputActionsData {
}

pub static INPUTACTIONSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionsData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputActionsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "NameSid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(InputActionsData, name_sid),
            },
            FieldInfoData {
                name: "ConceptIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InputActionsData, concept_identifier),
            },
            FieldInfoData {
                name: "CopyKeyBindingFrom",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(InputActionsData, copy_key_binding_from),
            },
            FieldInfoData {
                name: "HideInKeyBindings",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputActionsData, hide_in_key_bindings),
            },
            FieldInfoData {
                name: "InputActions",
                flags: MemberInfoFlags::new(144),
                field_type: "InputActionData-Array",
                rust_offset: offset_of!(InputActionsData, input_actions),
            },
        ],
    }),
    array_type: Some(INPUTACTIONSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputActionsData {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTACTIONSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INPUTACTIONSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputActionsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MessageInputActionData {
    pub _glacier_base: InputActionData,
    pub command: i32,
}

pub trait MessageInputActionDataTrait: InputActionDataTrait {
    fn command(&self) -> &i32;
    fn command_mut(&mut self) -> &mut i32;
}

impl MessageInputActionDataTrait for MessageInputActionData {
    fn command(&self) -> &i32 {
        &self.command
    }
    fn command_mut(&mut self) -> &mut i32 {
        &mut self.command
    }
}

impl InputActionDataTrait for MessageInputActionData {
    fn is_analog(&self) -> &bool {
        self._glacier_base.is_analog()
    }
    fn is_analog_mut(&mut self) -> &mut bool {
        self._glacier_base.is_analog_mut()
    }
    fn negate_value(&self) -> &bool {
        self._glacier_base.negate_value()
    }
    fn negate_value_mut(&mut self) -> &mut bool {
        self._glacier_base.negate_value_mut()
    }
}

impl super::core::DataContainerTrait for MessageInputActionData {
}

pub static MESSAGEINPUTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MessageInputActionData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(INPUTACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MessageInputActionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Command",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MessageInputActionData, command),
            },
        ],
    }),
    array_type: Some(MESSAGEINPUTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MessageInputActionData {
    fn type_info(&self) -> &'static TypeInfo {
        MESSAGEINPUTACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MESSAGEINPUTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MessageInputActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("MessageInputActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MouseInputActionData {
    pub _glacier_base: AxesInputActionData,
    pub button: InputDeviceMouseButtons,
    pub simulate_joystick_axis: bool,
    pub remember_excess_input: bool,
    pub scale_scroll_wheel_axis_input: bool,
}

pub trait MouseInputActionDataTrait: AxesInputActionDataTrait {
    fn button(&self) -> &InputDeviceMouseButtons;
    fn button_mut(&mut self) -> &mut InputDeviceMouseButtons;
    fn simulate_joystick_axis(&self) -> &bool;
    fn simulate_joystick_axis_mut(&mut self) -> &mut bool;
    fn remember_excess_input(&self) -> &bool;
    fn remember_excess_input_mut(&mut self) -> &mut bool;
    fn scale_scroll_wheel_axis_input(&self) -> &bool;
    fn scale_scroll_wheel_axis_input_mut(&mut self) -> &mut bool;
}

impl MouseInputActionDataTrait for MouseInputActionData {
    fn button(&self) -> &InputDeviceMouseButtons {
        &self.button
    }
    fn button_mut(&mut self) -> &mut InputDeviceMouseButtons {
        &mut self.button
    }
    fn simulate_joystick_axis(&self) -> &bool {
        &self.simulate_joystick_axis
    }
    fn simulate_joystick_axis_mut(&mut self) -> &mut bool {
        &mut self.simulate_joystick_axis
    }
    fn remember_excess_input(&self) -> &bool {
        &self.remember_excess_input
    }
    fn remember_excess_input_mut(&mut self) -> &mut bool {
        &mut self.remember_excess_input
    }
    fn scale_scroll_wheel_axis_input(&self) -> &bool {
        &self.scale_scroll_wheel_axis_input
    }
    fn scale_scroll_wheel_axis_input_mut(&mut self) -> &mut bool {
        &mut self.scale_scroll_wheel_axis_input
    }
}

impl AxesInputActionDataTrait for MouseInputActionData {
    fn axis(&self) -> &InputDeviceAxes {
        self._glacier_base.axis()
    }
    fn axis_mut(&mut self) -> &mut InputDeviceAxes {
        self._glacier_base.axis_mut()
    }
    fn normalize_input(&self) -> &bool {
        self._glacier_base.normalize_input()
    }
    fn normalize_input_mut(&mut self) -> &mut bool {
        self._glacier_base.normalize_input_mut()
    }
}

impl InputActionDataTrait for MouseInputActionData {
    fn is_analog(&self) -> &bool {
        self._glacier_base.is_analog()
    }
    fn is_analog_mut(&mut self) -> &mut bool {
        self._glacier_base.is_analog_mut()
    }
    fn negate_value(&self) -> &bool {
        self._glacier_base.negate_value()
    }
    fn negate_value_mut(&mut self) -> &mut bool {
        self._glacier_base.negate_value_mut()
    }
}

impl super::core::DataContainerTrait for MouseInputActionData {
}

pub static MOUSEINPUTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MouseInputActionData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AXESINPUTACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MouseInputActionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Button",
                flags: MemberInfoFlags::new(0),
                field_type: "InputDeviceMouseButtons",
                rust_offset: offset_of!(MouseInputActionData, button),
            },
            FieldInfoData {
                name: "SimulateJoystickAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MouseInputActionData, simulate_joystick_axis),
            },
            FieldInfoData {
                name: "RememberExcessInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MouseInputActionData, remember_excess_input),
            },
            FieldInfoData {
                name: "ScaleScrollWheelAxisInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MouseInputActionData, scale_scroll_wheel_axis_input),
            },
        ],
    }),
    array_type: Some(MOUSEINPUTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MouseInputActionData {
    fn type_info(&self) -> &'static TypeInfo {
        MOUSEINPUTACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MOUSEINPUTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MouseInputActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("MouseInputActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct KeyboardInputActionData {
    pub _glacier_base: InputActionData,
    pub key: InputDeviceKeys,
}

pub trait KeyboardInputActionDataTrait: InputActionDataTrait {
    fn key(&self) -> &InputDeviceKeys;
    fn key_mut(&mut self) -> &mut InputDeviceKeys;
}

impl KeyboardInputActionDataTrait for KeyboardInputActionData {
    fn key(&self) -> &InputDeviceKeys {
        &self.key
    }
    fn key_mut(&mut self) -> &mut InputDeviceKeys {
        &mut self.key
    }
}

impl InputActionDataTrait for KeyboardInputActionData {
    fn is_analog(&self) -> &bool {
        self._glacier_base.is_analog()
    }
    fn is_analog_mut(&mut self) -> &mut bool {
        self._glacier_base.is_analog_mut()
    }
    fn negate_value(&self) -> &bool {
        self._glacier_base.negate_value()
    }
    fn negate_value_mut(&mut self) -> &mut bool {
        self._glacier_base.negate_value_mut()
    }
}

impl super::core::DataContainerTrait for KeyboardInputActionData {
}

pub static KEYBOARDINPUTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "KeyboardInputActionData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(INPUTACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<KeyboardInputActionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Key",
                flags: MemberInfoFlags::new(0),
                field_type: "InputDeviceKeys",
                rust_offset: offset_of!(KeyboardInputActionData, key),
            },
        ],
    }),
    array_type: Some(KEYBOARDINPUTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for KeyboardInputActionData {
    fn type_info(&self) -> &'static TypeInfo {
        KEYBOARDINPUTACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static KEYBOARDINPUTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "KeyboardInputActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("KeyboardInputActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WheelInputActionData {
    pub _glacier_base: AxesInputActionData,
    pub button: InputDeviceWheelButtons,
}

pub trait WheelInputActionDataTrait: AxesInputActionDataTrait {
    fn button(&self) -> &InputDeviceWheelButtons;
    fn button_mut(&mut self) -> &mut InputDeviceWheelButtons;
}

impl WheelInputActionDataTrait for WheelInputActionData {
    fn button(&self) -> &InputDeviceWheelButtons {
        &self.button
    }
    fn button_mut(&mut self) -> &mut InputDeviceWheelButtons {
        &mut self.button
    }
}

impl AxesInputActionDataTrait for WheelInputActionData {
    fn axis(&self) -> &InputDeviceAxes {
        self._glacier_base.axis()
    }
    fn axis_mut(&mut self) -> &mut InputDeviceAxes {
        self._glacier_base.axis_mut()
    }
    fn normalize_input(&self) -> &bool {
        self._glacier_base.normalize_input()
    }
    fn normalize_input_mut(&mut self) -> &mut bool {
        self._glacier_base.normalize_input_mut()
    }
}

impl InputActionDataTrait for WheelInputActionData {
    fn is_analog(&self) -> &bool {
        self._glacier_base.is_analog()
    }
    fn is_analog_mut(&mut self) -> &mut bool {
        self._glacier_base.is_analog_mut()
    }
    fn negate_value(&self) -> &bool {
        self._glacier_base.negate_value()
    }
    fn negate_value_mut(&mut self) -> &mut bool {
        self._glacier_base.negate_value_mut()
    }
}

impl super::core::DataContainerTrait for WheelInputActionData {
}

pub static WHEELINPUTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelInputActionData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AXESINPUTACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WheelInputActionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Button",
                flags: MemberInfoFlags::new(0),
                field_type: "InputDeviceWheelButtons",
                rust_offset: offset_of!(WheelInputActionData, button),
            },
        ],
    }),
    array_type: Some(WHEELINPUTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WheelInputActionData {
    fn type_info(&self) -> &'static TypeInfo {
        WHEELINPUTACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static WHEELINPUTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WheelInputActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("WheelInputActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VrInputActionData {
    pub _glacier_base: AxesInputActionData,
}

pub trait VrInputActionDataTrait: AxesInputActionDataTrait {
}

impl VrInputActionDataTrait for VrInputActionData {
}

impl AxesInputActionDataTrait for VrInputActionData {
    fn axis(&self) -> &InputDeviceAxes {
        self._glacier_base.axis()
    }
    fn axis_mut(&mut self) -> &mut InputDeviceAxes {
        self._glacier_base.axis_mut()
    }
    fn normalize_input(&self) -> &bool {
        self._glacier_base.normalize_input()
    }
    fn normalize_input_mut(&mut self) -> &mut bool {
        self._glacier_base.normalize_input_mut()
    }
}

impl InputActionDataTrait for VrInputActionData {
    fn is_analog(&self) -> &bool {
        self._glacier_base.is_analog()
    }
    fn is_analog_mut(&mut self) -> &mut bool {
        self._glacier_base.is_analog_mut()
    }
    fn negate_value(&self) -> &bool {
        self._glacier_base.negate_value()
    }
    fn negate_value_mut(&mut self) -> &mut bool {
        self._glacier_base.negate_value_mut()
    }
}

impl super::core::DataContainerTrait for VrInputActionData {
}

pub static VRINPUTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VrInputActionData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AXESINPUTACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VrInputActionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VRINPUTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VrInputActionData {
    fn type_info(&self) -> &'static TypeInfo {
        VRINPUTACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VRINPUTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VrInputActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("VrInputActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MotionControllerInputActionData {
    pub _glacier_base: AxesInputActionData,
    pub button: InputDeviceMotionControllerButtons,
}

pub trait MotionControllerInputActionDataTrait: AxesInputActionDataTrait {
    fn button(&self) -> &InputDeviceMotionControllerButtons;
    fn button_mut(&mut self) -> &mut InputDeviceMotionControllerButtons;
}

impl MotionControllerInputActionDataTrait for MotionControllerInputActionData {
    fn button(&self) -> &InputDeviceMotionControllerButtons {
        &self.button
    }
    fn button_mut(&mut self) -> &mut InputDeviceMotionControllerButtons {
        &mut self.button
    }
}

impl AxesInputActionDataTrait for MotionControllerInputActionData {
    fn axis(&self) -> &InputDeviceAxes {
        self._glacier_base.axis()
    }
    fn axis_mut(&mut self) -> &mut InputDeviceAxes {
        self._glacier_base.axis_mut()
    }
    fn normalize_input(&self) -> &bool {
        self._glacier_base.normalize_input()
    }
    fn normalize_input_mut(&mut self) -> &mut bool {
        self._glacier_base.normalize_input_mut()
    }
}

impl InputActionDataTrait for MotionControllerInputActionData {
    fn is_analog(&self) -> &bool {
        self._glacier_base.is_analog()
    }
    fn is_analog_mut(&mut self) -> &mut bool {
        self._glacier_base.is_analog_mut()
    }
    fn negate_value(&self) -> &bool {
        self._glacier_base.negate_value()
    }
    fn negate_value_mut(&mut self) -> &mut bool {
        self._glacier_base.negate_value_mut()
    }
}

impl super::core::DataContainerTrait for MotionControllerInputActionData {
}

pub static MOTIONCONTROLLERINPUTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionControllerInputActionData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AXESINPUTACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MotionControllerInputActionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Button",
                flags: MemberInfoFlags::new(0),
                field_type: "InputDeviceMotionControllerButtons",
                rust_offset: offset_of!(MotionControllerInputActionData, button),
            },
        ],
    }),
    array_type: Some(MOTIONCONTROLLERINPUTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MotionControllerInputActionData {
    fn type_info(&self) -> &'static TypeInfo {
        MOTIONCONTROLLERINPUTACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MOTIONCONTROLLERINPUTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MotionControllerInputActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("MotionControllerInputActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct JoystickInputActionData {
    pub _glacier_base: AxesInputActionData,
    pub use_square_input: bool,
    pub button: InputDevicePadButtons,
}

pub trait JoystickInputActionDataTrait: AxesInputActionDataTrait {
    fn use_square_input(&self) -> &bool;
    fn use_square_input_mut(&mut self) -> &mut bool;
    fn button(&self) -> &InputDevicePadButtons;
    fn button_mut(&mut self) -> &mut InputDevicePadButtons;
}

impl JoystickInputActionDataTrait for JoystickInputActionData {
    fn use_square_input(&self) -> &bool {
        &self.use_square_input
    }
    fn use_square_input_mut(&mut self) -> &mut bool {
        &mut self.use_square_input
    }
    fn button(&self) -> &InputDevicePadButtons {
        &self.button
    }
    fn button_mut(&mut self) -> &mut InputDevicePadButtons {
        &mut self.button
    }
}

impl AxesInputActionDataTrait for JoystickInputActionData {
    fn axis(&self) -> &InputDeviceAxes {
        self._glacier_base.axis()
    }
    fn axis_mut(&mut self) -> &mut InputDeviceAxes {
        self._glacier_base.axis_mut()
    }
    fn normalize_input(&self) -> &bool {
        self._glacier_base.normalize_input()
    }
    fn normalize_input_mut(&mut self) -> &mut bool {
        self._glacier_base.normalize_input_mut()
    }
}

impl InputActionDataTrait for JoystickInputActionData {
    fn is_analog(&self) -> &bool {
        self._glacier_base.is_analog()
    }
    fn is_analog_mut(&mut self) -> &mut bool {
        self._glacier_base.is_analog_mut()
    }
    fn negate_value(&self) -> &bool {
        self._glacier_base.negate_value()
    }
    fn negate_value_mut(&mut self) -> &mut bool {
        self._glacier_base.negate_value_mut()
    }
}

impl super::core::DataContainerTrait for JoystickInputActionData {
}

pub static JOYSTICKINPUTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JoystickInputActionData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AXESINPUTACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<JoystickInputActionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "UseSquareInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(JoystickInputActionData, use_square_input),
            },
            FieldInfoData {
                name: "Button",
                flags: MemberInfoFlags::new(0),
                field_type: "InputDevicePadButtons",
                rust_offset: offset_of!(JoystickInputActionData, button),
            },
        ],
    }),
    array_type: Some(JOYSTICKINPUTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for JoystickInputActionData {
    fn type_info(&self) -> &'static TypeInfo {
        JOYSTICKINPUTACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static JOYSTICKINPUTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JoystickInputActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("JoystickInputActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PadInputActionData {
    pub _glacier_base: AxesInputActionData,
    pub use_square_input: bool,
    pub ignore_suppression: bool,
    pub button: InputDevicePadButtons,
    pub p_s3_alternative_button: InputDevicePadButtons,
    pub p_s_vita_button: InputDevicePadButtons,
    pub p_s_vita_alternative_button: InputDevicePadButtons,
}

pub trait PadInputActionDataTrait: AxesInputActionDataTrait {
    fn use_square_input(&self) -> &bool;
    fn use_square_input_mut(&mut self) -> &mut bool;
    fn ignore_suppression(&self) -> &bool;
    fn ignore_suppression_mut(&mut self) -> &mut bool;
    fn button(&self) -> &InputDevicePadButtons;
    fn button_mut(&mut self) -> &mut InputDevicePadButtons;
    fn p_s3_alternative_button(&self) -> &InputDevicePadButtons;
    fn p_s3_alternative_button_mut(&mut self) -> &mut InputDevicePadButtons;
    fn p_s_vita_button(&self) -> &InputDevicePadButtons;
    fn p_s_vita_button_mut(&mut self) -> &mut InputDevicePadButtons;
    fn p_s_vita_alternative_button(&self) -> &InputDevicePadButtons;
    fn p_s_vita_alternative_button_mut(&mut self) -> &mut InputDevicePadButtons;
}

impl PadInputActionDataTrait for PadInputActionData {
    fn use_square_input(&self) -> &bool {
        &self.use_square_input
    }
    fn use_square_input_mut(&mut self) -> &mut bool {
        &mut self.use_square_input
    }
    fn ignore_suppression(&self) -> &bool {
        &self.ignore_suppression
    }
    fn ignore_suppression_mut(&mut self) -> &mut bool {
        &mut self.ignore_suppression
    }
    fn button(&self) -> &InputDevicePadButtons {
        &self.button
    }
    fn button_mut(&mut self) -> &mut InputDevicePadButtons {
        &mut self.button
    }
    fn p_s3_alternative_button(&self) -> &InputDevicePadButtons {
        &self.p_s3_alternative_button
    }
    fn p_s3_alternative_button_mut(&mut self) -> &mut InputDevicePadButtons {
        &mut self.p_s3_alternative_button
    }
    fn p_s_vita_button(&self) -> &InputDevicePadButtons {
        &self.p_s_vita_button
    }
    fn p_s_vita_button_mut(&mut self) -> &mut InputDevicePadButtons {
        &mut self.p_s_vita_button
    }
    fn p_s_vita_alternative_button(&self) -> &InputDevicePadButtons {
        &self.p_s_vita_alternative_button
    }
    fn p_s_vita_alternative_button_mut(&mut self) -> &mut InputDevicePadButtons {
        &mut self.p_s_vita_alternative_button
    }
}

impl AxesInputActionDataTrait for PadInputActionData {
    fn axis(&self) -> &InputDeviceAxes {
        self._glacier_base.axis()
    }
    fn axis_mut(&mut self) -> &mut InputDeviceAxes {
        self._glacier_base.axis_mut()
    }
    fn normalize_input(&self) -> &bool {
        self._glacier_base.normalize_input()
    }
    fn normalize_input_mut(&mut self) -> &mut bool {
        self._glacier_base.normalize_input_mut()
    }
}

impl InputActionDataTrait for PadInputActionData {
    fn is_analog(&self) -> &bool {
        self._glacier_base.is_analog()
    }
    fn is_analog_mut(&mut self) -> &mut bool {
        self._glacier_base.is_analog_mut()
    }
    fn negate_value(&self) -> &bool {
        self._glacier_base.negate_value()
    }
    fn negate_value_mut(&mut self) -> &mut bool {
        self._glacier_base.negate_value_mut()
    }
}

impl super::core::DataContainerTrait for PadInputActionData {
}

pub static PADINPUTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PadInputActionData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AXESINPUTACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PadInputActionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "UseSquareInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PadInputActionData, use_square_input),
            },
            FieldInfoData {
                name: "IgnoreSuppression",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PadInputActionData, ignore_suppression),
            },
            FieldInfoData {
                name: "Button",
                flags: MemberInfoFlags::new(0),
                field_type: "InputDevicePadButtons",
                rust_offset: offset_of!(PadInputActionData, button),
            },
            FieldInfoData {
                name: "PS3AlternativeButton",
                flags: MemberInfoFlags::new(0),
                field_type: "InputDevicePadButtons",
                rust_offset: offset_of!(PadInputActionData, p_s3_alternative_button),
            },
            FieldInfoData {
                name: "PSVitaButton",
                flags: MemberInfoFlags::new(0),
                field_type: "InputDevicePadButtons",
                rust_offset: offset_of!(PadInputActionData, p_s_vita_button),
            },
            FieldInfoData {
                name: "PSVitaAlternativeButton",
                flags: MemberInfoFlags::new(0),
                field_type: "InputDevicePadButtons",
                rust_offset: offset_of!(PadInputActionData, p_s_vita_alternative_button),
            },
        ],
    }),
    array_type: Some(PADINPUTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PadInputActionData {
    fn type_info(&self) -> &'static TypeInfo {
        PADINPUTACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PADINPUTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PadInputActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("PadInputActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AxesInputActionData {
    pub _glacier_base: InputActionData,
    pub axis: InputDeviceAxes,
    pub normalize_input: bool,
}

pub trait AxesInputActionDataTrait: InputActionDataTrait {
    fn axis(&self) -> &InputDeviceAxes;
    fn axis_mut(&mut self) -> &mut InputDeviceAxes;
    fn normalize_input(&self) -> &bool;
    fn normalize_input_mut(&mut self) -> &mut bool;
}

impl AxesInputActionDataTrait for AxesInputActionData {
    fn axis(&self) -> &InputDeviceAxes {
        &self.axis
    }
    fn axis_mut(&mut self) -> &mut InputDeviceAxes {
        &mut self.axis
    }
    fn normalize_input(&self) -> &bool {
        &self.normalize_input
    }
    fn normalize_input_mut(&mut self) -> &mut bool {
        &mut self.normalize_input
    }
}

impl InputActionDataTrait for AxesInputActionData {
    fn is_analog(&self) -> &bool {
        self._glacier_base.is_analog()
    }
    fn is_analog_mut(&mut self) -> &mut bool {
        self._glacier_base.is_analog_mut()
    }
    fn negate_value(&self) -> &bool {
        self._glacier_base.negate_value()
    }
    fn negate_value_mut(&mut self) -> &mut bool {
        self._glacier_base.negate_value_mut()
    }
}

impl super::core::DataContainerTrait for AxesInputActionData {
}

pub static AXESINPUTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxesInputActionData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(INPUTACTIONDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AxesInputActionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Axis",
                flags: MemberInfoFlags::new(0),
                field_type: "InputDeviceAxes",
                rust_offset: offset_of!(AxesInputActionData, axis),
            },
            FieldInfoData {
                name: "NormalizeInput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AxesInputActionData, normalize_input),
            },
        ],
    }),
    array_type: Some(AXESINPUTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AxesInputActionData {
    fn type_info(&self) -> &'static TypeInfo {
        AXESINPUTACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static AXESINPUTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AxesInputActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("AxesInputActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputActionData {
    pub _glacier_base: super::core::DataContainer,
    pub is_analog: bool,
    pub negate_value: bool,
}

pub trait InputActionDataTrait: super::core::DataContainerTrait {
    fn is_analog(&self) -> &bool;
    fn is_analog_mut(&mut self) -> &mut bool;
    fn negate_value(&self) -> &bool;
    fn negate_value_mut(&mut self) -> &mut bool;
}

impl InputActionDataTrait for InputActionData {
    fn is_analog(&self) -> &bool {
        &self.is_analog
    }
    fn is_analog_mut(&mut self) -> &mut bool {
        &mut self.is_analog
    }
    fn negate_value(&self) -> &bool {
        &self.negate_value
    }
    fn negate_value_mut(&mut self) -> &mut bool {
        &mut self.negate_value
    }
}

impl super::core::DataContainerTrait for InputActionData {
}

pub static INPUTACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputActionData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IsAnalog",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputActionData, is_analog),
            },
            FieldInfoData {
                name: "NegateValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(InputActionData, negate_value),
            },
        ],
    }),
    array_type: Some(INPUTACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputActionData {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTACTIONDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INPUTACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputActionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InputActionMapSlot {
    #[default]
    InputActionMapSlot_Undefined = 0,
    InputActionMapSlot_Root1 = 1,
    InputActionMapSlot_Root2 = 2,
    InputActionMapSlot_Root3 = 3,
    InputActionMapSlot_Root4 = 4,
    InputActionMapSlot_Root5 = 5,
    InputActionMapSlot_Root6 = 6,
    InputActionMapSlot_Root7 = 7,
    InputActionMapSlot_Root8 = 8,
    InputActionMapSlot_Root9 = 9,
    InputActionMapSlot_Root10 = 10,
    InputActionMapSlot_Root11 = 11,
    InputActionMapSlot_Root12 = 12,
    InputActionMapSlot_Root13 = 13,
    InputActionMapSlot_Root14 = 14,
    InputActionMapSlot_Root15 = 15,
    InputActionMapSlot_Root16 = 16,
    InputActionMapSlot_Sticks1 = 17,
    InputActionMapSlot_Sticks2 = 18,
    InputActionMapSlot_Sticks3 = 19,
    InputActionMapSlot_Sticks4 = 20,
    InputActionMapSlot_Buttons1 = 21,
    InputActionMapSlot_Buttons2 = 22,
    InputActionMapSlot_Buttons3 = 23,
    InputActionMapSlot_Buttons4 = 24,
    InputActionMapSlot_Sticks1Buttons1 = 25,
    InputActionMapSlot_Sticks1Buttons2 = 26,
    InputActionMapSlot_Sticks1Buttons3 = 27,
    InputActionMapSlot_Sticks1Buttons4 = 28,
    InputActionMapSlot_Sticks2Buttons1 = 29,
    InputActionMapSlot_Sticks2Buttons2 = 30,
    InputActionMapSlot_Sticks2Buttons3 = 31,
    InputActionMapSlot_Sticks2Buttons4 = 32,
    InputActionMapSlot_Sticks3Buttons1 = 33,
    InputActionMapSlot_Sticks3Buttons2 = 34,
    InputActionMapSlot_Sticks3Buttons3 = 35,
    InputActionMapSlot_Sticks3Buttons4 = 36,
    InputActionMapSlot_Sticks4Buttons1 = 37,
    InputActionMapSlot_Sticks4Buttons2 = 38,
    InputActionMapSlot_Sticks4Buttons3 = 39,
    InputActionMapSlot_Sticks4Buttons4 = 40,
    InputActionMapSlot_Count = 41,
}

pub static INPUTACTIONMAPSLOT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionMapSlot",
    flags: MemberInfoFlags::new(49429),
    module: "InputShared",
    data: TypeInfoData::Enum,
    array_type: Some(INPUTACTIONMAPSLOT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputActionMapSlot {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTACTIONMAPSLOT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INPUTACTIONMAPSLOT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionMapSlot-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputActionMapSlot"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InputActionMapPlatform {
    #[default]
    IAMPWin32 = 0,
    IAMPXenon = 1,
    IAMPPs3 = 2,
    IAMPMobile = 3,
    IAMPAllPlatforms = 4,
}

pub static INPUTACTIONMAPPLATFORM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionMapPlatform",
    flags: MemberInfoFlags::new(49429),
    module: "InputShared",
    data: TypeInfoData::Enum,
    array_type: Some(INPUTACTIONMAPPLATFORM_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputActionMapPlatform {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTACTIONMAPPLATFORM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INPUTACTIONMAPPLATFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputActionMapPlatform-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("InputActionMapPlatform"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EntryInputActionBindingsData {
    pub _glacier_base: super::core::Asset,
    pub bound_actions: Vec<i32>,
    pub entry_input_action_index_pairs: Vec<EntryInputActionIndexPair>,
    pub num_networked_analog_inputs: u32,
    pub num_analog_inputs: u32,
    pub first_digital: u32,
    pub num_networked_digital_inputs: u32,
    pub num_digital_inputs: u32,
    pub num_inputs: u32,
    pub signed_analog_inputs: u64,
}

pub trait EntryInputActionBindingsDataTrait: super::core::AssetTrait {
    fn bound_actions(&self) -> &Vec<i32>;
    fn bound_actions_mut(&mut self) -> &mut Vec<i32>;
    fn entry_input_action_index_pairs(&self) -> &Vec<EntryInputActionIndexPair>;
    fn entry_input_action_index_pairs_mut(&mut self) -> &mut Vec<EntryInputActionIndexPair>;
    fn num_networked_analog_inputs(&self) -> &u32;
    fn num_networked_analog_inputs_mut(&mut self) -> &mut u32;
    fn num_analog_inputs(&self) -> &u32;
    fn num_analog_inputs_mut(&mut self) -> &mut u32;
    fn first_digital(&self) -> &u32;
    fn first_digital_mut(&mut self) -> &mut u32;
    fn num_networked_digital_inputs(&self) -> &u32;
    fn num_networked_digital_inputs_mut(&mut self) -> &mut u32;
    fn num_digital_inputs(&self) -> &u32;
    fn num_digital_inputs_mut(&mut self) -> &mut u32;
    fn num_inputs(&self) -> &u32;
    fn num_inputs_mut(&mut self) -> &mut u32;
    fn signed_analog_inputs(&self) -> &u64;
    fn signed_analog_inputs_mut(&mut self) -> &mut u64;
}

impl EntryInputActionBindingsDataTrait for EntryInputActionBindingsData {
    fn bound_actions(&self) -> &Vec<i32> {
        &self.bound_actions
    }
    fn bound_actions_mut(&mut self) -> &mut Vec<i32> {
        &mut self.bound_actions
    }
    fn entry_input_action_index_pairs(&self) -> &Vec<EntryInputActionIndexPair> {
        &self.entry_input_action_index_pairs
    }
    fn entry_input_action_index_pairs_mut(&mut self) -> &mut Vec<EntryInputActionIndexPair> {
        &mut self.entry_input_action_index_pairs
    }
    fn num_networked_analog_inputs(&self) -> &u32 {
        &self.num_networked_analog_inputs
    }
    fn num_networked_analog_inputs_mut(&mut self) -> &mut u32 {
        &mut self.num_networked_analog_inputs
    }
    fn num_analog_inputs(&self) -> &u32 {
        &self.num_analog_inputs
    }
    fn num_analog_inputs_mut(&mut self) -> &mut u32 {
        &mut self.num_analog_inputs
    }
    fn first_digital(&self) -> &u32 {
        &self.first_digital
    }
    fn first_digital_mut(&mut self) -> &mut u32 {
        &mut self.first_digital
    }
    fn num_networked_digital_inputs(&self) -> &u32 {
        &self.num_networked_digital_inputs
    }
    fn num_networked_digital_inputs_mut(&mut self) -> &mut u32 {
        &mut self.num_networked_digital_inputs
    }
    fn num_digital_inputs(&self) -> &u32 {
        &self.num_digital_inputs
    }
    fn num_digital_inputs_mut(&mut self) -> &mut u32 {
        &mut self.num_digital_inputs
    }
    fn num_inputs(&self) -> &u32 {
        &self.num_inputs
    }
    fn num_inputs_mut(&mut self) -> &mut u32 {
        &mut self.num_inputs
    }
    fn signed_analog_inputs(&self) -> &u64 {
        &self.signed_analog_inputs
    }
    fn signed_analog_inputs_mut(&mut self) -> &mut u64 {
        &mut self.signed_analog_inputs
    }
}

impl super::core::AssetTrait for EntryInputActionBindingsData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EntryInputActionBindingsData {
}

pub static ENTRYINPUTACTIONBINDINGSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryInputActionBindingsData",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntryInputActionBindingsData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BoundActions",
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(EntryInputActionBindingsData, bound_actions),
            },
            FieldInfoData {
                name: "EntryInputActionIndexPairs",
                flags: MemberInfoFlags::new(144),
                field_type: "EntryInputActionIndexPair-Array",
                rust_offset: offset_of!(EntryInputActionBindingsData, entry_input_action_index_pairs),
            },
            FieldInfoData {
                name: "NumNetworkedAnalogInputs",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EntryInputActionBindingsData, num_networked_analog_inputs),
            },
            FieldInfoData {
                name: "NumAnalogInputs",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EntryInputActionBindingsData, num_analog_inputs),
            },
            FieldInfoData {
                name: "FirstDigital",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EntryInputActionBindingsData, first_digital),
            },
            FieldInfoData {
                name: "NumNetworkedDigitalInputs",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EntryInputActionBindingsData, num_networked_digital_inputs),
            },
            FieldInfoData {
                name: "NumDigitalInputs",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EntryInputActionBindingsData, num_digital_inputs),
            },
            FieldInfoData {
                name: "NumInputs",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EntryInputActionBindingsData, num_inputs),
            },
            FieldInfoData {
                name: "SignedAnalogInputs",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(EntryInputActionBindingsData, signed_analog_inputs),
            },
        ],
    }),
    array_type: Some(ENTRYINPUTACTIONBINDINGSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntryInputActionBindingsData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYINPUTACTIONBINDINGSDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static ENTRYINPUTACTIONBINDINGSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryInputActionBindingsData-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("EntryInputActionBindingsData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EntryInputActionBinding {
    pub action: i32,
    pub alias: i32,
    pub action_type: EntryInputActionType,
    pub networked: bool,
}

pub trait EntryInputActionBindingTrait: TypeObject {
    fn action(&self) -> &i32;
    fn action_mut(&mut self) -> &mut i32;
    fn alias(&self) -> &i32;
    fn alias_mut(&mut self) -> &mut i32;
    fn action_type(&self) -> &EntryInputActionType;
    fn action_type_mut(&mut self) -> &mut EntryInputActionType;
    fn networked(&self) -> &bool;
    fn networked_mut(&mut self) -> &mut bool;
}

impl EntryInputActionBindingTrait for EntryInputActionBinding {
    fn action(&self) -> &i32 {
        &self.action
    }
    fn action_mut(&mut self) -> &mut i32 {
        &mut self.action
    }
    fn alias(&self) -> &i32 {
        &self.alias
    }
    fn alias_mut(&mut self) -> &mut i32 {
        &mut self.alias
    }
    fn action_type(&self) -> &EntryInputActionType {
        &self.action_type
    }
    fn action_type_mut(&mut self) -> &mut EntryInputActionType {
        &mut self.action_type
    }
    fn networked(&self) -> &bool {
        &self.networked
    }
    fn networked_mut(&mut self) -> &mut bool {
        &mut self.networked
    }
}

pub static ENTRYINPUTACTIONBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryInputActionBinding",
    flags: MemberInfoFlags::new(36937),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntryInputActionBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Action",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EntryInputActionBinding, action),
            },
            FieldInfoData {
                name: "Alias",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EntryInputActionBinding, alias),
            },
            FieldInfoData {
                name: "ActionType",
                flags: MemberInfoFlags::new(0),
                field_type: "EntryInputActionType",
                rust_offset: offset_of!(EntryInputActionBinding, action_type),
            },
            FieldInfoData {
                name: "Networked",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntryInputActionBinding, networked),
            },
        ],
    }),
    array_type: Some(ENTRYINPUTACTIONBINDING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EntryInputActionBinding {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYINPUTACTIONBINDING_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static ENTRYINPUTACTIONBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryInputActionBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("EntryInputActionBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EntryInputActionType {
    #[default]
    EntryInputActionTypeAnalog = 0,
    EntryInputActionTypeAnalogSigned = 1,
    EntryInputActionTypeDigital = 2,
}

pub static ENTRYINPUTACTIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryInputActionType",
    flags: MemberInfoFlags::new(49429),
    module: "InputShared",
    data: TypeInfoData::Enum,
    array_type: Some(ENTRYINPUTACTIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntryInputActionType {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYINPUTACTIONTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static ENTRYINPUTACTIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryInputActionType-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("EntryInputActionType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EntryInputActionIndexPair {
    pub key: i32,
    pub value: u32,
}

pub trait EntryInputActionIndexPairTrait: TypeObject {
    fn key(&self) -> &i32;
    fn key_mut(&mut self) -> &mut i32;
    fn value(&self) -> &u32;
    fn value_mut(&mut self) -> &mut u32;
}

impl EntryInputActionIndexPairTrait for EntryInputActionIndexPair {
    fn key(&self) -> &i32 {
        &self.key
    }
    fn key_mut(&mut self) -> &mut i32 {
        &mut self.key
    }
    fn value(&self) -> &u32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut u32 {
        &mut self.value
    }
}

pub static ENTRYINPUTACTIONINDEXPAIR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryInputActionIndexPair",
    flags: MemberInfoFlags::new(36937),
    module: "InputShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntryInputActionIndexPair as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Key",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EntryInputActionIndexPair, key),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(EntryInputActionIndexPair, value),
            },
        ],
    }),
    array_type: Some(ENTRYINPUTACTIONINDEXPAIR_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EntryInputActionIndexPair {
    fn type_info(&self) -> &'static TypeInfo {
        ENTRYINPUTACTIONINDEXPAIR_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static ENTRYINPUTACTIONINDEXPAIR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntryInputActionIndexPair-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("EntryInputActionIndexPair"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BaseInputSettings {
    pub _glacier_base: super::core::DataContainer,
}

pub trait BaseInputSettingsTrait: super::core::DataContainerTrait {
}

impl BaseInputSettingsTrait for BaseInputSettings {
}

impl super::core::DataContainerTrait for BaseInputSettings {
}

pub static BASEINPUTSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseInputSettings",
    flags: MemberInfoFlags::new(101),
    module: "InputShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BaseInputSettings as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BASEINPUTSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BaseInputSettings {
    fn type_info(&self) -> &'static TypeInfo {
        BASEINPUTSETTINGS_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static BASEINPUTSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseInputSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "InputShared",
    data: TypeInfoData::Array("BaseInputSettings"),
    array_type: None,
    alignment: 8,
};


