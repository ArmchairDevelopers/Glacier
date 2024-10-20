use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_telemetry_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(TELEMETRYSDKPINTRANSPORTDATA_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINTRANSPORTDATA_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINEVENTHEADERCONFIG_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINEVENTHEADERCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINSESSIONHEADERCONFIG_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINSESSIONHEADERCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINENDPOINTCONFIG_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINENDPOINTCONFIG_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYCSVTRANSPORTDATA_TYPE_INFO);
    registry.register_type(TELEMETRYCSVTRANSPORTDATA_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYTTYTRANSPORTDATA_TYPE_INFO);
    registry.register_type(TELEMETRYTTYTRANSPORTDATA_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDK3TRANSPORTDATA_TYPE_INFO);
    registry.register_type(TELEMETRYSDK3TRANSPORTDATA_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYTRANSPORTDATA_TYPE_INFO);
    registry.register_type(TELEMETRYTRANSPORTDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSACTIONALSTREAMDATA_TYPE_INFO);
    registry.register_type(TRANSACTIONALSTREAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(VAREVENTSTREAMDATA_TYPE_INFO);
    registry.register_type(VAREVENTSTREAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(FIXEDEVENTSTREAMDATA_TYPE_INFO);
    registry.register_type(FIXEDEVENTSTREAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(EVENTSTREAMDATA_TYPE_INFO);
    registry.register_type(EVENTSTREAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(STREAMDATA_TYPE_INFO);
    registry.register_type(STREAMDATA_ARRAY_TYPE_INFO);
    registry.register_type(FIXEDTRANSACTIONALSTREAMFORMAT_TYPE_INFO);
    registry.register_type(FIXEDTRANSACTIONALSTREAMFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(TRANSACTIONALTELEMETRYSTREAMFORMAT_TYPE_INFO);
    registry.register_type(TRANSACTIONALTELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYTRANSACTIONMODE_TYPE_INFO);
    registry.register_type(TELEMETRYTRANSACTIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(VARIABLEEVENTSTREAMFORMAT_TYPE_INFO);
    registry.register_type(VARIABLEEVENTSTREAMFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(FIXEDEVENTSTREAMFORMAT_TYPE_INFO);
    registry.register_type(FIXEDEVENTSTREAMFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(EVENTTELEMETRYSTREAMFORMAT_TYPE_INFO);
    registry.register_type(EVENTTELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSTREAMFORMAT_TYPE_INFO);
    registry.register_type(TELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSETTINGS_TYPE_INFO);
    registry.register_type(TELEMETRYSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYTRANSACTIONDATA_TYPE_INFO);
    registry.register_type(TELEMETRYTRANSACTIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDK3EVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDK3EVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYLOGEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYLOGEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYROWDATA_TYPE_INFO);
    registry.register_type(TELEMETRYROWDATA_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINFRIENDNETWORKEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINFRIENDNETWORKEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINCHALLENGEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINCHALLENGEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINSOCMESSAGEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINSOCMESSAGEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINGROUPEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINGROUPEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINFRIENDSEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINFRIENDSEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINCUSTOMERROREVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINCUSTOMERROREVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINCONNECTIONEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINCONNECTIONEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINERROREVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINERROREVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMATCHINFOEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMATCHINFOEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMATCHJOINEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMATCHJOINEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYPINEVENTHEADERCONFIGURATIONMESSAGEBASE_TYPE_INFO);
    registry.register_type(TELEMETRYPINSESSIONHEADERCONFIGURATIONMESSAGEBASE_TYPE_INFO);
    registry.register_type(TELEMETRYPINENDPOINTCONFIGURATIONMESSAGEBASE_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERSTATEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERSTATEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCPARTYEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCPARTYEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCINTERACTIONEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCINTERACTIONEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCRELATIONSHIPEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCRELATIONSHIPEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINCINEMATICEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINCINEMATICEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERDECISIONEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERDECISIONEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINSCOREEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINSCOREEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCOUTRESOURCEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCOUTRESOURCEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCINTERACTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCINTERACTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCDEATHEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCDEATHEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCSTATEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCSTATEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINGAMEOBJECTIVEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINGAMEOBJECTIVEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINFPSEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINFPSEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINGAMEACTIONEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINGAMEACTIONEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINVEHICLEHEALTHEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINVEHICLEHEALTHEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINVEHICLEINTERACTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINVEHICLEINTERACTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINVEHICLEDESTRUCTIONEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINVEHICLEDESTRUCTIONEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINVEHICLESPAWNEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINVEHICLESPAWNEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINTAGEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINTAGEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINITEMUPGRADEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINITEMUPGRADEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINITEMUNLOCKEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINITEMUNLOCKEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINITEMPICKUPEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINITEMPICKUPEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINITEMSPAWNEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINITEMSPAWNEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINITEMHEALTHEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINITEMHEALTHEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYEROUTRESOURCEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYEROUTRESOURCEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERTEAMSWITCHEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERTEAMSWITCHEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERINTERACTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERINTERACTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERABILITYAFFECTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERABILITYAFFECTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERABILITYEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERABILITYEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERUSEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERUSEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERKILLASSISTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERKILLASSISTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERHEALTHEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERHEALTHEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERDEATHEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERDEATHEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYEREQUIPEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYEREQUIPEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERCLASSEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERCLASSEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERSTATEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERSTATEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCSPAWNEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINNPCSPAWNEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERSPAWNEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERSPAWNEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERTICKEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERTICKEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINTRANSCTIONEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINTRANSCTIONEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPININVENTORYEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPININVENTORYEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINSURVEYEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINSURVEYEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINHEARTBEATEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINHEARTBEATEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMODEEXITEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMODEEXITEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMODEENTEREVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMODEENTEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYSESSIONENDEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYSESSIONENDEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYSESSIONSTARTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYSESSIONSTARTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINROUNDENDEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINROUNDENDEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINROUNDSTARTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINROUNDSTARTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINGAMEENDEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINGAMEENDEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINGAMESTARTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINGAMESTARTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINLOGOUTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINLOGOUTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINLOGINEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINLOGINEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINBOOTENDEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINBOOTENDEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINBOOTSTARTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINBOOTSTARTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERSERVICEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERSERVICEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERSTATSEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERSTATSEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINCAMERASTATEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINCAMERASTATEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYEROBEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYEROBEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINUIINTERACTIONEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINUIINTERACTIONEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINHARDWAREPROFILEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINHARDWAREPROFILEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERWEAPSMRYEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERWEAPSMRYEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINTIMEREVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINTIMEREVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMESSAGEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMESSAGEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPAGEVIEWEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPAGEVIEWEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINACHIEVEMENTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINACHIEVEMENTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMILESTONEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINMILESTONEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERLEVELEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINPLAYERLEVELEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINFAVORITEEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINFAVORITEEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINDOWNLOADEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINDOWNLOADEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINSETTINGSEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINSETTINGSEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINREGISTRATIONEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINREGISTRATIONEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINACCOUNTEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINACCOUNTEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINEVENT_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINEVENT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINEVENTHEADERMODIFIER_TYPE_INFO);
    registry.register_type(TELEMETRYSDKPINEVENTHEADERMODIFIER_ARRAY_TYPE_INFO);
    registry.register_type(TRANSACTIONALTELEMETRYHOOKENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSACTIONALTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VARSTREAMTELEMETRYHOOKENTITYDATA_TYPE_INFO);
    registry.register_type(VARSTREAMTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FIXEDSTREAMTELEMETRYHOOKENTITYDATA_TYPE_INFO);
    registry.register_type(FIXEDSTREAMTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKENTITYDATA_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYGENERICHOOKENTITYDATA_TYPE_INFO);
    registry.register_type(TELEMETRYGENERICHOOKENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYCLEARTELEMETRYTRANSACTIONMESSAGEBASE_TYPE_INFO);
    registry.register_type(TELEMETRYCOMMITTELEMETRYTRANSACTIONMESSAGEBASE_TYPE_INFO);
    registry.register_type(TELEMETRYSENDTELEMETRYTRANSACTIONROWMESSAGEBASE_TYPE_INFO);
    registry.register_type(TELEMETRYSENDTELEMETRYROWMESSAGEBASE_TYPE_INFO);
    registry.register_type(TELEMETRYSENDEVENTMESSAGE_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERSTRINGARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERSTRINGARRAY_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERRAWJSONSTRING_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERRAWJSONSTRING_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERTRANSFORM_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERTRANSFORM_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERVEC4_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERVEC4_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERVEC3_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERVEC3_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERVEC2_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERVEC2_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERBOOL_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERBOOL_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERUINT64_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERUINT64_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERUINT_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERUINT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERSTRING_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERSTRING_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERFLOAT_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERFLOAT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERINT_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERINT_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERCHAR_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETERCHAR_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETER_TYPE_INFO);
    registry.register_type(TELEMETRYHOOKPARAMETER_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYPARAMETERDATAPROPERTY_TYPE_INFO);
    registry.register_type(TELEMETRYPARAMETERDATAPROPERTY_ARRAY_TYPE_INFO);
    registry.register_type(RAWJSONSTRING_TYPE_INFO);
    registry.register_type(RAWJSONSTRING_ARRAY_TYPE_INFO);
    registry.register_type(SPECIALTYPEDATA_TYPE_INFO);
    registry.register_type(SPECIALTYPEDATA_ARRAY_TYPE_INFO);
    registry.register_type(TELEMETRYPARAMETERTYPE_TYPE_INFO);
    registry.register_type(TELEMETRYPARAMETERTYPE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySDKPinTransportData {
    pub end_point_config: TelemetrySDKPinEndPointConfig,
    pub log_level: i32,
    pub session_header_config: TelemetrySDKPinSessionHeaderConfig,
    pub event_header_config: TelemetrySDKPinEventHeaderConfig,
}

pub const TELEMETRYSDKPINTRANSPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinTransportData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYTRANSPORTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "EndPointConfig",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYSDKPINENDPOINTCONFIG_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinTransportData, end_point_config),
            },
            FieldInfoData {
                name: "logLevel",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinTransportData, log_level),
            },
            FieldInfoData {
                name: "SessionHeaderConfig",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYSDKPINSESSIONHEADERCONFIG_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinTransportData, session_header_config),
            },
            FieldInfoData {
                name: "EventHeaderConfig",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYSDKPINEVENTHEADERCONFIG_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinTransportData, event_header_config),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINTRANSPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDKPinTransportData {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINTRANSPORTDATA_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINTRANSPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinTransportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinTransportData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySDKPinEventHeaderConfig {
    pub other_player_ids: String,
    pub date_of_birth: String,
    pub experiment_id: String,
    pub player_id_type: String,
    pub player_id: String,
    pub title_id_type: String,
    pub title_id: String,
    pub release_type: String,
    pub platform: String,
    pub mac_address: String,
    pub device_id_map: String,
    pub game_mode: String,
    pub game_type: String,
    pub mode_type: String,
    pub map: String,
    pub level: String,
    pub level_name: String,
    pub is_sess: String,
    pub is_player: String,
    pub is_mlu: String,
    pub subs: String,
    pub custom_event_headers: String,
}

pub const TELEMETRYSDKPINEVENTHEADERCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEventHeaderConfig",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "OtherPlayerIds",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, other_player_ids),
            },
            FieldInfoData {
                name: "DateOfBirth",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, date_of_birth),
            },
            FieldInfoData {
                name: "ExperimentId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, experiment_id),
            },
            FieldInfoData {
                name: "PlayerIdType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, player_id_type),
            },
            FieldInfoData {
                name: "PlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, player_id),
            },
            FieldInfoData {
                name: "TitleIdType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, title_id_type),
            },
            FieldInfoData {
                name: "TitleId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, title_id),
            },
            FieldInfoData {
                name: "ReleaseType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, release_type),
            },
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, platform),
            },
            FieldInfoData {
                name: "MacAddress",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, mac_address),
            },
            FieldInfoData {
                name: "DeviceIdMap",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, device_id_map),
            },
            FieldInfoData {
                name: "GameMode",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, game_mode),
            },
            FieldInfoData {
                name: "GameType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, game_type),
            },
            FieldInfoData {
                name: "ModeType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, mode_type),
            },
            FieldInfoData {
                name: "Map",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, map),
            },
            FieldInfoData {
                name: "Level",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, level),
            },
            FieldInfoData {
                name: "LevelName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, level_name),
            },
            FieldInfoData {
                name: "IsSess",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, is_sess),
            },
            FieldInfoData {
                name: "IsPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, is_player),
            },
            FieldInfoData {
                name: "IsMlu",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, is_mlu),
            },
            FieldInfoData {
                name: "Subs",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, subs),
            },
            FieldInfoData {
                name: "CustomEventHeaders",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, custom_event_headers),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINEVENTHEADERCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDKPinEventHeaderConfig {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINEVENTHEADERCONFIG_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINEVENTHEADERCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEventHeaderConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinEventHeaderConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySDKPinSessionHeaderConfig {
    pub event_type: String,
    pub build_version: String,
    pub locale: String,
    pub custom_session_headers: String,
    pub is_mlu: String,
    pub subs: String,
    pub player_id_type: String,
    pub player_id: String,
    pub title_id_type: String,
    pub title_id: String,
    pub release_type: String,
    pub platform: String,
    pub mac_address: String,
    pub device_id_map: String,
}

pub const TELEMETRYSDKPINSESSIONHEADERCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinSessionHeaderConfig",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "EventType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, event_type),
            },
            FieldInfoData {
                name: "BuildVersion",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, build_version),
            },
            FieldInfoData {
                name: "Locale",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, locale),
            },
            FieldInfoData {
                name: "CustomSessionHeaders",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, custom_session_headers),
            },
            FieldInfoData {
                name: "IsMlu",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, is_mlu),
            },
            FieldInfoData {
                name: "Subs",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, subs),
            },
            FieldInfoData {
                name: "PlayerIdType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, player_id_type),
            },
            FieldInfoData {
                name: "PlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, player_id),
            },
            FieldInfoData {
                name: "TitleIdType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, title_id_type),
            },
            FieldInfoData {
                name: "TitleId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, title_id),
            },
            FieldInfoData {
                name: "ReleaseType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, release_type),
            },
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, platform),
            },
            FieldInfoData {
                name: "MacAddress",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, mac_address),
            },
            FieldInfoData {
                name: "DeviceIdMap",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, device_id_map),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINSESSIONHEADERCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDKPinSessionHeaderConfig {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINSESSIONHEADERCONFIG_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINSESSIONHEADERCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinSessionHeaderConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinSessionHeaderConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySDKPinEndPointConfig {
    pub server_address: String,
    pub server_port: i32,
    pub environment: String,
}

pub const TELEMETRYSDKPINENDPOINTCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEndPointConfig",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ServerAddress",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEndPointConfig, server_address),
            },
            FieldInfoData {
                name: "ServerPort",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEndPointConfig, server_port),
            },
            FieldInfoData {
                name: "Environment",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEndPointConfig, environment),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINENDPOINTCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDKPinEndPointConfig {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINENDPOINTCONFIG_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINENDPOINTCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEndPointConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinEndPointConfig-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryCSVTransportData {
    pub file_name: String,
    pub time_stamped: bool,
    pub writes_per_flush: u32,
    pub overwrite_file: bool,
}

pub const TELEMETRYCSVTRANSPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryCSVTransportData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYTRANSPORTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FileName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryCSVTransportData, file_name),
            },
            FieldInfoData {
                name: "TimeStamped",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetryCSVTransportData, time_stamped),
            },
            FieldInfoData {
                name: "WritesPerFlush",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetryCSVTransportData, writes_per_flush),
            },
            FieldInfoData {
                name: "OverwriteFile",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetryCSVTransportData, overwrite_file),
            },
        ],
    }),
    array_type: Some(TELEMETRYCSVTRANSPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryCSVTransportData {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYCSVTRANSPORTDATA_TYPE_INFO
    }
}


pub const TELEMETRYCSVTRANSPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryCSVTransportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryCSVTransportData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryTTYTransportData {
    pub max_buffer: u32,
}

pub const TELEMETRYTTYTRANSPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTTYTransportData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYTRANSPORTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "MaxBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetryTTYTransportData, max_buffer),
            },
        ],
    }),
    array_type: Some(TELEMETRYTTYTRANSPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryTTYTransportData {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYTTYTRANSPORTDATA_TYPE_INFO
    }
}


pub const TELEMETRYTTYTRANSPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTTYTransportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryTTYTransportData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySDK3TransportData {
    pub is_production: bool,
    pub project_id: u32,
    pub version_name: String,
    pub log_level: i32,
}

pub const TELEMETRYSDK3TRANSPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDK3TransportData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYTRANSPORTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "IsProduction",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDK3TransportData, is_production),
            },
            FieldInfoData {
                name: "ProjectId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDK3TransportData, project_id),
            },
            FieldInfoData {
                name: "VersionName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDK3TransportData, version_name),
            },
            FieldInfoData {
                name: "logLevel",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDK3TransportData, log_level),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDK3TRANSPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDK3TransportData {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDK3TRANSPORTDATA_TYPE_INFO
    }
}


pub const TELEMETRYSDK3TRANSPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDK3TransportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDK3TransportData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryTransportData {
    pub transport_id: u32,
}

pub const TELEMETRYTRANSPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransportData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TransportId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetryTransportData, transport_id),
            },
        ],
    }),
    array_type: Some(TELEMETRYTRANSPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryTransportData {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYTRANSPORTDATA_TYPE_INFO
    }
}


pub const TELEMETRYTRANSPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryTransportData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransactionalStreamData {
    pub format: FixedTransactionalStreamFormat,
    pub auto_commit: bool,
}

pub const TRANSACTIONALSTREAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalStreamData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STREAMDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Format",
                flags: MemberInfoFlags::new(0),
                field_type: FIXEDTRANSACTIONALSTREAMFORMAT_TYPE_INFO,
                rust_offset: offset_of!(TransactionalStreamData, format),
            },
            FieldInfoData {
                name: "AutoCommit",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransactionalStreamData, auto_commit),
            },
        ],
    }),
    array_type: Some(TRANSACTIONALSTREAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TransactionalStreamData {
    fn type_info() -> &'static TypeInfo {
        TRANSACTIONALSTREAMDATA_TYPE_INFO
    }
}


pub const TRANSACTIONALSTREAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalStreamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TransactionalStreamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VarEventStreamData {
    pub format: VariableEventStreamFormat,
}

pub const VAREVENTSTREAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarEventStreamData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVENTSTREAMDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Format",
                flags: MemberInfoFlags::new(0),
                field_type: VARIABLEEVENTSTREAMFORMAT_TYPE_INFO,
                rust_offset: offset_of!(VarEventStreamData, format),
            },
        ],
    }),
    array_type: Some(VAREVENTSTREAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VarEventStreamData {
    fn type_info() -> &'static TypeInfo {
        VAREVENTSTREAMDATA_TYPE_INFO
    }
}


pub const VAREVENTSTREAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarEventStreamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("VarEventStreamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FixedEventStreamData {
    pub format: FixedEventStreamFormat,
}

pub const FIXEDEVENTSTREAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedEventStreamData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVENTSTREAMDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Format",
                flags: MemberInfoFlags::new(0),
                field_type: FIXEDEVENTSTREAMFORMAT_TYPE_INFO,
                rust_offset: offset_of!(FixedEventStreamData, format),
            },
        ],
    }),
    array_type: Some(FIXEDEVENTSTREAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FixedEventStreamData {
    fn type_info() -> &'static TypeInfo {
        FIXEDEVENTSTREAMDATA_TYPE_INFO
    }
}


pub const FIXEDEVENTSTREAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedEventStreamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("FixedEventStreamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventStreamData {
}

pub const EVENTSTREAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventStreamData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STREAMDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EVENTSTREAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventStreamData {
    fn type_info() -> &'static TypeInfo {
        EVENTSTREAMDATA_TYPE_INFO
    }
}


pub const EVENTSTREAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventStreamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("EventStreamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamData {
    pub transports: Vec<TelemetryTransportData>,
    pub platform: super::core::GamePlatform,
    pub stream_id: u32,
}

pub const STREAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Transports",
                flags: MemberInfoFlags::new(144),
                field_type: TELEMETRYTRANSPORTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(StreamData, transports),
            },
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(StreamData, platform),
            },
            FieldInfoData {
                name: "StreamId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(StreamData, stream_id),
            },
        ],
    }),
    array_type: Some(STREAMDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StreamData {
    fn type_info() -> &'static TypeInfo {
        STREAMDATA_TYPE_INFO
    }
}


pub const STREAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("StreamData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FixedTransactionalStreamFormat {
    pub reference_row: TelemetryTransactionData,
}

pub const FIXEDTRANSACTIONALSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedTransactionalStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSACTIONALTELEMETRYSTREAMFORMAT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ReferenceRow",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYTRANSACTIONDATA_TYPE_INFO,
                rust_offset: offset_of!(FixedTransactionalStreamFormat, reference_row),
            },
        ],
    }),
    array_type: Some(FIXEDTRANSACTIONALSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FixedTransactionalStreamFormat {
    fn type_info() -> &'static TypeInfo {
        FIXEDTRANSACTIONALSTREAMFORMAT_TYPE_INFO
    }
}


pub const FIXEDTRANSACTIONALSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedTransactionalStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("FixedTransactionalStreamFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransactionalTelemetryStreamFormat {
    pub transaction_mode: TelemetryTransactionMode,
    pub autocommit: bool,
}

pub const TRANSACTIONALTELEMETRYSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSTREAMFORMAT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TransactionMode",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYTRANSACTIONMODE_TYPE_INFO,
                rust_offset: offset_of!(TransactionalTelemetryStreamFormat, transaction_mode),
            },
            FieldInfoData {
                name: "Autocommit",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TransactionalTelemetryStreamFormat, autocommit),
            },
        ],
    }),
    array_type: Some(TRANSACTIONALTELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransactionalTelemetryStreamFormat {
    fn type_info() -> &'static TypeInfo {
        TRANSACTIONALTELEMETRYSTREAMFORMAT_TYPE_INFO
    }
}


pub const TRANSACTIONALTELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TransactionalTelemetryStreamFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TelemetryTransactionMode {
    #[default]
    TelemetryTransactionMode_Binary = 0,
    TelemetryTransactionMode_String = 1,
}

pub const TELEMETRYTRANSACTIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransactionMode",
    flags: MemberInfoFlags::new(49429),
    module: "TelemetryShared",
    data: TypeInfoData::Enum,
    array_type: Some(TELEMETRYTRANSACTIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TelemetryTransactionMode {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYTRANSACTIONMODE_TYPE_INFO
    }
}


pub const TELEMETRYTRANSACTIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransactionMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryTransactionMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VariableEventStreamFormat {
    pub r#mod: String,
    pub grp: String,
    pub subgrp: String,
    pub params: Vec<TelemetryParameterDataProperty>,
}

pub const VARIABLEEVENTSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VariableEventStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVENTTELEMETRYSTREAMFORMAT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "mod",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VariableEventStreamFormat, r#mod),
            },
            FieldInfoData {
                name: "grp",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VariableEventStreamFormat, grp),
            },
            FieldInfoData {
                name: "subgrp",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VariableEventStreamFormat, subgrp),
            },
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(144),
                field_type: TELEMETRYPARAMETERDATAPROPERTY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(VariableEventStreamFormat, params),
            },
        ],
    }),
    array_type: Some(VARIABLEEVENTSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VariableEventStreamFormat {
    fn type_info() -> &'static TypeInfo {
        VARIABLEEVENTSTREAMFORMAT_TYPE_INFO
    }
}


pub const VARIABLEEVENTSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VariableEventStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("VariableEventStreamFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FixedEventStreamFormat {
    pub reference_event: TelemetryLogEvent,
}

pub const FIXEDEVENTSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedEventStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVENTTELEMETRYSTREAMFORMAT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ReferenceEvent",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYLOGEVENT_TYPE_INFO,
                rust_offset: offset_of!(FixedEventStreamFormat, reference_event),
            },
        ],
    }),
    array_type: Some(FIXEDEVENTSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FixedEventStreamFormat {
    fn type_info() -> &'static TypeInfo {
        FIXEDEVENTSTREAMFORMAT_TYPE_INFO
    }
}


pub const FIXEDEVENTSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedEventStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("FixedEventStreamFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventTelemetryStreamFormat {
}

pub const EVENTTELEMETRYSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTelemetryStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSTREAMFORMAT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EVENTTELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventTelemetryStreamFormat {
    fn type_info() -> &'static TypeInfo {
        EVENTTELEMETRYSTREAMFORMAT_TYPE_INFO
    }
}


pub const EVENTTELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTelemetryStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("EventTelemetryStreamFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryStreamFormat {
}

pub const TELEMETRYSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryStreamFormat {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSTREAMFORMAT_TYPE_INFO
    }
}


pub const TELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryStreamFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySettings {
    pub stream_formats: Vec<TelemetryStreamFormat>,
    pub transports: Vec<TelemetryTransportData>,
    pub streams: Vec<StreamData>,
    pub file_location: String,
}

pub const TELEMETRYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySettings",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "StreamFormats",
                flags: MemberInfoFlags::new(144),
                field_type: TELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySettings, stream_formats),
            },
            FieldInfoData {
                name: "Transports",
                flags: MemberInfoFlags::new(144),
                field_type: TELEMETRYTRANSPORTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySettings, transports),
            },
            FieldInfoData {
                name: "Streams",
                flags: MemberInfoFlags::new(144),
                field_type: STREAMDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySettings, streams),
            },
            FieldInfoData {
                name: "FileLocation",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySettings, file_location),
            },
        ],
    }),
    array_type: Some(TELEMETRYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySettings {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSETTINGS_TYPE_INFO
    }
}


pub const TELEMETRYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryTransactionData {
}

pub const TELEMETRYTRANSACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransactionData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYROWDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYTRANSACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryTransactionData {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYTRANSACTIONDATA_TYPE_INFO
    }
}


pub const TELEMETRYTRANSACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransactionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryTransactionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdk3Event {
    pub module_id: u32,
    pub group_id: u32,
    pub string_id: u32,
}

pub const TELEMETRYSDK3EVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdk3Event",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYLOGEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ModuleId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdk3Event, module_id),
            },
            FieldInfoData {
                name: "GroupId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdk3Event, group_id),
            },
            FieldInfoData {
                name: "StringId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdk3Event, string_id),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDK3EVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdk3Event {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDK3EVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDK3EVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdk3Event-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdk3Event-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryLogEvent {
}

pub const TELEMETRYLOGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryLogEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYROWDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYLOGEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryLogEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYLOGEVENT_TYPE_INFO
    }
}


pub const TELEMETRYLOGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryLogEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryLogEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryRowData {
}

pub const TELEMETRYROWDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryRowData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYROWDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryRowData {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYROWDATA_TYPE_INFO
    }
}


pub const TELEMETRYROWDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryRowData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryRowData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinFriendNetworkEvent {
    pub total_friends: u32,
    pub friends_online: u32,
    pub friends_same_title: u32,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINFRIENDNETWORKEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFriendNetworkEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "total_friends",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFriendNetworkEvent, total_friends),
            },
            FieldInfoData {
                name: "friends_online",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFriendNetworkEvent, friends_online),
            },
            FieldInfoData {
                name: "friends_same_title",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFriendNetworkEvent, friends_same_title),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFriendNetworkEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINFRIENDNETWORKEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinFriendNetworkEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINFRIENDNETWORKEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINFRIENDNETWORKEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFriendNetworkEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinFriendNetworkEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinChallengeEvent {
    pub r#type: String,
    pub status: String,
    pub status_code: String,
    pub challenge_id: String,
    pub recipient_id: String,
    pub recipient_type: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINCHALLENGEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinChallengeEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, r#type),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, status_code),
            },
            FieldInfoData {
                name: "challenge_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, challenge_id),
            },
            FieldInfoData {
                name: "recipient_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, recipient_id),
            },
            FieldInfoData {
                name: "recipient_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, recipient_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINCHALLENGEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinChallengeEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINCHALLENGEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINCHALLENGEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinChallengeEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinChallengeEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinSocMessageEvent {
    pub r#type: String,
    pub placement: String,
    pub content_type: String,
    pub network: String,
    pub format: String,
    pub status: String,
    pub status_code: String,
    pub message_id: String,
    pub recipient_id: Vec<String>,
    pub recipient_type: String,
    pub items: RawJsonString,
    pub event_id: String,
    pub event_name: String,
    pub event_type: String,
    pub count: i64,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINSOCMESSAGEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSocMessageEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, r#type),
            },
            FieldInfoData {
                name: "placement",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, placement),
            },
            FieldInfoData {
                name: "content_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, content_type),
            },
            FieldInfoData {
                name: "network",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, network),
            },
            FieldInfoData {
                name: "format",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, format),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, status_code),
            },
            FieldInfoData {
                name: "message_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, message_id),
            },
            FieldInfoData {
                name: "recipient_id",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, recipient_id),
            },
            FieldInfoData {
                name: "recipient_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, recipient_type),
            },
            FieldInfoData {
                name: "items",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, items),
            },
            FieldInfoData {
                name: "event_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, event_id),
            },
            FieldInfoData {
                name: "event_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, event_name),
            },
            FieldInfoData {
                name: "event_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, event_type),
            },
            FieldInfoData {
                name: "count",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, count),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINSOCMESSAGEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinSocMessageEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINSOCMESSAGEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINSOCMESSAGEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSocMessageEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinSocMessageEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinGroupEvent {
    pub r#type: String,
    pub _class: String,
    pub group_id: String,
    pub status: String,
    pub status_code: String,
    pub member_id: String,
    pub member_type: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINGROUPEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGroupEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, r#type),
            },
            FieldInfoData {
                name: "_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, _class),
            },
            FieldInfoData {
                name: "group_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, group_id),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, status_code),
            },
            FieldInfoData {
                name: "member_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, member_id),
            },
            FieldInfoData {
                name: "member_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, member_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINGROUPEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinGroupEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINGROUPEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINGROUPEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGroupEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinGroupEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinFriendsEvent {
    pub frid: String,
    pub friend_type: String,
    pub source: String,
    pub network: String,
    pub action: String,
    pub status_code: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINFRIENDSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFriendsEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "frid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, frid),
            },
            FieldInfoData {
                name: "friend_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, friend_type),
            },
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, source),
            },
            FieldInfoData {
                name: "network",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, network),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, action),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINFRIENDSEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinFriendsEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINFRIENDSEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINFRIENDSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFriendsEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinFriendsEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinCustomErrorEvent {
    pub severity: String,
    pub metadata: RawJsonString,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINCUSTOMERROREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCustomErrorEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "severity",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCustomErrorEvent, severity),
            },
            FieldInfoData {
                name: "metadata",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCustomErrorEvent, metadata),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCustomErrorEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINCUSTOMERROREVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinCustomErrorEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINCUSTOMERROREVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINCUSTOMERROREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCustomErrorEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinCustomErrorEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinConnectionEvent {
    pub goid: String,
    pub player_ps: String,
    pub target_ip: String,
    pub target_ps: String,
    pub game_ps: String,
    pub net_topo: String,
    pub join_method: String,
    pub mode: String,
    pub client_type: String,
    pub leave_reason: String,
    pub cxn_tech: String,
    pub pkt_loss: f32,
    pub avg_lat: f32,
    pub max_lat: f32,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINCONNECTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinConnectionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, goid),
            },
            FieldInfoData {
                name: "player_ps",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, player_ps),
            },
            FieldInfoData {
                name: "target_ip",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, target_ip),
            },
            FieldInfoData {
                name: "target_ps",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, target_ps),
            },
            FieldInfoData {
                name: "game_ps",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, game_ps),
            },
            FieldInfoData {
                name: "net_topo",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, net_topo),
            },
            FieldInfoData {
                name: "join_method",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, join_method),
            },
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, mode),
            },
            FieldInfoData {
                name: "client_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, client_type),
            },
            FieldInfoData {
                name: "leave_reason",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, leave_reason),
            },
            FieldInfoData {
                name: "cxn_tech",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, cxn_tech),
            },
            FieldInfoData {
                name: "pkt_loss",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, pkt_loss),
            },
            FieldInfoData {
                name: "avg_lat",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, avg_lat),
            },
            FieldInfoData {
                name: "max_lat",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, max_lat),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINCONNECTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinConnectionEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINCONNECTIONEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINCONNECTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinConnectionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinConnectionEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinErrorEvent {
    pub sid: String,
    pub r#type: String,
    pub errid: String,
    pub catgid: String,
    pub server_type: String,
    pub server_name: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINERROREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinErrorEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "sid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, sid),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, r#type),
            },
            FieldInfoData {
                name: "errid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, errid),
            },
            FieldInfoData {
                name: "catgid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, catgid),
            },
            FieldInfoData {
                name: "server_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, server_type),
            },
            FieldInfoData {
                name: "server_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, server_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINERROREVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinErrorEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINERROREVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINERROREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinErrorEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinErrorEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinMatchInfoEvent {
    pub mode: String,
    pub diff: String,
    pub map: String,
    pub mid: String,
    pub goid: String,
    pub player_ps: String,
    pub game_ps: String,
    pub net_topo: String,
    pub client_type: String,
    pub status: String,
    pub status_code: String,
    pub end_reason: String,
    pub phase: String,
    pub ts_screate: String,
    pub ts_mstart: String,
    pub ts_mjoin: String,
    pub ts_mend: String,
    pub player_cnt: u32,
    pub max_players: u32,
    pub num_teams: u32,
    pub teams_stats: RawJsonString,
    pub player_stats: RawJsonString,
    pub match_info: RawJsonString,
    pub field_flag_changed0: u32,
}

pub const TELEMETRYSDKPINMATCHINFOEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMatchInfoEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, mode),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, diff),
            },
            FieldInfoData {
                name: "map",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, map),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, mid),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, goid),
            },
            FieldInfoData {
                name: "player_ps",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, player_ps),
            },
            FieldInfoData {
                name: "game_ps",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, game_ps),
            },
            FieldInfoData {
                name: "net_topo",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, net_topo),
            },
            FieldInfoData {
                name: "client_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, client_type),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, status_code),
            },
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, end_reason),
            },
            FieldInfoData {
                name: "phase",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, phase),
            },
            FieldInfoData {
                name: "ts_screate",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, ts_screate),
            },
            FieldInfoData {
                name: "ts_mstart",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, ts_mstart),
            },
            FieldInfoData {
                name: "ts_mjoin",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, ts_mjoin),
            },
            FieldInfoData {
                name: "ts_mend",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, ts_mend),
            },
            FieldInfoData {
                name: "player_cnt",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, player_cnt),
            },
            FieldInfoData {
                name: "max_players",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, max_players),
            },
            FieldInfoData {
                name: "num_teams",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, num_teams),
            },
            FieldInfoData {
                name: "teams_stats",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, teams_stats),
            },
            FieldInfoData {
                name: "player_stats",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, player_stats),
            },
            FieldInfoData {
                name: "match_info",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, match_info),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMATCHINFOEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinMatchInfoEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINMATCHINFOEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINMATCHINFOEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMatchInfoEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinMatchInfoEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinMatchJoinEvent {
    pub mode: String,
    pub instance_id: String,
    pub map: String,
    pub mid: String,
    pub goid: String,
    pub net_topo: String,
    pub client_type: String,
    pub join_method: String,
    pub cxn_tech: String,
    pub status: String,
    pub status_code: String,
    pub phase: String,
    pub mmdur: i64,
    pub max_mmdur: i64,
    pub fitscore: i32,
    pub max_fitscore: i32,
    pub scenario: String,
    pub scenario_subsession: String,
    pub scenario_variant: String,
    pub scenario_version: String,
    pub scenario_params: RawJsonString,
    pub friend_id: Vec<String>,
    pub friend_type: String,
    pub server_id: String,
    pub server_name: String,
    pub field_flag_changed0: u32,
}

pub const TELEMETRYSDKPINMATCHJOINEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMatchJoinEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, mode),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, instance_id),
            },
            FieldInfoData {
                name: "map",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, map),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, mid),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, goid),
            },
            FieldInfoData {
                name: "net_topo",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, net_topo),
            },
            FieldInfoData {
                name: "client_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, client_type),
            },
            FieldInfoData {
                name: "join_method",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, join_method),
            },
            FieldInfoData {
                name: "cxn_tech",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, cxn_tech),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, status_code),
            },
            FieldInfoData {
                name: "phase",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, phase),
            },
            FieldInfoData {
                name: "mmdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, mmdur),
            },
            FieldInfoData {
                name: "max_mmdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, max_mmdur),
            },
            FieldInfoData {
                name: "fitscore",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, fitscore),
            },
            FieldInfoData {
                name: "max_fitscore",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, max_fitscore),
            },
            FieldInfoData {
                name: "scenario",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, scenario),
            },
            FieldInfoData {
                name: "scenario_subsession",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, scenario_subsession),
            },
            FieldInfoData {
                name: "scenario_variant",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, scenario_variant),
            },
            FieldInfoData {
                name: "scenario_version",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, scenario_version),
            },
            FieldInfoData {
                name: "scenario_params",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, scenario_params),
            },
            FieldInfoData {
                name: "friend_id",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, friend_id),
            },
            FieldInfoData {
                name: "friend_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, friend_type),
            },
            FieldInfoData {
                name: "server_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, server_id),
            },
            FieldInfoData {
                name: "server_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, server_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMATCHJOINEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinMatchJoinEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINMATCHJOINEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINMATCHJOINEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMatchJoinEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinMatchJoinEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryPinEventHeaderConfigurationMessageBase {
}

pub const TELEMETRYPINEVENTHEADERCONFIGURATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryPinEventHeaderConfigurationMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetryPinEventHeaderConfigurationMessageBase {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYPINEVENTHEADERCONFIGURATIONMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryPinSessionHeaderConfigurationMessageBase {
}

pub const TELEMETRYPINSESSIONHEADERCONFIGURATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryPinSessionHeaderConfigurationMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetryPinSessionHeaderConfigurationMessageBase {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYPINSESSIONHEADERCONFIGURATIONMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryPinEndPointConfigurationMessageBase {
}

pub const TELEMETRYPINENDPOINTCONFIGURATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryPinEndPointConfigurationMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetryPinEndPointConfigurationMessageBase {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYPINENDPOINTCONFIGURATIONMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinPlayerStatEvent {
    pub gdur: u32,
    pub stat_id: String,
    pub stat_value: f32,
    pub stat_category: String,
    pub p_loc: super::core::Vec3,
    pub p_class: String,
    pub p_team_id: String,
    pub p_veh_id: String,
    pub p_params: RawJsonString,
    pub rdur: u32,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINPLAYERSTATEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStatEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, gdur),
            },
            FieldInfoData {
                name: "stat_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, stat_id),
            },
            FieldInfoData {
                name: "stat_value",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, stat_value),
            },
            FieldInfoData {
                name: "stat_category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, stat_category),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, p_loc),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, p_team_id),
            },
            FieldInfoData {
                name: "p_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, p_veh_id),
            },
            FieldInfoData {
                name: "p_params",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, p_params),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, rdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERSTATEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerStatEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERSTATEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERSTATEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStatEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerStatEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinNpcPartyEvent {
    pub cdur: u32,
    pub gdur: u32,
    pub rdur: u32,
    pub instance_id: String,
    pub party_id: String,
    pub npc_id: String,
    pub npc_char: String,
    pub npc_class: String,
    pub affin: i32,
    pub max_affin: i32,
    pub min_affin: i32,
    pub status: String,
    pub members: Vec<String>,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINNPCPARTYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcPartyEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, cdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, rdur),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, instance_id),
            },
            FieldInfoData {
                name: "party_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, party_id),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_char",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, npc_char),
            },
            FieldInfoData {
                name: "npc_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, npc_class),
            },
            FieldInfoData {
                name: "affin",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, affin),
            },
            FieldInfoData {
                name: "max_affin",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, max_affin),
            },
            FieldInfoData {
                name: "min_affin",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, min_affin),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, status),
            },
            FieldInfoData {
                name: "members",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, members),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCPARTYEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinNpcPartyEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINNPCPARTYEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINNPCPARTYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcPartyEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcPartyEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinNpcInteractionEvent {
    pub cdur: u32,
    pub gdur: u32,
    pub rdur: u32,
    pub instance_id: String,
    pub npc_id: String,
    pub npc_char: String,
    pub npc_class: String,
    pub npc_loc: super::core::Vec3,
    pub action: String,
    pub content: RawJsonString,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINNPCINTERACTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcInteractionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, cdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, rdur),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, instance_id),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_char",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, npc_char),
            },
            FieldInfoData {
                name: "npc_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, npc_class),
            },
            FieldInfoData {
                name: "npc_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, npc_loc),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, action),
            },
            FieldInfoData {
                name: "content",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, content),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCINTERACTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinNpcInteractionEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINNPCINTERACTIONEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINNPCINTERACTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcInteractionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcInteractionEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinNpcRelationshipEvent {
    pub cdur: u32,
    pub gdur: u32,
    pub rdur: u32,
    pub instance_id: String,
    pub npc_id: String,
    pub npc_char: String,
    pub npc_class: String,
    pub amount: i32,
    pub affin: i32,
    pub max_affin: i32,
    pub min_affin: i32,
    pub choice_id: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINNPCRELATIONSHIPEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcRelationshipEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, cdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, rdur),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, instance_id),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_char",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, npc_char),
            },
            FieldInfoData {
                name: "npc_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, npc_class),
            },
            FieldInfoData {
                name: "amount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, amount),
            },
            FieldInfoData {
                name: "affin",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, affin),
            },
            FieldInfoData {
                name: "max_affin",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, max_affin),
            },
            FieldInfoData {
                name: "min_affin",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, min_affin),
            },
            FieldInfoData {
                name: "choice_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, choice_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCRELATIONSHIPEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinNpcRelationshipEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINNPCRELATIONSHIPEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINNPCRELATIONSHIPEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcRelationshipEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcRelationshipEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinCinematicEvent {
    pub instance_id: String,
    pub cdur: u32,
    pub gdur: u32,
    pub rdur: u32,
    pub cine_id: String,
    pub cine_dur: u32,
    pub status: String,
    pub status_code: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINCINEMATICEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCinematicEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, instance_id),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, cdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, rdur),
            },
            FieldInfoData {
                name: "cine_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, cine_id),
            },
            FieldInfoData {
                name: "cine_dur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, cine_dur),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINCINEMATICEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinCinematicEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINCINEMATICEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINCINEMATICEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCinematicEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinCinematicEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlayerDecisionEvent {
    pub cdur: u32,
    pub gdur: u32,
    pub rdur: u32,
    pub instance_id: String,
    pub choice_id: String,
    pub choice_type: String,
    pub response_id: String,
    pub choices: Vec<String>,
    pub cine_id: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINPLAYERDECISIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerDecisionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, cdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, rdur),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, instance_id),
            },
            FieldInfoData {
                name: "choice_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, choice_id),
            },
            FieldInfoData {
                name: "choice_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, choice_type),
            },
            FieldInfoData {
                name: "response_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, response_id),
            },
            FieldInfoData {
                name: "choices",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, choices),
            },
            FieldInfoData {
                name: "cine_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, cine_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERDECISIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerDecisionEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERDECISIONEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERDECISIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerDecisionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerDecisionEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinScoreEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub r#type: String,
    pub amount: u32,
    pub team_id: String,
    pub player_dbid: i32,
    pub clock_time: u32,
    pub shot_pos: super::core::Vec3,
    pub assist_pos: super::core::Vec3,
    pub score_pos: super::core::Vec3,
    pub category: i32,
    pub shot_flags: i32,
    pub goal_flags1: i32,
    pub goal_flags2: i32,
    pub goal_flags3: i32,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINSCOREEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinScoreEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, rdur),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, r#type),
            },
            FieldInfoData {
                name: "amount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, amount),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, team_id),
            },
            FieldInfoData {
                name: "player_dbid",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, player_dbid),
            },
            FieldInfoData {
                name: "clock_time",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, clock_time),
            },
            FieldInfoData {
                name: "shot_pos",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, shot_pos),
            },
            FieldInfoData {
                name: "assist_pos",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, assist_pos),
            },
            FieldInfoData {
                name: "score_pos",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, score_pos),
            },
            FieldInfoData {
                name: "category",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, category),
            },
            FieldInfoData {
                name: "shot_flags",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, shot_flags),
            },
            FieldInfoData {
                name: "goal_flags1",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, goal_flags1),
            },
            FieldInfoData {
                name: "goal_flags2",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, goal_flags2),
            },
            FieldInfoData {
                name: "goal_flags3",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, goal_flags3),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINSCOREEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinScoreEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINSCOREEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINSCOREEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinScoreEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinScoreEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinNpcOutResourceEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub resource_type: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINNPCOUTRESOURCEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcOutResourceEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcOutResourceEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcOutResourceEvent, rdur),
            },
            FieldInfoData {
                name: "resource_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcOutResourceEvent, resource_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcOutResourceEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCOUTRESOURCEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinNpcOutResourceEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINNPCOUTRESOURCEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINNPCOUTRESOURCEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcOutResourceEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcOutResourceEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinNpcInteractEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_loc: super::core::Vec3,
    pub action: String,
    pub p_dir: super::core::Vec3,
    pub p_state: String,
    pub p_loc: super::core::Vec3,
    pub npc_id: String,
    pub npc_dir: super::core::Vec3,
    pub npc_loc: super::core::Vec3,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINNPCINTERACTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcInteractEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, item_type),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, item_loc),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, action),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, p_dir),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, p_state),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, p_loc),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, npc_dir),
            },
            FieldInfoData {
                name: "npc_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, npc_loc),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCINTERACTEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinNpcInteractEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINNPCINTERACTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINNPCINTERACTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcInteractEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcInteractEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinNpcDeathEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub k_weap: String,
    pub cause: String,
    pub v_id: String,
    pub v_type: String,
    pub v_loc: super::core::Vec3,
    pub v_dir: super::core::Vec3,
    pub v_state: String,
    pub is_vads: bool,
    pub k_id: String,
    pub k_type: String,
    pub k_loc: super::core::Vec3,
    pub k_dir: super::core::Vec3,
    pub k_state: String,
    pub is_kads: bool,
    pub v_weap: String,
    pub v_buff: Vec<String>,
    pub k_buff: Vec<String>,
    pub k_class: String,
    pub v_class: String,
    pub npc_id: String,
    pub v_team_id: String,
    pub k_lifetime: u32,
    pub v_lifetime: u32,
    pub v_play_state: String,
    pub field_flag_changed0: u32,
}

pub const TELEMETRYSDKPINNPCDEATHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcDeathEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, rdur),
            },
            FieldInfoData {
                name: "k_weap",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_weap),
            },
            FieldInfoData {
                name: "cause",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, cause),
            },
            FieldInfoData {
                name: "v_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_id),
            },
            FieldInfoData {
                name: "v_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_type),
            },
            FieldInfoData {
                name: "v_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_loc),
            },
            FieldInfoData {
                name: "v_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_dir),
            },
            FieldInfoData {
                name: "v_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_state),
            },
            FieldInfoData {
                name: "is_vads",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, is_vads),
            },
            FieldInfoData {
                name: "k_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_id),
            },
            FieldInfoData {
                name: "k_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_type),
            },
            FieldInfoData {
                name: "k_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_loc),
            },
            FieldInfoData {
                name: "k_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_dir),
            },
            FieldInfoData {
                name: "k_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_state),
            },
            FieldInfoData {
                name: "is_kads",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, is_kads),
            },
            FieldInfoData {
                name: "v_weap",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_weap),
            },
            FieldInfoData {
                name: "v_buff",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_buff),
            },
            FieldInfoData {
                name: "k_buff",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_buff),
            },
            FieldInfoData {
                name: "k_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_class),
            },
            FieldInfoData {
                name: "v_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_class),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, npc_id),
            },
            FieldInfoData {
                name: "v_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_team_id),
            },
            FieldInfoData {
                name: "k_lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_lifetime),
            },
            FieldInfoData {
                name: "v_lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_lifetime),
            },
            FieldInfoData {
                name: "v_play_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_play_state),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCDEATHEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinNpcDeathEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINNPCDEATHEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINNPCDEATHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcDeathEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcDeathEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinNpcStateEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub ai_state: bool,
    pub npc_id: String,
    pub npc_loc: super::core::Vec3,
    pub cur_npc_state: String,
    pub prev_npc_state: String,
    pub source: String,
    pub p_loc: super::core::Vec3,
    pub p_aim: super::core::Vec3,
    pub p_state: String,
    pub is_alarm: bool,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINNPCSTATEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcStateEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, rdur),
            },
            FieldInfoData {
                name: "ai_state",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, ai_state),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, npc_loc),
            },
            FieldInfoData {
                name: "cur_npc_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, cur_npc_state),
            },
            FieldInfoData {
                name: "prev_npc_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, prev_npc_state),
            },
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, source),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, p_loc),
            },
            FieldInfoData {
                name: "p_aim",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, p_aim),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, p_state),
            },
            FieldInfoData {
                name: "is_alarm",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, is_alarm),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCSTATEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinNpcStateEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINNPCSTATEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINNPCSTATEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcStateEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcStateEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinGameObjectiveEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub p_loc: super::core::Vec3,
    pub diff: String,
    pub p_team_id: String,
    pub category: String,
    pub objective_id: String,
    pub objective_loc: super::core::Vec3,
    pub reqs: Vec<String>,
    pub reward: RawJsonString,
    pub status: String,
    pub percent: u32,
    pub players: Vec<String>,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINGAMEOBJECTIVEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameObjectiveEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, rdur),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, p_loc),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, diff),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, p_team_id),
            },
            FieldInfoData {
                name: "category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, category),
            },
            FieldInfoData {
                name: "objective_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, objective_id),
            },
            FieldInfoData {
                name: "objective_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, objective_loc),
            },
            FieldInfoData {
                name: "reqs",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, reqs),
            },
            FieldInfoData {
                name: "reward",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, reward),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, status),
            },
            FieldInfoData {
                name: "percent",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, percent),
            },
            FieldInfoData {
                name: "players",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, players),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINGAMEOBJECTIVEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinGameObjectiveEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINGAMEOBJECTIVEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINGAMEOBJECTIVEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameObjectiveEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinGameObjectiveEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinFpsEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub r#type: String,
    pub fps: f32,
    pub threshold: f32,
    pub p_loc: super::core::Vec3,
    pub p_dir: super::core::Vec3,
    pub metadata: RawJsonString,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINFPSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFpsEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, rdur),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, r#type),
            },
            FieldInfoData {
                name: "fps",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, fps),
            },
            FieldInfoData {
                name: "threshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, threshold),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, p_dir),
            },
            FieldInfoData {
                name: "metadata",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, metadata),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINFPSEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinFpsEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINFPSEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINFPSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFpsEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinFpsEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinGameActionEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub action: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINGAMEACTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameActionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameActionEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameActionEvent, rdur),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameActionEvent, action),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameActionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINGAMEACTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinGameActionEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINGAMEACTIONEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINGAMEACTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameActionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinGameActionEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinVehicleHealthEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub veh_id: String,
    pub veh_loc: super::core::Vec3,
    pub veh_type: String,
    pub veh_guid: String,
    pub is_heal: bool,
    pub source_id: String,
    pub source_type: String,
    pub source_loc: super::core::Vec3,
    pub source_item: String,
    pub cause: String,
    pub weapon: String,
    pub amount: i32,
    pub health: i32,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINVEHICLEHEALTHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleHealthEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, rdur),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, veh_id),
            },
            FieldInfoData {
                name: "veh_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, veh_loc),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, veh_type),
            },
            FieldInfoData {
                name: "veh_guid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, veh_guid),
            },
            FieldInfoData {
                name: "is_heal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, is_heal),
            },
            FieldInfoData {
                name: "source_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, source_id),
            },
            FieldInfoData {
                name: "source_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, source_type),
            },
            FieldInfoData {
                name: "source_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, source_loc),
            },
            FieldInfoData {
                name: "source_item",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, source_item),
            },
            FieldInfoData {
                name: "cause",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, cause),
            },
            FieldInfoData {
                name: "weapon",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, weapon),
            },
            FieldInfoData {
                name: "amount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, amount),
            },
            FieldInfoData {
                name: "health",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, health),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINVEHICLEHEALTHEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinVehicleHealthEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINVEHICLEHEALTHEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINVEHICLEHEALTHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleHealthEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinVehicleHealthEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinVehicleInteractEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub veh_dur: u32,
    pub veh_id: String,
    pub veh_guid: String,
    pub veh_type: String,
    pub action: String,
    pub veh_loc: super::core::Vec3,
    pub p_loc: super::core::Vec3,
    pub p_dir: super::core::Vec3,
    pub p_team_id: String,
    pub custom: RawJsonString,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINVEHICLEINTERACTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleInteractEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, rdur),
            },
            FieldInfoData {
                name: "veh_dur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, veh_dur),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, veh_id),
            },
            FieldInfoData {
                name: "veh_guid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, veh_guid),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, veh_type),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, action),
            },
            FieldInfoData {
                name: "veh_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, veh_loc),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, p_dir),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, p_team_id),
            },
            FieldInfoData {
                name: "custom",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, custom),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINVEHICLEINTERACTEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinVehicleInteractEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINVEHICLEINTERACTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINVEHICLEINTERACTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleInteractEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinVehicleInteractEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinVehicleDestructionEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub veh_id: String,
    pub cause: String,
    pub veh_loc: super::core::Vec3,
    pub veh_type: String,
    pub pilot_id: String,
    pub pilot_type: String,
    pub pilot_loc: super::core::Vec3,
    pub pilot_dir: super::core::Vec3,
    pub k_id: String,
    pub k_type: String,
    pub k_loc: super::core::Vec3,
    pub k_dir: super::core::Vec3,
    pub is_eject: bool,
    pub veh_dur: u32,
    pub veh_instance_id: String,
    pub k_weap: String,
    pub k_veh_id: String,
    pub field_flag_changed0: u32,
}

pub const TELEMETRYSDKPINVEHICLEDESTRUCTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleDestructionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, rdur),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, veh_id),
            },
            FieldInfoData {
                name: "cause",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, cause),
            },
            FieldInfoData {
                name: "veh_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, veh_loc),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, veh_type),
            },
            FieldInfoData {
                name: "pilot_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, pilot_id),
            },
            FieldInfoData {
                name: "pilot_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, pilot_type),
            },
            FieldInfoData {
                name: "pilot_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, pilot_loc),
            },
            FieldInfoData {
                name: "pilot_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, pilot_dir),
            },
            FieldInfoData {
                name: "k_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_id),
            },
            FieldInfoData {
                name: "k_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_type),
            },
            FieldInfoData {
                name: "k_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_loc),
            },
            FieldInfoData {
                name: "k_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_dir),
            },
            FieldInfoData {
                name: "is_eject",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, is_eject),
            },
            FieldInfoData {
                name: "veh_dur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, veh_dur),
            },
            FieldInfoData {
                name: "veh_instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, veh_instance_id),
            },
            FieldInfoData {
                name: "k_weap",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_weap),
            },
            FieldInfoData {
                name: "k_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_veh_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINVEHICLEDESTRUCTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinVehicleDestructionEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINVEHICLEDESTRUCTIONEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINVEHICLEDESTRUCTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleDestructionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinVehicleDestructionEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinVehicleSpawnEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub veh_id: String,
    pub veh_type: String,
    pub veh_loc: super::core::Vec3,
    pub veh_lev: i32,
    pub veh_loadout: RawJsonString,
    pub veh_instance_id: String,
    pub team_id: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINVEHICLESPAWNEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleSpawnEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, rdur),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_id),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_type),
            },
            FieldInfoData {
                name: "veh_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_loc),
            },
            FieldInfoData {
                name: "veh_lev",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_lev),
            },
            FieldInfoData {
                name: "veh_loadout",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_loadout),
            },
            FieldInfoData {
                name: "veh_instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_instance_id),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, team_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINVEHICLESPAWNEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinVehicleSpawnEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINVEHICLESPAWNEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINVEHICLESPAWNEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleSpawnEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinVehicleSpawnEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinTagEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub tag_type: String,
    pub tag_id: String,
    pub tag_loc: super::core::Vec3,
    pub p_loc: super::core::Vec3,
    pub tag_method: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINTAGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTagEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, rdur),
            },
            FieldInfoData {
                name: "tag_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, tag_type),
            },
            FieldInfoData {
                name: "tag_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, tag_id),
            },
            FieldInfoData {
                name: "tag_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, tag_loc),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, p_loc),
            },
            FieldInfoData {
                name: "tag_method",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, tag_method),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINTAGEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinTagEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINTAGEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINTAGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTagEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinTagEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinItemUpgradeEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_name: String,
    pub item_loc: super::core::Vec3,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINITEMUPGRADEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemUpgradeEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, item_type),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, item_name),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, item_loc),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINITEMUPGRADEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinItemUpgradeEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINITEMUPGRADEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINITEMUPGRADEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemUpgradeEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinItemUpgradeEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinItemUnlockEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_loc: super::core::Vec3,
    pub item_name: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINITEMUNLOCKEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemUnlockEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, item_type),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, item_loc),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, item_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINITEMUNLOCKEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinItemUnlockEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINITEMUNLOCKEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINITEMUNLOCKEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemUnlockEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinItemUnlockEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinItemPickupEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub item_dur: u32,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_name: String,
    pub item_loc: super::core::Vec3,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINITEMPICKUPEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemPickupEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, rdur),
            },
            FieldInfoData {
                name: "item_dur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_dur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_type),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_name),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_loc),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINITEMPICKUPEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinItemPickupEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINITEMPICKUPEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINITEMPICKUPEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemPickupEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinItemPickupEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinItemSpawnEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_name: String,
    pub item_loc: super::core::Vec3,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINITEMSPAWNEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemSpawnEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, item_type),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, item_name),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, item_loc),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINITEMSPAWNEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinItemSpawnEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINITEMSPAWNEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINITEMSPAWNEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemSpawnEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinItemSpawnEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinItemHealthEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_loc: super::core::Vec3,
    pub p_loc: super::core::Vec3,
    pub p_id: String,
    pub p_type: String,
    pub is_heal: bool,
    pub source_id: String,
    pub source_type: String,
    pub source_loc: super::core::Vec3,
    pub weapon: String,
    pub amount: i32,
    pub health: i32,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINITEMHEALTHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemHealthEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, item_type),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, item_loc),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, p_loc),
            },
            FieldInfoData {
                name: "p_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, p_id),
            },
            FieldInfoData {
                name: "p_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, p_type),
            },
            FieldInfoData {
                name: "is_heal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, is_heal),
            },
            FieldInfoData {
                name: "source_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, source_id),
            },
            FieldInfoData {
                name: "source_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, source_type),
            },
            FieldInfoData {
                name: "source_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, source_loc),
            },
            FieldInfoData {
                name: "weapon",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, weapon),
            },
            FieldInfoData {
                name: "amount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, amount),
            },
            FieldInfoData {
                name: "health",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, health),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINITEMHEALTHEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinItemHealthEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINITEMHEALTHEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINITEMHEALTHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemHealthEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinItemHealthEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlayerOutResourceEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub resource_type: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINPLAYEROUTRESOURCEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerOutResourceEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerOutResourceEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerOutResourceEvent, rdur),
            },
            FieldInfoData {
                name: "resource_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerOutResourceEvent, resource_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerOutResourceEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYEROUTRESOURCEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerOutResourceEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYEROUTRESOURCEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYEROUTRESOURCEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerOutResourceEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerOutResourceEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlayerTeamSwitchEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub prev_team_id: String,
    pub prev_team_score: u32,
    pub team_id: String,
    pub team_score: u32,
    pub reason: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINPLAYERTEAMSWITCHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerTeamSwitchEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, rdur),
            },
            FieldInfoData {
                name: "prev_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, prev_team_id),
            },
            FieldInfoData {
                name: "prev_team_score",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, prev_team_score),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, team_id),
            },
            FieldInfoData {
                name: "team_score",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, team_score),
            },
            FieldInfoData {
                name: "reason",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, reason),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERTEAMSWITCHEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerTeamSwitchEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERTEAMSWITCHEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERTEAMSWITCHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerTeamSwitchEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerTeamSwitchEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinPlayerInteractEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_loc: super::core::Vec3,
    pub action: String,
    pub p_dir: super::core::Vec3,
    pub p_state: String,
    pub p_loc: super::core::Vec3,
    pub p_class: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINPLAYERINTERACTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerInteractEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, item_type),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, item_loc),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, action),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, p_dir),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, p_state),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, p_loc),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, p_class),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERINTERACTEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerInteractEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERINTERACTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERINTERACTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerInteractEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerInteractEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinPlayerAbilityAffectEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub a_loc: super::core::Vec3,
    pub a_type: String,
    pub p_loc: super::core::Vec3,
    pub p_state: String,
    pub p_class: String,
    pub p_team_id: String,
    pub v_state: String,
    pub v_loc: super::core::Vec3,
    pub v_class: String,
    pub v_team_id: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINPLAYERABILITYAFFECTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerAbilityAffectEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, rdur),
            },
            FieldInfoData {
                name: "a_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, a_loc),
            },
            FieldInfoData {
                name: "a_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, a_type),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, p_loc),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, p_state),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, p_team_id),
            },
            FieldInfoData {
                name: "v_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, v_state),
            },
            FieldInfoData {
                name: "v_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, v_loc),
            },
            FieldInfoData {
                name: "v_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, v_class),
            },
            FieldInfoData {
                name: "v_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, v_team_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERABILITYAFFECTEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerAbilityAffectEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERABILITYAFFECTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERABILITYAFFECTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerAbilityAffectEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerAbilityAffectEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinPlayerAbilityEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub ability: String,
    pub ability_dur: u32,
    pub p_loc: super::core::Vec3,
    pub p_state: String,
    pub ability_data: RawJsonString,
    pub p_class: String,
    pub p_team_id: String,
    pub p_veh_id: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINPLAYERABILITYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerAbilityEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, rdur),
            },
            FieldInfoData {
                name: "ability",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, ability),
            },
            FieldInfoData {
                name: "ability_dur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, ability_dur),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, p_loc),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, p_state),
            },
            FieldInfoData {
                name: "ability_data",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, ability_data),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, p_team_id),
            },
            FieldInfoData {
                name: "p_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, p_veh_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERABILITYEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerAbilityEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERABILITYEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERABILITYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerAbilityEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerAbilityEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinPlayerUseEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_name: String,
    pub item_dur: u32,
    pub item_loc: super::core::Vec3,
    pub p_state: String,
    pub p_loc: super::core::Vec3,
    pub p_class: String,
    pub p_team_id: String,
    pub veh_id: String,
    pub action: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINPLAYERUSEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerUseEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_type),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_name),
            },
            FieldInfoData {
                name: "item_dur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_dur),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_loc),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, p_state),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, p_loc),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, p_team_id),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, veh_id),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, action),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERUSEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerUseEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERUSEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERUSEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerUseEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerUseEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlayerKillAssistEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub k_id: String,
    pub k_type: String,
    pub v_id: String,
    pub v_type: String,
    pub damage: u32,
    pub percent: u32,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINPLAYERKILLASSISTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerKillAssistEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, rdur),
            },
            FieldInfoData {
                name: "k_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, k_id),
            },
            FieldInfoData {
                name: "k_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, k_type),
            },
            FieldInfoData {
                name: "v_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, v_id),
            },
            FieldInfoData {
                name: "v_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, v_type),
            },
            FieldInfoData {
                name: "damage",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, damage),
            },
            FieldInfoData {
                name: "percent",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, percent),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERKILLASSISTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerKillAssistEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERKILLASSISTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERKILLASSISTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerKillAssistEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerKillAssistEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinPlayerHealthEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub source_id: String,
    pub source_type: String,
    pub source_loc: super::core::Vec3,
    pub is_heal: bool,
    pub is_revived: bool,
    pub amount: i32,
    pub health: i32,
    pub p_loc: super::core::Vec3,
    pub weapon: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINPLAYERHEALTHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerHealthEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, rdur),
            },
            FieldInfoData {
                name: "source_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, source_id),
            },
            FieldInfoData {
                name: "source_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, source_type),
            },
            FieldInfoData {
                name: "source_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, source_loc),
            },
            FieldInfoData {
                name: "is_heal",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, is_heal),
            },
            FieldInfoData {
                name: "is_revived",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, is_revived),
            },
            FieldInfoData {
                name: "amount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, amount),
            },
            FieldInfoData {
                name: "health",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, health),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, p_loc),
            },
            FieldInfoData {
                name: "weapon",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, weapon),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERHEALTHEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerHealthEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERHEALTHEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERHEALTHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerHealthEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerHealthEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinPlayerDeathEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub cause: String,
    pub v_loc: super::core::Vec3,
    pub v_dir: super::core::Vec3,
    pub v_state: String,
    pub is_vads: bool,
    pub k_id: String,
    pub k_type: String,
    pub k_class: String,
    pub k_weap: String,
    pub k_loc: super::core::Vec3,
    pub k_dir: super::core::Vec3,
    pub k_state: String,
    pub is_kads: bool,
    pub v_weap: String,
    pub v_class: String,
    pub k_lifetime: u32,
    pub v_lifetime: u32,
    pub v_buff: Vec<String>,
    pub k_buff: Vec<String>,
    pub v_team_id: String,
    pub v_res_points: u32,
    pub game_play_stats: RawJsonString,
    pub weapon_stats: RawJsonString,
    pub v_score_earned: u32,
    pub v_veh_id: String,
    pub k_veh_id: String,
    pub field_flag_changed0: u32,
}

pub const TELEMETRYSDKPINPLAYERDEATHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerDeathEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, rdur),
            },
            FieldInfoData {
                name: "cause",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, cause),
            },
            FieldInfoData {
                name: "v_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_loc),
            },
            FieldInfoData {
                name: "v_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_dir),
            },
            FieldInfoData {
                name: "v_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_state),
            },
            FieldInfoData {
                name: "is_vads",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, is_vads),
            },
            FieldInfoData {
                name: "k_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_id),
            },
            FieldInfoData {
                name: "k_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_type),
            },
            FieldInfoData {
                name: "k_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_class),
            },
            FieldInfoData {
                name: "k_weap",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_weap),
            },
            FieldInfoData {
                name: "k_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_loc),
            },
            FieldInfoData {
                name: "k_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_dir),
            },
            FieldInfoData {
                name: "k_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_state),
            },
            FieldInfoData {
                name: "is_kads",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, is_kads),
            },
            FieldInfoData {
                name: "v_weap",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_weap),
            },
            FieldInfoData {
                name: "v_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_class),
            },
            FieldInfoData {
                name: "k_lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_lifetime),
            },
            FieldInfoData {
                name: "v_lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_lifetime),
            },
            FieldInfoData {
                name: "v_buff",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_buff),
            },
            FieldInfoData {
                name: "k_buff",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_buff),
            },
            FieldInfoData {
                name: "v_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_team_id),
            },
            FieldInfoData {
                name: "v_res_points",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_res_points),
            },
            FieldInfoData {
                name: "game_play_stats",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, game_play_stats),
            },
            FieldInfoData {
                name: "weapon_stats",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, weapon_stats),
            },
            FieldInfoData {
                name: "v_score_earned",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_score_earned),
            },
            FieldInfoData {
                name: "v_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_veh_id),
            },
            FieldInfoData {
                name: "k_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_veh_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERDEATHEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerDeathEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERDEATHEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERDEATHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerDeathEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerDeathEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlayerEquipEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub loadout: RawJsonString,
    pub prev_loadout: RawJsonString,
    pub pgid: String,
    pub object_id: String,
    pub object_type: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINPLAYEREQUIPEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerEquipEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, rdur),
            },
            FieldInfoData {
                name: "loadout",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, loadout),
            },
            FieldInfoData {
                name: "prev_loadout",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, prev_loadout),
            },
            FieldInfoData {
                name: "pgid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, pgid),
            },
            FieldInfoData {
                name: "object_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, object_id),
            },
            FieldInfoData {
                name: "object_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, object_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYEREQUIPEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerEquipEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYEREQUIPEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYEREQUIPEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerEquipEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerEquipEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlayerClassEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub p_class: String,
    pub sub_class: String,
    pub prev_class: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINPLAYERCLASSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerClassEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, rdur),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, p_class),
            },
            FieldInfoData {
                name: "sub_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, sub_class),
            },
            FieldInfoData {
                name: "prev_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, prev_class),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERCLASSEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerClassEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERCLASSEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERCLASSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerClassEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerClassEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinPlayerStateEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub p_state: String,
    pub prev_state: String,
    pub prev_dur: u32,
    pub p_loc: super::core::Vec3,
    pub p_dir: super::core::Vec3,
    pub team_id: String,
    pub p_class: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINPLAYERSTATEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStateEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, rdur),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, p_state),
            },
            FieldInfoData {
                name: "prev_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, prev_state),
            },
            FieldInfoData {
                name: "prev_dur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, prev_dur),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, p_dir),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, team_id),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, p_class),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERSTATEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerStateEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERSTATEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERSTATEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStateEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerStateEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinNpcSpawnEvent {
    pub gdur: u32,
    pub npc_id: String,
    pub npc_loc: super::core::Vec3,
    pub npc_class: String,
    pub npc_dir: super::core::Vec3,
    pub npc_veh_id: String,
    pub npc_team_id: String,
    pub npc_loadout: RawJsonString,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINNPCSPAWNEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcSpawnEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, gdur),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_loc),
            },
            FieldInfoData {
                name: "npc_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_class),
            },
            FieldInfoData {
                name: "npc_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_dir),
            },
            FieldInfoData {
                name: "npc_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_veh_id),
            },
            FieldInfoData {
                name: "npc_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_team_id),
            },
            FieldInfoData {
                name: "npc_loadout",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_loadout),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCSPAWNEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinNpcSpawnEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINNPCSPAWNEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINNPCSPAWNEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcSpawnEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcSpawnEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinPlayerSpawnEvent {
    pub r#type: String,
    pub gdur: u32,
    pub rdur: u32,
    pub p_loc: super::core::Vec3,
    pub p_dir: super::core::Vec3,
    pub veh_id: String,
    pub veh_type: String,
    pub p_class: String,
    pub team_id: String,
    pub resources: RawJsonString,
    pub resources_spent: RawJsonString,
    pub resource_items: RawJsonString,
    pub loadout: RawJsonString,
    pub reason: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINPLAYERSPAWNEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerSpawnEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, r#type),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, rdur),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, p_dir),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, veh_id),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, veh_type),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, p_class),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, team_id),
            },
            FieldInfoData {
                name: "resources",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, resources),
            },
            FieldInfoData {
                name: "resources_spent",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, resources_spent),
            },
            FieldInfoData {
                name: "resource_items",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, resource_items),
            },
            FieldInfoData {
                name: "loadout",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, loadout),
            },
            FieldInfoData {
                name: "reason",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, reason),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERSPAWNEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerSpawnEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERSPAWNEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERSPAWNEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerSpawnEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerSpawnEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinPlayerTickEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub p_loc: super::core::Vec3,
    pub p_char: String,
    pub p_class: String,
    pub p_dir: super::core::Vec3,
    pub cam_dir: super::core::Vec3,
    pub p_state: String,
    pub veh_id: String,
    pub veh_type: String,
    pub veh_state: String,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_name: String,
    pub is_ads: bool,
    pub p_team_id: String,
    pub field_flag_changed0: u32,
}

pub const TELEMETRYSDKPINPLAYERTICKEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerTickEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, rdur),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_loc),
            },
            FieldInfoData {
                name: "p_char",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_char),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_class),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_dir),
            },
            FieldInfoData {
                name: "cam_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, cam_dir),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_state),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, veh_id),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, veh_type),
            },
            FieldInfoData {
                name: "veh_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, veh_state),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, item_type),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, item_name),
            },
            FieldInfoData {
                name: "is_ads",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, is_ads),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_team_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERTICKEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerTickEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERTICKEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERTICKEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerTickEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerTickEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinTransctionEvent {
    pub code: String,
    pub r#type: String,
    pub source: String,
    pub revenue_model: String,
    pub status: String,
    pub status_code: String,
    pub party1id: RawJsonString,
    pub party2id: RawJsonString,
    pub asset_out: RawJsonString,
    pub asset_in: RawJsonString,
    pub bal1: RawJsonString,
    pub bal2: RawJsonString,
    pub _meta: RawJsonString,
    pub txid: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINTRANSCTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTransctionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, code),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, r#type),
            },
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, source),
            },
            FieldInfoData {
                name: "revenue_model",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, revenue_model),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, status_code),
            },
            FieldInfoData {
                name: "party1id",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, party1id),
            },
            FieldInfoData {
                name: "party2id",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, party2id),
            },
            FieldInfoData {
                name: "asset_out",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, asset_out),
            },
            FieldInfoData {
                name: "asset_in",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, asset_in),
            },
            FieldInfoData {
                name: "bal1",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, bal1),
            },
            FieldInfoData {
                name: "bal2",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, bal2),
            },
            FieldInfoData {
                name: "_meta",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, _meta),
            },
            FieldInfoData {
                name: "txid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, txid),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINTRANSCTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinTransctionEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINTRANSCTIONEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINTRANSCTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTransctionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinTransctionEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinInventoryEvent {
    pub playerid: RawJsonString,
    pub assets: RawJsonString,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPININVENTORYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinInventoryEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "playerid",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinInventoryEvent, playerid),
            },
            FieldInfoData {
                name: "assets",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinInventoryEvent, assets),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinInventoryEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPININVENTORYEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinInventoryEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPININVENTORYEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPININVENTORYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinInventoryEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinInventoryEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinSurveyEvent {
    pub r#type: String,
    pub status: String,
    pub status_code: String,
    pub survey_id: String,
    pub wave_no: i32,
    pub lang: String,
    pub complete_flag: String,
    pub qtime: i64,
    pub j_s_o_n: RawJsonString,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINSURVEYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSurveyEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, r#type),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, status_code),
            },
            FieldInfoData {
                name: "survey_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, survey_id),
            },
            FieldInfoData {
                name: "wave_no",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, wave_no),
            },
            FieldInfoData {
                name: "lang",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, lang),
            },
            FieldInfoData {
                name: "complete_flag",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, complete_flag),
            },
            FieldInfoData {
                name: "qtime",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, qtime),
            },
            FieldInfoData {
                name: "JSON",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, j_s_o_n),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINSURVEYEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinSurveyEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINSURVEYEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINSURVEYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSurveyEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinSurveyEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinHeartBeatEvent {
    pub sdur: u32,
    pub gdur: u32,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINHEARTBEATEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinHeartBeatEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinHeartBeatEvent, sdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinHeartBeatEvent, gdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinHeartBeatEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINHEARTBEATEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinHeartBeatEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINHEARTBEATEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINHEARTBEATEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinHeartBeatEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinHeartBeatEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinModeExitEvent {
    pub instance_id: String,
    pub leave_reason: String,
    pub mdur: i64,
    pub cdur: i64,
    pub sdur: i64,
    pub gdur: i64,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINMODEEXITEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinModeExitEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, instance_id),
            },
            FieldInfoData {
                name: "leave_reason",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, leave_reason),
            },
            FieldInfoData {
                name: "mdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, mdur),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, cdur),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, sdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, gdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMODEEXITEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinModeExitEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINMODEEXITEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINMODEEXITEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinModeExitEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinModeExitEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinModeEnterEvent {
    pub instance_id: String,
    pub status: String,
    pub status_code: String,
    pub is_first: bool,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINMODEENTEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinModeEnterEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeEnterEvent, instance_id),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeEnterEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeEnterEvent, status_code),
            },
            FieldInfoData {
                name: "is_first",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeEnterEvent, is_first),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinModeEnterEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMODEENTEREVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinModeEnterEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINMODEENTEREVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINMODEENTEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinModeEnterEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinModeEnterEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlaySessionEndEvent {
    pub instance_id: String,
    pub sdur: i64,
    pub gdur: i64,
    pub diff: String,
    pub goid: String,
    pub status_code: String,
    pub end_reason: String,
    pub player_stats: RawJsonString,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINPLAYSESSIONENDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlaySessionEndEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, instance_id),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, sdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, gdur),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, diff),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, goid),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, status_code),
            },
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, end_reason),
            },
            FieldInfoData {
                name: "player_stats",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, player_stats),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYSESSIONENDEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlaySessionEndEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYSESSIONENDEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYSESSIONENDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlaySessionEndEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlaySessionEndEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlaySessionStartEvent {
    pub instance_id: String,
    pub diff: String,
    pub goid: String,
    pub status: String,
    pub status_code: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINPLAYSESSIONSTARTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlaySessionStartEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, instance_id),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, diff),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, goid),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYSESSIONSTARTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlaySessionStartEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYSESSIONSTARTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYSESSIONSTARTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlaySessionStartEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlaySessionStartEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinRoundEndEvent {
    pub round: u32,
    pub instance_id: String,
    pub gdur: i64,
    pub rdur: i64,
    pub diff: String,
    pub mid: String,
    pub goid: String,
    pub status_code: String,
    pub end_reason: String,
    pub player_cnt: u32,
    pub max_players: u32,
    pub num_teams: u32,
    pub teams_stats: RawJsonString,
    pub player_stats: RawJsonString,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINROUNDENDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRoundEndEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "round",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, round),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, instance_id),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, rdur),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, diff),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, mid),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, goid),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, status_code),
            },
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, end_reason),
            },
            FieldInfoData {
                name: "player_cnt",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, player_cnt),
            },
            FieldInfoData {
                name: "max_players",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, max_players),
            },
            FieldInfoData {
                name: "num_teams",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, num_teams),
            },
            FieldInfoData {
                name: "teams_stats",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, teams_stats),
            },
            FieldInfoData {
                name: "player_stats",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, player_stats),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINROUNDENDEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinRoundEndEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINROUNDENDEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINROUNDENDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRoundEndEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinRoundEndEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinRoundStartEvent {
    pub round: u32,
    pub instance_id: String,
    pub diff: String,
    pub mid: String,
    pub goid: String,
    pub status: String,
    pub status_code: String,
    pub player_cnt: u32,
    pub max_players: u32,
    pub num_teams: u32,
    pub team_id: String,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINROUNDSTARTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRoundStartEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "round",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, round),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, instance_id),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, diff),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, mid),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, goid),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, status_code),
            },
            FieldInfoData {
                name: "player_cnt",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, player_cnt),
            },
            FieldInfoData {
                name: "max_players",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, max_players),
            },
            FieldInfoData {
                name: "num_teams",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, num_teams),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, team_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINROUNDSTARTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinRoundStartEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINROUNDSTARTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINROUNDSTARTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRoundStartEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinRoundStartEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinGameEndEvent {
    pub r#type: String,
    pub mode: String,
    pub instance_id: String,
    pub mid: String,
    pub goid: String,
    pub end_reason: String,
    pub status_code: String,
    pub sdur: i64,
    pub gdur: i64,
    pub cdur: i64,
    pub player_stats: RawJsonString,
    pub teams_stats: RawJsonString,
    pub constraint_status: RawJsonString,
    pub mission_status: RawJsonString,
    pub items_initialized: RawJsonString,
    pub achievement: RawJsonString,
    pub asset_gained: RawJsonString,
    pub asset_used: RawJsonString,
    pub asset_balance: RawJsonString,
    pub weapon_stats: RawJsonString,
    pub field_flag_changed0: u32,
}

pub const TELEMETRYSDKPINGAMEENDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameEndEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, r#type),
            },
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, mode),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, instance_id),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, mid),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, goid),
            },
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, end_reason),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, status_code),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, sdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, gdur),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, cdur),
            },
            FieldInfoData {
                name: "player_stats",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, player_stats),
            },
            FieldInfoData {
                name: "teams_stats",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, teams_stats),
            },
            FieldInfoData {
                name: "constraint_status",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, constraint_status),
            },
            FieldInfoData {
                name: "mission_status",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, mission_status),
            },
            FieldInfoData {
                name: "items_initialized",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, items_initialized),
            },
            FieldInfoData {
                name: "achievement",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, achievement),
            },
            FieldInfoData {
                name: "asset_gained",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, asset_gained),
            },
            FieldInfoData {
                name: "asset_used",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, asset_used),
            },
            FieldInfoData {
                name: "asset_balance",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, asset_balance),
            },
            FieldInfoData {
                name: "weapon_stats",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, weapon_stats),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINGAMEENDEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinGameEndEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINGAMEENDEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINGAMEENDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameEndEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinGameEndEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinGameStartEvent {
    pub r#type: String,
    pub mode: String,
    pub instance_id: String,
    pub status: String,
    pub status_code: String,
    pub ldur: u32,
    pub diff: String,
    pub goid: String,
    pub mid: String,
    pub rond: String,
    pub map: String,
    pub _char: String,
    pub gend: String,
    pub _class: String,
    pub attempt: u32,
    pub max_level: u32,
    pub level_modifier: String,
    pub start_status: RawJsonString,
    pub level_constraints: RawJsonString,
    pub missions: RawJsonString,
    pub other_specs: RawJsonString,
    pub model_config: RawJsonString,
    pub knob_owner: RawJsonString,
    pub lives: i64,
    pub new_life_reqs: RawJsonString,
    pub team_id: String,
    pub asset_bal: RawJsonString,
    pub field_flag_changed0: u32,
}

pub const TELEMETRYSDKPINGAMESTARTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameStartEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, r#type),
            },
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, mode),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, instance_id),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, status_code),
            },
            FieldInfoData {
                name: "ldur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, ldur),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, diff),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, goid),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, mid),
            },
            FieldInfoData {
                name: "rond",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, rond),
            },
            FieldInfoData {
                name: "map",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, map),
            },
            FieldInfoData {
                name: "_char",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, _char),
            },
            FieldInfoData {
                name: "gend",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, gend),
            },
            FieldInfoData {
                name: "_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, _class),
            },
            FieldInfoData {
                name: "attempt",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, attempt),
            },
            FieldInfoData {
                name: "max_level",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, max_level),
            },
            FieldInfoData {
                name: "level_modifier",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, level_modifier),
            },
            FieldInfoData {
                name: "start_status",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, start_status),
            },
            FieldInfoData {
                name: "level_constraints",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, level_constraints),
            },
            FieldInfoData {
                name: "missions",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, missions),
            },
            FieldInfoData {
                name: "other_specs",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, other_specs),
            },
            FieldInfoData {
                name: "model_config",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, model_config),
            },
            FieldInfoData {
                name: "knob_owner",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, knob_owner),
            },
            FieldInfoData {
                name: "lives",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, lives),
            },
            FieldInfoData {
                name: "new_life_reqs",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, new_life_reqs),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, team_id),
            },
            FieldInfoData {
                name: "asset_bal",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, asset_bal),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINGAMESTARTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinGameStartEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINGAMESTARTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINGAMESTARTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameStartEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinGameStartEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinLogoutEvent {
    pub r#type: String,
    pub end_reason: String,
    pub status_code: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINLOGOUTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinLogoutEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinLogoutEvent, r#type),
            },
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinLogoutEvent, end_reason),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinLogoutEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinLogoutEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINLOGOUTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinLogoutEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINLOGOUTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINLOGOUTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinLogoutEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinLogoutEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinLoginEvent {
    pub r#type: String,
    pub status: String,
    pub status_code: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINLOGINEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinLoginEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinLoginEvent, r#type),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinLoginEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinLoginEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinLoginEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINLOGINEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinLoginEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINLOGINEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINLOGINEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinLoginEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinLoginEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinBootEndEvent {
    pub end_reason: String,
    pub status_code: String,
    pub sdur: i64,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINBOOTENDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinBootEndEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinBootEndEvent, end_reason),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinBootEndEvent, status_code),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinBootEndEvent, sdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinBootEndEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINBOOTENDEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinBootEndEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINBOOTENDEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINBOOTENDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinBootEndEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinBootEndEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinBootStartEvent {
    pub status: String,
    pub source: String,
    pub status_code: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINBOOTSTARTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinBootStartEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinBootStartEvent, status),
            },
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinBootStartEvent, source),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinBootStartEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinBootStartEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINBOOTSTARTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinBootStartEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINBOOTSTARTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINBOOTSTARTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinBootStartEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinBootStartEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlayerServiceEvent {
    pub service: String,
    pub action: String,
    pub metadata: RawJsonString,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINPLAYERSERVICEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerServiceEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "service",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerServiceEvent, service),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerServiceEvent, action),
            },
            FieldInfoData {
                name: "metadata",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerServiceEvent, metadata),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerServiceEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERSERVICEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerServiceEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERSERVICEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERSERVICEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerServiceEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerServiceEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlayerStatsEvent {
    pub player_stats: RawJsonString,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINPLAYERSTATSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStatsEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "player_stats",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatsEvent, player_stats),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatsEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERSTATSEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerStatsEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERSTATSEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERSTATSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStatsEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerStatsEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinCameraStateEvent {
    pub gdur: u32,
    pub cam_dur: u32,
    pub prev_cam_state: String,
    pub cur_cam_state: String,
    pub p_loc: super::core::Vec3,
    pub p_dir: super::core::Vec3,
    pub p_state: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINCAMERASTATEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCameraStateEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, gdur),
            },
            FieldInfoData {
                name: "cam_dur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, cam_dur),
            },
            FieldInfoData {
                name: "prev_cam_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, prev_cam_state),
            },
            FieldInfoData {
                name: "cur_cam_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, cur_cam_state),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, p_dir),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, p_state),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINCAMERASTATEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinCameraStateEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINCAMERASTATEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINCAMERASTATEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCameraStateEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinCameraStateEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinPlayerObEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub r#type: String,
    pub p_loc: super::core::Vec3,
    pub p_dir: super::core::Vec3,
    pub p_class: String,
    pub p_team_id: String,
    pub p_veh_id: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINPLAYEROBEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerObEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, rdur),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, r#type),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, p_dir),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, p_team_id),
            },
            FieldInfoData {
                name: "p_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, p_veh_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYEROBEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerObEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYEROBEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYEROBEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerObEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerObEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinUiInteractionEvent {
    pub r#type: String,
    pub object_id: String,
    pub pgid: String,
    pub sdur: u32,
    pub object_type: String,
    pub mdur: u32,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINUIINTERACTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinUiInteractionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, r#type),
            },
            FieldInfoData {
                name: "object_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, object_id),
            },
            FieldInfoData {
                name: "pgid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, pgid),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, sdur),
            },
            FieldInfoData {
                name: "object_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, object_type),
            },
            FieldInfoData {
                name: "mdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, mdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINUIINTERACTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinUiInteractionEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINUIINTERACTIONEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINUIINTERACTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinUiInteractionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinUiInteractionEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinHardwareProfileEvent {
    pub cpu: String,
    pub sys_mem: String,
    pub gpu: String,
    pub gpu_mem: String,
    pub gpu_id: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINHARDWAREPROFILEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinHardwareProfileEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "cpu",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, cpu),
            },
            FieldInfoData {
                name: "sys_mem",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, sys_mem),
            },
            FieldInfoData {
                name: "gpu",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, gpu),
            },
            FieldInfoData {
                name: "gpu_mem",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, gpu_mem),
            },
            FieldInfoData {
                name: "gpu_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, gpu_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINHARDWAREPROFILEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinHardwareProfileEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINHARDWAREPROFILEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINHARDWAREPROFILEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinHardwareProfileEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinHardwareProfileEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlayerWeapSmryEvent {
    pub gdur: u32,
    pub rdur: u32,
    pub weap_id: String,
    pub weap_category: String,
    pub weap_type: String,
    pub p_class: String,
    pub p_team_id: String,
    pub veh_id: String,
    pub sht_fired: i32,
    pub equip_dur: i32,
    pub sht_hit_sldr: i32,
    pub hd_sht_sldr: i32,
    pub fatl_sht_sldr: i32,
    pub dmg_sldr: i32,
    pub sht_hit_veh: i32,
    pub veh_dstr: i32,
    pub dmg_veh: i32,
    pub weap_mods: RawJsonString,
    pub field_flag_changed0: u32,
}

pub const TELEMETRYSDKPINPLAYERWEAPSMRYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerWeapSmryEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, rdur),
            },
            FieldInfoData {
                name: "weap_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, weap_id),
            },
            FieldInfoData {
                name: "weap_category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, weap_category),
            },
            FieldInfoData {
                name: "weap_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, weap_type),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, p_team_id),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, veh_id),
            },
            FieldInfoData {
                name: "sht_fired",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, sht_fired),
            },
            FieldInfoData {
                name: "equip_dur",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, equip_dur),
            },
            FieldInfoData {
                name: "sht_hit_sldr",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, sht_hit_sldr),
            },
            FieldInfoData {
                name: "hd_sht_sldr",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, hd_sht_sldr),
            },
            FieldInfoData {
                name: "fatl_sht_sldr",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, fatl_sht_sldr),
            },
            FieldInfoData {
                name: "dmg_sldr",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, dmg_sldr),
            },
            FieldInfoData {
                name: "sht_hit_veh",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, sht_hit_veh),
            },
            FieldInfoData {
                name: "veh_dstr",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, veh_dstr),
            },
            FieldInfoData {
                name: "dmg_veh",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, dmg_veh),
            },
            FieldInfoData {
                name: "weap_mods",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, weap_mods),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERWEAPSMRYEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerWeapSmryEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERWEAPSMRYEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERWEAPSMRYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerWeapSmryEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerWeapSmryEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinTimerEvent {
    pub category: String,
    pub measure: String,
    pub dur: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub meta_data: RawJsonString,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINTIMEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTimerEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "category",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, category),
            },
            FieldInfoData {
                name: "measure",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, measure),
            },
            FieldInfoData {
                name: "dur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, dur),
            },
            FieldInfoData {
                name: "start_time",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, start_time),
            },
            FieldInfoData {
                name: "end_time",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, end_time),
            },
            FieldInfoData {
                name: "meta_data",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, meta_data),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINTIMEREVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinTimerEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINTIMEREVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINTIMEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTimerEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinTimerEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinMessageEvent {
    pub r#type: String,
    pub service: String,
    pub content_type: String,
    pub format: String,
    pub media: String,
    pub campaign_id: String,
    pub client_state: String,
    pub msg_id: String,
    pub status: String,
    pub status_code: String,
    pub option: String,
    pub content: String,
    pub destination_name: String,
    pub destination_id: String,
    pub place: String,
    pub loc: super::core::Vec2,
    pub size: super::core::Vec2,
    pub segment_id: String,
    pub count: i64,
    pub field_flag_changed0: u32,
}

pub const TELEMETRYSDKPINMESSAGEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMessageEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, r#type),
            },
            FieldInfoData {
                name: "service",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, service),
            },
            FieldInfoData {
                name: "content_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, content_type),
            },
            FieldInfoData {
                name: "format",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, format),
            },
            FieldInfoData {
                name: "media",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, media),
            },
            FieldInfoData {
                name: "campaign_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, campaign_id),
            },
            FieldInfoData {
                name: "client_state",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, client_state),
            },
            FieldInfoData {
                name: "msg_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, msg_id),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, status_code),
            },
            FieldInfoData {
                name: "option",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, option),
            },
            FieldInfoData {
                name: "content",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, content),
            },
            FieldInfoData {
                name: "destination_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, destination_name),
            },
            FieldInfoData {
                name: "destination_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, destination_id),
            },
            FieldInfoData {
                name: "place",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, place),
            },
            FieldInfoData {
                name: "loc",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, loc),
            },
            FieldInfoData {
                name: "size",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, size),
            },
            FieldInfoData {
                name: "segment_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, segment_id),
            },
            FieldInfoData {
                name: "count",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, count),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMESSAGEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinMessageEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINMESSAGEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINMESSAGEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMessageEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinMessageEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPageViewEvent {
    pub r#type: String,
    pub pgid: String,
    pub pgdur: u32,
    pub fromid: String,
    pub toid: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINPAGEVIEWEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPageViewEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, r#type),
            },
            FieldInfoData {
                name: "pgid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, pgid),
            },
            FieldInfoData {
                name: "pgdur",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, pgdur),
            },
            FieldInfoData {
                name: "fromid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, fromid),
            },
            FieldInfoData {
                name: "toid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, toid),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPAGEVIEWEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPageViewEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPAGEVIEWEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPAGEVIEWEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPageViewEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPageViewEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinAchievementEvent {
    pub r#type: String,
    pub achv_id: String,
    pub instance_id: String,
    pub reqs: RawJsonString,
    pub reward: RawJsonString,
    pub status: String,
    pub percent: u32,
    pub diff: String,
    pub gdur: i64,
    pub sdur: i64,
    pub cdur: i64,
    pub tdur: i64,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINACHIEVEMENTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinAchievementEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, r#type),
            },
            FieldInfoData {
                name: "achv_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, achv_id),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, instance_id),
            },
            FieldInfoData {
                name: "reqs",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, reqs),
            },
            FieldInfoData {
                name: "reward",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, reward),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, status),
            },
            FieldInfoData {
                name: "percent",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, percent),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, diff),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, gdur),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, sdur),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, cdur),
            },
            FieldInfoData {
                name: "tdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, tdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINACHIEVEMENTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinAchievementEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINACHIEVEMENTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINACHIEVEMENTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinAchievementEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinAchievementEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinMileStoneEvent {
    pub r#type: String,
    pub moment: String,
    pub mstid: String,
    pub game_mode: String,
    pub instance_id: String,
    pub diff: String,
    pub gdur: i64,
    pub sdur: i64,
    pub cdur: i64,
    pub tdur: i64,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINMILESTONEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMileStoneEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, r#type),
            },
            FieldInfoData {
                name: "moment",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, moment),
            },
            FieldInfoData {
                name: "mstid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, mstid),
            },
            FieldInfoData {
                name: "game_mode",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, game_mode),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, instance_id),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, diff),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, gdur),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, sdur),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, cdur),
            },
            FieldInfoData {
                name: "tdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, tdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMILESTONEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinMileStoneEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINMILESTONEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINMILESTONEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMileStoneEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinMileStoneEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinPlayerLevelEvent {
    pub r#type: String,
    pub mode: String,
    pub instance_id: String,
    pub level: String,
    pub level_name: String,
    pub gdur: i64,
    pub sdur: i64,
    pub cdur: i64,
    pub tdur: i64,
    pub field_flag_changed0: u16,
}

pub const TELEMETRYSDKPINPLAYERLEVELEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerLevelEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, r#type),
            },
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, mode),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, instance_id),
            },
            FieldInfoData {
                name: "level",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, level),
            },
            FieldInfoData {
                name: "level_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, level_name),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, gdur),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, sdur),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, cdur),
            },
            FieldInfoData {
                name: "tdur",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, tdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERLEVELEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerLevelEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERLEVELEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINPLAYERLEVELEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerLevelEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerLevelEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinFavoriteEvent {
    pub type1: String,
    pub type1_id: String,
    pub type1_name: String,
    pub type2: String,
    pub type2_id: String,
    pub type2_name: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINFAVORITEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFavoriteEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type1",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type1),
            },
            FieldInfoData {
                name: "type1_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type1_id),
            },
            FieldInfoData {
                name: "type1_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type1_name),
            },
            FieldInfoData {
                name: "type2",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type2),
            },
            FieldInfoData {
                name: "type2_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type2_id),
            },
            FieldInfoData {
                name: "type2_name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type2_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINFAVORITEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinFavoriteEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINFAVORITEEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINFAVORITEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFavoriteEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinFavoriteEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetrySdkPinDownloadEvent {
    pub item_id: String,
    pub item_type: String,
    pub item_platform: String,
    pub status: String,
    pub dur: i32,
    pub status_code: String,
    pub percent: f32,
    pub download_id: i32,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINDOWNLOADEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinDownloadEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, item_id),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, item_type),
            },
            FieldInfoData {
                name: "item_platform",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, item_platform),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, status),
            },
            FieldInfoData {
                name: "dur",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, dur),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, status_code),
            },
            FieldInfoData {
                name: "percent",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, percent),
            },
            FieldInfoData {
                name: "download_id",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, download_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINDOWNLOADEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinDownloadEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINDOWNLOADEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINDOWNLOADEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinDownloadEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinDownloadEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinSettingsEvent {
    pub r#type: String,
    pub status: String,
    pub status_code: String,
    pub settings: RawJsonString,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINSETTINGSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSettingsEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSettingsEvent, r#type),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSettingsEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSettingsEvent, status_code),
            },
            FieldInfoData {
                name: "settings",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSettingsEvent, settings),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinSettingsEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINSETTINGSEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinSettingsEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINSETTINGSEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINSETTINGSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSettingsEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinSettingsEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinRegistrationEvent {
    pub source: String,
    pub status: String,
    pub status_code: String,
    pub domain: String,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINREGISTRATIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRegistrationEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRegistrationEvent, source),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRegistrationEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRegistrationEvent, status_code),
            },
            FieldInfoData {
                name: "domain",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRegistrationEvent, domain),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinRegistrationEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINREGISTRATIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinRegistrationEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINREGISTRATIONEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINREGISTRATIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRegistrationEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinRegistrationEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySdkPinAccountEvent {
    pub r#type: String,
    pub status_code: String,
    pub acntid: RawJsonString,
    pub source: String,
    pub reason: String,
    pub duration: i64,
    pub metadata: RawJsonString,
    pub field_flag_changed0: u8,
}

pub const TELEMETRYSDKPINACCOUNTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinAccountEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, r#type),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, status_code),
            },
            FieldInfoData {
                name: "acntid",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, acntid),
            },
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, source),
            },
            FieldInfoData {
                name: "reason",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, reason),
            },
            FieldInfoData {
                name: "duration",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, duration),
            },
            FieldInfoData {
                name: "metadata",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, metadata),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINACCOUNTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinAccountEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINACCOUNTEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINACCOUNTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinAccountEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinAccountEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySDKPinEvent {
    pub pin_event_name: String,
    pub event_header_modifier: TelemetrySDKPinEventHeaderModifier,
    pub track_changes: bool,
}

pub const TELEMETRYSDKPINEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYLOGEVENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PinEventName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEvent, pin_event_name),
            },
            FieldInfoData {
                name: "eventHeaderModifier",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYSDKPINEVENTHEADERMODIFIER_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEvent, event_header_modifier),
            },
            FieldInfoData {
                name: "TrackChanges",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEvent, track_changes),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TelemetrySDKPinEvent {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINEVENT_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinEvent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySDKPinEventHeaderModifier {
    pub session_id: i64,
    pub game_id: i64,
    pub player_id_type: String,
    pub player_id: String,
    pub title_id_type: String,
    pub title_id: String,
    pub date_of_birth: String,
    pub current_level: String,
    pub current_level_name: String,
    pub release_type: String,
    pub platform: String,
    pub player_id_map: String,
    pub experiment_id: String,
    pub mac_address: String,
    pub device_id_map: String,
    pub custom_event_header: String,
    pub is_session: String,
    pub is_player: String,
    pub is_mlu: String,
    pub subs: String,
    pub game_mode: String,
    pub game_type: String,
    pub mode_type: String,
    pub map: String,
    pub field_flag_changed0: u32,
}

pub const TELEMETRYSDKPINEVENTHEADERMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEventHeaderModifier",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "sessionId",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, session_id),
            },
            FieldInfoData {
                name: "gameId",
                flags: MemberInfoFlags::new(0),
                field_type: INT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, game_id),
            },
            FieldInfoData {
                name: "playerIdType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, player_id_type),
            },
            FieldInfoData {
                name: "playerId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, player_id),
            },
            FieldInfoData {
                name: "titleIdType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, title_id_type),
            },
            FieldInfoData {
                name: "titleId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, title_id),
            },
            FieldInfoData {
                name: "dateOfBirth",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, date_of_birth),
            },
            FieldInfoData {
                name: "currentLevel",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, current_level),
            },
            FieldInfoData {
                name: "currentLevelName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, current_level_name),
            },
            FieldInfoData {
                name: "releaseType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, release_type),
            },
            FieldInfoData {
                name: "platform",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, platform),
            },
            FieldInfoData {
                name: "playerIdMap",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, player_id_map),
            },
            FieldInfoData {
                name: "experimentId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, experiment_id),
            },
            FieldInfoData {
                name: "macAddress",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, mac_address),
            },
            FieldInfoData {
                name: "deviceIdMap",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, device_id_map),
            },
            FieldInfoData {
                name: "customEventHeader",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, custom_event_header),
            },
            FieldInfoData {
                name: "isSession",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, is_session),
            },
            FieldInfoData {
                name: "isPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, is_player),
            },
            FieldInfoData {
                name: "isMlu",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, is_mlu),
            },
            FieldInfoData {
                name: "subs",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, subs),
            },
            FieldInfoData {
                name: "gameMode",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, game_mode),
            },
            FieldInfoData {
                name: "gameType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, game_type),
            },
            FieldInfoData {
                name: "modeType",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, mode_type),
            },
            FieldInfoData {
                name: "map",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, map),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINEVENTHEADERMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDKPinEventHeaderModifier {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSDKPINEVENTHEADERMODIFIER_TYPE_INFO
    }
}


pub const TELEMETRYSDKPINEVENTHEADERMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEventHeaderModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinEventHeaderModifier-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransactionalTelemetryHookEntityData {
    pub realm: super::core::Realm,
    pub stream: TransactionalStreamData,
}

pub const TRANSACTIONALTELEMETRYHOOKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryHookEntityData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TransactionalTelemetryHookEntityData, realm),
            },
            FieldInfoData {
                name: "Stream",
                flags: MemberInfoFlags::new(0),
                field_type: TRANSACTIONALSTREAMDATA_TYPE_INFO,
                rust_offset: offset_of!(TransactionalTelemetryHookEntityData, stream),
            },
        ],
    }),
    array_type: Some(TRANSACTIONALTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TransactionalTelemetryHookEntityData {
    fn type_info() -> &'static TypeInfo {
        TRANSACTIONALTELEMETRYHOOKENTITYDATA_TYPE_INFO
    }
}


pub const TRANSACTIONALTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryHookEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TransactionalTelemetryHookEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VarStreamTelemetryHookEntityData {
    pub realm: super::core::Realm,
    pub r#mod: String,
    pub grp: String,
    pub subgrp: String,
    pub stream: VarEventStreamData,
}

pub const VARSTREAMTELEMETRYHOOKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarStreamTelemetryHookEntityData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(VarStreamTelemetryHookEntityData, realm),
            },
            FieldInfoData {
                name: "mod",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VarStreamTelemetryHookEntityData, r#mod),
            },
            FieldInfoData {
                name: "grp",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VarStreamTelemetryHookEntityData, grp),
            },
            FieldInfoData {
                name: "subgrp",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VarStreamTelemetryHookEntityData, subgrp),
            },
            FieldInfoData {
                name: "Stream",
                flags: MemberInfoFlags::new(0),
                field_type: VAREVENTSTREAMDATA_TYPE_INFO,
                rust_offset: offset_of!(VarStreamTelemetryHookEntityData, stream),
            },
        ],
    }),
    array_type: Some(VARSTREAMTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VarStreamTelemetryHookEntityData {
    fn type_info() -> &'static TypeInfo {
        VARSTREAMTELEMETRYHOOKENTITYDATA_TYPE_INFO
    }
}


pub const VARSTREAMTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarStreamTelemetryHookEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("VarStreamTelemetryHookEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FixedStreamTelemetryHookEntityData {
    pub realm: super::core::Realm,
    pub stream: FixedEventStreamData,
    pub has_telemetry_sdk3_event_fields: bool,
    pub r#mod: String,
    pub grp: String,
    pub subgrp: String,
}

pub const FIXEDSTREAMTELEMETRYHOOKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedStreamTelemetryHookEntityData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, realm),
            },
            FieldInfoData {
                name: "Stream",
                flags: MemberInfoFlags::new(0),
                field_type: FIXEDEVENTSTREAMDATA_TYPE_INFO,
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, stream),
            },
            FieldInfoData {
                name: "HasTelemetrySdk3EventFields",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, has_telemetry_sdk3_event_fields),
            },
            FieldInfoData {
                name: "mod",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, r#mod),
            },
            FieldInfoData {
                name: "grp",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, grp),
            },
            FieldInfoData {
                name: "subgrp",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, subgrp),
            },
        ],
    }),
    array_type: Some(FIXEDSTREAMTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FixedStreamTelemetryHookEntityData {
    fn type_info() -> &'static TypeInfo {
        FIXEDSTREAMTELEMETRYHOOKENTITYDATA_TYPE_INFO
    }
}


pub const FIXEDSTREAMTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedStreamTelemetryHookEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("FixedStreamTelemetryHookEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryHookEntityData {
    pub realm: super::core::Realm,
    pub r#mod: String,
    pub grp: String,
    pub subgrp: String,
    pub params: Vec<TelemetryParameterDataProperty>,
    pub transports: Vec<TelemetryTransportData>,
}

pub const TELEMETRYHOOKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookEntityData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookEntityData, realm),
            },
            FieldInfoData {
                name: "mod",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookEntityData, r#mod),
            },
            FieldInfoData {
                name: "grp",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookEntityData, grp),
            },
            FieldInfoData {
                name: "subgrp",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookEntityData, subgrp),
            },
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(144),
                field_type: TELEMETRYPARAMETERDATAPROPERTY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookEntityData, params),
            },
            FieldInfoData {
                name: "Transports",
                flags: MemberInfoFlags::new(144),
                field_type: TELEMETRYTRANSPORTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookEntityData, transports),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookEntityData {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKENTITYDATA_TYPE_INFO
    }
}


pub const TELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryGenericHookEntityData {
    pub realm: super::core::Realm,
    pub stream: EventStreamData,
    pub log_event: TelemetryLogEvent,
}

pub const TELEMETRYGENERICHOOKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryGenericHookEntityData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TelemetryGenericHookEntityData, realm),
            },
            FieldInfoData {
                name: "Stream",
                flags: MemberInfoFlags::new(0),
                field_type: EVENTSTREAMDATA_TYPE_INFO,
                rust_offset: offset_of!(TelemetryGenericHookEntityData, stream),
            },
            FieldInfoData {
                name: "LogEvent",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYLOGEVENT_TYPE_INFO,
                rust_offset: offset_of!(TelemetryGenericHookEntityData, log_event),
            },
        ],
    }),
    array_type: Some(TELEMETRYGENERICHOOKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryGenericHookEntityData {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYGENERICHOOKENTITYDATA_TYPE_INFO
    }
}


pub const TELEMETRYGENERICHOOKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryGenericHookEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryGenericHookEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryClearTelemetryTransactionMessageBase {
}

pub const TELEMETRYCLEARTELEMETRYTRANSACTIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryClearTelemetryTransactionMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetryClearTelemetryTransactionMessageBase {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYCLEARTELEMETRYTRANSACTIONMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryCommitTelemetryTransactionMessageBase {
}

pub const TELEMETRYCOMMITTELEMETRYTRANSACTIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryCommitTelemetryTransactionMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetryCommitTelemetryTransactionMessageBase {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYCOMMITTELEMETRYTRANSACTIONMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySendTelemetryTransactionRowMessageBase {
}

pub const TELEMETRYSENDTELEMETRYTRANSACTIONROWMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySendTelemetryTransactionRowMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetrySendTelemetryTransactionRowMessageBase {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSENDTELEMETRYTRANSACTIONROWMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySendTelemetryRowMessageBase {
}

pub const TELEMETRYSENDTELEMETRYROWMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySendTelemetryRowMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetrySendTelemetryRowMessageBase {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSENDTELEMETRYROWMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetrySendEventMessage {
}

pub const TELEMETRYSENDEVENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySendEventMessage",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for TelemetrySendEventMessage {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYSENDEVENTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryHookParameterStringArray {
    pub data: Vec<String>,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERSTRINGARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterStringArray",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterStringArray, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterStringArray, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterStringArray, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERSTRINGARRAY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterStringArray {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERSTRINGARRAY_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERSTRINGARRAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterStringArray-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterStringArray-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryHookParameterRawJsonString {
    pub data: RawJsonString,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERRAWJSONSTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterRawJsonString",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: RAWJSONSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterRawJsonString, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterRawJsonString, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterRawJsonString, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERRAWJSONSTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterRawJsonString {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERRAWJSONSTRING_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERRAWJSONSTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterRawJsonString-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterRawJsonString-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetryHookParameterTransform {
    pub data: super::core::LinearTransform,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERTRANSFORM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterTransform",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterTransform, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterTransform, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterTransform, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERTRANSFORM_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetryHookParameterTransform {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERTRANSFORM_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERTRANSFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterTransform-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterTransform-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetryHookParameterVec4 {
    pub data: super::core::Vec4,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERVEC4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec4",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterVec4, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterVec4, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterVec4, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERVEC4_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetryHookParameterVec4 {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERVEC4_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERVEC4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec4-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterVec4-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetryHookParameterVec3 {
    pub data: super::core::Vec3,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERVEC3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec3",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterVec3, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterVec3, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterVec3, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERVEC3_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetryHookParameterVec3 {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERVEC3_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERVEC3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec3-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterVec3-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetryHookParameterVec2 {
    pub data: super::core::Vec2,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERVEC2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec2",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterVec2, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterVec2, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterVec2, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERVEC2_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterVec2 {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERVEC2_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERVEC2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec2-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterVec2-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryHookParameterBool {
    pub data: bool,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERBOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterBool",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterBool, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterBool, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterBool, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERBOOL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterBool {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERBOOL_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERBOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterBool-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterBool-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryHookParameterUint64 {
    pub data: u64,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERUINT64_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterUint64",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterUint64, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterUint64, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterUint64, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERUINT64_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterUint64 {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERUINT64_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERUINT64_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterUint64-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterUint64-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryHookParameterUint {
    pub data: u32,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERUINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterUint",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterUint, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterUint, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterUint, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERUINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterUint {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERUINT_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERUINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterUint-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterUint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryHookParameterString {
    pub data: String,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERSTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterString",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterString, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterString, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterString, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERSTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterString {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERSTRING_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERSTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterString-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterString-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TelemetryHookParameterFloat {
    pub data: f32,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERFLOAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterFloat",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterFloat, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterFloat, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterFloat, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERFLOAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterFloat {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERFLOAT_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERFLOAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterFloat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterFloat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryHookParameterInt {
    pub data: i32,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterInt",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterInt, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterInt, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterInt, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterInt {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERINT_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterInt-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterInt-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryHookParameterChar {
    pub data: u8,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETERCHAR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterChar",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterChar, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterChar, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameterChar, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERCHAR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterChar {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERCHAR_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETERCHAR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterChar-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterChar-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryHookParameter {
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub const TELEMETRYHOOKPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameter",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameter, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryHookParameter, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameter {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETER_TYPE_INFO
    }
}


pub const TELEMETRYHOOKPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameter-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TelemetryParameterDataProperty {
    pub stat_name: String,
    pub data_type: TelemetryParameterType,
}

pub const TELEMETRYPARAMETERDATAPROPERTY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryParameterDataProperty",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "StatName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(TelemetryParameterDataProperty, stat_name),
            },
            FieldInfoData {
                name: "DataType",
                flags: MemberInfoFlags::new(0),
                field_type: TELEMETRYPARAMETERTYPE_TYPE_INFO,
                rust_offset: offset_of!(TelemetryParameterDataProperty, data_type),
            },
        ],
    }),
    array_type: Some(TELEMETRYPARAMETERDATAPROPERTY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryParameterDataProperty {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYPARAMETERDATAPROPERTY_TYPE_INFO
    }
}


pub const TELEMETRYPARAMETERDATAPROPERTY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryParameterDataProperty-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryParameterDataProperty-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RawJsonString {
    pub raw_json: String,
}

pub const RAWJSONSTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RawJsonString",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "RawJson",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(RawJsonString, raw_json),
            },
        ],
    }),
    array_type: Some(RAWJSONSTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RawJsonString {
    fn type_info() -> &'static TypeInfo {
        RAWJSONSTRING_TYPE_INFO
    }
}


pub const RAWJSONSTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RawJsonString-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("RawJsonString-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpecialTypeData {
    pub owns_value: bool,
}

pub const SPECIALTYPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpecialTypeData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OwnsValue",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(SpecialTypeData, owns_value),
            },
        ],
    }),
    array_type: Some(SPECIALTYPEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SpecialTypeData {
    fn type_info() -> &'static TypeInfo {
        SPECIALTYPEDATA_TYPE_INFO
    }
}


pub const SPECIALTYPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpecialTypeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("SpecialTypeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TelemetryParameterType {
    #[default]
    TelemetryParameterType_Char = 0,
    TelemetryParameterType_Int = 1,
    TelemetryParameterType_Float = 2,
    TelemetryParameterType_String = 3,
    TelemetryParameterType_Uint = 4,
    TelemetryParameterType_Int64 = 5,
    TelemetryParameterType_Uint64 = 6,
    TelemetryParameterType_Bool = 7,
    TelemetryParameterType_Vec2 = 8,
    TelemetryParameterType_Vec3 = 9,
    TelemetryParameterType_Vec4 = 10,
    TelemetryParameterType_Transform = 11,
    TelemetryParameterType_Special = 12,
    TelemetryParameterType_RawJsonString = 13,
    TelemetryParameterType_StringArray = 14,
}

pub const TELEMETRYPARAMETERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryParameterType",
    flags: MemberInfoFlags::new(49429),
    module: "TelemetryShared",
    data: TypeInfoData::Enum,
    array_type: Some(TELEMETRYPARAMETERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TelemetryParameterType {
    fn type_info() -> &'static TypeInfo {
        TELEMETRYPARAMETERTYPE_TYPE_INFO
    }
}


pub const TELEMETRYPARAMETERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryParameterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryParameterType-Array"),
    array_type: None,
    alignment: 8,
};


