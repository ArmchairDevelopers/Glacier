use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct TelemetrySDKPinTransportData {
    pub _glacier_base: TelemetryTransportData,
    pub end_point_config: TelemetrySDKPinEndPointConfig,
    pub log_level: i32,
    pub session_header_config: TelemetrySDKPinSessionHeaderConfig,
    pub event_header_config: TelemetrySDKPinEventHeaderConfig,
}

pub trait TelemetrySDKPinTransportDataTrait: TelemetryTransportDataTrait {
    fn end_point_config(&self) -> &TelemetrySDKPinEndPointConfig;
    fn end_point_config_mut(&mut self) -> &mut TelemetrySDKPinEndPointConfig;
    fn log_level(&self) -> &i32;
    fn log_level_mut(&mut self) -> &mut i32;
    fn session_header_config(&self) -> &TelemetrySDKPinSessionHeaderConfig;
    fn session_header_config_mut(&mut self) -> &mut TelemetrySDKPinSessionHeaderConfig;
    fn event_header_config(&self) -> &TelemetrySDKPinEventHeaderConfig;
    fn event_header_config_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderConfig;
}

impl TelemetrySDKPinTransportDataTrait for TelemetrySDKPinTransportData {
    fn end_point_config(&self) -> &TelemetrySDKPinEndPointConfig {
        &self.end_point_config
    }
    fn end_point_config_mut(&mut self) -> &mut TelemetrySDKPinEndPointConfig {
        &mut self.end_point_config
    }
    fn log_level(&self) -> &i32 {
        &self.log_level
    }
    fn log_level_mut(&mut self) -> &mut i32 {
        &mut self.log_level
    }
    fn session_header_config(&self) -> &TelemetrySDKPinSessionHeaderConfig {
        &self.session_header_config
    }
    fn session_header_config_mut(&mut self) -> &mut TelemetrySDKPinSessionHeaderConfig {
        &mut self.session_header_config
    }
    fn event_header_config(&self) -> &TelemetrySDKPinEventHeaderConfig {
        &self.event_header_config
    }
    fn event_header_config_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderConfig {
        &mut self.event_header_config
    }
}

impl TelemetryTransportDataTrait for TelemetrySDKPinTransportData {
    fn transport_id(&self) -> &u32 {
        self._glacier_base.transport_id()
    }
    fn transport_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.transport_id_mut()
    }
}

impl super::core::AssetTrait for TelemetrySDKPinTransportData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TelemetrySDKPinTransportData {
}

pub static TELEMETRYSDKPINTRANSPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinTransportData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYTRANSPORTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySDKPinTransportData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EndPointConfig",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetrySDKPinEndPointConfig",
                rust_offset: offset_of!(TelemetrySDKPinTransportData, end_point_config),
            },
            FieldInfoData {
                name: "logLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySDKPinTransportData, log_level),
            },
            FieldInfoData {
                name: "SessionHeaderConfig",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetrySDKPinSessionHeaderConfig",
                rust_offset: offset_of!(TelemetrySDKPinTransportData, session_header_config),
            },
            FieldInfoData {
                name: "EventHeaderConfig",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetrySDKPinEventHeaderConfig",
                rust_offset: offset_of!(TelemetrySDKPinTransportData, event_header_config),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINTRANSPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDKPinTransportData {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINTRANSPORTDATA_TYPE_INFO
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


pub static TELEMETRYSDKPINTRANSPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinTransportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinTransportData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait TelemetrySDKPinEventHeaderConfigTrait: TypeObject {
    fn other_player_ids(&self) -> &String;
    fn other_player_ids_mut(&mut self) -> &mut String;
    fn date_of_birth(&self) -> &String;
    fn date_of_birth_mut(&mut self) -> &mut String;
    fn experiment_id(&self) -> &String;
    fn experiment_id_mut(&mut self) -> &mut String;
    fn player_id_type(&self) -> &String;
    fn player_id_type_mut(&mut self) -> &mut String;
    fn player_id(&self) -> &String;
    fn player_id_mut(&mut self) -> &mut String;
    fn title_id_type(&self) -> &String;
    fn title_id_type_mut(&mut self) -> &mut String;
    fn title_id(&self) -> &String;
    fn title_id_mut(&mut self) -> &mut String;
    fn release_type(&self) -> &String;
    fn release_type_mut(&mut self) -> &mut String;
    fn platform(&self) -> &String;
    fn platform_mut(&mut self) -> &mut String;
    fn mac_address(&self) -> &String;
    fn mac_address_mut(&mut self) -> &mut String;
    fn device_id_map(&self) -> &String;
    fn device_id_map_mut(&mut self) -> &mut String;
    fn game_mode(&self) -> &String;
    fn game_mode_mut(&mut self) -> &mut String;
    fn game_type(&self) -> &String;
    fn game_type_mut(&mut self) -> &mut String;
    fn mode_type(&self) -> &String;
    fn mode_type_mut(&mut self) -> &mut String;
    fn map(&self) -> &String;
    fn map_mut(&mut self) -> &mut String;
    fn level(&self) -> &String;
    fn level_mut(&mut self) -> &mut String;
    fn level_name(&self) -> &String;
    fn level_name_mut(&mut self) -> &mut String;
    fn is_sess(&self) -> &String;
    fn is_sess_mut(&mut self) -> &mut String;
    fn is_player(&self) -> &String;
    fn is_player_mut(&mut self) -> &mut String;
    fn is_mlu(&self) -> &String;
    fn is_mlu_mut(&mut self) -> &mut String;
    fn subs(&self) -> &String;
    fn subs_mut(&mut self) -> &mut String;
    fn custom_event_headers(&self) -> &String;
    fn custom_event_headers_mut(&mut self) -> &mut String;
}

impl TelemetrySDKPinEventHeaderConfigTrait for TelemetrySDKPinEventHeaderConfig {
    fn other_player_ids(&self) -> &String {
        &self.other_player_ids
    }
    fn other_player_ids_mut(&mut self) -> &mut String {
        &mut self.other_player_ids
    }
    fn date_of_birth(&self) -> &String {
        &self.date_of_birth
    }
    fn date_of_birth_mut(&mut self) -> &mut String {
        &mut self.date_of_birth
    }
    fn experiment_id(&self) -> &String {
        &self.experiment_id
    }
    fn experiment_id_mut(&mut self) -> &mut String {
        &mut self.experiment_id
    }
    fn player_id_type(&self) -> &String {
        &self.player_id_type
    }
    fn player_id_type_mut(&mut self) -> &mut String {
        &mut self.player_id_type
    }
    fn player_id(&self) -> &String {
        &self.player_id
    }
    fn player_id_mut(&mut self) -> &mut String {
        &mut self.player_id
    }
    fn title_id_type(&self) -> &String {
        &self.title_id_type
    }
    fn title_id_type_mut(&mut self) -> &mut String {
        &mut self.title_id_type
    }
    fn title_id(&self) -> &String {
        &self.title_id
    }
    fn title_id_mut(&mut self) -> &mut String {
        &mut self.title_id
    }
    fn release_type(&self) -> &String {
        &self.release_type
    }
    fn release_type_mut(&mut self) -> &mut String {
        &mut self.release_type
    }
    fn platform(&self) -> &String {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut String {
        &mut self.platform
    }
    fn mac_address(&self) -> &String {
        &self.mac_address
    }
    fn mac_address_mut(&mut self) -> &mut String {
        &mut self.mac_address
    }
    fn device_id_map(&self) -> &String {
        &self.device_id_map
    }
    fn device_id_map_mut(&mut self) -> &mut String {
        &mut self.device_id_map
    }
    fn game_mode(&self) -> &String {
        &self.game_mode
    }
    fn game_mode_mut(&mut self) -> &mut String {
        &mut self.game_mode
    }
    fn game_type(&self) -> &String {
        &self.game_type
    }
    fn game_type_mut(&mut self) -> &mut String {
        &mut self.game_type
    }
    fn mode_type(&self) -> &String {
        &self.mode_type
    }
    fn mode_type_mut(&mut self) -> &mut String {
        &mut self.mode_type
    }
    fn map(&self) -> &String {
        &self.map
    }
    fn map_mut(&mut self) -> &mut String {
        &mut self.map
    }
    fn level(&self) -> &String {
        &self.level
    }
    fn level_mut(&mut self) -> &mut String {
        &mut self.level
    }
    fn level_name(&self) -> &String {
        &self.level_name
    }
    fn level_name_mut(&mut self) -> &mut String {
        &mut self.level_name
    }
    fn is_sess(&self) -> &String {
        &self.is_sess
    }
    fn is_sess_mut(&mut self) -> &mut String {
        &mut self.is_sess
    }
    fn is_player(&self) -> &String {
        &self.is_player
    }
    fn is_player_mut(&mut self) -> &mut String {
        &mut self.is_player
    }
    fn is_mlu(&self) -> &String {
        &self.is_mlu
    }
    fn is_mlu_mut(&mut self) -> &mut String {
        &mut self.is_mlu
    }
    fn subs(&self) -> &String {
        &self.subs
    }
    fn subs_mut(&mut self) -> &mut String {
        &mut self.subs
    }
    fn custom_event_headers(&self) -> &String {
        &self.custom_event_headers
    }
    fn custom_event_headers_mut(&mut self) -> &mut String {
        &mut self.custom_event_headers
    }
}

pub static TELEMETRYSDKPINEVENTHEADERCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEventHeaderConfig",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySDKPinEventHeaderConfig as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "OtherPlayerIds",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, other_player_ids),
            },
            FieldInfoData {
                name: "DateOfBirth",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, date_of_birth),
            },
            FieldInfoData {
                name: "ExperimentId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, experiment_id),
            },
            FieldInfoData {
                name: "PlayerIdType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, player_id_type),
            },
            FieldInfoData {
                name: "PlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, player_id),
            },
            FieldInfoData {
                name: "TitleIdType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, title_id_type),
            },
            FieldInfoData {
                name: "TitleId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, title_id),
            },
            FieldInfoData {
                name: "ReleaseType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, release_type),
            },
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, platform),
            },
            FieldInfoData {
                name: "MacAddress",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, mac_address),
            },
            FieldInfoData {
                name: "DeviceIdMap",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, device_id_map),
            },
            FieldInfoData {
                name: "GameMode",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, game_mode),
            },
            FieldInfoData {
                name: "GameType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, game_type),
            },
            FieldInfoData {
                name: "ModeType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, mode_type),
            },
            FieldInfoData {
                name: "Map",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, map),
            },
            FieldInfoData {
                name: "Level",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, level),
            },
            FieldInfoData {
                name: "LevelName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, level_name),
            },
            FieldInfoData {
                name: "IsSess",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, is_sess),
            },
            FieldInfoData {
                name: "IsPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, is_player),
            },
            FieldInfoData {
                name: "IsMlu",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, is_mlu),
            },
            FieldInfoData {
                name: "Subs",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, subs),
            },
            FieldInfoData {
                name: "CustomEventHeaders",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderConfig, custom_event_headers),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINEVENTHEADERCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDKPinEventHeaderConfig {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINEVENTHEADERCONFIG_TYPE_INFO
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


pub static TELEMETRYSDKPINEVENTHEADERCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEventHeaderConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinEventHeaderConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait TelemetrySDKPinSessionHeaderConfigTrait: TypeObject {
    fn event_type(&self) -> &String;
    fn event_type_mut(&mut self) -> &mut String;
    fn build_version(&self) -> &String;
    fn build_version_mut(&mut self) -> &mut String;
    fn locale(&self) -> &String;
    fn locale_mut(&mut self) -> &mut String;
    fn custom_session_headers(&self) -> &String;
    fn custom_session_headers_mut(&mut self) -> &mut String;
    fn is_mlu(&self) -> &String;
    fn is_mlu_mut(&mut self) -> &mut String;
    fn subs(&self) -> &String;
    fn subs_mut(&mut self) -> &mut String;
    fn player_id_type(&self) -> &String;
    fn player_id_type_mut(&mut self) -> &mut String;
    fn player_id(&self) -> &String;
    fn player_id_mut(&mut self) -> &mut String;
    fn title_id_type(&self) -> &String;
    fn title_id_type_mut(&mut self) -> &mut String;
    fn title_id(&self) -> &String;
    fn title_id_mut(&mut self) -> &mut String;
    fn release_type(&self) -> &String;
    fn release_type_mut(&mut self) -> &mut String;
    fn platform(&self) -> &String;
    fn platform_mut(&mut self) -> &mut String;
    fn mac_address(&self) -> &String;
    fn mac_address_mut(&mut self) -> &mut String;
    fn device_id_map(&self) -> &String;
    fn device_id_map_mut(&mut self) -> &mut String;
}

impl TelemetrySDKPinSessionHeaderConfigTrait for TelemetrySDKPinSessionHeaderConfig {
    fn event_type(&self) -> &String {
        &self.event_type
    }
    fn event_type_mut(&mut self) -> &mut String {
        &mut self.event_type
    }
    fn build_version(&self) -> &String {
        &self.build_version
    }
    fn build_version_mut(&mut self) -> &mut String {
        &mut self.build_version
    }
    fn locale(&self) -> &String {
        &self.locale
    }
    fn locale_mut(&mut self) -> &mut String {
        &mut self.locale
    }
    fn custom_session_headers(&self) -> &String {
        &self.custom_session_headers
    }
    fn custom_session_headers_mut(&mut self) -> &mut String {
        &mut self.custom_session_headers
    }
    fn is_mlu(&self) -> &String {
        &self.is_mlu
    }
    fn is_mlu_mut(&mut self) -> &mut String {
        &mut self.is_mlu
    }
    fn subs(&self) -> &String {
        &self.subs
    }
    fn subs_mut(&mut self) -> &mut String {
        &mut self.subs
    }
    fn player_id_type(&self) -> &String {
        &self.player_id_type
    }
    fn player_id_type_mut(&mut self) -> &mut String {
        &mut self.player_id_type
    }
    fn player_id(&self) -> &String {
        &self.player_id
    }
    fn player_id_mut(&mut self) -> &mut String {
        &mut self.player_id
    }
    fn title_id_type(&self) -> &String {
        &self.title_id_type
    }
    fn title_id_type_mut(&mut self) -> &mut String {
        &mut self.title_id_type
    }
    fn title_id(&self) -> &String {
        &self.title_id
    }
    fn title_id_mut(&mut self) -> &mut String {
        &mut self.title_id
    }
    fn release_type(&self) -> &String {
        &self.release_type
    }
    fn release_type_mut(&mut self) -> &mut String {
        &mut self.release_type
    }
    fn platform(&self) -> &String {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut String {
        &mut self.platform
    }
    fn mac_address(&self) -> &String {
        &self.mac_address
    }
    fn mac_address_mut(&mut self) -> &mut String {
        &mut self.mac_address
    }
    fn device_id_map(&self) -> &String {
        &self.device_id_map
    }
    fn device_id_map_mut(&mut self) -> &mut String {
        &mut self.device_id_map
    }
}

pub static TELEMETRYSDKPINSESSIONHEADERCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinSessionHeaderConfig",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySDKPinSessionHeaderConfig as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EventType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, event_type),
            },
            FieldInfoData {
                name: "BuildVersion",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, build_version),
            },
            FieldInfoData {
                name: "Locale",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, locale),
            },
            FieldInfoData {
                name: "CustomSessionHeaders",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, custom_session_headers),
            },
            FieldInfoData {
                name: "IsMlu",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, is_mlu),
            },
            FieldInfoData {
                name: "Subs",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, subs),
            },
            FieldInfoData {
                name: "PlayerIdType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, player_id_type),
            },
            FieldInfoData {
                name: "PlayerId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, player_id),
            },
            FieldInfoData {
                name: "TitleIdType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, title_id_type),
            },
            FieldInfoData {
                name: "TitleId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, title_id),
            },
            FieldInfoData {
                name: "ReleaseType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, release_type),
            },
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, platform),
            },
            FieldInfoData {
                name: "MacAddress",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, mac_address),
            },
            FieldInfoData {
                name: "DeviceIdMap",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinSessionHeaderConfig, device_id_map),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINSESSIONHEADERCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDKPinSessionHeaderConfig {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINSESSIONHEADERCONFIG_TYPE_INFO
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


pub static TELEMETRYSDKPINSESSIONHEADERCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinSessionHeaderConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinSessionHeaderConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySDKPinEndPointConfig {
    pub server_address: String,
    pub server_port: i32,
    pub environment: String,
}

pub trait TelemetrySDKPinEndPointConfigTrait: TypeObject {
    fn server_address(&self) -> &String;
    fn server_address_mut(&mut self) -> &mut String;
    fn server_port(&self) -> &i32;
    fn server_port_mut(&mut self) -> &mut i32;
    fn environment(&self) -> &String;
    fn environment_mut(&mut self) -> &mut String;
}

impl TelemetrySDKPinEndPointConfigTrait for TelemetrySDKPinEndPointConfig {
    fn server_address(&self) -> &String {
        &self.server_address
    }
    fn server_address_mut(&mut self) -> &mut String {
        &mut self.server_address
    }
    fn server_port(&self) -> &i32 {
        &self.server_port
    }
    fn server_port_mut(&mut self) -> &mut i32 {
        &mut self.server_port
    }
    fn environment(&self) -> &String {
        &self.environment
    }
    fn environment_mut(&mut self) -> &mut String {
        &mut self.environment
    }
}

pub static TELEMETRYSDKPINENDPOINTCONFIG_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEndPointConfig",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySDKPinEndPointConfig as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ServerAddress",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEndPointConfig, server_address),
            },
            FieldInfoData {
                name: "ServerPort",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySDKPinEndPointConfig, server_port),
            },
            FieldInfoData {
                name: "Environment",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEndPointConfig, environment),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINENDPOINTCONFIG_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDKPinEndPointConfig {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINENDPOINTCONFIG_TYPE_INFO
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


pub static TELEMETRYSDKPINENDPOINTCONFIG_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEndPointConfig-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinEndPointConfig"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryCSVTransportData {
    pub _glacier_base: TelemetryTransportData,
    pub file_name: String,
    pub time_stamped: bool,
    pub writes_per_flush: u32,
    pub overwrite_file: bool,
}

pub trait TelemetryCSVTransportDataTrait: TelemetryTransportDataTrait {
    fn file_name(&self) -> &String;
    fn file_name_mut(&mut self) -> &mut String;
    fn time_stamped(&self) -> &bool;
    fn time_stamped_mut(&mut self) -> &mut bool;
    fn writes_per_flush(&self) -> &u32;
    fn writes_per_flush_mut(&mut self) -> &mut u32;
    fn overwrite_file(&self) -> &bool;
    fn overwrite_file_mut(&mut self) -> &mut bool;
}

impl TelemetryCSVTransportDataTrait for TelemetryCSVTransportData {
    fn file_name(&self) -> &String {
        &self.file_name
    }
    fn file_name_mut(&mut self) -> &mut String {
        &mut self.file_name
    }
    fn time_stamped(&self) -> &bool {
        &self.time_stamped
    }
    fn time_stamped_mut(&mut self) -> &mut bool {
        &mut self.time_stamped
    }
    fn writes_per_flush(&self) -> &u32 {
        &self.writes_per_flush
    }
    fn writes_per_flush_mut(&mut self) -> &mut u32 {
        &mut self.writes_per_flush
    }
    fn overwrite_file(&self) -> &bool {
        &self.overwrite_file
    }
    fn overwrite_file_mut(&mut self) -> &mut bool {
        &mut self.overwrite_file
    }
}

impl TelemetryTransportDataTrait for TelemetryCSVTransportData {
    fn transport_id(&self) -> &u32 {
        self._glacier_base.transport_id()
    }
    fn transport_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.transport_id_mut()
    }
}

impl super::core::AssetTrait for TelemetryCSVTransportData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TelemetryCSVTransportData {
}

pub static TELEMETRYCSVTRANSPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryCSVTransportData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYTRANSPORTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryCSVTransportData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FileName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryCSVTransportData, file_name),
            },
            FieldInfoData {
                name: "TimeStamped",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetryCSVTransportData, time_stamped),
            },
            FieldInfoData {
                name: "WritesPerFlush",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetryCSVTransportData, writes_per_flush),
            },
            FieldInfoData {
                name: "OverwriteFile",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetryCSVTransportData, overwrite_file),
            },
        ],
    }),
    array_type: Some(TELEMETRYCSVTRANSPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryCSVTransportData {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYCSVTRANSPORTDATA_TYPE_INFO
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


pub static TELEMETRYCSVTRANSPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryCSVTransportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryCSVTransportData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryTTYTransportData {
    pub _glacier_base: TelemetryTransportData,
    pub max_buffer: u32,
}

pub trait TelemetryTTYTransportDataTrait: TelemetryTransportDataTrait {
    fn max_buffer(&self) -> &u32;
    fn max_buffer_mut(&mut self) -> &mut u32;
}

impl TelemetryTTYTransportDataTrait for TelemetryTTYTransportData {
    fn max_buffer(&self) -> &u32 {
        &self.max_buffer
    }
    fn max_buffer_mut(&mut self) -> &mut u32 {
        &mut self.max_buffer
    }
}

impl TelemetryTransportDataTrait for TelemetryTTYTransportData {
    fn transport_id(&self) -> &u32 {
        self._glacier_base.transport_id()
    }
    fn transport_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.transport_id_mut()
    }
}

impl super::core::AssetTrait for TelemetryTTYTransportData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TelemetryTTYTransportData {
}

pub static TELEMETRYTTYTRANSPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTTYTransportData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYTRANSPORTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryTTYTransportData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaxBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetryTTYTransportData, max_buffer),
            },
        ],
    }),
    array_type: Some(TELEMETRYTTYTRANSPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryTTYTransportData {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYTTYTRANSPORTDATA_TYPE_INFO
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


pub static TELEMETRYTTYTRANSPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTTYTransportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryTTYTransportData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySDK3TransportData {
    pub _glacier_base: TelemetryTransportData,
    pub is_production: bool,
    pub project_id: u32,
    pub version_name: String,
    pub log_level: i32,
}

pub trait TelemetrySDK3TransportDataTrait: TelemetryTransportDataTrait {
    fn is_production(&self) -> &bool;
    fn is_production_mut(&mut self) -> &mut bool;
    fn project_id(&self) -> &u32;
    fn project_id_mut(&mut self) -> &mut u32;
    fn version_name(&self) -> &String;
    fn version_name_mut(&mut self) -> &mut String;
    fn log_level(&self) -> &i32;
    fn log_level_mut(&mut self) -> &mut i32;
}

impl TelemetrySDK3TransportDataTrait for TelemetrySDK3TransportData {
    fn is_production(&self) -> &bool {
        &self.is_production
    }
    fn is_production_mut(&mut self) -> &mut bool {
        &mut self.is_production
    }
    fn project_id(&self) -> &u32 {
        &self.project_id
    }
    fn project_id_mut(&mut self) -> &mut u32 {
        &mut self.project_id
    }
    fn version_name(&self) -> &String {
        &self.version_name
    }
    fn version_name_mut(&mut self) -> &mut String {
        &mut self.version_name
    }
    fn log_level(&self) -> &i32 {
        &self.log_level
    }
    fn log_level_mut(&mut self) -> &mut i32 {
        &mut self.log_level
    }
}

impl TelemetryTransportDataTrait for TelemetrySDK3TransportData {
    fn transport_id(&self) -> &u32 {
        self._glacier_base.transport_id()
    }
    fn transport_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.transport_id_mut()
    }
}

impl super::core::AssetTrait for TelemetrySDK3TransportData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TelemetrySDK3TransportData {
}

pub static TELEMETRYSDK3TRANSPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDK3TransportData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYTRANSPORTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySDK3TransportData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IsProduction",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySDK3TransportData, is_production),
            },
            FieldInfoData {
                name: "ProjectId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySDK3TransportData, project_id),
            },
            FieldInfoData {
                name: "VersionName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDK3TransportData, version_name),
            },
            FieldInfoData {
                name: "logLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySDK3TransportData, log_level),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDK3TRANSPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDK3TransportData {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDK3TRANSPORTDATA_TYPE_INFO
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


pub static TELEMETRYSDK3TRANSPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDK3TransportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDK3TransportData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryTransportData {
    pub _glacier_base: super::core::Asset,
    pub transport_id: u32,
}

pub trait TelemetryTransportDataTrait: super::core::AssetTrait {
    fn transport_id(&self) -> &u32;
    fn transport_id_mut(&mut self) -> &mut u32;
}

impl TelemetryTransportDataTrait for TelemetryTransportData {
    fn transport_id(&self) -> &u32 {
        &self.transport_id
    }
    fn transport_id_mut(&mut self) -> &mut u32 {
        &mut self.transport_id
    }
}

impl super::core::AssetTrait for TelemetryTransportData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TelemetryTransportData {
}

pub static TELEMETRYTRANSPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransportData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryTransportData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransportId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetryTransportData, transport_id),
            },
        ],
    }),
    array_type: Some(TELEMETRYTRANSPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryTransportData {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYTRANSPORTDATA_TYPE_INFO
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


pub static TELEMETRYTRANSPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryTransportData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TransactionalStreamData {
    pub _glacier_base: StreamData,
    pub format: Option<Arc<Mutex<dyn FixedTransactionalStreamFormatTrait>>>,
    pub auto_commit: bool,
}

pub trait TransactionalStreamDataTrait: StreamDataTrait {
    fn format(&self) -> &Option<Arc<Mutex<dyn FixedTransactionalStreamFormatTrait>>>;
    fn format_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FixedTransactionalStreamFormatTrait>>>;
    fn auto_commit(&self) -> &bool;
    fn auto_commit_mut(&mut self) -> &mut bool;
}

impl TransactionalStreamDataTrait for TransactionalStreamData {
    fn format(&self) -> &Option<Arc<Mutex<dyn FixedTransactionalStreamFormatTrait>>> {
        &self.format
    }
    fn format_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FixedTransactionalStreamFormatTrait>>> {
        &mut self.format
    }
    fn auto_commit(&self) -> &bool {
        &self.auto_commit
    }
    fn auto_commit_mut(&mut self) -> &mut bool {
        &mut self.auto_commit
    }
}

impl StreamDataTrait for TransactionalStreamData {
    fn transports(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        self._glacier_base.transports()
    }
    fn transports_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        self._glacier_base.transports_mut()
    }
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
    fn stream_id(&self) -> &u32 {
        self._glacier_base.stream_id()
    }
    fn stream_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.stream_id_mut()
    }
}

impl super::core::AssetTrait for TransactionalStreamData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TransactionalStreamData {
}

pub static TRANSACTIONALSTREAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalStreamData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STREAMDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransactionalStreamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Format",
                flags: MemberInfoFlags::new(0),
                field_type: "FixedTransactionalStreamFormat",
                rust_offset: offset_of!(TransactionalStreamData, format),
            },
            FieldInfoData {
                name: "AutoCommit",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TransactionalStreamData, auto_commit),
            },
        ],
    }),
    array_type: Some(TRANSACTIONALSTREAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TransactionalStreamData {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSACTIONALSTREAMDATA_TYPE_INFO
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


pub static TRANSACTIONALSTREAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalStreamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TransactionalStreamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VarEventStreamData {
    pub _glacier_base: EventStreamData,
    pub format: Option<Arc<Mutex<dyn VariableEventStreamFormatTrait>>>,
}

pub trait VarEventStreamDataTrait: EventStreamDataTrait {
    fn format(&self) -> &Option<Arc<Mutex<dyn VariableEventStreamFormatTrait>>>;
    fn format_mut(&mut self) -> &mut Option<Arc<Mutex<dyn VariableEventStreamFormatTrait>>>;
}

impl VarEventStreamDataTrait for VarEventStreamData {
    fn format(&self) -> &Option<Arc<Mutex<dyn VariableEventStreamFormatTrait>>> {
        &self.format
    }
    fn format_mut(&mut self) -> &mut Option<Arc<Mutex<dyn VariableEventStreamFormatTrait>>> {
        &mut self.format
    }
}

impl EventStreamDataTrait for VarEventStreamData {
}

impl StreamDataTrait for VarEventStreamData {
    fn transports(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        self._glacier_base.transports()
    }
    fn transports_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        self._glacier_base.transports_mut()
    }
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
    fn stream_id(&self) -> &u32 {
        self._glacier_base.stream_id()
    }
    fn stream_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.stream_id_mut()
    }
}

impl super::core::AssetTrait for VarEventStreamData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for VarEventStreamData {
}

pub static VAREVENTSTREAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarEventStreamData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVENTSTREAMDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VarEventStreamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Format",
                flags: MemberInfoFlags::new(0),
                field_type: "VariableEventStreamFormat",
                rust_offset: offset_of!(VarEventStreamData, format),
            },
        ],
    }),
    array_type: Some(VAREVENTSTREAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VarEventStreamData {
    fn type_info(&self) -> &'static TypeInfo {
        VAREVENTSTREAMDATA_TYPE_INFO
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


pub static VAREVENTSTREAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarEventStreamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("VarEventStreamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FixedEventStreamData {
    pub _glacier_base: EventStreamData,
    pub format: Option<Arc<Mutex<dyn FixedEventStreamFormatTrait>>>,
}

pub trait FixedEventStreamDataTrait: EventStreamDataTrait {
    fn format(&self) -> &Option<Arc<Mutex<dyn FixedEventStreamFormatTrait>>>;
    fn format_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FixedEventStreamFormatTrait>>>;
}

impl FixedEventStreamDataTrait for FixedEventStreamData {
    fn format(&self) -> &Option<Arc<Mutex<dyn FixedEventStreamFormatTrait>>> {
        &self.format
    }
    fn format_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FixedEventStreamFormatTrait>>> {
        &mut self.format
    }
}

impl EventStreamDataTrait for FixedEventStreamData {
}

impl StreamDataTrait for FixedEventStreamData {
    fn transports(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        self._glacier_base.transports()
    }
    fn transports_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        self._glacier_base.transports_mut()
    }
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
    fn stream_id(&self) -> &u32 {
        self._glacier_base.stream_id()
    }
    fn stream_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.stream_id_mut()
    }
}

impl super::core::AssetTrait for FixedEventStreamData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for FixedEventStreamData {
}

pub static FIXEDEVENTSTREAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedEventStreamData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVENTSTREAMDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FixedEventStreamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Format",
                flags: MemberInfoFlags::new(0),
                field_type: "FixedEventStreamFormat",
                rust_offset: offset_of!(FixedEventStreamData, format),
            },
        ],
    }),
    array_type: Some(FIXEDEVENTSTREAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FixedEventStreamData {
    fn type_info(&self) -> &'static TypeInfo {
        FIXEDEVENTSTREAMDATA_TYPE_INFO
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


pub static FIXEDEVENTSTREAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedEventStreamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("FixedEventStreamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EventStreamData {
    pub _glacier_base: StreamData,
}

pub trait EventStreamDataTrait: StreamDataTrait {
}

impl EventStreamDataTrait for EventStreamData {
}

impl StreamDataTrait for EventStreamData {
    fn transports(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        self._glacier_base.transports()
    }
    fn transports_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        self._glacier_base.transports_mut()
    }
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
    fn stream_id(&self) -> &u32 {
        self._glacier_base.stream_id()
    }
    fn stream_id_mut(&mut self) -> &mut u32 {
        self._glacier_base.stream_id_mut()
    }
}

impl super::core::AssetTrait for EventStreamData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EventStreamData {
}

pub static EVENTSTREAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventStreamData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STREAMDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventStreamData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EVENTSTREAMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventStreamData {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTSTREAMDATA_TYPE_INFO
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


pub static EVENTSTREAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventStreamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("EventStreamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StreamData {
    pub _glacier_base: super::core::Asset,
    pub transports: Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>>,
    pub platform: super::core::GamePlatform,
    pub stream_id: u32,
}

pub trait StreamDataTrait: super::core::AssetTrait {
    fn transports(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>>;
    fn transports_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>>;
    fn platform(&self) -> &super::core::GamePlatform;
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform;
    fn stream_id(&self) -> &u32;
    fn stream_id_mut(&mut self) -> &mut u32;
}

impl StreamDataTrait for StreamData {
    fn transports(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        &self.transports
    }
    fn transports_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        &mut self.transports
    }
    fn platform(&self) -> &super::core::GamePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        &mut self.platform
    }
    fn stream_id(&self) -> &u32 {
        &self.stream_id
    }
    fn stream_id_mut(&mut self) -> &mut u32 {
        &mut self.stream_id
    }
}

impl super::core::AssetTrait for StreamData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for StreamData {
}

pub static STREAMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transports",
                flags: MemberInfoFlags::new(144),
                field_type: "TelemetryTransportData-Array",
                rust_offset: offset_of!(StreamData, transports),
            },
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(StreamData, platform),
            },
            FieldInfoData {
                name: "StreamId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(StreamData, stream_id),
            },
        ],
    }),
    array_type: Some(STREAMDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StreamData {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMDATA_TYPE_INFO
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


pub static STREAMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("StreamData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FixedTransactionalStreamFormat {
    pub _glacier_base: TransactionalTelemetryStreamFormat,
    pub reference_row: Option<Arc<Mutex<dyn TelemetryTransactionDataTrait>>>,
}

pub trait FixedTransactionalStreamFormatTrait: TransactionalTelemetryStreamFormatTrait {
    fn reference_row(&self) -> &Option<Arc<Mutex<dyn TelemetryTransactionDataTrait>>>;
    fn reference_row_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TelemetryTransactionDataTrait>>>;
}

impl FixedTransactionalStreamFormatTrait for FixedTransactionalStreamFormat {
    fn reference_row(&self) -> &Option<Arc<Mutex<dyn TelemetryTransactionDataTrait>>> {
        &self.reference_row
    }
    fn reference_row_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TelemetryTransactionDataTrait>>> {
        &mut self.reference_row
    }
}

impl TransactionalTelemetryStreamFormatTrait for FixedTransactionalStreamFormat {
    fn transaction_mode(&self) -> &TelemetryTransactionMode {
        self._glacier_base.transaction_mode()
    }
    fn transaction_mode_mut(&mut self) -> &mut TelemetryTransactionMode {
        self._glacier_base.transaction_mode_mut()
    }
    fn autocommit(&self) -> &bool {
        self._glacier_base.autocommit()
    }
    fn autocommit_mut(&mut self) -> &mut bool {
        self._glacier_base.autocommit_mut()
    }
}

impl TelemetryStreamFormatTrait for FixedTransactionalStreamFormat {
}

impl super::core::AssetTrait for FixedTransactionalStreamFormat {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for FixedTransactionalStreamFormat {
}

pub static FIXEDTRANSACTIONALSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedTransactionalStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSACTIONALTELEMETRYSTREAMFORMAT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FixedTransactionalStreamFormat as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ReferenceRow",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryTransactionData",
                rust_offset: offset_of!(FixedTransactionalStreamFormat, reference_row),
            },
        ],
    }),
    array_type: Some(FIXEDTRANSACTIONALSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FixedTransactionalStreamFormat {
    fn type_info(&self) -> &'static TypeInfo {
        FIXEDTRANSACTIONALSTREAMFORMAT_TYPE_INFO
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


pub static FIXEDTRANSACTIONALSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedTransactionalStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("FixedTransactionalStreamFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TransactionalTelemetryStreamFormat {
    pub _glacier_base: TelemetryStreamFormat,
    pub transaction_mode: TelemetryTransactionMode,
    pub autocommit: bool,
}

pub trait TransactionalTelemetryStreamFormatTrait: TelemetryStreamFormatTrait {
    fn transaction_mode(&self) -> &TelemetryTransactionMode;
    fn transaction_mode_mut(&mut self) -> &mut TelemetryTransactionMode;
    fn autocommit(&self) -> &bool;
    fn autocommit_mut(&mut self) -> &mut bool;
}

impl TransactionalTelemetryStreamFormatTrait for TransactionalTelemetryStreamFormat {
    fn transaction_mode(&self) -> &TelemetryTransactionMode {
        &self.transaction_mode
    }
    fn transaction_mode_mut(&mut self) -> &mut TelemetryTransactionMode {
        &mut self.transaction_mode
    }
    fn autocommit(&self) -> &bool {
        &self.autocommit
    }
    fn autocommit_mut(&mut self) -> &mut bool {
        &mut self.autocommit
    }
}

impl TelemetryStreamFormatTrait for TransactionalTelemetryStreamFormat {
}

impl super::core::AssetTrait for TransactionalTelemetryStreamFormat {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TransactionalTelemetryStreamFormat {
}

pub static TRANSACTIONALTELEMETRYSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSTREAMFORMAT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransactionalTelemetryStreamFormat as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TransactionMode",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryTransactionMode",
                rust_offset: offset_of!(TransactionalTelemetryStreamFormat, transaction_mode),
            },
            FieldInfoData {
                name: "Autocommit",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TransactionalTelemetryStreamFormat, autocommit),
            },
        ],
    }),
    array_type: Some(TRANSACTIONALTELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransactionalTelemetryStreamFormat {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSACTIONALTELEMETRYSTREAMFORMAT_TYPE_INFO
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


pub static TRANSACTIONALTELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TransactionalTelemetryStreamFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TelemetryTransactionMode {
    #[default]
    TelemetryTransactionMode_Binary = 0,
    TelemetryTransactionMode_String = 1,
}

pub static TELEMETRYTRANSACTIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransactionMode",
    flags: MemberInfoFlags::new(49429),
    module: "TelemetryShared",
    data: TypeInfoData::Enum,
    array_type: Some(TELEMETRYTRANSACTIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TelemetryTransactionMode {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYTRANSACTIONMODE_TYPE_INFO
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


pub static TELEMETRYTRANSACTIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransactionMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryTransactionMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VariableEventStreamFormat {
    pub _glacier_base: EventTelemetryStreamFormat,
    pub r#mod: String,
    pub grp: String,
    pub subgrp: String,
    pub params: Vec<TelemetryParameterDataProperty>,
}

pub trait VariableEventStreamFormatTrait: EventTelemetryStreamFormatTrait {
    fn r#mod(&self) -> &String;
    fn r#mod_mut(&mut self) -> &mut String;
    fn grp(&self) -> &String;
    fn grp_mut(&mut self) -> &mut String;
    fn subgrp(&self) -> &String;
    fn subgrp_mut(&mut self) -> &mut String;
    fn params(&self) -> &Vec<TelemetryParameterDataProperty>;
    fn params_mut(&mut self) -> &mut Vec<TelemetryParameterDataProperty>;
}

impl VariableEventStreamFormatTrait for VariableEventStreamFormat {
    fn r#mod(&self) -> &String {
        &self.r#mod
    }
    fn r#mod_mut(&mut self) -> &mut String {
        &mut self.r#mod
    }
    fn grp(&self) -> &String {
        &self.grp
    }
    fn grp_mut(&mut self) -> &mut String {
        &mut self.grp
    }
    fn subgrp(&self) -> &String {
        &self.subgrp
    }
    fn subgrp_mut(&mut self) -> &mut String {
        &mut self.subgrp
    }
    fn params(&self) -> &Vec<TelemetryParameterDataProperty> {
        &self.params
    }
    fn params_mut(&mut self) -> &mut Vec<TelemetryParameterDataProperty> {
        &mut self.params
    }
}

impl EventTelemetryStreamFormatTrait for VariableEventStreamFormat {
}

impl TelemetryStreamFormatTrait for VariableEventStreamFormat {
}

impl super::core::AssetTrait for VariableEventStreamFormat {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for VariableEventStreamFormat {
}

pub static VARIABLEEVENTSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VariableEventStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVENTTELEMETRYSTREAMFORMAT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VariableEventStreamFormat as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "mod",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VariableEventStreamFormat, r#mod),
            },
            FieldInfoData {
                name: "grp",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VariableEventStreamFormat, grp),
            },
            FieldInfoData {
                name: "subgrp",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VariableEventStreamFormat, subgrp),
            },
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(144),
                field_type: "TelemetryParameterDataProperty-Array",
                rust_offset: offset_of!(VariableEventStreamFormat, params),
            },
        ],
    }),
    array_type: Some(VARIABLEEVENTSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VariableEventStreamFormat {
    fn type_info(&self) -> &'static TypeInfo {
        VARIABLEEVENTSTREAMFORMAT_TYPE_INFO
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


pub static VARIABLEEVENTSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VariableEventStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("VariableEventStreamFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FixedEventStreamFormat {
    pub _glacier_base: EventTelemetryStreamFormat,
    pub reference_event: Option<Arc<Mutex<dyn TelemetryLogEventTrait>>>,
}

pub trait FixedEventStreamFormatTrait: EventTelemetryStreamFormatTrait {
    fn reference_event(&self) -> &Option<Arc<Mutex<dyn TelemetryLogEventTrait>>>;
    fn reference_event_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TelemetryLogEventTrait>>>;
}

impl FixedEventStreamFormatTrait for FixedEventStreamFormat {
    fn reference_event(&self) -> &Option<Arc<Mutex<dyn TelemetryLogEventTrait>>> {
        &self.reference_event
    }
    fn reference_event_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TelemetryLogEventTrait>>> {
        &mut self.reference_event
    }
}

impl EventTelemetryStreamFormatTrait for FixedEventStreamFormat {
}

impl TelemetryStreamFormatTrait for FixedEventStreamFormat {
}

impl super::core::AssetTrait for FixedEventStreamFormat {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for FixedEventStreamFormat {
}

pub static FIXEDEVENTSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedEventStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(EVENTTELEMETRYSTREAMFORMAT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FixedEventStreamFormat as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ReferenceEvent",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryLogEvent",
                rust_offset: offset_of!(FixedEventStreamFormat, reference_event),
            },
        ],
    }),
    array_type: Some(FIXEDEVENTSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FixedEventStreamFormat {
    fn type_info(&self) -> &'static TypeInfo {
        FIXEDEVENTSTREAMFORMAT_TYPE_INFO
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


pub static FIXEDEVENTSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedEventStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("FixedEventStreamFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EventTelemetryStreamFormat {
    pub _glacier_base: TelemetryStreamFormat,
}

pub trait EventTelemetryStreamFormatTrait: TelemetryStreamFormatTrait {
}

impl EventTelemetryStreamFormatTrait for EventTelemetryStreamFormat {
}

impl TelemetryStreamFormatTrait for EventTelemetryStreamFormat {
}

impl super::core::AssetTrait for EventTelemetryStreamFormat {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for EventTelemetryStreamFormat {
}

pub static EVENTTELEMETRYSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTelemetryStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSTREAMFORMAT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventTelemetryStreamFormat as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EVENTTELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventTelemetryStreamFormat {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTTELEMETRYSTREAMFORMAT_TYPE_INFO
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


pub static EVENTTELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTelemetryStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("EventTelemetryStreamFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryStreamFormat {
    pub _glacier_base: super::core::Asset,
}

pub trait TelemetryStreamFormatTrait: super::core::AssetTrait {
}

impl TelemetryStreamFormatTrait for TelemetryStreamFormat {
}

impl super::core::AssetTrait for TelemetryStreamFormat {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TelemetryStreamFormat {
}

pub static TELEMETRYSTREAMFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryStreamFormat",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryStreamFormat as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryStreamFormat {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSTREAMFORMAT_TYPE_INFO
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


pub static TELEMETRYSTREAMFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryStreamFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryStreamFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySettings {
    pub _glacier_base: super::core::SystemSettings,
    pub stream_formats: Vec<Option<Arc<Mutex<dyn TelemetryStreamFormatTrait>>>>,
    pub transports: Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>>,
    pub streams: Vec<Option<Arc<Mutex<dyn StreamDataTrait>>>>,
    pub file_location: String,
}

pub trait TelemetrySettingsTrait: super::core::SystemSettingsTrait {
    fn stream_formats(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryStreamFormatTrait>>>>;
    fn stream_formats_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryStreamFormatTrait>>>>;
    fn transports(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>>;
    fn transports_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>>;
    fn streams(&self) -> &Vec<Option<Arc<Mutex<dyn StreamDataTrait>>>>;
    fn streams_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn StreamDataTrait>>>>;
    fn file_location(&self) -> &String;
    fn file_location_mut(&mut self) -> &mut String;
}

impl TelemetrySettingsTrait for TelemetrySettings {
    fn stream_formats(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryStreamFormatTrait>>>> {
        &self.stream_formats
    }
    fn stream_formats_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryStreamFormatTrait>>>> {
        &mut self.stream_formats
    }
    fn transports(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        &self.transports
    }
    fn transports_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        &mut self.transports
    }
    fn streams(&self) -> &Vec<Option<Arc<Mutex<dyn StreamDataTrait>>>> {
        &self.streams
    }
    fn streams_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn StreamDataTrait>>>> {
        &mut self.streams
    }
    fn file_location(&self) -> &String {
        &self.file_location
    }
    fn file_location_mut(&mut self) -> &mut String {
        &mut self.file_location
    }
}

impl super::core::SystemSettingsTrait for TelemetrySettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for TelemetrySettings {
}

pub static TELEMETRYSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySettings",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StreamFormats",
                flags: MemberInfoFlags::new(144),
                field_type: "TelemetryStreamFormat-Array",
                rust_offset: offset_of!(TelemetrySettings, stream_formats),
            },
            FieldInfoData {
                name: "Transports",
                flags: MemberInfoFlags::new(144),
                field_type: "TelemetryTransportData-Array",
                rust_offset: offset_of!(TelemetrySettings, transports),
            },
            FieldInfoData {
                name: "Streams",
                flags: MemberInfoFlags::new(144),
                field_type: "StreamData-Array",
                rust_offset: offset_of!(TelemetrySettings, streams),
            },
            FieldInfoData {
                name: "FileLocation",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySettings, file_location),
            },
        ],
    }),
    array_type: Some(TELEMETRYSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySettings {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSETTINGS_TYPE_INFO
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


pub static TELEMETRYSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryTransactionData {
    pub _glacier_base: TelemetryRowData,
}

pub trait TelemetryTransactionDataTrait: TelemetryRowDataTrait {
}

impl TelemetryTransactionDataTrait for TelemetryTransactionData {
}

impl TelemetryRowDataTrait for TelemetryTransactionData {
}

impl super::core::DataContainerTrait for TelemetryTransactionData {
}

pub static TELEMETRYTRANSACTIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransactionData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYROWDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryTransactionData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYTRANSACTIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryTransactionData {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYTRANSACTIONDATA_TYPE_INFO
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


pub static TELEMETRYTRANSACTIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryTransactionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryTransactionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdk3Event {
    pub _glacier_base: TelemetryLogEvent,
    pub module_id: u32,
    pub group_id: u32,
    pub string_id: u32,
}

pub trait TelemetrySdk3EventTrait: TelemetryLogEventTrait {
    fn module_id(&self) -> &u32;
    fn module_id_mut(&mut self) -> &mut u32;
    fn group_id(&self) -> &u32;
    fn group_id_mut(&mut self) -> &mut u32;
    fn string_id(&self) -> &u32;
    fn string_id_mut(&mut self) -> &mut u32;
}

impl TelemetrySdk3EventTrait for TelemetrySdk3Event {
    fn module_id(&self) -> &u32 {
        &self.module_id
    }
    fn module_id_mut(&mut self) -> &mut u32 {
        &mut self.module_id
    }
    fn group_id(&self) -> &u32 {
        &self.group_id
    }
    fn group_id_mut(&mut self) -> &mut u32 {
        &mut self.group_id
    }
    fn string_id(&self) -> &u32 {
        &self.string_id
    }
    fn string_id_mut(&mut self) -> &mut u32 {
        &mut self.string_id
    }
}

impl TelemetryLogEventTrait for TelemetrySdk3Event {
}

impl TelemetryRowDataTrait for TelemetrySdk3Event {
}

impl super::core::DataContainerTrait for TelemetrySdk3Event {
}

pub static TELEMETRYSDK3EVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdk3Event",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYLOGEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdk3Event as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ModuleId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdk3Event, module_id),
            },
            FieldInfoData {
                name: "GroupId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdk3Event, group_id),
            },
            FieldInfoData {
                name: "StringId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdk3Event, string_id),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDK3EVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdk3Event {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDK3EVENT_TYPE_INFO
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


pub static TELEMETRYSDK3EVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdk3Event-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdk3Event"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryLogEvent {
    pub _glacier_base: TelemetryRowData,
}

pub trait TelemetryLogEventTrait: TelemetryRowDataTrait {
}

impl TelemetryLogEventTrait for TelemetryLogEvent {
}

impl TelemetryRowDataTrait for TelemetryLogEvent {
}

impl super::core::DataContainerTrait for TelemetryLogEvent {
}

pub static TELEMETRYLOGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryLogEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYROWDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryLogEvent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYLOGEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryLogEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYLOGEVENT_TYPE_INFO
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


pub static TELEMETRYLOGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryLogEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryLogEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryRowData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait TelemetryRowDataTrait: super::core::DataContainerTrait {
}

impl TelemetryRowDataTrait for TelemetryRowData {
}

impl super::core::DataContainerTrait for TelemetryRowData {
}

pub static TELEMETRYROWDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryRowData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryRowData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TELEMETRYROWDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryRowData {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYROWDATA_TYPE_INFO
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


pub static TELEMETRYROWDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryRowData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryRowData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinFriendNetworkEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub total_friends: u32,
    pub friends_online: u32,
    pub friends_same_title: u32,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinFriendNetworkEventTrait: TelemetrySDKPinEventTrait {
    fn total_friends(&self) -> &u32;
    fn total_friends_mut(&mut self) -> &mut u32;
    fn friends_online(&self) -> &u32;
    fn friends_online_mut(&mut self) -> &mut u32;
    fn friends_same_title(&self) -> &u32;
    fn friends_same_title_mut(&mut self) -> &mut u32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinFriendNetworkEventTrait for TelemetrySdkPinFriendNetworkEvent {
    fn total_friends(&self) -> &u32 {
        &self.total_friends
    }
    fn total_friends_mut(&mut self) -> &mut u32 {
        &mut self.total_friends
    }
    fn friends_online(&self) -> &u32 {
        &self.friends_online
    }
    fn friends_online_mut(&mut self) -> &mut u32 {
        &mut self.friends_online
    }
    fn friends_same_title(&self) -> &u32 {
        &self.friends_same_title
    }
    fn friends_same_title_mut(&mut self) -> &mut u32 {
        &mut self.friends_same_title
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinFriendNetworkEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinFriendNetworkEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinFriendNetworkEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinFriendNetworkEvent {
}

pub static TELEMETRYSDKPINFRIENDNETWORKEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFriendNetworkEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinFriendNetworkEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "total_friends",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinFriendNetworkEvent, total_friends),
            },
            FieldInfoData {
                name: "friends_online",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinFriendNetworkEvent, friends_online),
            },
            FieldInfoData {
                name: "friends_same_title",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinFriendNetworkEvent, friends_same_title),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinFriendNetworkEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINFRIENDNETWORKEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinFriendNetworkEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINFRIENDNETWORKEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINFRIENDNETWORKEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFriendNetworkEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinFriendNetworkEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinChallengeEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub r#type: String,
    pub status: String,
    pub status_code: String,
    pub challenge_id: String,
    pub recipient_id: String,
    pub recipient_type: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinChallengeEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn challenge_id(&self) -> &String;
    fn challenge_id_mut(&mut self) -> &mut String;
    fn recipient_id(&self) -> &String;
    fn recipient_id_mut(&mut self) -> &mut String;
    fn recipient_type(&self) -> &String;
    fn recipient_type_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinChallengeEventTrait for TelemetrySdkPinChallengeEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn challenge_id(&self) -> &String {
        &self.challenge_id
    }
    fn challenge_id_mut(&mut self) -> &mut String {
        &mut self.challenge_id
    }
    fn recipient_id(&self) -> &String {
        &self.recipient_id
    }
    fn recipient_id_mut(&mut self) -> &mut String {
        &mut self.recipient_id
    }
    fn recipient_type(&self) -> &String {
        &self.recipient_type
    }
    fn recipient_type_mut(&mut self) -> &mut String {
        &mut self.recipient_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinChallengeEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinChallengeEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinChallengeEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinChallengeEvent {
}

pub static TELEMETRYSDKPINCHALLENGEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinChallengeEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinChallengeEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, r#type),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, status_code),
            },
            FieldInfoData {
                name: "challenge_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, challenge_id),
            },
            FieldInfoData {
                name: "recipient_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, recipient_id),
            },
            FieldInfoData {
                name: "recipient_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, recipient_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinChallengeEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINCHALLENGEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinChallengeEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINCHALLENGEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINCHALLENGEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinChallengeEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinChallengeEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinSocMessageEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinSocMessageEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn placement(&self) -> &String;
    fn placement_mut(&mut self) -> &mut String;
    fn content_type(&self) -> &String;
    fn content_type_mut(&mut self) -> &mut String;
    fn network(&self) -> &String;
    fn network_mut(&mut self) -> &mut String;
    fn format(&self) -> &String;
    fn format_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn message_id(&self) -> &String;
    fn message_id_mut(&mut self) -> &mut String;
    fn recipient_id(&self) -> &Vec<String>;
    fn recipient_id_mut(&mut self) -> &mut Vec<String>;
    fn recipient_type(&self) -> &String;
    fn recipient_type_mut(&mut self) -> &mut String;
    fn items(&self) -> &RawJsonString;
    fn items_mut(&mut self) -> &mut RawJsonString;
    fn event_id(&self) -> &String;
    fn event_id_mut(&mut self) -> &mut String;
    fn event_name(&self) -> &String;
    fn event_name_mut(&mut self) -> &mut String;
    fn event_type(&self) -> &String;
    fn event_type_mut(&mut self) -> &mut String;
    fn count(&self) -> &i64;
    fn count_mut(&mut self) -> &mut i64;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinSocMessageEventTrait for TelemetrySdkPinSocMessageEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn placement(&self) -> &String {
        &self.placement
    }
    fn placement_mut(&mut self) -> &mut String {
        &mut self.placement
    }
    fn content_type(&self) -> &String {
        &self.content_type
    }
    fn content_type_mut(&mut self) -> &mut String {
        &mut self.content_type
    }
    fn network(&self) -> &String {
        &self.network
    }
    fn network_mut(&mut self) -> &mut String {
        &mut self.network
    }
    fn format(&self) -> &String {
        &self.format
    }
    fn format_mut(&mut self) -> &mut String {
        &mut self.format
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn message_id(&self) -> &String {
        &self.message_id
    }
    fn message_id_mut(&mut self) -> &mut String {
        &mut self.message_id
    }
    fn recipient_id(&self) -> &Vec<String> {
        &self.recipient_id
    }
    fn recipient_id_mut(&mut self) -> &mut Vec<String> {
        &mut self.recipient_id
    }
    fn recipient_type(&self) -> &String {
        &self.recipient_type
    }
    fn recipient_type_mut(&mut self) -> &mut String {
        &mut self.recipient_type
    }
    fn items(&self) -> &RawJsonString {
        &self.items
    }
    fn items_mut(&mut self) -> &mut RawJsonString {
        &mut self.items
    }
    fn event_id(&self) -> &String {
        &self.event_id
    }
    fn event_id_mut(&mut self) -> &mut String {
        &mut self.event_id
    }
    fn event_name(&self) -> &String {
        &self.event_name
    }
    fn event_name_mut(&mut self) -> &mut String {
        &mut self.event_name
    }
    fn event_type(&self) -> &String {
        &self.event_type
    }
    fn event_type_mut(&mut self) -> &mut String {
        &mut self.event_type
    }
    fn count(&self) -> &i64 {
        &self.count
    }
    fn count_mut(&mut self) -> &mut i64 {
        &mut self.count
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinSocMessageEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinSocMessageEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinSocMessageEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinSocMessageEvent {
}

pub static TELEMETRYSDKPINSOCMESSAGEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSocMessageEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinSocMessageEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, r#type),
            },
            FieldInfoData {
                name: "placement",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, placement),
            },
            FieldInfoData {
                name: "content_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, content_type),
            },
            FieldInfoData {
                name: "network",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, network),
            },
            FieldInfoData {
                name: "format",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, format),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, status_code),
            },
            FieldInfoData {
                name: "message_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, message_id),
            },
            FieldInfoData {
                name: "recipient_id",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, recipient_id),
            },
            FieldInfoData {
                name: "recipient_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, recipient_type),
            },
            FieldInfoData {
                name: "items",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, items),
            },
            FieldInfoData {
                name: "event_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, event_id),
            },
            FieldInfoData {
                name: "event_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, event_name),
            },
            FieldInfoData {
                name: "event_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, event_type),
            },
            FieldInfoData {
                name: "count",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, count),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinSocMessageEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINSOCMESSAGEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinSocMessageEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINSOCMESSAGEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINSOCMESSAGEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSocMessageEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinSocMessageEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinGroupEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub r#type: String,
    pub _class: String,
    pub group_id: String,
    pub status: String,
    pub status_code: String,
    pub member_id: String,
    pub member_type: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinGroupEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn _class(&self) -> &String;
    fn _class_mut(&mut self) -> &mut String;
    fn group_id(&self) -> &String;
    fn group_id_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn member_id(&self) -> &String;
    fn member_id_mut(&mut self) -> &mut String;
    fn member_type(&self) -> &String;
    fn member_type_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinGroupEventTrait for TelemetrySdkPinGroupEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn _class(&self) -> &String {
        &self._class
    }
    fn _class_mut(&mut self) -> &mut String {
        &mut self._class
    }
    fn group_id(&self) -> &String {
        &self.group_id
    }
    fn group_id_mut(&mut self) -> &mut String {
        &mut self.group_id
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn member_id(&self) -> &String {
        &self.member_id
    }
    fn member_id_mut(&mut self) -> &mut String {
        &mut self.member_id
    }
    fn member_type(&self) -> &String {
        &self.member_type
    }
    fn member_type_mut(&mut self) -> &mut String {
        &mut self.member_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinGroupEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinGroupEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinGroupEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinGroupEvent {
}

pub static TELEMETRYSDKPINGROUPEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGroupEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinGroupEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, r#type),
            },
            FieldInfoData {
                name: "_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, _class),
            },
            FieldInfoData {
                name: "group_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, group_id),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, status_code),
            },
            FieldInfoData {
                name: "member_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, member_id),
            },
            FieldInfoData {
                name: "member_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, member_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinGroupEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINGROUPEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinGroupEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINGROUPEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINGROUPEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGroupEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinGroupEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinFriendsEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub frid: String,
    pub friend_type: String,
    pub source: String,
    pub network: String,
    pub action: String,
    pub status_code: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinFriendsEventTrait: TelemetrySDKPinEventTrait {
    fn frid(&self) -> &String;
    fn frid_mut(&mut self) -> &mut String;
    fn friend_type(&self) -> &String;
    fn friend_type_mut(&mut self) -> &mut String;
    fn source(&self) -> &String;
    fn source_mut(&mut self) -> &mut String;
    fn network(&self) -> &String;
    fn network_mut(&mut self) -> &mut String;
    fn action(&self) -> &String;
    fn action_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinFriendsEventTrait for TelemetrySdkPinFriendsEvent {
    fn frid(&self) -> &String {
        &self.frid
    }
    fn frid_mut(&mut self) -> &mut String {
        &mut self.frid
    }
    fn friend_type(&self) -> &String {
        &self.friend_type
    }
    fn friend_type_mut(&mut self) -> &mut String {
        &mut self.friend_type
    }
    fn source(&self) -> &String {
        &self.source
    }
    fn source_mut(&mut self) -> &mut String {
        &mut self.source
    }
    fn network(&self) -> &String {
        &self.network
    }
    fn network_mut(&mut self) -> &mut String {
        &mut self.network
    }
    fn action(&self) -> &String {
        &self.action
    }
    fn action_mut(&mut self) -> &mut String {
        &mut self.action
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinFriendsEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinFriendsEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinFriendsEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinFriendsEvent {
}

pub static TELEMETRYSDKPINFRIENDSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFriendsEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinFriendsEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "frid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, frid),
            },
            FieldInfoData {
                name: "friend_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, friend_type),
            },
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, source),
            },
            FieldInfoData {
                name: "network",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, network),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, action),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinFriendsEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINFRIENDSEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinFriendsEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINFRIENDSEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINFRIENDSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFriendsEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinFriendsEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinCustomErrorEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub severity: String,
    pub metadata: RawJsonString,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinCustomErrorEventTrait: TelemetrySDKPinEventTrait {
    fn severity(&self) -> &String;
    fn severity_mut(&mut self) -> &mut String;
    fn metadata(&self) -> &RawJsonString;
    fn metadata_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinCustomErrorEventTrait for TelemetrySdkPinCustomErrorEvent {
    fn severity(&self) -> &String {
        &self.severity
    }
    fn severity_mut(&mut self) -> &mut String {
        &mut self.severity
    }
    fn metadata(&self) -> &RawJsonString {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut RawJsonString {
        &mut self.metadata
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinCustomErrorEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinCustomErrorEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinCustomErrorEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinCustomErrorEvent {
}

pub static TELEMETRYSDKPINCUSTOMERROREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCustomErrorEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinCustomErrorEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "severity",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinCustomErrorEvent, severity),
            },
            FieldInfoData {
                name: "metadata",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinCustomErrorEvent, metadata),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinCustomErrorEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINCUSTOMERROREVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinCustomErrorEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINCUSTOMERROREVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINCUSTOMERROREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCustomErrorEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinCustomErrorEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinConnectionEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinConnectionEventTrait: TelemetrySDKPinEventTrait {
    fn goid(&self) -> &String;
    fn goid_mut(&mut self) -> &mut String;
    fn player_ps(&self) -> &String;
    fn player_ps_mut(&mut self) -> &mut String;
    fn target_ip(&self) -> &String;
    fn target_ip_mut(&mut self) -> &mut String;
    fn target_ps(&self) -> &String;
    fn target_ps_mut(&mut self) -> &mut String;
    fn game_ps(&self) -> &String;
    fn game_ps_mut(&mut self) -> &mut String;
    fn net_topo(&self) -> &String;
    fn net_topo_mut(&mut self) -> &mut String;
    fn join_method(&self) -> &String;
    fn join_method_mut(&mut self) -> &mut String;
    fn mode(&self) -> &String;
    fn mode_mut(&mut self) -> &mut String;
    fn client_type(&self) -> &String;
    fn client_type_mut(&mut self) -> &mut String;
    fn leave_reason(&self) -> &String;
    fn leave_reason_mut(&mut self) -> &mut String;
    fn cxn_tech(&self) -> &String;
    fn cxn_tech_mut(&mut self) -> &mut String;
    fn pkt_loss(&self) -> &f32;
    fn pkt_loss_mut(&mut self) -> &mut f32;
    fn avg_lat(&self) -> &f32;
    fn avg_lat_mut(&mut self) -> &mut f32;
    fn max_lat(&self) -> &f32;
    fn max_lat_mut(&mut self) -> &mut f32;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinConnectionEventTrait for TelemetrySdkPinConnectionEvent {
    fn goid(&self) -> &String {
        &self.goid
    }
    fn goid_mut(&mut self) -> &mut String {
        &mut self.goid
    }
    fn player_ps(&self) -> &String {
        &self.player_ps
    }
    fn player_ps_mut(&mut self) -> &mut String {
        &mut self.player_ps
    }
    fn target_ip(&self) -> &String {
        &self.target_ip
    }
    fn target_ip_mut(&mut self) -> &mut String {
        &mut self.target_ip
    }
    fn target_ps(&self) -> &String {
        &self.target_ps
    }
    fn target_ps_mut(&mut self) -> &mut String {
        &mut self.target_ps
    }
    fn game_ps(&self) -> &String {
        &self.game_ps
    }
    fn game_ps_mut(&mut self) -> &mut String {
        &mut self.game_ps
    }
    fn net_topo(&self) -> &String {
        &self.net_topo
    }
    fn net_topo_mut(&mut self) -> &mut String {
        &mut self.net_topo
    }
    fn join_method(&self) -> &String {
        &self.join_method
    }
    fn join_method_mut(&mut self) -> &mut String {
        &mut self.join_method
    }
    fn mode(&self) -> &String {
        &self.mode
    }
    fn mode_mut(&mut self) -> &mut String {
        &mut self.mode
    }
    fn client_type(&self) -> &String {
        &self.client_type
    }
    fn client_type_mut(&mut self) -> &mut String {
        &mut self.client_type
    }
    fn leave_reason(&self) -> &String {
        &self.leave_reason
    }
    fn leave_reason_mut(&mut self) -> &mut String {
        &mut self.leave_reason
    }
    fn cxn_tech(&self) -> &String {
        &self.cxn_tech
    }
    fn cxn_tech_mut(&mut self) -> &mut String {
        &mut self.cxn_tech
    }
    fn pkt_loss(&self) -> &f32 {
        &self.pkt_loss
    }
    fn pkt_loss_mut(&mut self) -> &mut f32 {
        &mut self.pkt_loss
    }
    fn avg_lat(&self) -> &f32 {
        &self.avg_lat
    }
    fn avg_lat_mut(&mut self) -> &mut f32 {
        &mut self.avg_lat
    }
    fn max_lat(&self) -> &f32 {
        &self.max_lat
    }
    fn max_lat_mut(&mut self) -> &mut f32 {
        &mut self.max_lat
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinConnectionEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinConnectionEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinConnectionEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinConnectionEvent {
}

pub static TELEMETRYSDKPINCONNECTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinConnectionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinConnectionEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, goid),
            },
            FieldInfoData {
                name: "player_ps",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, player_ps),
            },
            FieldInfoData {
                name: "target_ip",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, target_ip),
            },
            FieldInfoData {
                name: "target_ps",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, target_ps),
            },
            FieldInfoData {
                name: "game_ps",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, game_ps),
            },
            FieldInfoData {
                name: "net_topo",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, net_topo),
            },
            FieldInfoData {
                name: "join_method",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, join_method),
            },
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, mode),
            },
            FieldInfoData {
                name: "client_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, client_type),
            },
            FieldInfoData {
                name: "leave_reason",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, leave_reason),
            },
            FieldInfoData {
                name: "cxn_tech",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, cxn_tech),
            },
            FieldInfoData {
                name: "pkt_loss",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, pkt_loss),
            },
            FieldInfoData {
                name: "avg_lat",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, avg_lat),
            },
            FieldInfoData {
                name: "max_lat",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, max_lat),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinConnectionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINCONNECTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinConnectionEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINCONNECTIONEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINCONNECTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinConnectionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinConnectionEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinErrorEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub sid: String,
    pub r#type: String,
    pub errid: String,
    pub catgid: String,
    pub server_type: String,
    pub server_name: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinErrorEventTrait: TelemetrySDKPinEventTrait {
    fn sid(&self) -> &String;
    fn sid_mut(&mut self) -> &mut String;
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn errid(&self) -> &String;
    fn errid_mut(&mut self) -> &mut String;
    fn catgid(&self) -> &String;
    fn catgid_mut(&mut self) -> &mut String;
    fn server_type(&self) -> &String;
    fn server_type_mut(&mut self) -> &mut String;
    fn server_name(&self) -> &String;
    fn server_name_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinErrorEventTrait for TelemetrySdkPinErrorEvent {
    fn sid(&self) -> &String {
        &self.sid
    }
    fn sid_mut(&mut self) -> &mut String {
        &mut self.sid
    }
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn errid(&self) -> &String {
        &self.errid
    }
    fn errid_mut(&mut self) -> &mut String {
        &mut self.errid
    }
    fn catgid(&self) -> &String {
        &self.catgid
    }
    fn catgid_mut(&mut self) -> &mut String {
        &mut self.catgid
    }
    fn server_type(&self) -> &String {
        &self.server_type
    }
    fn server_type_mut(&mut self) -> &mut String {
        &mut self.server_type
    }
    fn server_name(&self) -> &String {
        &self.server_name
    }
    fn server_name_mut(&mut self) -> &mut String {
        &mut self.server_name
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinErrorEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinErrorEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinErrorEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinErrorEvent {
}

pub static TELEMETRYSDKPINERROREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinErrorEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinErrorEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "sid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, sid),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, r#type),
            },
            FieldInfoData {
                name: "errid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, errid),
            },
            FieldInfoData {
                name: "catgid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, catgid),
            },
            FieldInfoData {
                name: "server_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, server_type),
            },
            FieldInfoData {
                name: "server_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, server_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinErrorEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINERROREVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinErrorEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINERROREVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINERROREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinErrorEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinErrorEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinMatchInfoEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinMatchInfoEventTrait: TelemetrySDKPinEventTrait {
    fn mode(&self) -> &String;
    fn mode_mut(&mut self) -> &mut String;
    fn diff(&self) -> &String;
    fn diff_mut(&mut self) -> &mut String;
    fn map(&self) -> &String;
    fn map_mut(&mut self) -> &mut String;
    fn mid(&self) -> &String;
    fn mid_mut(&mut self) -> &mut String;
    fn goid(&self) -> &String;
    fn goid_mut(&mut self) -> &mut String;
    fn player_ps(&self) -> &String;
    fn player_ps_mut(&mut self) -> &mut String;
    fn game_ps(&self) -> &String;
    fn game_ps_mut(&mut self) -> &mut String;
    fn net_topo(&self) -> &String;
    fn net_topo_mut(&mut self) -> &mut String;
    fn client_type(&self) -> &String;
    fn client_type_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn end_reason(&self) -> &String;
    fn end_reason_mut(&mut self) -> &mut String;
    fn phase(&self) -> &String;
    fn phase_mut(&mut self) -> &mut String;
    fn ts_screate(&self) -> &String;
    fn ts_screate_mut(&mut self) -> &mut String;
    fn ts_mstart(&self) -> &String;
    fn ts_mstart_mut(&mut self) -> &mut String;
    fn ts_mjoin(&self) -> &String;
    fn ts_mjoin_mut(&mut self) -> &mut String;
    fn ts_mend(&self) -> &String;
    fn ts_mend_mut(&mut self) -> &mut String;
    fn player_cnt(&self) -> &u32;
    fn player_cnt_mut(&mut self) -> &mut u32;
    fn max_players(&self) -> &u32;
    fn max_players_mut(&mut self) -> &mut u32;
    fn num_teams(&self) -> &u32;
    fn num_teams_mut(&mut self) -> &mut u32;
    fn teams_stats(&self) -> &RawJsonString;
    fn teams_stats_mut(&mut self) -> &mut RawJsonString;
    fn player_stats(&self) -> &RawJsonString;
    fn player_stats_mut(&mut self) -> &mut RawJsonString;
    fn match_info(&self) -> &RawJsonString;
    fn match_info_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl TelemetrySdkPinMatchInfoEventTrait for TelemetrySdkPinMatchInfoEvent {
    fn mode(&self) -> &String {
        &self.mode
    }
    fn mode_mut(&mut self) -> &mut String {
        &mut self.mode
    }
    fn diff(&self) -> &String {
        &self.diff
    }
    fn diff_mut(&mut self) -> &mut String {
        &mut self.diff
    }
    fn map(&self) -> &String {
        &self.map
    }
    fn map_mut(&mut self) -> &mut String {
        &mut self.map
    }
    fn mid(&self) -> &String {
        &self.mid
    }
    fn mid_mut(&mut self) -> &mut String {
        &mut self.mid
    }
    fn goid(&self) -> &String {
        &self.goid
    }
    fn goid_mut(&mut self) -> &mut String {
        &mut self.goid
    }
    fn player_ps(&self) -> &String {
        &self.player_ps
    }
    fn player_ps_mut(&mut self) -> &mut String {
        &mut self.player_ps
    }
    fn game_ps(&self) -> &String {
        &self.game_ps
    }
    fn game_ps_mut(&mut self) -> &mut String {
        &mut self.game_ps
    }
    fn net_topo(&self) -> &String {
        &self.net_topo
    }
    fn net_topo_mut(&mut self) -> &mut String {
        &mut self.net_topo
    }
    fn client_type(&self) -> &String {
        &self.client_type
    }
    fn client_type_mut(&mut self) -> &mut String {
        &mut self.client_type
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn end_reason(&self) -> &String {
        &self.end_reason
    }
    fn end_reason_mut(&mut self) -> &mut String {
        &mut self.end_reason
    }
    fn phase(&self) -> &String {
        &self.phase
    }
    fn phase_mut(&mut self) -> &mut String {
        &mut self.phase
    }
    fn ts_screate(&self) -> &String {
        &self.ts_screate
    }
    fn ts_screate_mut(&mut self) -> &mut String {
        &mut self.ts_screate
    }
    fn ts_mstart(&self) -> &String {
        &self.ts_mstart
    }
    fn ts_mstart_mut(&mut self) -> &mut String {
        &mut self.ts_mstart
    }
    fn ts_mjoin(&self) -> &String {
        &self.ts_mjoin
    }
    fn ts_mjoin_mut(&mut self) -> &mut String {
        &mut self.ts_mjoin
    }
    fn ts_mend(&self) -> &String {
        &self.ts_mend
    }
    fn ts_mend_mut(&mut self) -> &mut String {
        &mut self.ts_mend
    }
    fn player_cnt(&self) -> &u32 {
        &self.player_cnt
    }
    fn player_cnt_mut(&mut self) -> &mut u32 {
        &mut self.player_cnt
    }
    fn max_players(&self) -> &u32 {
        &self.max_players
    }
    fn max_players_mut(&mut self) -> &mut u32 {
        &mut self.max_players
    }
    fn num_teams(&self) -> &u32 {
        &self.num_teams
    }
    fn num_teams_mut(&mut self) -> &mut u32 {
        &mut self.num_teams
    }
    fn teams_stats(&self) -> &RawJsonString {
        &self.teams_stats
    }
    fn teams_stats_mut(&mut self) -> &mut RawJsonString {
        &mut self.teams_stats
    }
    fn player_stats(&self) -> &RawJsonString {
        &self.player_stats
    }
    fn player_stats_mut(&mut self) -> &mut RawJsonString {
        &mut self.player_stats
    }
    fn match_info(&self) -> &RawJsonString {
        &self.match_info
    }
    fn match_info_mut(&mut self) -> &mut RawJsonString {
        &mut self.match_info
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinMatchInfoEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinMatchInfoEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinMatchInfoEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinMatchInfoEvent {
}

pub static TELEMETRYSDKPINMATCHINFOEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMatchInfoEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinMatchInfoEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, mode),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, diff),
            },
            FieldInfoData {
                name: "map",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, map),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, mid),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, goid),
            },
            FieldInfoData {
                name: "player_ps",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, player_ps),
            },
            FieldInfoData {
                name: "game_ps",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, game_ps),
            },
            FieldInfoData {
                name: "net_topo",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, net_topo),
            },
            FieldInfoData {
                name: "client_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, client_type),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, status_code),
            },
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, end_reason),
            },
            FieldInfoData {
                name: "phase",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, phase),
            },
            FieldInfoData {
                name: "ts_screate",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, ts_screate),
            },
            FieldInfoData {
                name: "ts_mstart",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, ts_mstart),
            },
            FieldInfoData {
                name: "ts_mjoin",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, ts_mjoin),
            },
            FieldInfoData {
                name: "ts_mend",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, ts_mend),
            },
            FieldInfoData {
                name: "player_cnt",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, player_cnt),
            },
            FieldInfoData {
                name: "max_players",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, max_players),
            },
            FieldInfoData {
                name: "num_teams",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, num_teams),
            },
            FieldInfoData {
                name: "teams_stats",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, teams_stats),
            },
            FieldInfoData {
                name: "player_stats",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, player_stats),
            },
            FieldInfoData {
                name: "match_info",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, match_info),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinMatchInfoEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMATCHINFOEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinMatchInfoEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINMATCHINFOEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINMATCHINFOEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMatchInfoEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinMatchInfoEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinMatchJoinEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinMatchJoinEventTrait: TelemetrySDKPinEventTrait {
    fn mode(&self) -> &String;
    fn mode_mut(&mut self) -> &mut String;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn map(&self) -> &String;
    fn map_mut(&mut self) -> &mut String;
    fn mid(&self) -> &String;
    fn mid_mut(&mut self) -> &mut String;
    fn goid(&self) -> &String;
    fn goid_mut(&mut self) -> &mut String;
    fn net_topo(&self) -> &String;
    fn net_topo_mut(&mut self) -> &mut String;
    fn client_type(&self) -> &String;
    fn client_type_mut(&mut self) -> &mut String;
    fn join_method(&self) -> &String;
    fn join_method_mut(&mut self) -> &mut String;
    fn cxn_tech(&self) -> &String;
    fn cxn_tech_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn phase(&self) -> &String;
    fn phase_mut(&mut self) -> &mut String;
    fn mmdur(&self) -> &i64;
    fn mmdur_mut(&mut self) -> &mut i64;
    fn max_mmdur(&self) -> &i64;
    fn max_mmdur_mut(&mut self) -> &mut i64;
    fn fitscore(&self) -> &i32;
    fn fitscore_mut(&mut self) -> &mut i32;
    fn max_fitscore(&self) -> &i32;
    fn max_fitscore_mut(&mut self) -> &mut i32;
    fn scenario(&self) -> &String;
    fn scenario_mut(&mut self) -> &mut String;
    fn scenario_subsession(&self) -> &String;
    fn scenario_subsession_mut(&mut self) -> &mut String;
    fn scenario_variant(&self) -> &String;
    fn scenario_variant_mut(&mut self) -> &mut String;
    fn scenario_version(&self) -> &String;
    fn scenario_version_mut(&mut self) -> &mut String;
    fn scenario_params(&self) -> &RawJsonString;
    fn scenario_params_mut(&mut self) -> &mut RawJsonString;
    fn friend_id(&self) -> &Vec<String>;
    fn friend_id_mut(&mut self) -> &mut Vec<String>;
    fn friend_type(&self) -> &String;
    fn friend_type_mut(&mut self) -> &mut String;
    fn server_id(&self) -> &String;
    fn server_id_mut(&mut self) -> &mut String;
    fn server_name(&self) -> &String;
    fn server_name_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl TelemetrySdkPinMatchJoinEventTrait for TelemetrySdkPinMatchJoinEvent {
    fn mode(&self) -> &String {
        &self.mode
    }
    fn mode_mut(&mut self) -> &mut String {
        &mut self.mode
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn map(&self) -> &String {
        &self.map
    }
    fn map_mut(&mut self) -> &mut String {
        &mut self.map
    }
    fn mid(&self) -> &String {
        &self.mid
    }
    fn mid_mut(&mut self) -> &mut String {
        &mut self.mid
    }
    fn goid(&self) -> &String {
        &self.goid
    }
    fn goid_mut(&mut self) -> &mut String {
        &mut self.goid
    }
    fn net_topo(&self) -> &String {
        &self.net_topo
    }
    fn net_topo_mut(&mut self) -> &mut String {
        &mut self.net_topo
    }
    fn client_type(&self) -> &String {
        &self.client_type
    }
    fn client_type_mut(&mut self) -> &mut String {
        &mut self.client_type
    }
    fn join_method(&self) -> &String {
        &self.join_method
    }
    fn join_method_mut(&mut self) -> &mut String {
        &mut self.join_method
    }
    fn cxn_tech(&self) -> &String {
        &self.cxn_tech
    }
    fn cxn_tech_mut(&mut self) -> &mut String {
        &mut self.cxn_tech
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn phase(&self) -> &String {
        &self.phase
    }
    fn phase_mut(&mut self) -> &mut String {
        &mut self.phase
    }
    fn mmdur(&self) -> &i64 {
        &self.mmdur
    }
    fn mmdur_mut(&mut self) -> &mut i64 {
        &mut self.mmdur
    }
    fn max_mmdur(&self) -> &i64 {
        &self.max_mmdur
    }
    fn max_mmdur_mut(&mut self) -> &mut i64 {
        &mut self.max_mmdur
    }
    fn fitscore(&self) -> &i32 {
        &self.fitscore
    }
    fn fitscore_mut(&mut self) -> &mut i32 {
        &mut self.fitscore
    }
    fn max_fitscore(&self) -> &i32 {
        &self.max_fitscore
    }
    fn max_fitscore_mut(&mut self) -> &mut i32 {
        &mut self.max_fitscore
    }
    fn scenario(&self) -> &String {
        &self.scenario
    }
    fn scenario_mut(&mut self) -> &mut String {
        &mut self.scenario
    }
    fn scenario_subsession(&self) -> &String {
        &self.scenario_subsession
    }
    fn scenario_subsession_mut(&mut self) -> &mut String {
        &mut self.scenario_subsession
    }
    fn scenario_variant(&self) -> &String {
        &self.scenario_variant
    }
    fn scenario_variant_mut(&mut self) -> &mut String {
        &mut self.scenario_variant
    }
    fn scenario_version(&self) -> &String {
        &self.scenario_version
    }
    fn scenario_version_mut(&mut self) -> &mut String {
        &mut self.scenario_version
    }
    fn scenario_params(&self) -> &RawJsonString {
        &self.scenario_params
    }
    fn scenario_params_mut(&mut self) -> &mut RawJsonString {
        &mut self.scenario_params
    }
    fn friend_id(&self) -> &Vec<String> {
        &self.friend_id
    }
    fn friend_id_mut(&mut self) -> &mut Vec<String> {
        &mut self.friend_id
    }
    fn friend_type(&self) -> &String {
        &self.friend_type
    }
    fn friend_type_mut(&mut self) -> &mut String {
        &mut self.friend_type
    }
    fn server_id(&self) -> &String {
        &self.server_id
    }
    fn server_id_mut(&mut self) -> &mut String {
        &mut self.server_id
    }
    fn server_name(&self) -> &String {
        &self.server_name
    }
    fn server_name_mut(&mut self) -> &mut String {
        &mut self.server_name
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinMatchJoinEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinMatchJoinEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinMatchJoinEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinMatchJoinEvent {
}

pub static TELEMETRYSDKPINMATCHJOINEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMatchJoinEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinMatchJoinEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, mode),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, instance_id),
            },
            FieldInfoData {
                name: "map",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, map),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, mid),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, goid),
            },
            FieldInfoData {
                name: "net_topo",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, net_topo),
            },
            FieldInfoData {
                name: "client_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, client_type),
            },
            FieldInfoData {
                name: "join_method",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, join_method),
            },
            FieldInfoData {
                name: "cxn_tech",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, cxn_tech),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, status_code),
            },
            FieldInfoData {
                name: "phase",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, phase),
            },
            FieldInfoData {
                name: "mmdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, mmdur),
            },
            FieldInfoData {
                name: "max_mmdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, max_mmdur),
            },
            FieldInfoData {
                name: "fitscore",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, fitscore),
            },
            FieldInfoData {
                name: "max_fitscore",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, max_fitscore),
            },
            FieldInfoData {
                name: "scenario",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, scenario),
            },
            FieldInfoData {
                name: "scenario_subsession",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, scenario_subsession),
            },
            FieldInfoData {
                name: "scenario_variant",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, scenario_variant),
            },
            FieldInfoData {
                name: "scenario_version",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, scenario_version),
            },
            FieldInfoData {
                name: "scenario_params",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, scenario_params),
            },
            FieldInfoData {
                name: "friend_id",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, friend_id),
            },
            FieldInfoData {
                name: "friend_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, friend_type),
            },
            FieldInfoData {
                name: "server_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, server_id),
            },
            FieldInfoData {
                name: "server_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, server_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinMatchJoinEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMATCHJOINEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinMatchJoinEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINMATCHJOINEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINMATCHJOINEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMatchJoinEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinMatchJoinEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryPinEventHeaderConfigurationMessageBase {
}

pub trait TelemetryPinEventHeaderConfigurationMessageBaseTrait: TypeObject {
}

impl TelemetryPinEventHeaderConfigurationMessageBaseTrait for TelemetryPinEventHeaderConfigurationMessageBase {
}

pub static TELEMETRYPINEVENTHEADERCONFIGURATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryPinEventHeaderConfigurationMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryPinEventHeaderConfigurationMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetryPinEventHeaderConfigurationMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYPINEVENTHEADERCONFIGURATIONMESSAGEBASE_TYPE_INFO
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
pub struct TelemetryPinSessionHeaderConfigurationMessageBase {
}

pub trait TelemetryPinSessionHeaderConfigurationMessageBaseTrait: TypeObject {
}

impl TelemetryPinSessionHeaderConfigurationMessageBaseTrait for TelemetryPinSessionHeaderConfigurationMessageBase {
}

pub static TELEMETRYPINSESSIONHEADERCONFIGURATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryPinSessionHeaderConfigurationMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryPinSessionHeaderConfigurationMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetryPinSessionHeaderConfigurationMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYPINSESSIONHEADERCONFIGURATIONMESSAGEBASE_TYPE_INFO
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
pub struct TelemetryPinEndPointConfigurationMessageBase {
}

pub trait TelemetryPinEndPointConfigurationMessageBaseTrait: TypeObject {
}

impl TelemetryPinEndPointConfigurationMessageBaseTrait for TelemetryPinEndPointConfigurationMessageBase {
}

pub static TELEMETRYPINENDPOINTCONFIGURATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryPinEndPointConfigurationMessageBase",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryPinEndPointConfigurationMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetryPinEndPointConfigurationMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYPINENDPOINTCONFIGURATIONMESSAGEBASE_TYPE_INFO
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
pub struct TelemetrySdkPinPlayerStatEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerStatEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn stat_id(&self) -> &String;
    fn stat_id_mut(&mut self) -> &mut String;
    fn stat_value(&self) -> &f32;
    fn stat_value_mut(&mut self) -> &mut f32;
    fn stat_category(&self) -> &String;
    fn stat_category_mut(&mut self) -> &mut String;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_class(&self) -> &String;
    fn p_class_mut(&mut self) -> &mut String;
    fn p_team_id(&self) -> &String;
    fn p_team_id_mut(&mut self) -> &mut String;
    fn p_veh_id(&self) -> &String;
    fn p_veh_id_mut(&mut self) -> &mut String;
    fn p_params(&self) -> &RawJsonString;
    fn p_params_mut(&mut self) -> &mut RawJsonString;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinPlayerStatEventTrait for TelemetrySdkPinPlayerStatEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn stat_id(&self) -> &String {
        &self.stat_id
    }
    fn stat_id_mut(&mut self) -> &mut String {
        &mut self.stat_id
    }
    fn stat_value(&self) -> &f32 {
        &self.stat_value
    }
    fn stat_value_mut(&mut self) -> &mut f32 {
        &mut self.stat_value
    }
    fn stat_category(&self) -> &String {
        &self.stat_category
    }
    fn stat_category_mut(&mut self) -> &mut String {
        &mut self.stat_category
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_class(&self) -> &String {
        &self.p_class
    }
    fn p_class_mut(&mut self) -> &mut String {
        &mut self.p_class
    }
    fn p_team_id(&self) -> &String {
        &self.p_team_id
    }
    fn p_team_id_mut(&mut self) -> &mut String {
        &mut self.p_team_id
    }
    fn p_veh_id(&self) -> &String {
        &self.p_veh_id
    }
    fn p_veh_id_mut(&mut self) -> &mut String {
        &mut self.p_veh_id
    }
    fn p_params(&self) -> &RawJsonString {
        &self.p_params
    }
    fn p_params_mut(&mut self) -> &mut RawJsonString {
        &mut self.p_params
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerStatEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerStatEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerStatEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerStatEvent {
}

pub static TELEMETRYSDKPINPLAYERSTATEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStatEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerStatEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, gdur),
            },
            FieldInfoData {
                name: "stat_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, stat_id),
            },
            FieldInfoData {
                name: "stat_value",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, stat_value),
            },
            FieldInfoData {
                name: "stat_category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, stat_category),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, p_loc),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, p_team_id),
            },
            FieldInfoData {
                name: "p_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, p_veh_id),
            },
            FieldInfoData {
                name: "p_params",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, p_params),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, rdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERSTATEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerStatEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERSTATEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERSTATEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStatEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerStatEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinNpcPartyEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinNpcPartyEventTrait: TelemetrySDKPinEventTrait {
    fn cdur(&self) -> &u32;
    fn cdur_mut(&mut self) -> &mut u32;
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn party_id(&self) -> &String;
    fn party_id_mut(&mut self) -> &mut String;
    fn npc_id(&self) -> &String;
    fn npc_id_mut(&mut self) -> &mut String;
    fn npc_char(&self) -> &String;
    fn npc_char_mut(&mut self) -> &mut String;
    fn npc_class(&self) -> &String;
    fn npc_class_mut(&mut self) -> &mut String;
    fn affin(&self) -> &i32;
    fn affin_mut(&mut self) -> &mut i32;
    fn max_affin(&self) -> &i32;
    fn max_affin_mut(&mut self) -> &mut i32;
    fn min_affin(&self) -> &i32;
    fn min_affin_mut(&mut self) -> &mut i32;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn members(&self) -> &Vec<String>;
    fn members_mut(&mut self) -> &mut Vec<String>;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinNpcPartyEventTrait for TelemetrySdkPinNpcPartyEvent {
    fn cdur(&self) -> &u32 {
        &self.cdur
    }
    fn cdur_mut(&mut self) -> &mut u32 {
        &mut self.cdur
    }
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn party_id(&self) -> &String {
        &self.party_id
    }
    fn party_id_mut(&mut self) -> &mut String {
        &mut self.party_id
    }
    fn npc_id(&self) -> &String {
        &self.npc_id
    }
    fn npc_id_mut(&mut self) -> &mut String {
        &mut self.npc_id
    }
    fn npc_char(&self) -> &String {
        &self.npc_char
    }
    fn npc_char_mut(&mut self) -> &mut String {
        &mut self.npc_char
    }
    fn npc_class(&self) -> &String {
        &self.npc_class
    }
    fn npc_class_mut(&mut self) -> &mut String {
        &mut self.npc_class
    }
    fn affin(&self) -> &i32 {
        &self.affin
    }
    fn affin_mut(&mut self) -> &mut i32 {
        &mut self.affin
    }
    fn max_affin(&self) -> &i32 {
        &self.max_affin
    }
    fn max_affin_mut(&mut self) -> &mut i32 {
        &mut self.max_affin
    }
    fn min_affin(&self) -> &i32 {
        &self.min_affin
    }
    fn min_affin_mut(&mut self) -> &mut i32 {
        &mut self.min_affin
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn members(&self) -> &Vec<String> {
        &self.members
    }
    fn members_mut(&mut self) -> &mut Vec<String> {
        &mut self.members
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinNpcPartyEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinNpcPartyEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinNpcPartyEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinNpcPartyEvent {
}

pub static TELEMETRYSDKPINNPCPARTYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcPartyEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinNpcPartyEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, cdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, rdur),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, instance_id),
            },
            FieldInfoData {
                name: "party_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, party_id),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_char",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, npc_char),
            },
            FieldInfoData {
                name: "npc_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, npc_class),
            },
            FieldInfoData {
                name: "affin",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, affin),
            },
            FieldInfoData {
                name: "max_affin",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, max_affin),
            },
            FieldInfoData {
                name: "min_affin",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, min_affin),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, status),
            },
            FieldInfoData {
                name: "members",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, members),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinNpcPartyEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCPARTYEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinNpcPartyEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINNPCPARTYEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINNPCPARTYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcPartyEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcPartyEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinNpcInteractionEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinNpcInteractionEventTrait: TelemetrySDKPinEventTrait {
    fn cdur(&self) -> &u32;
    fn cdur_mut(&mut self) -> &mut u32;
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn npc_id(&self) -> &String;
    fn npc_id_mut(&mut self) -> &mut String;
    fn npc_char(&self) -> &String;
    fn npc_char_mut(&mut self) -> &mut String;
    fn npc_class(&self) -> &String;
    fn npc_class_mut(&mut self) -> &mut String;
    fn npc_loc(&self) -> &super::core::Vec3;
    fn npc_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn action(&self) -> &String;
    fn action_mut(&mut self) -> &mut String;
    fn content(&self) -> &RawJsonString;
    fn content_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinNpcInteractionEventTrait for TelemetrySdkPinNpcInteractionEvent {
    fn cdur(&self) -> &u32 {
        &self.cdur
    }
    fn cdur_mut(&mut self) -> &mut u32 {
        &mut self.cdur
    }
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn npc_id(&self) -> &String {
        &self.npc_id
    }
    fn npc_id_mut(&mut self) -> &mut String {
        &mut self.npc_id
    }
    fn npc_char(&self) -> &String {
        &self.npc_char
    }
    fn npc_char_mut(&mut self) -> &mut String {
        &mut self.npc_char
    }
    fn npc_class(&self) -> &String {
        &self.npc_class
    }
    fn npc_class_mut(&mut self) -> &mut String {
        &mut self.npc_class
    }
    fn npc_loc(&self) -> &super::core::Vec3 {
        &self.npc_loc
    }
    fn npc_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.npc_loc
    }
    fn action(&self) -> &String {
        &self.action
    }
    fn action_mut(&mut self) -> &mut String {
        &mut self.action
    }
    fn content(&self) -> &RawJsonString {
        &self.content
    }
    fn content_mut(&mut self) -> &mut RawJsonString {
        &mut self.content
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinNpcInteractionEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinNpcInteractionEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinNpcInteractionEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinNpcInteractionEvent {
}

pub static TELEMETRYSDKPINNPCINTERACTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcInteractionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinNpcInteractionEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, cdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, rdur),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, instance_id),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_char",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, npc_char),
            },
            FieldInfoData {
                name: "npc_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, npc_class),
            },
            FieldInfoData {
                name: "npc_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, npc_loc),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, action),
            },
            FieldInfoData {
                name: "content",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, content),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCINTERACTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinNpcInteractionEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINNPCINTERACTIONEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINNPCINTERACTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcInteractionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcInteractionEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinNpcRelationshipEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinNpcRelationshipEventTrait: TelemetrySDKPinEventTrait {
    fn cdur(&self) -> &u32;
    fn cdur_mut(&mut self) -> &mut u32;
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn npc_id(&self) -> &String;
    fn npc_id_mut(&mut self) -> &mut String;
    fn npc_char(&self) -> &String;
    fn npc_char_mut(&mut self) -> &mut String;
    fn npc_class(&self) -> &String;
    fn npc_class_mut(&mut self) -> &mut String;
    fn amount(&self) -> &i32;
    fn amount_mut(&mut self) -> &mut i32;
    fn affin(&self) -> &i32;
    fn affin_mut(&mut self) -> &mut i32;
    fn max_affin(&self) -> &i32;
    fn max_affin_mut(&mut self) -> &mut i32;
    fn min_affin(&self) -> &i32;
    fn min_affin_mut(&mut self) -> &mut i32;
    fn choice_id(&self) -> &String;
    fn choice_id_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinNpcRelationshipEventTrait for TelemetrySdkPinNpcRelationshipEvent {
    fn cdur(&self) -> &u32 {
        &self.cdur
    }
    fn cdur_mut(&mut self) -> &mut u32 {
        &mut self.cdur
    }
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn npc_id(&self) -> &String {
        &self.npc_id
    }
    fn npc_id_mut(&mut self) -> &mut String {
        &mut self.npc_id
    }
    fn npc_char(&self) -> &String {
        &self.npc_char
    }
    fn npc_char_mut(&mut self) -> &mut String {
        &mut self.npc_char
    }
    fn npc_class(&self) -> &String {
        &self.npc_class
    }
    fn npc_class_mut(&mut self) -> &mut String {
        &mut self.npc_class
    }
    fn amount(&self) -> &i32 {
        &self.amount
    }
    fn amount_mut(&mut self) -> &mut i32 {
        &mut self.amount
    }
    fn affin(&self) -> &i32 {
        &self.affin
    }
    fn affin_mut(&mut self) -> &mut i32 {
        &mut self.affin
    }
    fn max_affin(&self) -> &i32 {
        &self.max_affin
    }
    fn max_affin_mut(&mut self) -> &mut i32 {
        &mut self.max_affin
    }
    fn min_affin(&self) -> &i32 {
        &self.min_affin
    }
    fn min_affin_mut(&mut self) -> &mut i32 {
        &mut self.min_affin
    }
    fn choice_id(&self) -> &String {
        &self.choice_id
    }
    fn choice_id_mut(&mut self) -> &mut String {
        &mut self.choice_id
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinNpcRelationshipEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinNpcRelationshipEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinNpcRelationshipEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinNpcRelationshipEvent {
}

pub static TELEMETRYSDKPINNPCRELATIONSHIPEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcRelationshipEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinNpcRelationshipEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, cdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, rdur),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, instance_id),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_char",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, npc_char),
            },
            FieldInfoData {
                name: "npc_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, npc_class),
            },
            FieldInfoData {
                name: "amount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, amount),
            },
            FieldInfoData {
                name: "affin",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, affin),
            },
            FieldInfoData {
                name: "max_affin",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, max_affin),
            },
            FieldInfoData {
                name: "min_affin",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, min_affin),
            },
            FieldInfoData {
                name: "choice_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, choice_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinNpcRelationshipEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCRELATIONSHIPEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinNpcRelationshipEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINNPCRELATIONSHIPEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINNPCRELATIONSHIPEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcRelationshipEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcRelationshipEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinCinematicEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinCinematicEventTrait: TelemetrySDKPinEventTrait {
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn cdur(&self) -> &u32;
    fn cdur_mut(&mut self) -> &mut u32;
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn cine_id(&self) -> &String;
    fn cine_id_mut(&mut self) -> &mut String;
    fn cine_dur(&self) -> &u32;
    fn cine_dur_mut(&mut self) -> &mut u32;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinCinematicEventTrait for TelemetrySdkPinCinematicEvent {
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn cdur(&self) -> &u32 {
        &self.cdur
    }
    fn cdur_mut(&mut self) -> &mut u32 {
        &mut self.cdur
    }
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn cine_id(&self) -> &String {
        &self.cine_id
    }
    fn cine_id_mut(&mut self) -> &mut String {
        &mut self.cine_id
    }
    fn cine_dur(&self) -> &u32 {
        &self.cine_dur
    }
    fn cine_dur_mut(&mut self) -> &mut u32 {
        &mut self.cine_dur
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinCinematicEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinCinematicEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinCinematicEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinCinematicEvent {
}

pub static TELEMETRYSDKPINCINEMATICEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCinematicEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinCinematicEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, instance_id),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, cdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, rdur),
            },
            FieldInfoData {
                name: "cine_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, cine_id),
            },
            FieldInfoData {
                name: "cine_dur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, cine_dur),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinCinematicEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINCINEMATICEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinCinematicEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINCINEMATICEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINCINEMATICEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCinematicEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinCinematicEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerDecisionEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerDecisionEventTrait: TelemetrySDKPinEventTrait {
    fn cdur(&self) -> &u32;
    fn cdur_mut(&mut self) -> &mut u32;
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn choice_id(&self) -> &String;
    fn choice_id_mut(&mut self) -> &mut String;
    fn choice_type(&self) -> &String;
    fn choice_type_mut(&mut self) -> &mut String;
    fn response_id(&self) -> &String;
    fn response_id_mut(&mut self) -> &mut String;
    fn choices(&self) -> &Vec<String>;
    fn choices_mut(&mut self) -> &mut Vec<String>;
    fn cine_id(&self) -> &String;
    fn cine_id_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinPlayerDecisionEventTrait for TelemetrySdkPinPlayerDecisionEvent {
    fn cdur(&self) -> &u32 {
        &self.cdur
    }
    fn cdur_mut(&mut self) -> &mut u32 {
        &mut self.cdur
    }
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn choice_id(&self) -> &String {
        &self.choice_id
    }
    fn choice_id_mut(&mut self) -> &mut String {
        &mut self.choice_id
    }
    fn choice_type(&self) -> &String {
        &self.choice_type
    }
    fn choice_type_mut(&mut self) -> &mut String {
        &mut self.choice_type
    }
    fn response_id(&self) -> &String {
        &self.response_id
    }
    fn response_id_mut(&mut self) -> &mut String {
        &mut self.response_id
    }
    fn choices(&self) -> &Vec<String> {
        &self.choices
    }
    fn choices_mut(&mut self) -> &mut Vec<String> {
        &mut self.choices
    }
    fn cine_id(&self) -> &String {
        &self.cine_id
    }
    fn cine_id_mut(&mut self) -> &mut String {
        &mut self.cine_id
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerDecisionEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerDecisionEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerDecisionEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerDecisionEvent {
}

pub static TELEMETRYSDKPINPLAYERDECISIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerDecisionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerDecisionEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, cdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, rdur),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, instance_id),
            },
            FieldInfoData {
                name: "choice_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, choice_id),
            },
            FieldInfoData {
                name: "choice_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, choice_type),
            },
            FieldInfoData {
                name: "response_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, response_id),
            },
            FieldInfoData {
                name: "choices",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, choices),
            },
            FieldInfoData {
                name: "cine_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, cine_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDecisionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERDECISIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerDecisionEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERDECISIONEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERDECISIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerDecisionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerDecisionEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinScoreEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinScoreEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn amount(&self) -> &u32;
    fn amount_mut(&mut self) -> &mut u32;
    fn team_id(&self) -> &String;
    fn team_id_mut(&mut self) -> &mut String;
    fn player_dbid(&self) -> &i32;
    fn player_dbid_mut(&mut self) -> &mut i32;
    fn clock_time(&self) -> &u32;
    fn clock_time_mut(&mut self) -> &mut u32;
    fn shot_pos(&self) -> &super::core::Vec3;
    fn shot_pos_mut(&mut self) -> &mut super::core::Vec3;
    fn assist_pos(&self) -> &super::core::Vec3;
    fn assist_pos_mut(&mut self) -> &mut super::core::Vec3;
    fn score_pos(&self) -> &super::core::Vec3;
    fn score_pos_mut(&mut self) -> &mut super::core::Vec3;
    fn category(&self) -> &i32;
    fn category_mut(&mut self) -> &mut i32;
    fn shot_flags(&self) -> &i32;
    fn shot_flags_mut(&mut self) -> &mut i32;
    fn goal_flags1(&self) -> &i32;
    fn goal_flags1_mut(&mut self) -> &mut i32;
    fn goal_flags2(&self) -> &i32;
    fn goal_flags2_mut(&mut self) -> &mut i32;
    fn goal_flags3(&self) -> &i32;
    fn goal_flags3_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinScoreEventTrait for TelemetrySdkPinScoreEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn amount(&self) -> &u32 {
        &self.amount
    }
    fn amount_mut(&mut self) -> &mut u32 {
        &mut self.amount
    }
    fn team_id(&self) -> &String {
        &self.team_id
    }
    fn team_id_mut(&mut self) -> &mut String {
        &mut self.team_id
    }
    fn player_dbid(&self) -> &i32 {
        &self.player_dbid
    }
    fn player_dbid_mut(&mut self) -> &mut i32 {
        &mut self.player_dbid
    }
    fn clock_time(&self) -> &u32 {
        &self.clock_time
    }
    fn clock_time_mut(&mut self) -> &mut u32 {
        &mut self.clock_time
    }
    fn shot_pos(&self) -> &super::core::Vec3 {
        &self.shot_pos
    }
    fn shot_pos_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.shot_pos
    }
    fn assist_pos(&self) -> &super::core::Vec3 {
        &self.assist_pos
    }
    fn assist_pos_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.assist_pos
    }
    fn score_pos(&self) -> &super::core::Vec3 {
        &self.score_pos
    }
    fn score_pos_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.score_pos
    }
    fn category(&self) -> &i32 {
        &self.category
    }
    fn category_mut(&mut self) -> &mut i32 {
        &mut self.category
    }
    fn shot_flags(&self) -> &i32 {
        &self.shot_flags
    }
    fn shot_flags_mut(&mut self) -> &mut i32 {
        &mut self.shot_flags
    }
    fn goal_flags1(&self) -> &i32 {
        &self.goal_flags1
    }
    fn goal_flags1_mut(&mut self) -> &mut i32 {
        &mut self.goal_flags1
    }
    fn goal_flags2(&self) -> &i32 {
        &self.goal_flags2
    }
    fn goal_flags2_mut(&mut self) -> &mut i32 {
        &mut self.goal_flags2
    }
    fn goal_flags3(&self) -> &i32 {
        &self.goal_flags3
    }
    fn goal_flags3_mut(&mut self) -> &mut i32 {
        &mut self.goal_flags3
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinScoreEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinScoreEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinScoreEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinScoreEvent {
}

pub static TELEMETRYSDKPINSCOREEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinScoreEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinScoreEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, rdur),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, r#type),
            },
            FieldInfoData {
                name: "amount",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, amount),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, team_id),
            },
            FieldInfoData {
                name: "player_dbid",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, player_dbid),
            },
            FieldInfoData {
                name: "clock_time",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, clock_time),
            },
            FieldInfoData {
                name: "shot_pos",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, shot_pos),
            },
            FieldInfoData {
                name: "assist_pos",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, assist_pos),
            },
            FieldInfoData {
                name: "score_pos",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, score_pos),
            },
            FieldInfoData {
                name: "category",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, category),
            },
            FieldInfoData {
                name: "shot_flags",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, shot_flags),
            },
            FieldInfoData {
                name: "goal_flags1",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, goal_flags1),
            },
            FieldInfoData {
                name: "goal_flags2",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, goal_flags2),
            },
            FieldInfoData {
                name: "goal_flags3",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, goal_flags3),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinScoreEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINSCOREEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinScoreEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINSCOREEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINSCOREEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinScoreEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinScoreEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinNpcOutResourceEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub gdur: u32,
    pub rdur: u32,
    pub resource_type: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinNpcOutResourceEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn resource_type(&self) -> &String;
    fn resource_type_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinNpcOutResourceEventTrait for TelemetrySdkPinNpcOutResourceEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn resource_type(&self) -> &String {
        &self.resource_type
    }
    fn resource_type_mut(&mut self) -> &mut String {
        &mut self.resource_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinNpcOutResourceEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinNpcOutResourceEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinNpcOutResourceEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinNpcOutResourceEvent {
}

pub static TELEMETRYSDKPINNPCOUTRESOURCEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcOutResourceEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinNpcOutResourceEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcOutResourceEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcOutResourceEvent, rdur),
            },
            FieldInfoData {
                name: "resource_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcOutResourceEvent, resource_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinNpcOutResourceEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCOUTRESOURCEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinNpcOutResourceEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINNPCOUTRESOURCEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINNPCOUTRESOURCEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcOutResourceEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcOutResourceEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinNpcInteractEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinNpcInteractEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn item_id(&self) -> &String;
    fn item_id_mut(&mut self) -> &mut String;
    fn item_category(&self) -> &String;
    fn item_category_mut(&mut self) -> &mut String;
    fn item_type(&self) -> &String;
    fn item_type_mut(&mut self) -> &mut String;
    fn item_loc(&self) -> &super::core::Vec3;
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn action(&self) -> &String;
    fn action_mut(&mut self) -> &mut String;
    fn p_dir(&self) -> &super::core::Vec3;
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn p_state(&self) -> &String;
    fn p_state_mut(&mut self) -> &mut String;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn npc_id(&self) -> &String;
    fn npc_id_mut(&mut self) -> &mut String;
    fn npc_dir(&self) -> &super::core::Vec3;
    fn npc_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn npc_loc(&self) -> &super::core::Vec3;
    fn npc_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinNpcInteractEventTrait for TelemetrySdkPinNpcInteractEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn item_id(&self) -> &String {
        &self.item_id
    }
    fn item_id_mut(&mut self) -> &mut String {
        &mut self.item_id
    }
    fn item_category(&self) -> &String {
        &self.item_category
    }
    fn item_category_mut(&mut self) -> &mut String {
        &mut self.item_category
    }
    fn item_type(&self) -> &String {
        &self.item_type
    }
    fn item_type_mut(&mut self) -> &mut String {
        &mut self.item_type
    }
    fn item_loc(&self) -> &super::core::Vec3 {
        &self.item_loc
    }
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.item_loc
    }
    fn action(&self) -> &String {
        &self.action
    }
    fn action_mut(&mut self) -> &mut String {
        &mut self.action
    }
    fn p_dir(&self) -> &super::core::Vec3 {
        &self.p_dir
    }
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_dir
    }
    fn p_state(&self) -> &String {
        &self.p_state
    }
    fn p_state_mut(&mut self) -> &mut String {
        &mut self.p_state
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn npc_id(&self) -> &String {
        &self.npc_id
    }
    fn npc_id_mut(&mut self) -> &mut String {
        &mut self.npc_id
    }
    fn npc_dir(&self) -> &super::core::Vec3 {
        &self.npc_dir
    }
    fn npc_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.npc_dir
    }
    fn npc_loc(&self) -> &super::core::Vec3 {
        &self.npc_loc
    }
    fn npc_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.npc_loc
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinNpcInteractEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinNpcInteractEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinNpcInteractEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinNpcInteractEvent {
}

pub static TELEMETRYSDKPINNPCINTERACTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcInteractEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinNpcInteractEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, item_type),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, item_loc),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, action),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, p_dir),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, p_state),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, p_loc),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, npc_dir),
            },
            FieldInfoData {
                name: "npc_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, npc_loc),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinNpcInteractEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCINTERACTEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinNpcInteractEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINNPCINTERACTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINNPCINTERACTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcInteractEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcInteractEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinNpcDeathEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinNpcDeathEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn k_weap(&self) -> &String;
    fn k_weap_mut(&mut self) -> &mut String;
    fn cause(&self) -> &String;
    fn cause_mut(&mut self) -> &mut String;
    fn v_id(&self) -> &String;
    fn v_id_mut(&mut self) -> &mut String;
    fn v_type(&self) -> &String;
    fn v_type_mut(&mut self) -> &mut String;
    fn v_loc(&self) -> &super::core::Vec3;
    fn v_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn v_dir(&self) -> &super::core::Vec3;
    fn v_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn v_state(&self) -> &String;
    fn v_state_mut(&mut self) -> &mut String;
    fn is_vads(&self) -> &bool;
    fn is_vads_mut(&mut self) -> &mut bool;
    fn k_id(&self) -> &String;
    fn k_id_mut(&mut self) -> &mut String;
    fn k_type(&self) -> &String;
    fn k_type_mut(&mut self) -> &mut String;
    fn k_loc(&self) -> &super::core::Vec3;
    fn k_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn k_dir(&self) -> &super::core::Vec3;
    fn k_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn k_state(&self) -> &String;
    fn k_state_mut(&mut self) -> &mut String;
    fn is_kads(&self) -> &bool;
    fn is_kads_mut(&mut self) -> &mut bool;
    fn v_weap(&self) -> &String;
    fn v_weap_mut(&mut self) -> &mut String;
    fn v_buff(&self) -> &Vec<String>;
    fn v_buff_mut(&mut self) -> &mut Vec<String>;
    fn k_buff(&self) -> &Vec<String>;
    fn k_buff_mut(&mut self) -> &mut Vec<String>;
    fn k_class(&self) -> &String;
    fn k_class_mut(&mut self) -> &mut String;
    fn v_class(&self) -> &String;
    fn v_class_mut(&mut self) -> &mut String;
    fn npc_id(&self) -> &String;
    fn npc_id_mut(&mut self) -> &mut String;
    fn v_team_id(&self) -> &String;
    fn v_team_id_mut(&mut self) -> &mut String;
    fn k_lifetime(&self) -> &u32;
    fn k_lifetime_mut(&mut self) -> &mut u32;
    fn v_lifetime(&self) -> &u32;
    fn v_lifetime_mut(&mut self) -> &mut u32;
    fn v_play_state(&self) -> &String;
    fn v_play_state_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl TelemetrySdkPinNpcDeathEventTrait for TelemetrySdkPinNpcDeathEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn k_weap(&self) -> &String {
        &self.k_weap
    }
    fn k_weap_mut(&mut self) -> &mut String {
        &mut self.k_weap
    }
    fn cause(&self) -> &String {
        &self.cause
    }
    fn cause_mut(&mut self) -> &mut String {
        &mut self.cause
    }
    fn v_id(&self) -> &String {
        &self.v_id
    }
    fn v_id_mut(&mut self) -> &mut String {
        &mut self.v_id
    }
    fn v_type(&self) -> &String {
        &self.v_type
    }
    fn v_type_mut(&mut self) -> &mut String {
        &mut self.v_type
    }
    fn v_loc(&self) -> &super::core::Vec3 {
        &self.v_loc
    }
    fn v_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.v_loc
    }
    fn v_dir(&self) -> &super::core::Vec3 {
        &self.v_dir
    }
    fn v_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.v_dir
    }
    fn v_state(&self) -> &String {
        &self.v_state
    }
    fn v_state_mut(&mut self) -> &mut String {
        &mut self.v_state
    }
    fn is_vads(&self) -> &bool {
        &self.is_vads
    }
    fn is_vads_mut(&mut self) -> &mut bool {
        &mut self.is_vads
    }
    fn k_id(&self) -> &String {
        &self.k_id
    }
    fn k_id_mut(&mut self) -> &mut String {
        &mut self.k_id
    }
    fn k_type(&self) -> &String {
        &self.k_type
    }
    fn k_type_mut(&mut self) -> &mut String {
        &mut self.k_type
    }
    fn k_loc(&self) -> &super::core::Vec3 {
        &self.k_loc
    }
    fn k_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.k_loc
    }
    fn k_dir(&self) -> &super::core::Vec3 {
        &self.k_dir
    }
    fn k_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.k_dir
    }
    fn k_state(&self) -> &String {
        &self.k_state
    }
    fn k_state_mut(&mut self) -> &mut String {
        &mut self.k_state
    }
    fn is_kads(&self) -> &bool {
        &self.is_kads
    }
    fn is_kads_mut(&mut self) -> &mut bool {
        &mut self.is_kads
    }
    fn v_weap(&self) -> &String {
        &self.v_weap
    }
    fn v_weap_mut(&mut self) -> &mut String {
        &mut self.v_weap
    }
    fn v_buff(&self) -> &Vec<String> {
        &self.v_buff
    }
    fn v_buff_mut(&mut self) -> &mut Vec<String> {
        &mut self.v_buff
    }
    fn k_buff(&self) -> &Vec<String> {
        &self.k_buff
    }
    fn k_buff_mut(&mut self) -> &mut Vec<String> {
        &mut self.k_buff
    }
    fn k_class(&self) -> &String {
        &self.k_class
    }
    fn k_class_mut(&mut self) -> &mut String {
        &mut self.k_class
    }
    fn v_class(&self) -> &String {
        &self.v_class
    }
    fn v_class_mut(&mut self) -> &mut String {
        &mut self.v_class
    }
    fn npc_id(&self) -> &String {
        &self.npc_id
    }
    fn npc_id_mut(&mut self) -> &mut String {
        &mut self.npc_id
    }
    fn v_team_id(&self) -> &String {
        &self.v_team_id
    }
    fn v_team_id_mut(&mut self) -> &mut String {
        &mut self.v_team_id
    }
    fn k_lifetime(&self) -> &u32 {
        &self.k_lifetime
    }
    fn k_lifetime_mut(&mut self) -> &mut u32 {
        &mut self.k_lifetime
    }
    fn v_lifetime(&self) -> &u32 {
        &self.v_lifetime
    }
    fn v_lifetime_mut(&mut self) -> &mut u32 {
        &mut self.v_lifetime
    }
    fn v_play_state(&self) -> &String {
        &self.v_play_state
    }
    fn v_play_state_mut(&mut self) -> &mut String {
        &mut self.v_play_state
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinNpcDeathEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinNpcDeathEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinNpcDeathEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinNpcDeathEvent {
}

pub static TELEMETRYSDKPINNPCDEATHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcDeathEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinNpcDeathEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, rdur),
            },
            FieldInfoData {
                name: "k_weap",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_weap),
            },
            FieldInfoData {
                name: "cause",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, cause),
            },
            FieldInfoData {
                name: "v_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_id),
            },
            FieldInfoData {
                name: "v_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_type),
            },
            FieldInfoData {
                name: "v_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_loc),
            },
            FieldInfoData {
                name: "v_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_dir),
            },
            FieldInfoData {
                name: "v_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_state),
            },
            FieldInfoData {
                name: "is_vads",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, is_vads),
            },
            FieldInfoData {
                name: "k_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_id),
            },
            FieldInfoData {
                name: "k_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_type),
            },
            FieldInfoData {
                name: "k_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_loc),
            },
            FieldInfoData {
                name: "k_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_dir),
            },
            FieldInfoData {
                name: "k_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_state),
            },
            FieldInfoData {
                name: "is_kads",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, is_kads),
            },
            FieldInfoData {
                name: "v_weap",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_weap),
            },
            FieldInfoData {
                name: "v_buff",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_buff),
            },
            FieldInfoData {
                name: "k_buff",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_buff),
            },
            FieldInfoData {
                name: "k_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_class),
            },
            FieldInfoData {
                name: "v_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_class),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, npc_id),
            },
            FieldInfoData {
                name: "v_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_team_id),
            },
            FieldInfoData {
                name: "k_lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, k_lifetime),
            },
            FieldInfoData {
                name: "v_lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_lifetime),
            },
            FieldInfoData {
                name: "v_play_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, v_play_state),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcDeathEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCDEATHEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinNpcDeathEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINNPCDEATHEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINNPCDEATHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcDeathEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcDeathEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinNpcStateEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinNpcStateEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn ai_state(&self) -> &bool;
    fn ai_state_mut(&mut self) -> &mut bool;
    fn npc_id(&self) -> &String;
    fn npc_id_mut(&mut self) -> &mut String;
    fn npc_loc(&self) -> &super::core::Vec3;
    fn npc_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn cur_npc_state(&self) -> &String;
    fn cur_npc_state_mut(&mut self) -> &mut String;
    fn prev_npc_state(&self) -> &String;
    fn prev_npc_state_mut(&mut self) -> &mut String;
    fn source(&self) -> &String;
    fn source_mut(&mut self) -> &mut String;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_aim(&self) -> &super::core::Vec3;
    fn p_aim_mut(&mut self) -> &mut super::core::Vec3;
    fn p_state(&self) -> &String;
    fn p_state_mut(&mut self) -> &mut String;
    fn is_alarm(&self) -> &bool;
    fn is_alarm_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinNpcStateEventTrait for TelemetrySdkPinNpcStateEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn ai_state(&self) -> &bool {
        &self.ai_state
    }
    fn ai_state_mut(&mut self) -> &mut bool {
        &mut self.ai_state
    }
    fn npc_id(&self) -> &String {
        &self.npc_id
    }
    fn npc_id_mut(&mut self) -> &mut String {
        &mut self.npc_id
    }
    fn npc_loc(&self) -> &super::core::Vec3 {
        &self.npc_loc
    }
    fn npc_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.npc_loc
    }
    fn cur_npc_state(&self) -> &String {
        &self.cur_npc_state
    }
    fn cur_npc_state_mut(&mut self) -> &mut String {
        &mut self.cur_npc_state
    }
    fn prev_npc_state(&self) -> &String {
        &self.prev_npc_state
    }
    fn prev_npc_state_mut(&mut self) -> &mut String {
        &mut self.prev_npc_state
    }
    fn source(&self) -> &String {
        &self.source
    }
    fn source_mut(&mut self) -> &mut String {
        &mut self.source
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_aim(&self) -> &super::core::Vec3 {
        &self.p_aim
    }
    fn p_aim_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_aim
    }
    fn p_state(&self) -> &String {
        &self.p_state
    }
    fn p_state_mut(&mut self) -> &mut String {
        &mut self.p_state
    }
    fn is_alarm(&self) -> &bool {
        &self.is_alarm
    }
    fn is_alarm_mut(&mut self) -> &mut bool {
        &mut self.is_alarm
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinNpcStateEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinNpcStateEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinNpcStateEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinNpcStateEvent {
}

pub static TELEMETRYSDKPINNPCSTATEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcStateEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinNpcStateEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, rdur),
            },
            FieldInfoData {
                name: "ai_state",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, ai_state),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, npc_loc),
            },
            FieldInfoData {
                name: "cur_npc_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, cur_npc_state),
            },
            FieldInfoData {
                name: "prev_npc_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, prev_npc_state),
            },
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, source),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, p_loc),
            },
            FieldInfoData {
                name: "p_aim",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, p_aim),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, p_state),
            },
            FieldInfoData {
                name: "is_alarm",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, is_alarm),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinNpcStateEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCSTATEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinNpcStateEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINNPCSTATEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINNPCSTATEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcStateEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcStateEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinGameObjectiveEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinGameObjectiveEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn diff(&self) -> &String;
    fn diff_mut(&mut self) -> &mut String;
    fn p_team_id(&self) -> &String;
    fn p_team_id_mut(&mut self) -> &mut String;
    fn category(&self) -> &String;
    fn category_mut(&mut self) -> &mut String;
    fn objective_id(&self) -> &String;
    fn objective_id_mut(&mut self) -> &mut String;
    fn objective_loc(&self) -> &super::core::Vec3;
    fn objective_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn reqs(&self) -> &Vec<String>;
    fn reqs_mut(&mut self) -> &mut Vec<String>;
    fn reward(&self) -> &RawJsonString;
    fn reward_mut(&mut self) -> &mut RawJsonString;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn percent(&self) -> &u32;
    fn percent_mut(&mut self) -> &mut u32;
    fn players(&self) -> &Vec<String>;
    fn players_mut(&mut self) -> &mut Vec<String>;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinGameObjectiveEventTrait for TelemetrySdkPinGameObjectiveEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn diff(&self) -> &String {
        &self.diff
    }
    fn diff_mut(&mut self) -> &mut String {
        &mut self.diff
    }
    fn p_team_id(&self) -> &String {
        &self.p_team_id
    }
    fn p_team_id_mut(&mut self) -> &mut String {
        &mut self.p_team_id
    }
    fn category(&self) -> &String {
        &self.category
    }
    fn category_mut(&mut self) -> &mut String {
        &mut self.category
    }
    fn objective_id(&self) -> &String {
        &self.objective_id
    }
    fn objective_id_mut(&mut self) -> &mut String {
        &mut self.objective_id
    }
    fn objective_loc(&self) -> &super::core::Vec3 {
        &self.objective_loc
    }
    fn objective_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.objective_loc
    }
    fn reqs(&self) -> &Vec<String> {
        &self.reqs
    }
    fn reqs_mut(&mut self) -> &mut Vec<String> {
        &mut self.reqs
    }
    fn reward(&self) -> &RawJsonString {
        &self.reward
    }
    fn reward_mut(&mut self) -> &mut RawJsonString {
        &mut self.reward
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn percent(&self) -> &u32 {
        &self.percent
    }
    fn percent_mut(&mut self) -> &mut u32 {
        &mut self.percent
    }
    fn players(&self) -> &Vec<String> {
        &self.players
    }
    fn players_mut(&mut self) -> &mut Vec<String> {
        &mut self.players
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinGameObjectiveEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinGameObjectiveEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinGameObjectiveEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinGameObjectiveEvent {
}

pub static TELEMETRYSDKPINGAMEOBJECTIVEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameObjectiveEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinGameObjectiveEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, rdur),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, p_loc),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, diff),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, p_team_id),
            },
            FieldInfoData {
                name: "category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, category),
            },
            FieldInfoData {
                name: "objective_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, objective_id),
            },
            FieldInfoData {
                name: "objective_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, objective_loc),
            },
            FieldInfoData {
                name: "reqs",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, reqs),
            },
            FieldInfoData {
                name: "reward",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, reward),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, status),
            },
            FieldInfoData {
                name: "percent",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, percent),
            },
            FieldInfoData {
                name: "players",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, players),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinGameObjectiveEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINGAMEOBJECTIVEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinGameObjectiveEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINGAMEOBJECTIVEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINGAMEOBJECTIVEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameObjectiveEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinGameObjectiveEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinFpsEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinFpsEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn fps(&self) -> &f32;
    fn fps_mut(&mut self) -> &mut f32;
    fn threshold(&self) -> &f32;
    fn threshold_mut(&mut self) -> &mut f32;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_dir(&self) -> &super::core::Vec3;
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn metadata(&self) -> &RawJsonString;
    fn metadata_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinFpsEventTrait for TelemetrySdkPinFpsEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn fps(&self) -> &f32 {
        &self.fps
    }
    fn fps_mut(&mut self) -> &mut f32 {
        &mut self.fps
    }
    fn threshold(&self) -> &f32 {
        &self.threshold
    }
    fn threshold_mut(&mut self) -> &mut f32 {
        &mut self.threshold
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_dir(&self) -> &super::core::Vec3 {
        &self.p_dir
    }
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_dir
    }
    fn metadata(&self) -> &RawJsonString {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut RawJsonString {
        &mut self.metadata
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinFpsEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinFpsEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinFpsEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinFpsEvent {
}

pub static TELEMETRYSDKPINFPSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFpsEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinFpsEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, rdur),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, r#type),
            },
            FieldInfoData {
                name: "fps",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, fps),
            },
            FieldInfoData {
                name: "threshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, threshold),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, p_dir),
            },
            FieldInfoData {
                name: "metadata",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, metadata),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinFpsEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINFPSEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinFpsEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINFPSEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINFPSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFpsEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinFpsEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinGameActionEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub gdur: u32,
    pub rdur: u32,
    pub action: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinGameActionEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn action(&self) -> &String;
    fn action_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinGameActionEventTrait for TelemetrySdkPinGameActionEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn action(&self) -> &String {
        &self.action
    }
    fn action_mut(&mut self) -> &mut String {
        &mut self.action
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinGameActionEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinGameActionEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinGameActionEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinGameActionEvent {
}

pub static TELEMETRYSDKPINGAMEACTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameActionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinGameActionEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinGameActionEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinGameActionEvent, rdur),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameActionEvent, action),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinGameActionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINGAMEACTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinGameActionEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINGAMEACTIONEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINGAMEACTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameActionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinGameActionEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinVehicleHealthEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinVehicleHealthEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn veh_id(&self) -> &String;
    fn veh_id_mut(&mut self) -> &mut String;
    fn veh_loc(&self) -> &super::core::Vec3;
    fn veh_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn veh_type(&self) -> &String;
    fn veh_type_mut(&mut self) -> &mut String;
    fn veh_guid(&self) -> &String;
    fn veh_guid_mut(&mut self) -> &mut String;
    fn is_heal(&self) -> &bool;
    fn is_heal_mut(&mut self) -> &mut bool;
    fn source_id(&self) -> &String;
    fn source_id_mut(&mut self) -> &mut String;
    fn source_type(&self) -> &String;
    fn source_type_mut(&mut self) -> &mut String;
    fn source_loc(&self) -> &super::core::Vec3;
    fn source_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn source_item(&self) -> &String;
    fn source_item_mut(&mut self) -> &mut String;
    fn cause(&self) -> &String;
    fn cause_mut(&mut self) -> &mut String;
    fn weapon(&self) -> &String;
    fn weapon_mut(&mut self) -> &mut String;
    fn amount(&self) -> &i32;
    fn amount_mut(&mut self) -> &mut i32;
    fn health(&self) -> &i32;
    fn health_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinVehicleHealthEventTrait for TelemetrySdkPinVehicleHealthEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn veh_id(&self) -> &String {
        &self.veh_id
    }
    fn veh_id_mut(&mut self) -> &mut String {
        &mut self.veh_id
    }
    fn veh_loc(&self) -> &super::core::Vec3 {
        &self.veh_loc
    }
    fn veh_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.veh_loc
    }
    fn veh_type(&self) -> &String {
        &self.veh_type
    }
    fn veh_type_mut(&mut self) -> &mut String {
        &mut self.veh_type
    }
    fn veh_guid(&self) -> &String {
        &self.veh_guid
    }
    fn veh_guid_mut(&mut self) -> &mut String {
        &mut self.veh_guid
    }
    fn is_heal(&self) -> &bool {
        &self.is_heal
    }
    fn is_heal_mut(&mut self) -> &mut bool {
        &mut self.is_heal
    }
    fn source_id(&self) -> &String {
        &self.source_id
    }
    fn source_id_mut(&mut self) -> &mut String {
        &mut self.source_id
    }
    fn source_type(&self) -> &String {
        &self.source_type
    }
    fn source_type_mut(&mut self) -> &mut String {
        &mut self.source_type
    }
    fn source_loc(&self) -> &super::core::Vec3 {
        &self.source_loc
    }
    fn source_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.source_loc
    }
    fn source_item(&self) -> &String {
        &self.source_item
    }
    fn source_item_mut(&mut self) -> &mut String {
        &mut self.source_item
    }
    fn cause(&self) -> &String {
        &self.cause
    }
    fn cause_mut(&mut self) -> &mut String {
        &mut self.cause
    }
    fn weapon(&self) -> &String {
        &self.weapon
    }
    fn weapon_mut(&mut self) -> &mut String {
        &mut self.weapon
    }
    fn amount(&self) -> &i32 {
        &self.amount
    }
    fn amount_mut(&mut self) -> &mut i32 {
        &mut self.amount
    }
    fn health(&self) -> &i32 {
        &self.health
    }
    fn health_mut(&mut self) -> &mut i32 {
        &mut self.health
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinVehicleHealthEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinVehicleHealthEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinVehicleHealthEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinVehicleHealthEvent {
}

pub static TELEMETRYSDKPINVEHICLEHEALTHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleHealthEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinVehicleHealthEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, rdur),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, veh_id),
            },
            FieldInfoData {
                name: "veh_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, veh_loc),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, veh_type),
            },
            FieldInfoData {
                name: "veh_guid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, veh_guid),
            },
            FieldInfoData {
                name: "is_heal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, is_heal),
            },
            FieldInfoData {
                name: "source_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, source_id),
            },
            FieldInfoData {
                name: "source_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, source_type),
            },
            FieldInfoData {
                name: "source_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, source_loc),
            },
            FieldInfoData {
                name: "source_item",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, source_item),
            },
            FieldInfoData {
                name: "cause",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, cause),
            },
            FieldInfoData {
                name: "weapon",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, weapon),
            },
            FieldInfoData {
                name: "amount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, amount),
            },
            FieldInfoData {
                name: "health",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, health),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinVehicleHealthEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINVEHICLEHEALTHEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinVehicleHealthEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINVEHICLEHEALTHEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINVEHICLEHEALTHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleHealthEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinVehicleHealthEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinVehicleInteractEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinVehicleInteractEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn veh_dur(&self) -> &u32;
    fn veh_dur_mut(&mut self) -> &mut u32;
    fn veh_id(&self) -> &String;
    fn veh_id_mut(&mut self) -> &mut String;
    fn veh_guid(&self) -> &String;
    fn veh_guid_mut(&mut self) -> &mut String;
    fn veh_type(&self) -> &String;
    fn veh_type_mut(&mut self) -> &mut String;
    fn action(&self) -> &String;
    fn action_mut(&mut self) -> &mut String;
    fn veh_loc(&self) -> &super::core::Vec3;
    fn veh_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_dir(&self) -> &super::core::Vec3;
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn p_team_id(&self) -> &String;
    fn p_team_id_mut(&mut self) -> &mut String;
    fn custom(&self) -> &RawJsonString;
    fn custom_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinVehicleInteractEventTrait for TelemetrySdkPinVehicleInteractEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn veh_dur(&self) -> &u32 {
        &self.veh_dur
    }
    fn veh_dur_mut(&mut self) -> &mut u32 {
        &mut self.veh_dur
    }
    fn veh_id(&self) -> &String {
        &self.veh_id
    }
    fn veh_id_mut(&mut self) -> &mut String {
        &mut self.veh_id
    }
    fn veh_guid(&self) -> &String {
        &self.veh_guid
    }
    fn veh_guid_mut(&mut self) -> &mut String {
        &mut self.veh_guid
    }
    fn veh_type(&self) -> &String {
        &self.veh_type
    }
    fn veh_type_mut(&mut self) -> &mut String {
        &mut self.veh_type
    }
    fn action(&self) -> &String {
        &self.action
    }
    fn action_mut(&mut self) -> &mut String {
        &mut self.action
    }
    fn veh_loc(&self) -> &super::core::Vec3 {
        &self.veh_loc
    }
    fn veh_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.veh_loc
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_dir(&self) -> &super::core::Vec3 {
        &self.p_dir
    }
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_dir
    }
    fn p_team_id(&self) -> &String {
        &self.p_team_id
    }
    fn p_team_id_mut(&mut self) -> &mut String {
        &mut self.p_team_id
    }
    fn custom(&self) -> &RawJsonString {
        &self.custom
    }
    fn custom_mut(&mut self) -> &mut RawJsonString {
        &mut self.custom
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinVehicleInteractEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinVehicleInteractEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinVehicleInteractEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinVehicleInteractEvent {
}

pub static TELEMETRYSDKPINVEHICLEINTERACTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleInteractEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinVehicleInteractEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, rdur),
            },
            FieldInfoData {
                name: "veh_dur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, veh_dur),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, veh_id),
            },
            FieldInfoData {
                name: "veh_guid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, veh_guid),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, veh_type),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, action),
            },
            FieldInfoData {
                name: "veh_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, veh_loc),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, p_dir),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, p_team_id),
            },
            FieldInfoData {
                name: "custom",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, custom),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinVehicleInteractEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINVEHICLEINTERACTEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinVehicleInteractEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINVEHICLEINTERACTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINVEHICLEINTERACTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleInteractEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinVehicleInteractEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinVehicleDestructionEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinVehicleDestructionEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn veh_id(&self) -> &String;
    fn veh_id_mut(&mut self) -> &mut String;
    fn cause(&self) -> &String;
    fn cause_mut(&mut self) -> &mut String;
    fn veh_loc(&self) -> &super::core::Vec3;
    fn veh_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn veh_type(&self) -> &String;
    fn veh_type_mut(&mut self) -> &mut String;
    fn pilot_id(&self) -> &String;
    fn pilot_id_mut(&mut self) -> &mut String;
    fn pilot_type(&self) -> &String;
    fn pilot_type_mut(&mut self) -> &mut String;
    fn pilot_loc(&self) -> &super::core::Vec3;
    fn pilot_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn pilot_dir(&self) -> &super::core::Vec3;
    fn pilot_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn k_id(&self) -> &String;
    fn k_id_mut(&mut self) -> &mut String;
    fn k_type(&self) -> &String;
    fn k_type_mut(&mut self) -> &mut String;
    fn k_loc(&self) -> &super::core::Vec3;
    fn k_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn k_dir(&self) -> &super::core::Vec3;
    fn k_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn is_eject(&self) -> &bool;
    fn is_eject_mut(&mut self) -> &mut bool;
    fn veh_dur(&self) -> &u32;
    fn veh_dur_mut(&mut self) -> &mut u32;
    fn veh_instance_id(&self) -> &String;
    fn veh_instance_id_mut(&mut self) -> &mut String;
    fn k_weap(&self) -> &String;
    fn k_weap_mut(&mut self) -> &mut String;
    fn k_veh_id(&self) -> &String;
    fn k_veh_id_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl TelemetrySdkPinVehicleDestructionEventTrait for TelemetrySdkPinVehicleDestructionEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn veh_id(&self) -> &String {
        &self.veh_id
    }
    fn veh_id_mut(&mut self) -> &mut String {
        &mut self.veh_id
    }
    fn cause(&self) -> &String {
        &self.cause
    }
    fn cause_mut(&mut self) -> &mut String {
        &mut self.cause
    }
    fn veh_loc(&self) -> &super::core::Vec3 {
        &self.veh_loc
    }
    fn veh_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.veh_loc
    }
    fn veh_type(&self) -> &String {
        &self.veh_type
    }
    fn veh_type_mut(&mut self) -> &mut String {
        &mut self.veh_type
    }
    fn pilot_id(&self) -> &String {
        &self.pilot_id
    }
    fn pilot_id_mut(&mut self) -> &mut String {
        &mut self.pilot_id
    }
    fn pilot_type(&self) -> &String {
        &self.pilot_type
    }
    fn pilot_type_mut(&mut self) -> &mut String {
        &mut self.pilot_type
    }
    fn pilot_loc(&self) -> &super::core::Vec3 {
        &self.pilot_loc
    }
    fn pilot_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.pilot_loc
    }
    fn pilot_dir(&self) -> &super::core::Vec3 {
        &self.pilot_dir
    }
    fn pilot_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.pilot_dir
    }
    fn k_id(&self) -> &String {
        &self.k_id
    }
    fn k_id_mut(&mut self) -> &mut String {
        &mut self.k_id
    }
    fn k_type(&self) -> &String {
        &self.k_type
    }
    fn k_type_mut(&mut self) -> &mut String {
        &mut self.k_type
    }
    fn k_loc(&self) -> &super::core::Vec3 {
        &self.k_loc
    }
    fn k_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.k_loc
    }
    fn k_dir(&self) -> &super::core::Vec3 {
        &self.k_dir
    }
    fn k_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.k_dir
    }
    fn is_eject(&self) -> &bool {
        &self.is_eject
    }
    fn is_eject_mut(&mut self) -> &mut bool {
        &mut self.is_eject
    }
    fn veh_dur(&self) -> &u32 {
        &self.veh_dur
    }
    fn veh_dur_mut(&mut self) -> &mut u32 {
        &mut self.veh_dur
    }
    fn veh_instance_id(&self) -> &String {
        &self.veh_instance_id
    }
    fn veh_instance_id_mut(&mut self) -> &mut String {
        &mut self.veh_instance_id
    }
    fn k_weap(&self) -> &String {
        &self.k_weap
    }
    fn k_weap_mut(&mut self) -> &mut String {
        &mut self.k_weap
    }
    fn k_veh_id(&self) -> &String {
        &self.k_veh_id
    }
    fn k_veh_id_mut(&mut self) -> &mut String {
        &mut self.k_veh_id
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinVehicleDestructionEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinVehicleDestructionEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinVehicleDestructionEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinVehicleDestructionEvent {
}

pub static TELEMETRYSDKPINVEHICLEDESTRUCTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleDestructionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinVehicleDestructionEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, rdur),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, veh_id),
            },
            FieldInfoData {
                name: "cause",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, cause),
            },
            FieldInfoData {
                name: "veh_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, veh_loc),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, veh_type),
            },
            FieldInfoData {
                name: "pilot_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, pilot_id),
            },
            FieldInfoData {
                name: "pilot_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, pilot_type),
            },
            FieldInfoData {
                name: "pilot_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, pilot_loc),
            },
            FieldInfoData {
                name: "pilot_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, pilot_dir),
            },
            FieldInfoData {
                name: "k_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_id),
            },
            FieldInfoData {
                name: "k_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_type),
            },
            FieldInfoData {
                name: "k_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_loc),
            },
            FieldInfoData {
                name: "k_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_dir),
            },
            FieldInfoData {
                name: "is_eject",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, is_eject),
            },
            FieldInfoData {
                name: "veh_dur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, veh_dur),
            },
            FieldInfoData {
                name: "veh_instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, veh_instance_id),
            },
            FieldInfoData {
                name: "k_weap",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_weap),
            },
            FieldInfoData {
                name: "k_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, k_veh_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleDestructionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINVEHICLEDESTRUCTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinVehicleDestructionEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINVEHICLEDESTRUCTIONEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINVEHICLEDESTRUCTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleDestructionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinVehicleDestructionEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinVehicleSpawnEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinVehicleSpawnEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn veh_id(&self) -> &String;
    fn veh_id_mut(&mut self) -> &mut String;
    fn veh_type(&self) -> &String;
    fn veh_type_mut(&mut self) -> &mut String;
    fn veh_loc(&self) -> &super::core::Vec3;
    fn veh_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn veh_lev(&self) -> &i32;
    fn veh_lev_mut(&mut self) -> &mut i32;
    fn veh_loadout(&self) -> &RawJsonString;
    fn veh_loadout_mut(&mut self) -> &mut RawJsonString;
    fn veh_instance_id(&self) -> &String;
    fn veh_instance_id_mut(&mut self) -> &mut String;
    fn team_id(&self) -> &String;
    fn team_id_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinVehicleSpawnEventTrait for TelemetrySdkPinVehicleSpawnEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn veh_id(&self) -> &String {
        &self.veh_id
    }
    fn veh_id_mut(&mut self) -> &mut String {
        &mut self.veh_id
    }
    fn veh_type(&self) -> &String {
        &self.veh_type
    }
    fn veh_type_mut(&mut self) -> &mut String {
        &mut self.veh_type
    }
    fn veh_loc(&self) -> &super::core::Vec3 {
        &self.veh_loc
    }
    fn veh_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.veh_loc
    }
    fn veh_lev(&self) -> &i32 {
        &self.veh_lev
    }
    fn veh_lev_mut(&mut self) -> &mut i32 {
        &mut self.veh_lev
    }
    fn veh_loadout(&self) -> &RawJsonString {
        &self.veh_loadout
    }
    fn veh_loadout_mut(&mut self) -> &mut RawJsonString {
        &mut self.veh_loadout
    }
    fn veh_instance_id(&self) -> &String {
        &self.veh_instance_id
    }
    fn veh_instance_id_mut(&mut self) -> &mut String {
        &mut self.veh_instance_id
    }
    fn team_id(&self) -> &String {
        &self.team_id
    }
    fn team_id_mut(&mut self) -> &mut String {
        &mut self.team_id
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinVehicleSpawnEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinVehicleSpawnEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinVehicleSpawnEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinVehicleSpawnEvent {
}

pub static TELEMETRYSDKPINVEHICLESPAWNEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleSpawnEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinVehicleSpawnEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, rdur),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_id),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_type),
            },
            FieldInfoData {
                name: "veh_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_loc),
            },
            FieldInfoData {
                name: "veh_lev",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_lev),
            },
            FieldInfoData {
                name: "veh_loadout",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_loadout),
            },
            FieldInfoData {
                name: "veh_instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, veh_instance_id),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, team_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinVehicleSpawnEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINVEHICLESPAWNEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinVehicleSpawnEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINVEHICLESPAWNEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINVEHICLESPAWNEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinVehicleSpawnEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinVehicleSpawnEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinTagEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub gdur: u32,
    pub rdur: u32,
    pub tag_type: String,
    pub tag_id: String,
    pub tag_loc: super::core::Vec3,
    pub p_loc: super::core::Vec3,
    pub tag_method: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinTagEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn tag_type(&self) -> &String;
    fn tag_type_mut(&mut self) -> &mut String;
    fn tag_id(&self) -> &String;
    fn tag_id_mut(&mut self) -> &mut String;
    fn tag_loc(&self) -> &super::core::Vec3;
    fn tag_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn tag_method(&self) -> &String;
    fn tag_method_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinTagEventTrait for TelemetrySdkPinTagEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn tag_type(&self) -> &String {
        &self.tag_type
    }
    fn tag_type_mut(&mut self) -> &mut String {
        &mut self.tag_type
    }
    fn tag_id(&self) -> &String {
        &self.tag_id
    }
    fn tag_id_mut(&mut self) -> &mut String {
        &mut self.tag_id
    }
    fn tag_loc(&self) -> &super::core::Vec3 {
        &self.tag_loc
    }
    fn tag_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.tag_loc
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn tag_method(&self) -> &String {
        &self.tag_method
    }
    fn tag_method_mut(&mut self) -> &mut String {
        &mut self.tag_method
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinTagEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinTagEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinTagEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinTagEvent {
}

pub static TELEMETRYSDKPINTAGEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTagEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinTagEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, rdur),
            },
            FieldInfoData {
                name: "tag_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, tag_type),
            },
            FieldInfoData {
                name: "tag_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, tag_id),
            },
            FieldInfoData {
                name: "tag_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, tag_loc),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, p_loc),
            },
            FieldInfoData {
                name: "tag_method",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, tag_method),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinTagEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINTAGEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinTagEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINTAGEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINTAGEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTagEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinTagEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinItemUpgradeEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub gdur: u32,
    pub rdur: u32,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_name: String,
    pub item_loc: super::core::Vec3,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinItemUpgradeEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn item_id(&self) -> &String;
    fn item_id_mut(&mut self) -> &mut String;
    fn item_category(&self) -> &String;
    fn item_category_mut(&mut self) -> &mut String;
    fn item_type(&self) -> &String;
    fn item_type_mut(&mut self) -> &mut String;
    fn item_name(&self) -> &String;
    fn item_name_mut(&mut self) -> &mut String;
    fn item_loc(&self) -> &super::core::Vec3;
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinItemUpgradeEventTrait for TelemetrySdkPinItemUpgradeEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn item_id(&self) -> &String {
        &self.item_id
    }
    fn item_id_mut(&mut self) -> &mut String {
        &mut self.item_id
    }
    fn item_category(&self) -> &String {
        &self.item_category
    }
    fn item_category_mut(&mut self) -> &mut String {
        &mut self.item_category
    }
    fn item_type(&self) -> &String {
        &self.item_type
    }
    fn item_type_mut(&mut self) -> &mut String {
        &mut self.item_type
    }
    fn item_name(&self) -> &String {
        &self.item_name
    }
    fn item_name_mut(&mut self) -> &mut String {
        &mut self.item_name
    }
    fn item_loc(&self) -> &super::core::Vec3 {
        &self.item_loc
    }
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.item_loc
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinItemUpgradeEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinItemUpgradeEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinItemUpgradeEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinItemUpgradeEvent {
}

pub static TELEMETRYSDKPINITEMUPGRADEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemUpgradeEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinItemUpgradeEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, item_type),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, item_name),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, item_loc),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinItemUpgradeEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINITEMUPGRADEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinItemUpgradeEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINITEMUPGRADEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINITEMUPGRADEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemUpgradeEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinItemUpgradeEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinItemUnlockEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub gdur: u32,
    pub rdur: u32,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_loc: super::core::Vec3,
    pub item_name: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinItemUnlockEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn item_id(&self) -> &String;
    fn item_id_mut(&mut self) -> &mut String;
    fn item_category(&self) -> &String;
    fn item_category_mut(&mut self) -> &mut String;
    fn item_type(&self) -> &String;
    fn item_type_mut(&mut self) -> &mut String;
    fn item_loc(&self) -> &super::core::Vec3;
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn item_name(&self) -> &String;
    fn item_name_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinItemUnlockEventTrait for TelemetrySdkPinItemUnlockEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn item_id(&self) -> &String {
        &self.item_id
    }
    fn item_id_mut(&mut self) -> &mut String {
        &mut self.item_id
    }
    fn item_category(&self) -> &String {
        &self.item_category
    }
    fn item_category_mut(&mut self) -> &mut String {
        &mut self.item_category
    }
    fn item_type(&self) -> &String {
        &self.item_type
    }
    fn item_type_mut(&mut self) -> &mut String {
        &mut self.item_type
    }
    fn item_loc(&self) -> &super::core::Vec3 {
        &self.item_loc
    }
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.item_loc
    }
    fn item_name(&self) -> &String {
        &self.item_name
    }
    fn item_name_mut(&mut self) -> &mut String {
        &mut self.item_name
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinItemUnlockEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinItemUnlockEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinItemUnlockEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinItemUnlockEvent {
}

pub static TELEMETRYSDKPINITEMUNLOCKEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemUnlockEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinItemUnlockEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, item_type),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, item_loc),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, item_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinItemUnlockEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINITEMUNLOCKEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinItemUnlockEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINITEMUNLOCKEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINITEMUNLOCKEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemUnlockEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinItemUnlockEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinItemPickupEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinItemPickupEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn item_dur(&self) -> &u32;
    fn item_dur_mut(&mut self) -> &mut u32;
    fn item_id(&self) -> &String;
    fn item_id_mut(&mut self) -> &mut String;
    fn item_category(&self) -> &String;
    fn item_category_mut(&mut self) -> &mut String;
    fn item_type(&self) -> &String;
    fn item_type_mut(&mut self) -> &mut String;
    fn item_name(&self) -> &String;
    fn item_name_mut(&mut self) -> &mut String;
    fn item_loc(&self) -> &super::core::Vec3;
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinItemPickupEventTrait for TelemetrySdkPinItemPickupEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn item_dur(&self) -> &u32 {
        &self.item_dur
    }
    fn item_dur_mut(&mut self) -> &mut u32 {
        &mut self.item_dur
    }
    fn item_id(&self) -> &String {
        &self.item_id
    }
    fn item_id_mut(&mut self) -> &mut String {
        &mut self.item_id
    }
    fn item_category(&self) -> &String {
        &self.item_category
    }
    fn item_category_mut(&mut self) -> &mut String {
        &mut self.item_category
    }
    fn item_type(&self) -> &String {
        &self.item_type
    }
    fn item_type_mut(&mut self) -> &mut String {
        &mut self.item_type
    }
    fn item_name(&self) -> &String {
        &self.item_name
    }
    fn item_name_mut(&mut self) -> &mut String {
        &mut self.item_name
    }
    fn item_loc(&self) -> &super::core::Vec3 {
        &self.item_loc
    }
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.item_loc
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinItemPickupEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinItemPickupEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinItemPickupEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinItemPickupEvent {
}

pub static TELEMETRYSDKPINITEMPICKUPEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemPickupEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinItemPickupEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, rdur),
            },
            FieldInfoData {
                name: "item_dur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_dur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_type),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_name),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, item_loc),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinItemPickupEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINITEMPICKUPEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinItemPickupEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINITEMPICKUPEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINITEMPICKUPEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemPickupEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinItemPickupEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinItemSpawnEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub gdur: u32,
    pub rdur: u32,
    pub item_id: String,
    pub item_category: String,
    pub item_type: String,
    pub item_name: String,
    pub item_loc: super::core::Vec3,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinItemSpawnEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn item_id(&self) -> &String;
    fn item_id_mut(&mut self) -> &mut String;
    fn item_category(&self) -> &String;
    fn item_category_mut(&mut self) -> &mut String;
    fn item_type(&self) -> &String;
    fn item_type_mut(&mut self) -> &mut String;
    fn item_name(&self) -> &String;
    fn item_name_mut(&mut self) -> &mut String;
    fn item_loc(&self) -> &super::core::Vec3;
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinItemSpawnEventTrait for TelemetrySdkPinItemSpawnEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn item_id(&self) -> &String {
        &self.item_id
    }
    fn item_id_mut(&mut self) -> &mut String {
        &mut self.item_id
    }
    fn item_category(&self) -> &String {
        &self.item_category
    }
    fn item_category_mut(&mut self) -> &mut String {
        &mut self.item_category
    }
    fn item_type(&self) -> &String {
        &self.item_type
    }
    fn item_type_mut(&mut self) -> &mut String {
        &mut self.item_type
    }
    fn item_name(&self) -> &String {
        &self.item_name
    }
    fn item_name_mut(&mut self) -> &mut String {
        &mut self.item_name
    }
    fn item_loc(&self) -> &super::core::Vec3 {
        &self.item_loc
    }
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.item_loc
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinItemSpawnEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinItemSpawnEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinItemSpawnEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinItemSpawnEvent {
}

pub static TELEMETRYSDKPINITEMSPAWNEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemSpawnEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinItemSpawnEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, item_type),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, item_name),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, item_loc),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinItemSpawnEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINITEMSPAWNEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinItemSpawnEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINITEMSPAWNEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINITEMSPAWNEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemSpawnEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinItemSpawnEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinItemHealthEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinItemHealthEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn item_id(&self) -> &String;
    fn item_id_mut(&mut self) -> &mut String;
    fn item_category(&self) -> &String;
    fn item_category_mut(&mut self) -> &mut String;
    fn item_type(&self) -> &String;
    fn item_type_mut(&mut self) -> &mut String;
    fn item_loc(&self) -> &super::core::Vec3;
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_id(&self) -> &String;
    fn p_id_mut(&mut self) -> &mut String;
    fn p_type(&self) -> &String;
    fn p_type_mut(&mut self) -> &mut String;
    fn is_heal(&self) -> &bool;
    fn is_heal_mut(&mut self) -> &mut bool;
    fn source_id(&self) -> &String;
    fn source_id_mut(&mut self) -> &mut String;
    fn source_type(&self) -> &String;
    fn source_type_mut(&mut self) -> &mut String;
    fn source_loc(&self) -> &super::core::Vec3;
    fn source_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn weapon(&self) -> &String;
    fn weapon_mut(&mut self) -> &mut String;
    fn amount(&self) -> &i32;
    fn amount_mut(&mut self) -> &mut i32;
    fn health(&self) -> &i32;
    fn health_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinItemHealthEventTrait for TelemetrySdkPinItemHealthEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn item_id(&self) -> &String {
        &self.item_id
    }
    fn item_id_mut(&mut self) -> &mut String {
        &mut self.item_id
    }
    fn item_category(&self) -> &String {
        &self.item_category
    }
    fn item_category_mut(&mut self) -> &mut String {
        &mut self.item_category
    }
    fn item_type(&self) -> &String {
        &self.item_type
    }
    fn item_type_mut(&mut self) -> &mut String {
        &mut self.item_type
    }
    fn item_loc(&self) -> &super::core::Vec3 {
        &self.item_loc
    }
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.item_loc
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_id(&self) -> &String {
        &self.p_id
    }
    fn p_id_mut(&mut self) -> &mut String {
        &mut self.p_id
    }
    fn p_type(&self) -> &String {
        &self.p_type
    }
    fn p_type_mut(&mut self) -> &mut String {
        &mut self.p_type
    }
    fn is_heal(&self) -> &bool {
        &self.is_heal
    }
    fn is_heal_mut(&mut self) -> &mut bool {
        &mut self.is_heal
    }
    fn source_id(&self) -> &String {
        &self.source_id
    }
    fn source_id_mut(&mut self) -> &mut String {
        &mut self.source_id
    }
    fn source_type(&self) -> &String {
        &self.source_type
    }
    fn source_type_mut(&mut self) -> &mut String {
        &mut self.source_type
    }
    fn source_loc(&self) -> &super::core::Vec3 {
        &self.source_loc
    }
    fn source_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.source_loc
    }
    fn weapon(&self) -> &String {
        &self.weapon
    }
    fn weapon_mut(&mut self) -> &mut String {
        &mut self.weapon
    }
    fn amount(&self) -> &i32 {
        &self.amount
    }
    fn amount_mut(&mut self) -> &mut i32 {
        &mut self.amount
    }
    fn health(&self) -> &i32 {
        &self.health
    }
    fn health_mut(&mut self) -> &mut i32 {
        &mut self.health
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinItemHealthEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinItemHealthEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinItemHealthEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinItemHealthEvent {
}

pub static TELEMETRYSDKPINITEMHEALTHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemHealthEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinItemHealthEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, item_type),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, item_loc),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, p_loc),
            },
            FieldInfoData {
                name: "p_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, p_id),
            },
            FieldInfoData {
                name: "p_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, p_type),
            },
            FieldInfoData {
                name: "is_heal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, is_heal),
            },
            FieldInfoData {
                name: "source_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, source_id),
            },
            FieldInfoData {
                name: "source_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, source_type),
            },
            FieldInfoData {
                name: "source_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, source_loc),
            },
            FieldInfoData {
                name: "weapon",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, weapon),
            },
            FieldInfoData {
                name: "amount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, amount),
            },
            FieldInfoData {
                name: "health",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, health),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinItemHealthEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINITEMHEALTHEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinItemHealthEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINITEMHEALTHEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINITEMHEALTHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinItemHealthEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinItemHealthEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerOutResourceEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub gdur: u32,
    pub rdur: u32,
    pub resource_type: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinPlayerOutResourceEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn resource_type(&self) -> &String;
    fn resource_type_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinPlayerOutResourceEventTrait for TelemetrySdkPinPlayerOutResourceEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn resource_type(&self) -> &String {
        &self.resource_type
    }
    fn resource_type_mut(&mut self) -> &mut String {
        &mut self.resource_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerOutResourceEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerOutResourceEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerOutResourceEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerOutResourceEvent {
}

pub static TELEMETRYSDKPINPLAYEROUTRESOURCEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerOutResourceEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerOutResourceEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerOutResourceEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerOutResourceEvent, rdur),
            },
            FieldInfoData {
                name: "resource_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerOutResourceEvent, resource_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinPlayerOutResourceEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYEROUTRESOURCEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerOutResourceEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYEROUTRESOURCEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYEROUTRESOURCEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerOutResourceEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerOutResourceEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerTeamSwitchEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub gdur: u32,
    pub rdur: u32,
    pub prev_team_id: String,
    pub prev_team_score: u32,
    pub team_id: String,
    pub team_score: u32,
    pub reason: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinPlayerTeamSwitchEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn prev_team_id(&self) -> &String;
    fn prev_team_id_mut(&mut self) -> &mut String;
    fn prev_team_score(&self) -> &u32;
    fn prev_team_score_mut(&mut self) -> &mut u32;
    fn team_id(&self) -> &String;
    fn team_id_mut(&mut self) -> &mut String;
    fn team_score(&self) -> &u32;
    fn team_score_mut(&mut self) -> &mut u32;
    fn reason(&self) -> &String;
    fn reason_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinPlayerTeamSwitchEventTrait for TelemetrySdkPinPlayerTeamSwitchEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn prev_team_id(&self) -> &String {
        &self.prev_team_id
    }
    fn prev_team_id_mut(&mut self) -> &mut String {
        &mut self.prev_team_id
    }
    fn prev_team_score(&self) -> &u32 {
        &self.prev_team_score
    }
    fn prev_team_score_mut(&mut self) -> &mut u32 {
        &mut self.prev_team_score
    }
    fn team_id(&self) -> &String {
        &self.team_id
    }
    fn team_id_mut(&mut self) -> &mut String {
        &mut self.team_id
    }
    fn team_score(&self) -> &u32 {
        &self.team_score
    }
    fn team_score_mut(&mut self) -> &mut u32 {
        &mut self.team_score
    }
    fn reason(&self) -> &String {
        &self.reason
    }
    fn reason_mut(&mut self) -> &mut String {
        &mut self.reason
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerTeamSwitchEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerTeamSwitchEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerTeamSwitchEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerTeamSwitchEvent {
}

pub static TELEMETRYSDKPINPLAYERTEAMSWITCHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerTeamSwitchEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerTeamSwitchEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, rdur),
            },
            FieldInfoData {
                name: "prev_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, prev_team_id),
            },
            FieldInfoData {
                name: "prev_team_score",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, prev_team_score),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, team_id),
            },
            FieldInfoData {
                name: "team_score",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, team_score),
            },
            FieldInfoData {
                name: "reason",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, reason),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTeamSwitchEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERTEAMSWITCHEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerTeamSwitchEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERTEAMSWITCHEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERTEAMSWITCHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerTeamSwitchEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerTeamSwitchEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerInteractEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerInteractEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn item_id(&self) -> &String;
    fn item_id_mut(&mut self) -> &mut String;
    fn item_category(&self) -> &String;
    fn item_category_mut(&mut self) -> &mut String;
    fn item_type(&self) -> &String;
    fn item_type_mut(&mut self) -> &mut String;
    fn item_loc(&self) -> &super::core::Vec3;
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn action(&self) -> &String;
    fn action_mut(&mut self) -> &mut String;
    fn p_dir(&self) -> &super::core::Vec3;
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn p_state(&self) -> &String;
    fn p_state_mut(&mut self) -> &mut String;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_class(&self) -> &String;
    fn p_class_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinPlayerInteractEventTrait for TelemetrySdkPinPlayerInteractEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn item_id(&self) -> &String {
        &self.item_id
    }
    fn item_id_mut(&mut self) -> &mut String {
        &mut self.item_id
    }
    fn item_category(&self) -> &String {
        &self.item_category
    }
    fn item_category_mut(&mut self) -> &mut String {
        &mut self.item_category
    }
    fn item_type(&self) -> &String {
        &self.item_type
    }
    fn item_type_mut(&mut self) -> &mut String {
        &mut self.item_type
    }
    fn item_loc(&self) -> &super::core::Vec3 {
        &self.item_loc
    }
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.item_loc
    }
    fn action(&self) -> &String {
        &self.action
    }
    fn action_mut(&mut self) -> &mut String {
        &mut self.action
    }
    fn p_dir(&self) -> &super::core::Vec3 {
        &self.p_dir
    }
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_dir
    }
    fn p_state(&self) -> &String {
        &self.p_state
    }
    fn p_state_mut(&mut self) -> &mut String {
        &mut self.p_state
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_class(&self) -> &String {
        &self.p_class
    }
    fn p_class_mut(&mut self) -> &mut String {
        &mut self.p_class
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerInteractEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerInteractEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerInteractEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerInteractEvent {
}

pub static TELEMETRYSDKPINPLAYERINTERACTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerInteractEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerInteractEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, item_type),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, item_loc),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, action),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, p_dir),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, p_state),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, p_loc),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, p_class),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinPlayerInteractEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERINTERACTEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerInteractEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERINTERACTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERINTERACTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerInteractEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerInteractEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerAbilityAffectEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerAbilityAffectEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn a_loc(&self) -> &super::core::Vec3;
    fn a_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn a_type(&self) -> &String;
    fn a_type_mut(&mut self) -> &mut String;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_state(&self) -> &String;
    fn p_state_mut(&mut self) -> &mut String;
    fn p_class(&self) -> &String;
    fn p_class_mut(&mut self) -> &mut String;
    fn p_team_id(&self) -> &String;
    fn p_team_id_mut(&mut self) -> &mut String;
    fn v_state(&self) -> &String;
    fn v_state_mut(&mut self) -> &mut String;
    fn v_loc(&self) -> &super::core::Vec3;
    fn v_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn v_class(&self) -> &String;
    fn v_class_mut(&mut self) -> &mut String;
    fn v_team_id(&self) -> &String;
    fn v_team_id_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinPlayerAbilityAffectEventTrait for TelemetrySdkPinPlayerAbilityAffectEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn a_loc(&self) -> &super::core::Vec3 {
        &self.a_loc
    }
    fn a_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.a_loc
    }
    fn a_type(&self) -> &String {
        &self.a_type
    }
    fn a_type_mut(&mut self) -> &mut String {
        &mut self.a_type
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_state(&self) -> &String {
        &self.p_state
    }
    fn p_state_mut(&mut self) -> &mut String {
        &mut self.p_state
    }
    fn p_class(&self) -> &String {
        &self.p_class
    }
    fn p_class_mut(&mut self) -> &mut String {
        &mut self.p_class
    }
    fn p_team_id(&self) -> &String {
        &self.p_team_id
    }
    fn p_team_id_mut(&mut self) -> &mut String {
        &mut self.p_team_id
    }
    fn v_state(&self) -> &String {
        &self.v_state
    }
    fn v_state_mut(&mut self) -> &mut String {
        &mut self.v_state
    }
    fn v_loc(&self) -> &super::core::Vec3 {
        &self.v_loc
    }
    fn v_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.v_loc
    }
    fn v_class(&self) -> &String {
        &self.v_class
    }
    fn v_class_mut(&mut self) -> &mut String {
        &mut self.v_class
    }
    fn v_team_id(&self) -> &String {
        &self.v_team_id
    }
    fn v_team_id_mut(&mut self) -> &mut String {
        &mut self.v_team_id
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerAbilityAffectEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerAbilityAffectEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerAbilityAffectEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerAbilityAffectEvent {
}

pub static TELEMETRYSDKPINPLAYERABILITYAFFECTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerAbilityAffectEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerAbilityAffectEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, rdur),
            },
            FieldInfoData {
                name: "a_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, a_loc),
            },
            FieldInfoData {
                name: "a_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, a_type),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, p_loc),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, p_state),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, p_team_id),
            },
            FieldInfoData {
                name: "v_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, v_state),
            },
            FieldInfoData {
                name: "v_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, v_loc),
            },
            FieldInfoData {
                name: "v_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, v_class),
            },
            FieldInfoData {
                name: "v_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, v_team_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityAffectEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERABILITYAFFECTEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerAbilityAffectEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERABILITYAFFECTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERABILITYAFFECTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerAbilityAffectEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerAbilityAffectEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerAbilityEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerAbilityEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn ability(&self) -> &String;
    fn ability_mut(&mut self) -> &mut String;
    fn ability_dur(&self) -> &u32;
    fn ability_dur_mut(&mut self) -> &mut u32;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_state(&self) -> &String;
    fn p_state_mut(&mut self) -> &mut String;
    fn ability_data(&self) -> &RawJsonString;
    fn ability_data_mut(&mut self) -> &mut RawJsonString;
    fn p_class(&self) -> &String;
    fn p_class_mut(&mut self) -> &mut String;
    fn p_team_id(&self) -> &String;
    fn p_team_id_mut(&mut self) -> &mut String;
    fn p_veh_id(&self) -> &String;
    fn p_veh_id_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinPlayerAbilityEventTrait for TelemetrySdkPinPlayerAbilityEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn ability(&self) -> &String {
        &self.ability
    }
    fn ability_mut(&mut self) -> &mut String {
        &mut self.ability
    }
    fn ability_dur(&self) -> &u32 {
        &self.ability_dur
    }
    fn ability_dur_mut(&mut self) -> &mut u32 {
        &mut self.ability_dur
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_state(&self) -> &String {
        &self.p_state
    }
    fn p_state_mut(&mut self) -> &mut String {
        &mut self.p_state
    }
    fn ability_data(&self) -> &RawJsonString {
        &self.ability_data
    }
    fn ability_data_mut(&mut self) -> &mut RawJsonString {
        &mut self.ability_data
    }
    fn p_class(&self) -> &String {
        &self.p_class
    }
    fn p_class_mut(&mut self) -> &mut String {
        &mut self.p_class
    }
    fn p_team_id(&self) -> &String {
        &self.p_team_id
    }
    fn p_team_id_mut(&mut self) -> &mut String {
        &mut self.p_team_id
    }
    fn p_veh_id(&self) -> &String {
        &self.p_veh_id
    }
    fn p_veh_id_mut(&mut self) -> &mut String {
        &mut self.p_veh_id
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerAbilityEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerAbilityEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerAbilityEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerAbilityEvent {
}

pub static TELEMETRYSDKPINPLAYERABILITYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerAbilityEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerAbilityEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, rdur),
            },
            FieldInfoData {
                name: "ability",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, ability),
            },
            FieldInfoData {
                name: "ability_dur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, ability_dur),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, p_loc),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, p_state),
            },
            FieldInfoData {
                name: "ability_data",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, ability_data),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, p_team_id),
            },
            FieldInfoData {
                name: "p_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, p_veh_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinPlayerAbilityEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERABILITYEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerAbilityEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERABILITYEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERABILITYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerAbilityEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerAbilityEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerUseEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerUseEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn item_id(&self) -> &String;
    fn item_id_mut(&mut self) -> &mut String;
    fn item_category(&self) -> &String;
    fn item_category_mut(&mut self) -> &mut String;
    fn item_type(&self) -> &String;
    fn item_type_mut(&mut self) -> &mut String;
    fn item_name(&self) -> &String;
    fn item_name_mut(&mut self) -> &mut String;
    fn item_dur(&self) -> &u32;
    fn item_dur_mut(&mut self) -> &mut u32;
    fn item_loc(&self) -> &super::core::Vec3;
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_state(&self) -> &String;
    fn p_state_mut(&mut self) -> &mut String;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_class(&self) -> &String;
    fn p_class_mut(&mut self) -> &mut String;
    fn p_team_id(&self) -> &String;
    fn p_team_id_mut(&mut self) -> &mut String;
    fn veh_id(&self) -> &String;
    fn veh_id_mut(&mut self) -> &mut String;
    fn action(&self) -> &String;
    fn action_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinPlayerUseEventTrait for TelemetrySdkPinPlayerUseEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn item_id(&self) -> &String {
        &self.item_id
    }
    fn item_id_mut(&mut self) -> &mut String {
        &mut self.item_id
    }
    fn item_category(&self) -> &String {
        &self.item_category
    }
    fn item_category_mut(&mut self) -> &mut String {
        &mut self.item_category
    }
    fn item_type(&self) -> &String {
        &self.item_type
    }
    fn item_type_mut(&mut self) -> &mut String {
        &mut self.item_type
    }
    fn item_name(&self) -> &String {
        &self.item_name
    }
    fn item_name_mut(&mut self) -> &mut String {
        &mut self.item_name
    }
    fn item_dur(&self) -> &u32 {
        &self.item_dur
    }
    fn item_dur_mut(&mut self) -> &mut u32 {
        &mut self.item_dur
    }
    fn item_loc(&self) -> &super::core::Vec3 {
        &self.item_loc
    }
    fn item_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.item_loc
    }
    fn p_state(&self) -> &String {
        &self.p_state
    }
    fn p_state_mut(&mut self) -> &mut String {
        &mut self.p_state
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_class(&self) -> &String {
        &self.p_class
    }
    fn p_class_mut(&mut self) -> &mut String {
        &mut self.p_class
    }
    fn p_team_id(&self) -> &String {
        &self.p_team_id
    }
    fn p_team_id_mut(&mut self) -> &mut String {
        &mut self.p_team_id
    }
    fn veh_id(&self) -> &String {
        &self.veh_id
    }
    fn veh_id_mut(&mut self) -> &mut String {
        &mut self.veh_id
    }
    fn action(&self) -> &String {
        &self.action
    }
    fn action_mut(&mut self) -> &mut String {
        &mut self.action
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerUseEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerUseEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerUseEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerUseEvent {
}

pub static TELEMETRYSDKPINPLAYERUSEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerUseEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerUseEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, rdur),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_type),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_name),
            },
            FieldInfoData {
                name: "item_dur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_dur),
            },
            FieldInfoData {
                name: "item_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, item_loc),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, p_state),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, p_loc),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, p_team_id),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, veh_id),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, action),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinPlayerUseEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERUSEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerUseEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERUSEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERUSEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerUseEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerUseEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerKillAssistEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerKillAssistEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn k_id(&self) -> &String;
    fn k_id_mut(&mut self) -> &mut String;
    fn k_type(&self) -> &String;
    fn k_type_mut(&mut self) -> &mut String;
    fn v_id(&self) -> &String;
    fn v_id_mut(&mut self) -> &mut String;
    fn v_type(&self) -> &String;
    fn v_type_mut(&mut self) -> &mut String;
    fn damage(&self) -> &u32;
    fn damage_mut(&mut self) -> &mut u32;
    fn percent(&self) -> &u32;
    fn percent_mut(&mut self) -> &mut u32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinPlayerKillAssistEventTrait for TelemetrySdkPinPlayerKillAssistEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn k_id(&self) -> &String {
        &self.k_id
    }
    fn k_id_mut(&mut self) -> &mut String {
        &mut self.k_id
    }
    fn k_type(&self) -> &String {
        &self.k_type
    }
    fn k_type_mut(&mut self) -> &mut String {
        &mut self.k_type
    }
    fn v_id(&self) -> &String {
        &self.v_id
    }
    fn v_id_mut(&mut self) -> &mut String {
        &mut self.v_id
    }
    fn v_type(&self) -> &String {
        &self.v_type
    }
    fn v_type_mut(&mut self) -> &mut String {
        &mut self.v_type
    }
    fn damage(&self) -> &u32 {
        &self.damage
    }
    fn damage_mut(&mut self) -> &mut u32 {
        &mut self.damage
    }
    fn percent(&self) -> &u32 {
        &self.percent
    }
    fn percent_mut(&mut self) -> &mut u32 {
        &mut self.percent
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerKillAssistEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerKillAssistEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerKillAssistEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerKillAssistEvent {
}

pub static TELEMETRYSDKPINPLAYERKILLASSISTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerKillAssistEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerKillAssistEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, rdur),
            },
            FieldInfoData {
                name: "k_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, k_id),
            },
            FieldInfoData {
                name: "k_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, k_type),
            },
            FieldInfoData {
                name: "v_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, v_id),
            },
            FieldInfoData {
                name: "v_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, v_type),
            },
            FieldInfoData {
                name: "damage",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, damage),
            },
            FieldInfoData {
                name: "percent",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, percent),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinPlayerKillAssistEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERKILLASSISTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerKillAssistEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERKILLASSISTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERKILLASSISTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerKillAssistEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerKillAssistEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerHealthEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerHealthEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn source_id(&self) -> &String;
    fn source_id_mut(&mut self) -> &mut String;
    fn source_type(&self) -> &String;
    fn source_type_mut(&mut self) -> &mut String;
    fn source_loc(&self) -> &super::core::Vec3;
    fn source_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn is_heal(&self) -> &bool;
    fn is_heal_mut(&mut self) -> &mut bool;
    fn is_revived(&self) -> &bool;
    fn is_revived_mut(&mut self) -> &mut bool;
    fn amount(&self) -> &i32;
    fn amount_mut(&mut self) -> &mut i32;
    fn health(&self) -> &i32;
    fn health_mut(&mut self) -> &mut i32;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn weapon(&self) -> &String;
    fn weapon_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinPlayerHealthEventTrait for TelemetrySdkPinPlayerHealthEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn source_id(&self) -> &String {
        &self.source_id
    }
    fn source_id_mut(&mut self) -> &mut String {
        &mut self.source_id
    }
    fn source_type(&self) -> &String {
        &self.source_type
    }
    fn source_type_mut(&mut self) -> &mut String {
        &mut self.source_type
    }
    fn source_loc(&self) -> &super::core::Vec3 {
        &self.source_loc
    }
    fn source_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.source_loc
    }
    fn is_heal(&self) -> &bool {
        &self.is_heal
    }
    fn is_heal_mut(&mut self) -> &mut bool {
        &mut self.is_heal
    }
    fn is_revived(&self) -> &bool {
        &self.is_revived
    }
    fn is_revived_mut(&mut self) -> &mut bool {
        &mut self.is_revived
    }
    fn amount(&self) -> &i32 {
        &self.amount
    }
    fn amount_mut(&mut self) -> &mut i32 {
        &mut self.amount
    }
    fn health(&self) -> &i32 {
        &self.health
    }
    fn health_mut(&mut self) -> &mut i32 {
        &mut self.health
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn weapon(&self) -> &String {
        &self.weapon
    }
    fn weapon_mut(&mut self) -> &mut String {
        &mut self.weapon
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerHealthEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerHealthEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerHealthEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerHealthEvent {
}

pub static TELEMETRYSDKPINPLAYERHEALTHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerHealthEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerHealthEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, rdur),
            },
            FieldInfoData {
                name: "source_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, source_id),
            },
            FieldInfoData {
                name: "source_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, source_type),
            },
            FieldInfoData {
                name: "source_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, source_loc),
            },
            FieldInfoData {
                name: "is_heal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, is_heal),
            },
            FieldInfoData {
                name: "is_revived",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, is_revived),
            },
            FieldInfoData {
                name: "amount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, amount),
            },
            FieldInfoData {
                name: "health",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, health),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, p_loc),
            },
            FieldInfoData {
                name: "weapon",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, weapon),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinPlayerHealthEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERHEALTHEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerHealthEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERHEALTHEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERHEALTHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerHealthEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerHealthEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerDeathEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerDeathEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn cause(&self) -> &String;
    fn cause_mut(&mut self) -> &mut String;
    fn v_loc(&self) -> &super::core::Vec3;
    fn v_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn v_dir(&self) -> &super::core::Vec3;
    fn v_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn v_state(&self) -> &String;
    fn v_state_mut(&mut self) -> &mut String;
    fn is_vads(&self) -> &bool;
    fn is_vads_mut(&mut self) -> &mut bool;
    fn k_id(&self) -> &String;
    fn k_id_mut(&mut self) -> &mut String;
    fn k_type(&self) -> &String;
    fn k_type_mut(&mut self) -> &mut String;
    fn k_class(&self) -> &String;
    fn k_class_mut(&mut self) -> &mut String;
    fn k_weap(&self) -> &String;
    fn k_weap_mut(&mut self) -> &mut String;
    fn k_loc(&self) -> &super::core::Vec3;
    fn k_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn k_dir(&self) -> &super::core::Vec3;
    fn k_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn k_state(&self) -> &String;
    fn k_state_mut(&mut self) -> &mut String;
    fn is_kads(&self) -> &bool;
    fn is_kads_mut(&mut self) -> &mut bool;
    fn v_weap(&self) -> &String;
    fn v_weap_mut(&mut self) -> &mut String;
    fn v_class(&self) -> &String;
    fn v_class_mut(&mut self) -> &mut String;
    fn k_lifetime(&self) -> &u32;
    fn k_lifetime_mut(&mut self) -> &mut u32;
    fn v_lifetime(&self) -> &u32;
    fn v_lifetime_mut(&mut self) -> &mut u32;
    fn v_buff(&self) -> &Vec<String>;
    fn v_buff_mut(&mut self) -> &mut Vec<String>;
    fn k_buff(&self) -> &Vec<String>;
    fn k_buff_mut(&mut self) -> &mut Vec<String>;
    fn v_team_id(&self) -> &String;
    fn v_team_id_mut(&mut self) -> &mut String;
    fn v_res_points(&self) -> &u32;
    fn v_res_points_mut(&mut self) -> &mut u32;
    fn game_play_stats(&self) -> &RawJsonString;
    fn game_play_stats_mut(&mut self) -> &mut RawJsonString;
    fn weapon_stats(&self) -> &RawJsonString;
    fn weapon_stats_mut(&mut self) -> &mut RawJsonString;
    fn v_score_earned(&self) -> &u32;
    fn v_score_earned_mut(&mut self) -> &mut u32;
    fn v_veh_id(&self) -> &String;
    fn v_veh_id_mut(&mut self) -> &mut String;
    fn k_veh_id(&self) -> &String;
    fn k_veh_id_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl TelemetrySdkPinPlayerDeathEventTrait for TelemetrySdkPinPlayerDeathEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn cause(&self) -> &String {
        &self.cause
    }
    fn cause_mut(&mut self) -> &mut String {
        &mut self.cause
    }
    fn v_loc(&self) -> &super::core::Vec3 {
        &self.v_loc
    }
    fn v_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.v_loc
    }
    fn v_dir(&self) -> &super::core::Vec3 {
        &self.v_dir
    }
    fn v_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.v_dir
    }
    fn v_state(&self) -> &String {
        &self.v_state
    }
    fn v_state_mut(&mut self) -> &mut String {
        &mut self.v_state
    }
    fn is_vads(&self) -> &bool {
        &self.is_vads
    }
    fn is_vads_mut(&mut self) -> &mut bool {
        &mut self.is_vads
    }
    fn k_id(&self) -> &String {
        &self.k_id
    }
    fn k_id_mut(&mut self) -> &mut String {
        &mut self.k_id
    }
    fn k_type(&self) -> &String {
        &self.k_type
    }
    fn k_type_mut(&mut self) -> &mut String {
        &mut self.k_type
    }
    fn k_class(&self) -> &String {
        &self.k_class
    }
    fn k_class_mut(&mut self) -> &mut String {
        &mut self.k_class
    }
    fn k_weap(&self) -> &String {
        &self.k_weap
    }
    fn k_weap_mut(&mut self) -> &mut String {
        &mut self.k_weap
    }
    fn k_loc(&self) -> &super::core::Vec3 {
        &self.k_loc
    }
    fn k_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.k_loc
    }
    fn k_dir(&self) -> &super::core::Vec3 {
        &self.k_dir
    }
    fn k_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.k_dir
    }
    fn k_state(&self) -> &String {
        &self.k_state
    }
    fn k_state_mut(&mut self) -> &mut String {
        &mut self.k_state
    }
    fn is_kads(&self) -> &bool {
        &self.is_kads
    }
    fn is_kads_mut(&mut self) -> &mut bool {
        &mut self.is_kads
    }
    fn v_weap(&self) -> &String {
        &self.v_weap
    }
    fn v_weap_mut(&mut self) -> &mut String {
        &mut self.v_weap
    }
    fn v_class(&self) -> &String {
        &self.v_class
    }
    fn v_class_mut(&mut self) -> &mut String {
        &mut self.v_class
    }
    fn k_lifetime(&self) -> &u32 {
        &self.k_lifetime
    }
    fn k_lifetime_mut(&mut self) -> &mut u32 {
        &mut self.k_lifetime
    }
    fn v_lifetime(&self) -> &u32 {
        &self.v_lifetime
    }
    fn v_lifetime_mut(&mut self) -> &mut u32 {
        &mut self.v_lifetime
    }
    fn v_buff(&self) -> &Vec<String> {
        &self.v_buff
    }
    fn v_buff_mut(&mut self) -> &mut Vec<String> {
        &mut self.v_buff
    }
    fn k_buff(&self) -> &Vec<String> {
        &self.k_buff
    }
    fn k_buff_mut(&mut self) -> &mut Vec<String> {
        &mut self.k_buff
    }
    fn v_team_id(&self) -> &String {
        &self.v_team_id
    }
    fn v_team_id_mut(&mut self) -> &mut String {
        &mut self.v_team_id
    }
    fn v_res_points(&self) -> &u32 {
        &self.v_res_points
    }
    fn v_res_points_mut(&mut self) -> &mut u32 {
        &mut self.v_res_points
    }
    fn game_play_stats(&self) -> &RawJsonString {
        &self.game_play_stats
    }
    fn game_play_stats_mut(&mut self) -> &mut RawJsonString {
        &mut self.game_play_stats
    }
    fn weapon_stats(&self) -> &RawJsonString {
        &self.weapon_stats
    }
    fn weapon_stats_mut(&mut self) -> &mut RawJsonString {
        &mut self.weapon_stats
    }
    fn v_score_earned(&self) -> &u32 {
        &self.v_score_earned
    }
    fn v_score_earned_mut(&mut self) -> &mut u32 {
        &mut self.v_score_earned
    }
    fn v_veh_id(&self) -> &String {
        &self.v_veh_id
    }
    fn v_veh_id_mut(&mut self) -> &mut String {
        &mut self.v_veh_id
    }
    fn k_veh_id(&self) -> &String {
        &self.k_veh_id
    }
    fn k_veh_id_mut(&mut self) -> &mut String {
        &mut self.k_veh_id
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerDeathEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerDeathEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerDeathEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerDeathEvent {
}

pub static TELEMETRYSDKPINPLAYERDEATHEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerDeathEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerDeathEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, rdur),
            },
            FieldInfoData {
                name: "cause",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, cause),
            },
            FieldInfoData {
                name: "v_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_loc),
            },
            FieldInfoData {
                name: "v_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_dir),
            },
            FieldInfoData {
                name: "v_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_state),
            },
            FieldInfoData {
                name: "is_vads",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, is_vads),
            },
            FieldInfoData {
                name: "k_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_id),
            },
            FieldInfoData {
                name: "k_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_type),
            },
            FieldInfoData {
                name: "k_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_class),
            },
            FieldInfoData {
                name: "k_weap",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_weap),
            },
            FieldInfoData {
                name: "k_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_loc),
            },
            FieldInfoData {
                name: "k_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_dir),
            },
            FieldInfoData {
                name: "k_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_state),
            },
            FieldInfoData {
                name: "is_kads",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, is_kads),
            },
            FieldInfoData {
                name: "v_weap",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_weap),
            },
            FieldInfoData {
                name: "v_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_class),
            },
            FieldInfoData {
                name: "k_lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_lifetime),
            },
            FieldInfoData {
                name: "v_lifetime",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_lifetime),
            },
            FieldInfoData {
                name: "v_buff",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_buff),
            },
            FieldInfoData {
                name: "k_buff",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_buff),
            },
            FieldInfoData {
                name: "v_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_team_id),
            },
            FieldInfoData {
                name: "v_res_points",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_res_points),
            },
            FieldInfoData {
                name: "game_play_stats",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, game_play_stats),
            },
            FieldInfoData {
                name: "weapon_stats",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, weapon_stats),
            },
            FieldInfoData {
                name: "v_score_earned",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_score_earned),
            },
            FieldInfoData {
                name: "v_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, v_veh_id),
            },
            FieldInfoData {
                name: "k_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, k_veh_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerDeathEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERDEATHEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerDeathEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERDEATHEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERDEATHEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerDeathEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerDeathEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerEquipEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub gdur: u32,
    pub rdur: u32,
    pub loadout: RawJsonString,
    pub prev_loadout: RawJsonString,
    pub pgid: String,
    pub object_id: String,
    pub object_type: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinPlayerEquipEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn loadout(&self) -> &RawJsonString;
    fn loadout_mut(&mut self) -> &mut RawJsonString;
    fn prev_loadout(&self) -> &RawJsonString;
    fn prev_loadout_mut(&mut self) -> &mut RawJsonString;
    fn pgid(&self) -> &String;
    fn pgid_mut(&mut self) -> &mut String;
    fn object_id(&self) -> &String;
    fn object_id_mut(&mut self) -> &mut String;
    fn object_type(&self) -> &String;
    fn object_type_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinPlayerEquipEventTrait for TelemetrySdkPinPlayerEquipEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn loadout(&self) -> &RawJsonString {
        &self.loadout
    }
    fn loadout_mut(&mut self) -> &mut RawJsonString {
        &mut self.loadout
    }
    fn prev_loadout(&self) -> &RawJsonString {
        &self.prev_loadout
    }
    fn prev_loadout_mut(&mut self) -> &mut RawJsonString {
        &mut self.prev_loadout
    }
    fn pgid(&self) -> &String {
        &self.pgid
    }
    fn pgid_mut(&mut self) -> &mut String {
        &mut self.pgid
    }
    fn object_id(&self) -> &String {
        &self.object_id
    }
    fn object_id_mut(&mut self) -> &mut String {
        &mut self.object_id
    }
    fn object_type(&self) -> &String {
        &self.object_type
    }
    fn object_type_mut(&mut self) -> &mut String {
        &mut self.object_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerEquipEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerEquipEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerEquipEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerEquipEvent {
}

pub static TELEMETRYSDKPINPLAYEREQUIPEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerEquipEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerEquipEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, rdur),
            },
            FieldInfoData {
                name: "loadout",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, loadout),
            },
            FieldInfoData {
                name: "prev_loadout",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, prev_loadout),
            },
            FieldInfoData {
                name: "pgid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, pgid),
            },
            FieldInfoData {
                name: "object_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, object_id),
            },
            FieldInfoData {
                name: "object_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, object_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinPlayerEquipEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYEREQUIPEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerEquipEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYEREQUIPEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYEREQUIPEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerEquipEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerEquipEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerClassEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub gdur: u32,
    pub rdur: u32,
    pub p_class: String,
    pub sub_class: String,
    pub prev_class: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinPlayerClassEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn p_class(&self) -> &String;
    fn p_class_mut(&mut self) -> &mut String;
    fn sub_class(&self) -> &String;
    fn sub_class_mut(&mut self) -> &mut String;
    fn prev_class(&self) -> &String;
    fn prev_class_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinPlayerClassEventTrait for TelemetrySdkPinPlayerClassEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn p_class(&self) -> &String {
        &self.p_class
    }
    fn p_class_mut(&mut self) -> &mut String {
        &mut self.p_class
    }
    fn sub_class(&self) -> &String {
        &self.sub_class
    }
    fn sub_class_mut(&mut self) -> &mut String {
        &mut self.sub_class
    }
    fn prev_class(&self) -> &String {
        &self.prev_class
    }
    fn prev_class_mut(&mut self) -> &mut String {
        &mut self.prev_class
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerClassEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerClassEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerClassEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerClassEvent {
}

pub static TELEMETRYSDKPINPLAYERCLASSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerClassEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerClassEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, rdur),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, p_class),
            },
            FieldInfoData {
                name: "sub_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, sub_class),
            },
            FieldInfoData {
                name: "prev_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, prev_class),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinPlayerClassEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERCLASSEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerClassEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERCLASSEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERCLASSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerClassEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerClassEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerStateEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerStateEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn p_state(&self) -> &String;
    fn p_state_mut(&mut self) -> &mut String;
    fn prev_state(&self) -> &String;
    fn prev_state_mut(&mut self) -> &mut String;
    fn prev_dur(&self) -> &u32;
    fn prev_dur_mut(&mut self) -> &mut u32;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_dir(&self) -> &super::core::Vec3;
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn team_id(&self) -> &String;
    fn team_id_mut(&mut self) -> &mut String;
    fn p_class(&self) -> &String;
    fn p_class_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinPlayerStateEventTrait for TelemetrySdkPinPlayerStateEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn p_state(&self) -> &String {
        &self.p_state
    }
    fn p_state_mut(&mut self) -> &mut String {
        &mut self.p_state
    }
    fn prev_state(&self) -> &String {
        &self.prev_state
    }
    fn prev_state_mut(&mut self) -> &mut String {
        &mut self.prev_state
    }
    fn prev_dur(&self) -> &u32 {
        &self.prev_dur
    }
    fn prev_dur_mut(&mut self) -> &mut u32 {
        &mut self.prev_dur
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_dir(&self) -> &super::core::Vec3 {
        &self.p_dir
    }
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_dir
    }
    fn team_id(&self) -> &String {
        &self.team_id
    }
    fn team_id_mut(&mut self) -> &mut String {
        &mut self.team_id
    }
    fn p_class(&self) -> &String {
        &self.p_class
    }
    fn p_class_mut(&mut self) -> &mut String {
        &mut self.p_class
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerStateEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerStateEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerStateEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerStateEvent {
}

pub static TELEMETRYSDKPINPLAYERSTATEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStateEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerStateEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, rdur),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, p_state),
            },
            FieldInfoData {
                name: "prev_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, prev_state),
            },
            FieldInfoData {
                name: "prev_dur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, prev_dur),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, p_dir),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, team_id),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, p_class),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStateEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERSTATEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerStateEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERSTATEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERSTATEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStateEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerStateEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinNpcSpawnEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinNpcSpawnEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn npc_id(&self) -> &String;
    fn npc_id_mut(&mut self) -> &mut String;
    fn npc_loc(&self) -> &super::core::Vec3;
    fn npc_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn npc_class(&self) -> &String;
    fn npc_class_mut(&mut self) -> &mut String;
    fn npc_dir(&self) -> &super::core::Vec3;
    fn npc_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn npc_veh_id(&self) -> &String;
    fn npc_veh_id_mut(&mut self) -> &mut String;
    fn npc_team_id(&self) -> &String;
    fn npc_team_id_mut(&mut self) -> &mut String;
    fn npc_loadout(&self) -> &RawJsonString;
    fn npc_loadout_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinNpcSpawnEventTrait for TelemetrySdkPinNpcSpawnEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn npc_id(&self) -> &String {
        &self.npc_id
    }
    fn npc_id_mut(&mut self) -> &mut String {
        &mut self.npc_id
    }
    fn npc_loc(&self) -> &super::core::Vec3 {
        &self.npc_loc
    }
    fn npc_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.npc_loc
    }
    fn npc_class(&self) -> &String {
        &self.npc_class
    }
    fn npc_class_mut(&mut self) -> &mut String {
        &mut self.npc_class
    }
    fn npc_dir(&self) -> &super::core::Vec3 {
        &self.npc_dir
    }
    fn npc_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.npc_dir
    }
    fn npc_veh_id(&self) -> &String {
        &self.npc_veh_id
    }
    fn npc_veh_id_mut(&mut self) -> &mut String {
        &mut self.npc_veh_id
    }
    fn npc_team_id(&self) -> &String {
        &self.npc_team_id
    }
    fn npc_team_id_mut(&mut self) -> &mut String {
        &mut self.npc_team_id
    }
    fn npc_loadout(&self) -> &RawJsonString {
        &self.npc_loadout
    }
    fn npc_loadout_mut(&mut self) -> &mut RawJsonString {
        &mut self.npc_loadout
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinNpcSpawnEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinNpcSpawnEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinNpcSpawnEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinNpcSpawnEvent {
}

pub static TELEMETRYSDKPINNPCSPAWNEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcSpawnEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinNpcSpawnEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, gdur),
            },
            FieldInfoData {
                name: "npc_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_id),
            },
            FieldInfoData {
                name: "npc_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_loc),
            },
            FieldInfoData {
                name: "npc_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_class),
            },
            FieldInfoData {
                name: "npc_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_dir),
            },
            FieldInfoData {
                name: "npc_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_veh_id),
            },
            FieldInfoData {
                name: "npc_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_team_id),
            },
            FieldInfoData {
                name: "npc_loadout",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, npc_loadout),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinNpcSpawnEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINNPCSPAWNEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinNpcSpawnEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINNPCSPAWNEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINNPCSPAWNEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinNpcSpawnEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinNpcSpawnEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerSpawnEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerSpawnEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_dir(&self) -> &super::core::Vec3;
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn veh_id(&self) -> &String;
    fn veh_id_mut(&mut self) -> &mut String;
    fn veh_type(&self) -> &String;
    fn veh_type_mut(&mut self) -> &mut String;
    fn p_class(&self) -> &String;
    fn p_class_mut(&mut self) -> &mut String;
    fn team_id(&self) -> &String;
    fn team_id_mut(&mut self) -> &mut String;
    fn resources(&self) -> &RawJsonString;
    fn resources_mut(&mut self) -> &mut RawJsonString;
    fn resources_spent(&self) -> &RawJsonString;
    fn resources_spent_mut(&mut self) -> &mut RawJsonString;
    fn resource_items(&self) -> &RawJsonString;
    fn resource_items_mut(&mut self) -> &mut RawJsonString;
    fn loadout(&self) -> &RawJsonString;
    fn loadout_mut(&mut self) -> &mut RawJsonString;
    fn reason(&self) -> &String;
    fn reason_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinPlayerSpawnEventTrait for TelemetrySdkPinPlayerSpawnEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_dir(&self) -> &super::core::Vec3 {
        &self.p_dir
    }
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_dir
    }
    fn veh_id(&self) -> &String {
        &self.veh_id
    }
    fn veh_id_mut(&mut self) -> &mut String {
        &mut self.veh_id
    }
    fn veh_type(&self) -> &String {
        &self.veh_type
    }
    fn veh_type_mut(&mut self) -> &mut String {
        &mut self.veh_type
    }
    fn p_class(&self) -> &String {
        &self.p_class
    }
    fn p_class_mut(&mut self) -> &mut String {
        &mut self.p_class
    }
    fn team_id(&self) -> &String {
        &self.team_id
    }
    fn team_id_mut(&mut self) -> &mut String {
        &mut self.team_id
    }
    fn resources(&self) -> &RawJsonString {
        &self.resources
    }
    fn resources_mut(&mut self) -> &mut RawJsonString {
        &mut self.resources
    }
    fn resources_spent(&self) -> &RawJsonString {
        &self.resources_spent
    }
    fn resources_spent_mut(&mut self) -> &mut RawJsonString {
        &mut self.resources_spent
    }
    fn resource_items(&self) -> &RawJsonString {
        &self.resource_items
    }
    fn resource_items_mut(&mut self) -> &mut RawJsonString {
        &mut self.resource_items
    }
    fn loadout(&self) -> &RawJsonString {
        &self.loadout
    }
    fn loadout_mut(&mut self) -> &mut RawJsonString {
        &mut self.loadout
    }
    fn reason(&self) -> &String {
        &self.reason
    }
    fn reason_mut(&mut self) -> &mut String {
        &mut self.reason
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerSpawnEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerSpawnEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerSpawnEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerSpawnEvent {
}

pub static TELEMETRYSDKPINPLAYERSPAWNEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerSpawnEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerSpawnEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, r#type),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, rdur),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, p_dir),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, veh_id),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, veh_type),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, p_class),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, team_id),
            },
            FieldInfoData {
                name: "resources",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, resources),
            },
            FieldInfoData {
                name: "resources_spent",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, resources_spent),
            },
            FieldInfoData {
                name: "resource_items",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, resource_items),
            },
            FieldInfoData {
                name: "loadout",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, loadout),
            },
            FieldInfoData {
                name: "reason",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, reason),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinPlayerSpawnEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERSPAWNEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerSpawnEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERSPAWNEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERSPAWNEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerSpawnEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerSpawnEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerTickEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerTickEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_char(&self) -> &String;
    fn p_char_mut(&mut self) -> &mut String;
    fn p_class(&self) -> &String;
    fn p_class_mut(&mut self) -> &mut String;
    fn p_dir(&self) -> &super::core::Vec3;
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn cam_dir(&self) -> &super::core::Vec3;
    fn cam_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn p_state(&self) -> &String;
    fn p_state_mut(&mut self) -> &mut String;
    fn veh_id(&self) -> &String;
    fn veh_id_mut(&mut self) -> &mut String;
    fn veh_type(&self) -> &String;
    fn veh_type_mut(&mut self) -> &mut String;
    fn veh_state(&self) -> &String;
    fn veh_state_mut(&mut self) -> &mut String;
    fn item_id(&self) -> &String;
    fn item_id_mut(&mut self) -> &mut String;
    fn item_category(&self) -> &String;
    fn item_category_mut(&mut self) -> &mut String;
    fn item_type(&self) -> &String;
    fn item_type_mut(&mut self) -> &mut String;
    fn item_name(&self) -> &String;
    fn item_name_mut(&mut self) -> &mut String;
    fn is_ads(&self) -> &bool;
    fn is_ads_mut(&mut self) -> &mut bool;
    fn p_team_id(&self) -> &String;
    fn p_team_id_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl TelemetrySdkPinPlayerTickEventTrait for TelemetrySdkPinPlayerTickEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_char(&self) -> &String {
        &self.p_char
    }
    fn p_char_mut(&mut self) -> &mut String {
        &mut self.p_char
    }
    fn p_class(&self) -> &String {
        &self.p_class
    }
    fn p_class_mut(&mut self) -> &mut String {
        &mut self.p_class
    }
    fn p_dir(&self) -> &super::core::Vec3 {
        &self.p_dir
    }
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_dir
    }
    fn cam_dir(&self) -> &super::core::Vec3 {
        &self.cam_dir
    }
    fn cam_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.cam_dir
    }
    fn p_state(&self) -> &String {
        &self.p_state
    }
    fn p_state_mut(&mut self) -> &mut String {
        &mut self.p_state
    }
    fn veh_id(&self) -> &String {
        &self.veh_id
    }
    fn veh_id_mut(&mut self) -> &mut String {
        &mut self.veh_id
    }
    fn veh_type(&self) -> &String {
        &self.veh_type
    }
    fn veh_type_mut(&mut self) -> &mut String {
        &mut self.veh_type
    }
    fn veh_state(&self) -> &String {
        &self.veh_state
    }
    fn veh_state_mut(&mut self) -> &mut String {
        &mut self.veh_state
    }
    fn item_id(&self) -> &String {
        &self.item_id
    }
    fn item_id_mut(&mut self) -> &mut String {
        &mut self.item_id
    }
    fn item_category(&self) -> &String {
        &self.item_category
    }
    fn item_category_mut(&mut self) -> &mut String {
        &mut self.item_category
    }
    fn item_type(&self) -> &String {
        &self.item_type
    }
    fn item_type_mut(&mut self) -> &mut String {
        &mut self.item_type
    }
    fn item_name(&self) -> &String {
        &self.item_name
    }
    fn item_name_mut(&mut self) -> &mut String {
        &mut self.item_name
    }
    fn is_ads(&self) -> &bool {
        &self.is_ads
    }
    fn is_ads_mut(&mut self) -> &mut bool {
        &mut self.is_ads
    }
    fn p_team_id(&self) -> &String {
        &self.p_team_id
    }
    fn p_team_id_mut(&mut self) -> &mut String {
        &mut self.p_team_id
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerTickEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerTickEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerTickEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerTickEvent {
}

pub static TELEMETRYSDKPINPLAYERTICKEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerTickEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerTickEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, rdur),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_loc),
            },
            FieldInfoData {
                name: "p_char",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_char),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_class),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_dir),
            },
            FieldInfoData {
                name: "cam_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, cam_dir),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_state),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, veh_id),
            },
            FieldInfoData {
                name: "veh_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, veh_type),
            },
            FieldInfoData {
                name: "veh_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, veh_state),
            },
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, item_id),
            },
            FieldInfoData {
                name: "item_category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, item_category),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, item_type),
            },
            FieldInfoData {
                name: "item_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, item_name),
            },
            FieldInfoData {
                name: "is_ads",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, is_ads),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, p_team_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerTickEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERTICKEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerTickEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERTICKEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERTICKEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerTickEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerTickEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinTransctionEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinTransctionEventTrait: TelemetrySDKPinEventTrait {
    fn code(&self) -> &String;
    fn code_mut(&mut self) -> &mut String;
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn source(&self) -> &String;
    fn source_mut(&mut self) -> &mut String;
    fn revenue_model(&self) -> &String;
    fn revenue_model_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn party1id(&self) -> &RawJsonString;
    fn party1id_mut(&mut self) -> &mut RawJsonString;
    fn party2id(&self) -> &RawJsonString;
    fn party2id_mut(&mut self) -> &mut RawJsonString;
    fn asset_out(&self) -> &RawJsonString;
    fn asset_out_mut(&mut self) -> &mut RawJsonString;
    fn asset_in(&self) -> &RawJsonString;
    fn asset_in_mut(&mut self) -> &mut RawJsonString;
    fn bal1(&self) -> &RawJsonString;
    fn bal1_mut(&mut self) -> &mut RawJsonString;
    fn bal2(&self) -> &RawJsonString;
    fn bal2_mut(&mut self) -> &mut RawJsonString;
    fn _meta(&self) -> &RawJsonString;
    fn _meta_mut(&mut self) -> &mut RawJsonString;
    fn txid(&self) -> &String;
    fn txid_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinTransctionEventTrait for TelemetrySdkPinTransctionEvent {
    fn code(&self) -> &String {
        &self.code
    }
    fn code_mut(&mut self) -> &mut String {
        &mut self.code
    }
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn source(&self) -> &String {
        &self.source
    }
    fn source_mut(&mut self) -> &mut String {
        &mut self.source
    }
    fn revenue_model(&self) -> &String {
        &self.revenue_model
    }
    fn revenue_model_mut(&mut self) -> &mut String {
        &mut self.revenue_model
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn party1id(&self) -> &RawJsonString {
        &self.party1id
    }
    fn party1id_mut(&mut self) -> &mut RawJsonString {
        &mut self.party1id
    }
    fn party2id(&self) -> &RawJsonString {
        &self.party2id
    }
    fn party2id_mut(&mut self) -> &mut RawJsonString {
        &mut self.party2id
    }
    fn asset_out(&self) -> &RawJsonString {
        &self.asset_out
    }
    fn asset_out_mut(&mut self) -> &mut RawJsonString {
        &mut self.asset_out
    }
    fn asset_in(&self) -> &RawJsonString {
        &self.asset_in
    }
    fn asset_in_mut(&mut self) -> &mut RawJsonString {
        &mut self.asset_in
    }
    fn bal1(&self) -> &RawJsonString {
        &self.bal1
    }
    fn bal1_mut(&mut self) -> &mut RawJsonString {
        &mut self.bal1
    }
    fn bal2(&self) -> &RawJsonString {
        &self.bal2
    }
    fn bal2_mut(&mut self) -> &mut RawJsonString {
        &mut self.bal2
    }
    fn _meta(&self) -> &RawJsonString {
        &self._meta
    }
    fn _meta_mut(&mut self) -> &mut RawJsonString {
        &mut self._meta
    }
    fn txid(&self) -> &String {
        &self.txid
    }
    fn txid_mut(&mut self) -> &mut String {
        &mut self.txid
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinTransctionEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinTransctionEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinTransctionEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinTransctionEvent {
}

pub static TELEMETRYSDKPINTRANSCTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTransctionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinTransctionEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, code),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, r#type),
            },
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, source),
            },
            FieldInfoData {
                name: "revenue_model",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, revenue_model),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, status_code),
            },
            FieldInfoData {
                name: "party1id",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, party1id),
            },
            FieldInfoData {
                name: "party2id",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, party2id),
            },
            FieldInfoData {
                name: "asset_out",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, asset_out),
            },
            FieldInfoData {
                name: "asset_in",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, asset_in),
            },
            FieldInfoData {
                name: "bal1",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, bal1),
            },
            FieldInfoData {
                name: "bal2",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, bal2),
            },
            FieldInfoData {
                name: "_meta",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, _meta),
            },
            FieldInfoData {
                name: "txid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, txid),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinTransctionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINTRANSCTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinTransctionEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINTRANSCTIONEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINTRANSCTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTransctionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinTransctionEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinInventoryEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub playerid: RawJsonString,
    pub assets: RawJsonString,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinInventoryEventTrait: TelemetrySDKPinEventTrait {
    fn playerid(&self) -> &RawJsonString;
    fn playerid_mut(&mut self) -> &mut RawJsonString;
    fn assets(&self) -> &RawJsonString;
    fn assets_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinInventoryEventTrait for TelemetrySdkPinInventoryEvent {
    fn playerid(&self) -> &RawJsonString {
        &self.playerid
    }
    fn playerid_mut(&mut self) -> &mut RawJsonString {
        &mut self.playerid
    }
    fn assets(&self) -> &RawJsonString {
        &self.assets
    }
    fn assets_mut(&mut self) -> &mut RawJsonString {
        &mut self.assets
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinInventoryEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinInventoryEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinInventoryEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinInventoryEvent {
}

pub static TELEMETRYSDKPININVENTORYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinInventoryEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinInventoryEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "playerid",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinInventoryEvent, playerid),
            },
            FieldInfoData {
                name: "assets",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinInventoryEvent, assets),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinInventoryEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPININVENTORYEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinInventoryEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPININVENTORYEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPININVENTORYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinInventoryEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinInventoryEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinSurveyEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinSurveyEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn survey_id(&self) -> &String;
    fn survey_id_mut(&mut self) -> &mut String;
    fn wave_no(&self) -> &i32;
    fn wave_no_mut(&mut self) -> &mut i32;
    fn lang(&self) -> &String;
    fn lang_mut(&mut self) -> &mut String;
    fn complete_flag(&self) -> &String;
    fn complete_flag_mut(&mut self) -> &mut String;
    fn qtime(&self) -> &i64;
    fn qtime_mut(&mut self) -> &mut i64;
    fn j_s_o_n(&self) -> &RawJsonString;
    fn j_s_o_n_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinSurveyEventTrait for TelemetrySdkPinSurveyEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn survey_id(&self) -> &String {
        &self.survey_id
    }
    fn survey_id_mut(&mut self) -> &mut String {
        &mut self.survey_id
    }
    fn wave_no(&self) -> &i32 {
        &self.wave_no
    }
    fn wave_no_mut(&mut self) -> &mut i32 {
        &mut self.wave_no
    }
    fn lang(&self) -> &String {
        &self.lang
    }
    fn lang_mut(&mut self) -> &mut String {
        &mut self.lang
    }
    fn complete_flag(&self) -> &String {
        &self.complete_flag
    }
    fn complete_flag_mut(&mut self) -> &mut String {
        &mut self.complete_flag
    }
    fn qtime(&self) -> &i64 {
        &self.qtime
    }
    fn qtime_mut(&mut self) -> &mut i64 {
        &mut self.qtime
    }
    fn j_s_o_n(&self) -> &RawJsonString {
        &self.j_s_o_n
    }
    fn j_s_o_n_mut(&mut self) -> &mut RawJsonString {
        &mut self.j_s_o_n
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinSurveyEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinSurveyEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinSurveyEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinSurveyEvent {
}

pub static TELEMETRYSDKPINSURVEYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSurveyEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinSurveyEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, r#type),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, status_code),
            },
            FieldInfoData {
                name: "survey_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, survey_id),
            },
            FieldInfoData {
                name: "wave_no",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, wave_no),
            },
            FieldInfoData {
                name: "lang",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, lang),
            },
            FieldInfoData {
                name: "complete_flag",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, complete_flag),
            },
            FieldInfoData {
                name: "qtime",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, qtime),
            },
            FieldInfoData {
                name: "JSON",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, j_s_o_n),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinSurveyEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINSURVEYEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinSurveyEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINSURVEYEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINSURVEYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSurveyEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinSurveyEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinHeartBeatEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub sdur: u32,
    pub gdur: u32,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinHeartBeatEventTrait: TelemetrySDKPinEventTrait {
    fn sdur(&self) -> &u32;
    fn sdur_mut(&mut self) -> &mut u32;
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinHeartBeatEventTrait for TelemetrySdkPinHeartBeatEvent {
    fn sdur(&self) -> &u32 {
        &self.sdur
    }
    fn sdur_mut(&mut self) -> &mut u32 {
        &mut self.sdur
    }
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinHeartBeatEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinHeartBeatEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinHeartBeatEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinHeartBeatEvent {
}

pub static TELEMETRYSDKPINHEARTBEATEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinHeartBeatEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinHeartBeatEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinHeartBeatEvent, sdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinHeartBeatEvent, gdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinHeartBeatEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINHEARTBEATEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinHeartBeatEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINHEARTBEATEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINHEARTBEATEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinHeartBeatEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinHeartBeatEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinModeExitEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub instance_id: String,
    pub leave_reason: String,
    pub mdur: i64,
    pub cdur: i64,
    pub sdur: i64,
    pub gdur: i64,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinModeExitEventTrait: TelemetrySDKPinEventTrait {
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn leave_reason(&self) -> &String;
    fn leave_reason_mut(&mut self) -> &mut String;
    fn mdur(&self) -> &i64;
    fn mdur_mut(&mut self) -> &mut i64;
    fn cdur(&self) -> &i64;
    fn cdur_mut(&mut self) -> &mut i64;
    fn sdur(&self) -> &i64;
    fn sdur_mut(&mut self) -> &mut i64;
    fn gdur(&self) -> &i64;
    fn gdur_mut(&mut self) -> &mut i64;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinModeExitEventTrait for TelemetrySdkPinModeExitEvent {
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn leave_reason(&self) -> &String {
        &self.leave_reason
    }
    fn leave_reason_mut(&mut self) -> &mut String {
        &mut self.leave_reason
    }
    fn mdur(&self) -> &i64 {
        &self.mdur
    }
    fn mdur_mut(&mut self) -> &mut i64 {
        &mut self.mdur
    }
    fn cdur(&self) -> &i64 {
        &self.cdur
    }
    fn cdur_mut(&mut self) -> &mut i64 {
        &mut self.cdur
    }
    fn sdur(&self) -> &i64 {
        &self.sdur
    }
    fn sdur_mut(&mut self) -> &mut i64 {
        &mut self.sdur
    }
    fn gdur(&self) -> &i64 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut i64 {
        &mut self.gdur
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinModeExitEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinModeExitEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinModeExitEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinModeExitEvent {
}

pub static TELEMETRYSDKPINMODEEXITEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinModeExitEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinModeExitEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, instance_id),
            },
            FieldInfoData {
                name: "leave_reason",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, leave_reason),
            },
            FieldInfoData {
                name: "mdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, mdur),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, cdur),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, sdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, gdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinModeExitEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMODEEXITEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinModeExitEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINMODEEXITEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINMODEEXITEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinModeExitEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinModeExitEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinModeEnterEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub instance_id: String,
    pub status: String,
    pub status_code: String,
    pub is_first: bool,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinModeEnterEventTrait: TelemetrySDKPinEventTrait {
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn is_first(&self) -> &bool;
    fn is_first_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinModeEnterEventTrait for TelemetrySdkPinModeEnterEvent {
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn is_first(&self) -> &bool {
        &self.is_first
    }
    fn is_first_mut(&mut self) -> &mut bool {
        &mut self.is_first
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinModeEnterEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinModeEnterEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinModeEnterEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinModeEnterEvent {
}

pub static TELEMETRYSDKPINMODEENTEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinModeEnterEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinModeEnterEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinModeEnterEvent, instance_id),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinModeEnterEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinModeEnterEvent, status_code),
            },
            FieldInfoData {
                name: "is_first",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySdkPinModeEnterEvent, is_first),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinModeEnterEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMODEENTEREVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinModeEnterEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINMODEENTEREVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINMODEENTEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinModeEnterEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinModeEnterEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlaySessionEndEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlaySessionEndEventTrait: TelemetrySDKPinEventTrait {
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn sdur(&self) -> &i64;
    fn sdur_mut(&mut self) -> &mut i64;
    fn gdur(&self) -> &i64;
    fn gdur_mut(&mut self) -> &mut i64;
    fn diff(&self) -> &String;
    fn diff_mut(&mut self) -> &mut String;
    fn goid(&self) -> &String;
    fn goid_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn end_reason(&self) -> &String;
    fn end_reason_mut(&mut self) -> &mut String;
    fn player_stats(&self) -> &RawJsonString;
    fn player_stats_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinPlaySessionEndEventTrait for TelemetrySdkPinPlaySessionEndEvent {
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn sdur(&self) -> &i64 {
        &self.sdur
    }
    fn sdur_mut(&mut self) -> &mut i64 {
        &mut self.sdur
    }
    fn gdur(&self) -> &i64 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut i64 {
        &mut self.gdur
    }
    fn diff(&self) -> &String {
        &self.diff
    }
    fn diff_mut(&mut self) -> &mut String {
        &mut self.diff
    }
    fn goid(&self) -> &String {
        &self.goid
    }
    fn goid_mut(&mut self) -> &mut String {
        &mut self.goid
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn end_reason(&self) -> &String {
        &self.end_reason
    }
    fn end_reason_mut(&mut self) -> &mut String {
        &mut self.end_reason
    }
    fn player_stats(&self) -> &RawJsonString {
        &self.player_stats
    }
    fn player_stats_mut(&mut self) -> &mut RawJsonString {
        &mut self.player_stats
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlaySessionEndEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlaySessionEndEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlaySessionEndEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlaySessionEndEvent {
}

pub static TELEMETRYSDKPINPLAYSESSIONENDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlaySessionEndEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlaySessionEndEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, instance_id),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, sdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, gdur),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, diff),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, goid),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, status_code),
            },
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, end_reason),
            },
            FieldInfoData {
                name: "player_stats",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, player_stats),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionEndEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYSESSIONENDEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlaySessionEndEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYSESSIONENDEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYSESSIONENDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlaySessionEndEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlaySessionEndEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlaySessionStartEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub instance_id: String,
    pub diff: String,
    pub goid: String,
    pub status: String,
    pub status_code: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinPlaySessionStartEventTrait: TelemetrySDKPinEventTrait {
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn diff(&self) -> &String;
    fn diff_mut(&mut self) -> &mut String;
    fn goid(&self) -> &String;
    fn goid_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinPlaySessionStartEventTrait for TelemetrySdkPinPlaySessionStartEvent {
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn diff(&self) -> &String {
        &self.diff
    }
    fn diff_mut(&mut self) -> &mut String {
        &mut self.diff
    }
    fn goid(&self) -> &String {
        &self.goid
    }
    fn goid_mut(&mut self) -> &mut String {
        &mut self.goid
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlaySessionStartEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlaySessionStartEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlaySessionStartEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlaySessionStartEvent {
}

pub static TELEMETRYSDKPINPLAYSESSIONSTARTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlaySessionStartEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlaySessionStartEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, instance_id),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, diff),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, goid),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinPlaySessionStartEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYSESSIONSTARTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlaySessionStartEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYSESSIONSTARTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYSESSIONSTARTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlaySessionStartEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlaySessionStartEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinRoundEndEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinRoundEndEventTrait: TelemetrySDKPinEventTrait {
    fn round(&self) -> &u32;
    fn round_mut(&mut self) -> &mut u32;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn gdur(&self) -> &i64;
    fn gdur_mut(&mut self) -> &mut i64;
    fn rdur(&self) -> &i64;
    fn rdur_mut(&mut self) -> &mut i64;
    fn diff(&self) -> &String;
    fn diff_mut(&mut self) -> &mut String;
    fn mid(&self) -> &String;
    fn mid_mut(&mut self) -> &mut String;
    fn goid(&self) -> &String;
    fn goid_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn end_reason(&self) -> &String;
    fn end_reason_mut(&mut self) -> &mut String;
    fn player_cnt(&self) -> &u32;
    fn player_cnt_mut(&mut self) -> &mut u32;
    fn max_players(&self) -> &u32;
    fn max_players_mut(&mut self) -> &mut u32;
    fn num_teams(&self) -> &u32;
    fn num_teams_mut(&mut self) -> &mut u32;
    fn teams_stats(&self) -> &RawJsonString;
    fn teams_stats_mut(&mut self) -> &mut RawJsonString;
    fn player_stats(&self) -> &RawJsonString;
    fn player_stats_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinRoundEndEventTrait for TelemetrySdkPinRoundEndEvent {
    fn round(&self) -> &u32 {
        &self.round
    }
    fn round_mut(&mut self) -> &mut u32 {
        &mut self.round
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn gdur(&self) -> &i64 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut i64 {
        &mut self.gdur
    }
    fn rdur(&self) -> &i64 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut i64 {
        &mut self.rdur
    }
    fn diff(&self) -> &String {
        &self.diff
    }
    fn diff_mut(&mut self) -> &mut String {
        &mut self.diff
    }
    fn mid(&self) -> &String {
        &self.mid
    }
    fn mid_mut(&mut self) -> &mut String {
        &mut self.mid
    }
    fn goid(&self) -> &String {
        &self.goid
    }
    fn goid_mut(&mut self) -> &mut String {
        &mut self.goid
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn end_reason(&self) -> &String {
        &self.end_reason
    }
    fn end_reason_mut(&mut self) -> &mut String {
        &mut self.end_reason
    }
    fn player_cnt(&self) -> &u32 {
        &self.player_cnt
    }
    fn player_cnt_mut(&mut self) -> &mut u32 {
        &mut self.player_cnt
    }
    fn max_players(&self) -> &u32 {
        &self.max_players
    }
    fn max_players_mut(&mut self) -> &mut u32 {
        &mut self.max_players
    }
    fn num_teams(&self) -> &u32 {
        &self.num_teams
    }
    fn num_teams_mut(&mut self) -> &mut u32 {
        &mut self.num_teams
    }
    fn teams_stats(&self) -> &RawJsonString {
        &self.teams_stats
    }
    fn teams_stats_mut(&mut self) -> &mut RawJsonString {
        &mut self.teams_stats
    }
    fn player_stats(&self) -> &RawJsonString {
        &self.player_stats
    }
    fn player_stats_mut(&mut self) -> &mut RawJsonString {
        &mut self.player_stats
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinRoundEndEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinRoundEndEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinRoundEndEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinRoundEndEvent {
}

pub static TELEMETRYSDKPINROUNDENDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRoundEndEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinRoundEndEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "round",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, round),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, instance_id),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, rdur),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, diff),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, mid),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, goid),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, status_code),
            },
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, end_reason),
            },
            FieldInfoData {
                name: "player_cnt",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, player_cnt),
            },
            FieldInfoData {
                name: "max_players",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, max_players),
            },
            FieldInfoData {
                name: "num_teams",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, num_teams),
            },
            FieldInfoData {
                name: "teams_stats",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, teams_stats),
            },
            FieldInfoData {
                name: "player_stats",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, player_stats),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinRoundEndEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINROUNDENDEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinRoundEndEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINROUNDENDEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINROUNDENDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRoundEndEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinRoundEndEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinRoundStartEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinRoundStartEventTrait: TelemetrySDKPinEventTrait {
    fn round(&self) -> &u32;
    fn round_mut(&mut self) -> &mut u32;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn diff(&self) -> &String;
    fn diff_mut(&mut self) -> &mut String;
    fn mid(&self) -> &String;
    fn mid_mut(&mut self) -> &mut String;
    fn goid(&self) -> &String;
    fn goid_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn player_cnt(&self) -> &u32;
    fn player_cnt_mut(&mut self) -> &mut u32;
    fn max_players(&self) -> &u32;
    fn max_players_mut(&mut self) -> &mut u32;
    fn num_teams(&self) -> &u32;
    fn num_teams_mut(&mut self) -> &mut u32;
    fn team_id(&self) -> &String;
    fn team_id_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinRoundStartEventTrait for TelemetrySdkPinRoundStartEvent {
    fn round(&self) -> &u32 {
        &self.round
    }
    fn round_mut(&mut self) -> &mut u32 {
        &mut self.round
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn diff(&self) -> &String {
        &self.diff
    }
    fn diff_mut(&mut self) -> &mut String {
        &mut self.diff
    }
    fn mid(&self) -> &String {
        &self.mid
    }
    fn mid_mut(&mut self) -> &mut String {
        &mut self.mid
    }
    fn goid(&self) -> &String {
        &self.goid
    }
    fn goid_mut(&mut self) -> &mut String {
        &mut self.goid
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn player_cnt(&self) -> &u32 {
        &self.player_cnt
    }
    fn player_cnt_mut(&mut self) -> &mut u32 {
        &mut self.player_cnt
    }
    fn max_players(&self) -> &u32 {
        &self.max_players
    }
    fn max_players_mut(&mut self) -> &mut u32 {
        &mut self.max_players
    }
    fn num_teams(&self) -> &u32 {
        &self.num_teams
    }
    fn num_teams_mut(&mut self) -> &mut u32 {
        &mut self.num_teams
    }
    fn team_id(&self) -> &String {
        &self.team_id
    }
    fn team_id_mut(&mut self) -> &mut String {
        &mut self.team_id
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinRoundStartEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinRoundStartEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinRoundStartEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinRoundStartEvent {
}

pub static TELEMETRYSDKPINROUNDSTARTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRoundStartEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinRoundStartEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "round",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, round),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, instance_id),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, diff),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, mid),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, goid),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, status_code),
            },
            FieldInfoData {
                name: "player_cnt",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, player_cnt),
            },
            FieldInfoData {
                name: "max_players",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, max_players),
            },
            FieldInfoData {
                name: "num_teams",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, num_teams),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, team_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinRoundStartEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINROUNDSTARTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinRoundStartEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINROUNDSTARTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINROUNDSTARTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRoundStartEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinRoundStartEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinGameEndEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinGameEndEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn mode(&self) -> &String;
    fn mode_mut(&mut self) -> &mut String;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn mid(&self) -> &String;
    fn mid_mut(&mut self) -> &mut String;
    fn goid(&self) -> &String;
    fn goid_mut(&mut self) -> &mut String;
    fn end_reason(&self) -> &String;
    fn end_reason_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn sdur(&self) -> &i64;
    fn sdur_mut(&mut self) -> &mut i64;
    fn gdur(&self) -> &i64;
    fn gdur_mut(&mut self) -> &mut i64;
    fn cdur(&self) -> &i64;
    fn cdur_mut(&mut self) -> &mut i64;
    fn player_stats(&self) -> &RawJsonString;
    fn player_stats_mut(&mut self) -> &mut RawJsonString;
    fn teams_stats(&self) -> &RawJsonString;
    fn teams_stats_mut(&mut self) -> &mut RawJsonString;
    fn constraint_status(&self) -> &RawJsonString;
    fn constraint_status_mut(&mut self) -> &mut RawJsonString;
    fn mission_status(&self) -> &RawJsonString;
    fn mission_status_mut(&mut self) -> &mut RawJsonString;
    fn items_initialized(&self) -> &RawJsonString;
    fn items_initialized_mut(&mut self) -> &mut RawJsonString;
    fn achievement(&self) -> &RawJsonString;
    fn achievement_mut(&mut self) -> &mut RawJsonString;
    fn asset_gained(&self) -> &RawJsonString;
    fn asset_gained_mut(&mut self) -> &mut RawJsonString;
    fn asset_used(&self) -> &RawJsonString;
    fn asset_used_mut(&mut self) -> &mut RawJsonString;
    fn asset_balance(&self) -> &RawJsonString;
    fn asset_balance_mut(&mut self) -> &mut RawJsonString;
    fn weapon_stats(&self) -> &RawJsonString;
    fn weapon_stats_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl TelemetrySdkPinGameEndEventTrait for TelemetrySdkPinGameEndEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn mode(&self) -> &String {
        &self.mode
    }
    fn mode_mut(&mut self) -> &mut String {
        &mut self.mode
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn mid(&self) -> &String {
        &self.mid
    }
    fn mid_mut(&mut self) -> &mut String {
        &mut self.mid
    }
    fn goid(&self) -> &String {
        &self.goid
    }
    fn goid_mut(&mut self) -> &mut String {
        &mut self.goid
    }
    fn end_reason(&self) -> &String {
        &self.end_reason
    }
    fn end_reason_mut(&mut self) -> &mut String {
        &mut self.end_reason
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn sdur(&self) -> &i64 {
        &self.sdur
    }
    fn sdur_mut(&mut self) -> &mut i64 {
        &mut self.sdur
    }
    fn gdur(&self) -> &i64 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut i64 {
        &mut self.gdur
    }
    fn cdur(&self) -> &i64 {
        &self.cdur
    }
    fn cdur_mut(&mut self) -> &mut i64 {
        &mut self.cdur
    }
    fn player_stats(&self) -> &RawJsonString {
        &self.player_stats
    }
    fn player_stats_mut(&mut self) -> &mut RawJsonString {
        &mut self.player_stats
    }
    fn teams_stats(&self) -> &RawJsonString {
        &self.teams_stats
    }
    fn teams_stats_mut(&mut self) -> &mut RawJsonString {
        &mut self.teams_stats
    }
    fn constraint_status(&self) -> &RawJsonString {
        &self.constraint_status
    }
    fn constraint_status_mut(&mut self) -> &mut RawJsonString {
        &mut self.constraint_status
    }
    fn mission_status(&self) -> &RawJsonString {
        &self.mission_status
    }
    fn mission_status_mut(&mut self) -> &mut RawJsonString {
        &mut self.mission_status
    }
    fn items_initialized(&self) -> &RawJsonString {
        &self.items_initialized
    }
    fn items_initialized_mut(&mut self) -> &mut RawJsonString {
        &mut self.items_initialized
    }
    fn achievement(&self) -> &RawJsonString {
        &self.achievement
    }
    fn achievement_mut(&mut self) -> &mut RawJsonString {
        &mut self.achievement
    }
    fn asset_gained(&self) -> &RawJsonString {
        &self.asset_gained
    }
    fn asset_gained_mut(&mut self) -> &mut RawJsonString {
        &mut self.asset_gained
    }
    fn asset_used(&self) -> &RawJsonString {
        &self.asset_used
    }
    fn asset_used_mut(&mut self) -> &mut RawJsonString {
        &mut self.asset_used
    }
    fn asset_balance(&self) -> &RawJsonString {
        &self.asset_balance
    }
    fn asset_balance_mut(&mut self) -> &mut RawJsonString {
        &mut self.asset_balance
    }
    fn weapon_stats(&self) -> &RawJsonString {
        &self.weapon_stats
    }
    fn weapon_stats_mut(&mut self) -> &mut RawJsonString {
        &mut self.weapon_stats
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinGameEndEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinGameEndEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinGameEndEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinGameEndEvent {
}

pub static TELEMETRYSDKPINGAMEENDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameEndEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinGameEndEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, r#type),
            },
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, mode),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, instance_id),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, mid),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, goid),
            },
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, end_reason),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, status_code),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, sdur),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, gdur),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, cdur),
            },
            FieldInfoData {
                name: "player_stats",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, player_stats),
            },
            FieldInfoData {
                name: "teams_stats",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, teams_stats),
            },
            FieldInfoData {
                name: "constraint_status",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, constraint_status),
            },
            FieldInfoData {
                name: "mission_status",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, mission_status),
            },
            FieldInfoData {
                name: "items_initialized",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, items_initialized),
            },
            FieldInfoData {
                name: "achievement",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, achievement),
            },
            FieldInfoData {
                name: "asset_gained",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, asset_gained),
            },
            FieldInfoData {
                name: "asset_used",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, asset_used),
            },
            FieldInfoData {
                name: "asset_balance",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, asset_balance),
            },
            FieldInfoData {
                name: "weapon_stats",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, weapon_stats),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinGameEndEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINGAMEENDEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinGameEndEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINGAMEENDEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINGAMEENDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameEndEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinGameEndEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinGameStartEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinGameStartEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn mode(&self) -> &String;
    fn mode_mut(&mut self) -> &mut String;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn ldur(&self) -> &u32;
    fn ldur_mut(&mut self) -> &mut u32;
    fn diff(&self) -> &String;
    fn diff_mut(&mut self) -> &mut String;
    fn goid(&self) -> &String;
    fn goid_mut(&mut self) -> &mut String;
    fn mid(&self) -> &String;
    fn mid_mut(&mut self) -> &mut String;
    fn rond(&self) -> &String;
    fn rond_mut(&mut self) -> &mut String;
    fn map(&self) -> &String;
    fn map_mut(&mut self) -> &mut String;
    fn _char(&self) -> &String;
    fn _char_mut(&mut self) -> &mut String;
    fn gend(&self) -> &String;
    fn gend_mut(&mut self) -> &mut String;
    fn _class(&self) -> &String;
    fn _class_mut(&mut self) -> &mut String;
    fn attempt(&self) -> &u32;
    fn attempt_mut(&mut self) -> &mut u32;
    fn max_level(&self) -> &u32;
    fn max_level_mut(&mut self) -> &mut u32;
    fn level_modifier(&self) -> &String;
    fn level_modifier_mut(&mut self) -> &mut String;
    fn start_status(&self) -> &RawJsonString;
    fn start_status_mut(&mut self) -> &mut RawJsonString;
    fn level_constraints(&self) -> &RawJsonString;
    fn level_constraints_mut(&mut self) -> &mut RawJsonString;
    fn missions(&self) -> &RawJsonString;
    fn missions_mut(&mut self) -> &mut RawJsonString;
    fn other_specs(&self) -> &RawJsonString;
    fn other_specs_mut(&mut self) -> &mut RawJsonString;
    fn model_config(&self) -> &RawJsonString;
    fn model_config_mut(&mut self) -> &mut RawJsonString;
    fn knob_owner(&self) -> &RawJsonString;
    fn knob_owner_mut(&mut self) -> &mut RawJsonString;
    fn lives(&self) -> &i64;
    fn lives_mut(&mut self) -> &mut i64;
    fn new_life_reqs(&self) -> &RawJsonString;
    fn new_life_reqs_mut(&mut self) -> &mut RawJsonString;
    fn team_id(&self) -> &String;
    fn team_id_mut(&mut self) -> &mut String;
    fn asset_bal(&self) -> &RawJsonString;
    fn asset_bal_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl TelemetrySdkPinGameStartEventTrait for TelemetrySdkPinGameStartEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn mode(&self) -> &String {
        &self.mode
    }
    fn mode_mut(&mut self) -> &mut String {
        &mut self.mode
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn ldur(&self) -> &u32 {
        &self.ldur
    }
    fn ldur_mut(&mut self) -> &mut u32 {
        &mut self.ldur
    }
    fn diff(&self) -> &String {
        &self.diff
    }
    fn diff_mut(&mut self) -> &mut String {
        &mut self.diff
    }
    fn goid(&self) -> &String {
        &self.goid
    }
    fn goid_mut(&mut self) -> &mut String {
        &mut self.goid
    }
    fn mid(&self) -> &String {
        &self.mid
    }
    fn mid_mut(&mut self) -> &mut String {
        &mut self.mid
    }
    fn rond(&self) -> &String {
        &self.rond
    }
    fn rond_mut(&mut self) -> &mut String {
        &mut self.rond
    }
    fn map(&self) -> &String {
        &self.map
    }
    fn map_mut(&mut self) -> &mut String {
        &mut self.map
    }
    fn _char(&self) -> &String {
        &self._char
    }
    fn _char_mut(&mut self) -> &mut String {
        &mut self._char
    }
    fn gend(&self) -> &String {
        &self.gend
    }
    fn gend_mut(&mut self) -> &mut String {
        &mut self.gend
    }
    fn _class(&self) -> &String {
        &self._class
    }
    fn _class_mut(&mut self) -> &mut String {
        &mut self._class
    }
    fn attempt(&self) -> &u32 {
        &self.attempt
    }
    fn attempt_mut(&mut self) -> &mut u32 {
        &mut self.attempt
    }
    fn max_level(&self) -> &u32 {
        &self.max_level
    }
    fn max_level_mut(&mut self) -> &mut u32 {
        &mut self.max_level
    }
    fn level_modifier(&self) -> &String {
        &self.level_modifier
    }
    fn level_modifier_mut(&mut self) -> &mut String {
        &mut self.level_modifier
    }
    fn start_status(&self) -> &RawJsonString {
        &self.start_status
    }
    fn start_status_mut(&mut self) -> &mut RawJsonString {
        &mut self.start_status
    }
    fn level_constraints(&self) -> &RawJsonString {
        &self.level_constraints
    }
    fn level_constraints_mut(&mut self) -> &mut RawJsonString {
        &mut self.level_constraints
    }
    fn missions(&self) -> &RawJsonString {
        &self.missions
    }
    fn missions_mut(&mut self) -> &mut RawJsonString {
        &mut self.missions
    }
    fn other_specs(&self) -> &RawJsonString {
        &self.other_specs
    }
    fn other_specs_mut(&mut self) -> &mut RawJsonString {
        &mut self.other_specs
    }
    fn model_config(&self) -> &RawJsonString {
        &self.model_config
    }
    fn model_config_mut(&mut self) -> &mut RawJsonString {
        &mut self.model_config
    }
    fn knob_owner(&self) -> &RawJsonString {
        &self.knob_owner
    }
    fn knob_owner_mut(&mut self) -> &mut RawJsonString {
        &mut self.knob_owner
    }
    fn lives(&self) -> &i64 {
        &self.lives
    }
    fn lives_mut(&mut self) -> &mut i64 {
        &mut self.lives
    }
    fn new_life_reqs(&self) -> &RawJsonString {
        &self.new_life_reqs
    }
    fn new_life_reqs_mut(&mut self) -> &mut RawJsonString {
        &mut self.new_life_reqs
    }
    fn team_id(&self) -> &String {
        &self.team_id
    }
    fn team_id_mut(&mut self) -> &mut String {
        &mut self.team_id
    }
    fn asset_bal(&self) -> &RawJsonString {
        &self.asset_bal
    }
    fn asset_bal_mut(&mut self) -> &mut RawJsonString {
        &mut self.asset_bal
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinGameStartEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinGameStartEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinGameStartEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinGameStartEvent {
}

pub static TELEMETRYSDKPINGAMESTARTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameStartEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinGameStartEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, r#type),
            },
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, mode),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, instance_id),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, status_code),
            },
            FieldInfoData {
                name: "ldur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, ldur),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, diff),
            },
            FieldInfoData {
                name: "goid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, goid),
            },
            FieldInfoData {
                name: "mid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, mid),
            },
            FieldInfoData {
                name: "rond",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, rond),
            },
            FieldInfoData {
                name: "map",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, map),
            },
            FieldInfoData {
                name: "_char",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, _char),
            },
            FieldInfoData {
                name: "gend",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, gend),
            },
            FieldInfoData {
                name: "_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, _class),
            },
            FieldInfoData {
                name: "attempt",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, attempt),
            },
            FieldInfoData {
                name: "max_level",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, max_level),
            },
            FieldInfoData {
                name: "level_modifier",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, level_modifier),
            },
            FieldInfoData {
                name: "start_status",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, start_status),
            },
            FieldInfoData {
                name: "level_constraints",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, level_constraints),
            },
            FieldInfoData {
                name: "missions",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, missions),
            },
            FieldInfoData {
                name: "other_specs",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, other_specs),
            },
            FieldInfoData {
                name: "model_config",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, model_config),
            },
            FieldInfoData {
                name: "knob_owner",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, knob_owner),
            },
            FieldInfoData {
                name: "lives",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, lives),
            },
            FieldInfoData {
                name: "new_life_reqs",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, new_life_reqs),
            },
            FieldInfoData {
                name: "team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, team_id),
            },
            FieldInfoData {
                name: "asset_bal",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, asset_bal),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinGameStartEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINGAMESTARTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinGameStartEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINGAMESTARTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINGAMESTARTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinGameStartEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinGameStartEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinLogoutEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub r#type: String,
    pub end_reason: String,
    pub status_code: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinLogoutEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn end_reason(&self) -> &String;
    fn end_reason_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinLogoutEventTrait for TelemetrySdkPinLogoutEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn end_reason(&self) -> &String {
        &self.end_reason
    }
    fn end_reason_mut(&mut self) -> &mut String {
        &mut self.end_reason
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinLogoutEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinLogoutEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinLogoutEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinLogoutEvent {
}

pub static TELEMETRYSDKPINLOGOUTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinLogoutEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinLogoutEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinLogoutEvent, r#type),
            },
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinLogoutEvent, end_reason),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinLogoutEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinLogoutEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINLOGOUTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinLogoutEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINLOGOUTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINLOGOUTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinLogoutEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinLogoutEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinLoginEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub r#type: String,
    pub status: String,
    pub status_code: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinLoginEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinLoginEventTrait for TelemetrySdkPinLoginEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinLoginEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinLoginEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinLoginEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinLoginEvent {
}

pub static TELEMETRYSDKPINLOGINEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinLoginEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinLoginEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinLoginEvent, r#type),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinLoginEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinLoginEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinLoginEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINLOGINEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinLoginEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINLOGINEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINLOGINEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinLoginEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinLoginEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinBootEndEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub end_reason: String,
    pub status_code: String,
    pub sdur: i64,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinBootEndEventTrait: TelemetrySDKPinEventTrait {
    fn end_reason(&self) -> &String;
    fn end_reason_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn sdur(&self) -> &i64;
    fn sdur_mut(&mut self) -> &mut i64;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinBootEndEventTrait for TelemetrySdkPinBootEndEvent {
    fn end_reason(&self) -> &String {
        &self.end_reason
    }
    fn end_reason_mut(&mut self) -> &mut String {
        &mut self.end_reason
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn sdur(&self) -> &i64 {
        &self.sdur
    }
    fn sdur_mut(&mut self) -> &mut i64 {
        &mut self.sdur
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinBootEndEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinBootEndEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinBootEndEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinBootEndEvent {
}

pub static TELEMETRYSDKPINBOOTENDEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinBootEndEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinBootEndEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "end_reason",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinBootEndEvent, end_reason),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinBootEndEvent, status_code),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinBootEndEvent, sdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinBootEndEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINBOOTENDEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinBootEndEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINBOOTENDEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINBOOTENDEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinBootEndEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinBootEndEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinBootStartEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub status: String,
    pub source: String,
    pub status_code: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinBootStartEventTrait: TelemetrySDKPinEventTrait {
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn source(&self) -> &String;
    fn source_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinBootStartEventTrait for TelemetrySdkPinBootStartEvent {
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn source(&self) -> &String {
        &self.source
    }
    fn source_mut(&mut self) -> &mut String {
        &mut self.source
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinBootStartEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinBootStartEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinBootStartEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinBootStartEvent {
}

pub static TELEMETRYSDKPINBOOTSTARTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinBootStartEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinBootStartEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinBootStartEvent, status),
            },
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinBootStartEvent, source),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinBootStartEvent, status_code),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinBootStartEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINBOOTSTARTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinBootStartEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINBOOTSTARTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINBOOTSTARTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinBootStartEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinBootStartEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerServiceEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub service: String,
    pub action: String,
    pub metadata: RawJsonString,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinPlayerServiceEventTrait: TelemetrySDKPinEventTrait {
    fn service(&self) -> &String;
    fn service_mut(&mut self) -> &mut String;
    fn action(&self) -> &String;
    fn action_mut(&mut self) -> &mut String;
    fn metadata(&self) -> &RawJsonString;
    fn metadata_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinPlayerServiceEventTrait for TelemetrySdkPinPlayerServiceEvent {
    fn service(&self) -> &String {
        &self.service
    }
    fn service_mut(&mut self) -> &mut String {
        &mut self.service
    }
    fn action(&self) -> &String {
        &self.action
    }
    fn action_mut(&mut self) -> &mut String {
        &mut self.action
    }
    fn metadata(&self) -> &RawJsonString {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut RawJsonString {
        &mut self.metadata
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerServiceEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerServiceEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerServiceEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerServiceEvent {
}

pub static TELEMETRYSDKPINPLAYERSERVICEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerServiceEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerServiceEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "service",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerServiceEvent, service),
            },
            FieldInfoData {
                name: "action",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerServiceEvent, action),
            },
            FieldInfoData {
                name: "metadata",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerServiceEvent, metadata),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinPlayerServiceEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERSERVICEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerServiceEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERSERVICEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERSERVICEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerServiceEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerServiceEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerStatsEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub player_stats: RawJsonString,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinPlayerStatsEventTrait: TelemetrySDKPinEventTrait {
    fn player_stats(&self) -> &RawJsonString;
    fn player_stats_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinPlayerStatsEventTrait for TelemetrySdkPinPlayerStatsEvent {
    fn player_stats(&self) -> &RawJsonString {
        &self.player_stats
    }
    fn player_stats_mut(&mut self) -> &mut RawJsonString {
        &mut self.player_stats
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerStatsEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerStatsEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerStatsEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerStatsEvent {
}

pub static TELEMETRYSDKPINPLAYERSTATSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStatsEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerStatsEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "player_stats",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatsEvent, player_stats),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinPlayerStatsEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERSTATSEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerStatsEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERSTATSEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERSTATSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerStatsEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerStatsEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinCameraStateEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub gdur: u32,
    pub cam_dur: u32,
    pub prev_cam_state: String,
    pub cur_cam_state: String,
    pub p_loc: super::core::Vec3,
    pub p_dir: super::core::Vec3,
    pub p_state: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinCameraStateEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn cam_dur(&self) -> &u32;
    fn cam_dur_mut(&mut self) -> &mut u32;
    fn prev_cam_state(&self) -> &String;
    fn prev_cam_state_mut(&mut self) -> &mut String;
    fn cur_cam_state(&self) -> &String;
    fn cur_cam_state_mut(&mut self) -> &mut String;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_dir(&self) -> &super::core::Vec3;
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn p_state(&self) -> &String;
    fn p_state_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinCameraStateEventTrait for TelemetrySdkPinCameraStateEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn cam_dur(&self) -> &u32 {
        &self.cam_dur
    }
    fn cam_dur_mut(&mut self) -> &mut u32 {
        &mut self.cam_dur
    }
    fn prev_cam_state(&self) -> &String {
        &self.prev_cam_state
    }
    fn prev_cam_state_mut(&mut self) -> &mut String {
        &mut self.prev_cam_state
    }
    fn cur_cam_state(&self) -> &String {
        &self.cur_cam_state
    }
    fn cur_cam_state_mut(&mut self) -> &mut String {
        &mut self.cur_cam_state
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_dir(&self) -> &super::core::Vec3 {
        &self.p_dir
    }
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_dir
    }
    fn p_state(&self) -> &String {
        &self.p_state
    }
    fn p_state_mut(&mut self) -> &mut String {
        &mut self.p_state
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinCameraStateEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinCameraStateEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinCameraStateEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinCameraStateEvent {
}

pub static TELEMETRYSDKPINCAMERASTATEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCameraStateEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinCameraStateEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, gdur),
            },
            FieldInfoData {
                name: "cam_dur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, cam_dur),
            },
            FieldInfoData {
                name: "prev_cam_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, prev_cam_state),
            },
            FieldInfoData {
                name: "cur_cam_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, cur_cam_state),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, p_dir),
            },
            FieldInfoData {
                name: "p_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, p_state),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinCameraStateEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINCAMERASTATEEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinCameraStateEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINCAMERASTATEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINCAMERASTATEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinCameraStateEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinCameraStateEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerObEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerObEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn p_loc(&self) -> &super::core::Vec3;
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3;
    fn p_dir(&self) -> &super::core::Vec3;
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn p_class(&self) -> &String;
    fn p_class_mut(&mut self) -> &mut String;
    fn p_team_id(&self) -> &String;
    fn p_team_id_mut(&mut self) -> &mut String;
    fn p_veh_id(&self) -> &String;
    fn p_veh_id_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinPlayerObEventTrait for TelemetrySdkPinPlayerObEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn p_loc(&self) -> &super::core::Vec3 {
        &self.p_loc
    }
    fn p_loc_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_loc
    }
    fn p_dir(&self) -> &super::core::Vec3 {
        &self.p_dir
    }
    fn p_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.p_dir
    }
    fn p_class(&self) -> &String {
        &self.p_class
    }
    fn p_class_mut(&mut self) -> &mut String {
        &mut self.p_class
    }
    fn p_team_id(&self) -> &String {
        &self.p_team_id
    }
    fn p_team_id_mut(&mut self) -> &mut String {
        &mut self.p_team_id
    }
    fn p_veh_id(&self) -> &String {
        &self.p_veh_id
    }
    fn p_veh_id_mut(&mut self) -> &mut String {
        &mut self.p_veh_id
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerObEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerObEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerObEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerObEvent {
}

pub static TELEMETRYSDKPINPLAYEROBEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerObEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerObEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, rdur),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, r#type),
            },
            FieldInfoData {
                name: "p_loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, p_loc),
            },
            FieldInfoData {
                name: "p_dir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, p_dir),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, p_team_id),
            },
            FieldInfoData {
                name: "p_veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, p_veh_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinPlayerObEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYEROBEVENT_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetrySdkPinPlayerObEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYEROBEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYEROBEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerObEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerObEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinUiInteractionEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub r#type: String,
    pub object_id: String,
    pub pgid: String,
    pub sdur: u32,
    pub object_type: String,
    pub mdur: u32,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinUiInteractionEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn object_id(&self) -> &String;
    fn object_id_mut(&mut self) -> &mut String;
    fn pgid(&self) -> &String;
    fn pgid_mut(&mut self) -> &mut String;
    fn sdur(&self) -> &u32;
    fn sdur_mut(&mut self) -> &mut u32;
    fn object_type(&self) -> &String;
    fn object_type_mut(&mut self) -> &mut String;
    fn mdur(&self) -> &u32;
    fn mdur_mut(&mut self) -> &mut u32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinUiInteractionEventTrait for TelemetrySdkPinUiInteractionEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn object_id(&self) -> &String {
        &self.object_id
    }
    fn object_id_mut(&mut self) -> &mut String {
        &mut self.object_id
    }
    fn pgid(&self) -> &String {
        &self.pgid
    }
    fn pgid_mut(&mut self) -> &mut String {
        &mut self.pgid
    }
    fn sdur(&self) -> &u32 {
        &self.sdur
    }
    fn sdur_mut(&mut self) -> &mut u32 {
        &mut self.sdur
    }
    fn object_type(&self) -> &String {
        &self.object_type
    }
    fn object_type_mut(&mut self) -> &mut String {
        &mut self.object_type
    }
    fn mdur(&self) -> &u32 {
        &self.mdur
    }
    fn mdur_mut(&mut self) -> &mut u32 {
        &mut self.mdur
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinUiInteractionEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinUiInteractionEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinUiInteractionEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinUiInteractionEvent {
}

pub static TELEMETRYSDKPINUIINTERACTIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinUiInteractionEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinUiInteractionEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, r#type),
            },
            FieldInfoData {
                name: "object_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, object_id),
            },
            FieldInfoData {
                name: "pgid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, pgid),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, sdur),
            },
            FieldInfoData {
                name: "object_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, object_type),
            },
            FieldInfoData {
                name: "mdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, mdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinUiInteractionEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINUIINTERACTIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinUiInteractionEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINUIINTERACTIONEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINUIINTERACTIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinUiInteractionEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinUiInteractionEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinHardwareProfileEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub cpu: String,
    pub sys_mem: String,
    pub gpu: String,
    pub gpu_mem: String,
    pub gpu_id: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinHardwareProfileEventTrait: TelemetrySDKPinEventTrait {
    fn cpu(&self) -> &String;
    fn cpu_mut(&mut self) -> &mut String;
    fn sys_mem(&self) -> &String;
    fn sys_mem_mut(&mut self) -> &mut String;
    fn gpu(&self) -> &String;
    fn gpu_mut(&mut self) -> &mut String;
    fn gpu_mem(&self) -> &String;
    fn gpu_mem_mut(&mut self) -> &mut String;
    fn gpu_id(&self) -> &String;
    fn gpu_id_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinHardwareProfileEventTrait for TelemetrySdkPinHardwareProfileEvent {
    fn cpu(&self) -> &String {
        &self.cpu
    }
    fn cpu_mut(&mut self) -> &mut String {
        &mut self.cpu
    }
    fn sys_mem(&self) -> &String {
        &self.sys_mem
    }
    fn sys_mem_mut(&mut self) -> &mut String {
        &mut self.sys_mem
    }
    fn gpu(&self) -> &String {
        &self.gpu
    }
    fn gpu_mut(&mut self) -> &mut String {
        &mut self.gpu
    }
    fn gpu_mem(&self) -> &String {
        &self.gpu_mem
    }
    fn gpu_mem_mut(&mut self) -> &mut String {
        &mut self.gpu_mem
    }
    fn gpu_id(&self) -> &String {
        &self.gpu_id
    }
    fn gpu_id_mut(&mut self) -> &mut String {
        &mut self.gpu_id
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinHardwareProfileEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinHardwareProfileEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinHardwareProfileEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinHardwareProfileEvent {
}

pub static TELEMETRYSDKPINHARDWAREPROFILEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinHardwareProfileEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinHardwareProfileEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "cpu",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, cpu),
            },
            FieldInfoData {
                name: "sys_mem",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, sys_mem),
            },
            FieldInfoData {
                name: "gpu",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, gpu),
            },
            FieldInfoData {
                name: "gpu_mem",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, gpu_mem),
            },
            FieldInfoData {
                name: "gpu_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, gpu_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinHardwareProfileEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINHARDWAREPROFILEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinHardwareProfileEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINHARDWAREPROFILEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINHARDWAREPROFILEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinHardwareProfileEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinHardwareProfileEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerWeapSmryEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerWeapSmryEventTrait: TelemetrySDKPinEventTrait {
    fn gdur(&self) -> &u32;
    fn gdur_mut(&mut self) -> &mut u32;
    fn rdur(&self) -> &u32;
    fn rdur_mut(&mut self) -> &mut u32;
    fn weap_id(&self) -> &String;
    fn weap_id_mut(&mut self) -> &mut String;
    fn weap_category(&self) -> &String;
    fn weap_category_mut(&mut self) -> &mut String;
    fn weap_type(&self) -> &String;
    fn weap_type_mut(&mut self) -> &mut String;
    fn p_class(&self) -> &String;
    fn p_class_mut(&mut self) -> &mut String;
    fn p_team_id(&self) -> &String;
    fn p_team_id_mut(&mut self) -> &mut String;
    fn veh_id(&self) -> &String;
    fn veh_id_mut(&mut self) -> &mut String;
    fn sht_fired(&self) -> &i32;
    fn sht_fired_mut(&mut self) -> &mut i32;
    fn equip_dur(&self) -> &i32;
    fn equip_dur_mut(&mut self) -> &mut i32;
    fn sht_hit_sldr(&self) -> &i32;
    fn sht_hit_sldr_mut(&mut self) -> &mut i32;
    fn hd_sht_sldr(&self) -> &i32;
    fn hd_sht_sldr_mut(&mut self) -> &mut i32;
    fn fatl_sht_sldr(&self) -> &i32;
    fn fatl_sht_sldr_mut(&mut self) -> &mut i32;
    fn dmg_sldr(&self) -> &i32;
    fn dmg_sldr_mut(&mut self) -> &mut i32;
    fn sht_hit_veh(&self) -> &i32;
    fn sht_hit_veh_mut(&mut self) -> &mut i32;
    fn veh_dstr(&self) -> &i32;
    fn veh_dstr_mut(&mut self) -> &mut i32;
    fn dmg_veh(&self) -> &i32;
    fn dmg_veh_mut(&mut self) -> &mut i32;
    fn weap_mods(&self) -> &RawJsonString;
    fn weap_mods_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl TelemetrySdkPinPlayerWeapSmryEventTrait for TelemetrySdkPinPlayerWeapSmryEvent {
    fn gdur(&self) -> &u32 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut u32 {
        &mut self.gdur
    }
    fn rdur(&self) -> &u32 {
        &self.rdur
    }
    fn rdur_mut(&mut self) -> &mut u32 {
        &mut self.rdur
    }
    fn weap_id(&self) -> &String {
        &self.weap_id
    }
    fn weap_id_mut(&mut self) -> &mut String {
        &mut self.weap_id
    }
    fn weap_category(&self) -> &String {
        &self.weap_category
    }
    fn weap_category_mut(&mut self) -> &mut String {
        &mut self.weap_category
    }
    fn weap_type(&self) -> &String {
        &self.weap_type
    }
    fn weap_type_mut(&mut self) -> &mut String {
        &mut self.weap_type
    }
    fn p_class(&self) -> &String {
        &self.p_class
    }
    fn p_class_mut(&mut self) -> &mut String {
        &mut self.p_class
    }
    fn p_team_id(&self) -> &String {
        &self.p_team_id
    }
    fn p_team_id_mut(&mut self) -> &mut String {
        &mut self.p_team_id
    }
    fn veh_id(&self) -> &String {
        &self.veh_id
    }
    fn veh_id_mut(&mut self) -> &mut String {
        &mut self.veh_id
    }
    fn sht_fired(&self) -> &i32 {
        &self.sht_fired
    }
    fn sht_fired_mut(&mut self) -> &mut i32 {
        &mut self.sht_fired
    }
    fn equip_dur(&self) -> &i32 {
        &self.equip_dur
    }
    fn equip_dur_mut(&mut self) -> &mut i32 {
        &mut self.equip_dur
    }
    fn sht_hit_sldr(&self) -> &i32 {
        &self.sht_hit_sldr
    }
    fn sht_hit_sldr_mut(&mut self) -> &mut i32 {
        &mut self.sht_hit_sldr
    }
    fn hd_sht_sldr(&self) -> &i32 {
        &self.hd_sht_sldr
    }
    fn hd_sht_sldr_mut(&mut self) -> &mut i32 {
        &mut self.hd_sht_sldr
    }
    fn fatl_sht_sldr(&self) -> &i32 {
        &self.fatl_sht_sldr
    }
    fn fatl_sht_sldr_mut(&mut self) -> &mut i32 {
        &mut self.fatl_sht_sldr
    }
    fn dmg_sldr(&self) -> &i32 {
        &self.dmg_sldr
    }
    fn dmg_sldr_mut(&mut self) -> &mut i32 {
        &mut self.dmg_sldr
    }
    fn sht_hit_veh(&self) -> &i32 {
        &self.sht_hit_veh
    }
    fn sht_hit_veh_mut(&mut self) -> &mut i32 {
        &mut self.sht_hit_veh
    }
    fn veh_dstr(&self) -> &i32 {
        &self.veh_dstr
    }
    fn veh_dstr_mut(&mut self) -> &mut i32 {
        &mut self.veh_dstr
    }
    fn dmg_veh(&self) -> &i32 {
        &self.dmg_veh
    }
    fn dmg_veh_mut(&mut self) -> &mut i32 {
        &mut self.dmg_veh
    }
    fn weap_mods(&self) -> &RawJsonString {
        &self.weap_mods
    }
    fn weap_mods_mut(&mut self) -> &mut RawJsonString {
        &mut self.weap_mods
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerWeapSmryEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerWeapSmryEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerWeapSmryEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerWeapSmryEvent {
}

pub static TELEMETRYSDKPINPLAYERWEAPSMRYEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerWeapSmryEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerWeapSmryEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, gdur),
            },
            FieldInfoData {
                name: "rdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, rdur),
            },
            FieldInfoData {
                name: "weap_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, weap_id),
            },
            FieldInfoData {
                name: "weap_category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, weap_category),
            },
            FieldInfoData {
                name: "weap_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, weap_type),
            },
            FieldInfoData {
                name: "p_class",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, p_class),
            },
            FieldInfoData {
                name: "p_team_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, p_team_id),
            },
            FieldInfoData {
                name: "veh_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, veh_id),
            },
            FieldInfoData {
                name: "sht_fired",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, sht_fired),
            },
            FieldInfoData {
                name: "equip_dur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, equip_dur),
            },
            FieldInfoData {
                name: "sht_hit_sldr",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, sht_hit_sldr),
            },
            FieldInfoData {
                name: "hd_sht_sldr",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, hd_sht_sldr),
            },
            FieldInfoData {
                name: "fatl_sht_sldr",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, fatl_sht_sldr),
            },
            FieldInfoData {
                name: "dmg_sldr",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, dmg_sldr),
            },
            FieldInfoData {
                name: "sht_hit_veh",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, sht_hit_veh),
            },
            FieldInfoData {
                name: "veh_dstr",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, veh_dstr),
            },
            FieldInfoData {
                name: "dmg_veh",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, dmg_veh),
            },
            FieldInfoData {
                name: "weap_mods",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, weap_mods),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPlayerWeapSmryEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERWEAPSMRYEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerWeapSmryEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERWEAPSMRYEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERWEAPSMRYEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerWeapSmryEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerWeapSmryEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinTimerEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub category: String,
    pub measure: String,
    pub dur: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub meta_data: RawJsonString,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinTimerEventTrait: TelemetrySDKPinEventTrait {
    fn category(&self) -> &String;
    fn category_mut(&mut self) -> &mut String;
    fn measure(&self) -> &String;
    fn measure_mut(&mut self) -> &mut String;
    fn dur(&self) -> &i64;
    fn dur_mut(&mut self) -> &mut i64;
    fn start_time(&self) -> &i64;
    fn start_time_mut(&mut self) -> &mut i64;
    fn end_time(&self) -> &i64;
    fn end_time_mut(&mut self) -> &mut i64;
    fn meta_data(&self) -> &RawJsonString;
    fn meta_data_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinTimerEventTrait for TelemetrySdkPinTimerEvent {
    fn category(&self) -> &String {
        &self.category
    }
    fn category_mut(&mut self) -> &mut String {
        &mut self.category
    }
    fn measure(&self) -> &String {
        &self.measure
    }
    fn measure_mut(&mut self) -> &mut String {
        &mut self.measure
    }
    fn dur(&self) -> &i64 {
        &self.dur
    }
    fn dur_mut(&mut self) -> &mut i64 {
        &mut self.dur
    }
    fn start_time(&self) -> &i64 {
        &self.start_time
    }
    fn start_time_mut(&mut self) -> &mut i64 {
        &mut self.start_time
    }
    fn end_time(&self) -> &i64 {
        &self.end_time
    }
    fn end_time_mut(&mut self) -> &mut i64 {
        &mut self.end_time
    }
    fn meta_data(&self) -> &RawJsonString {
        &self.meta_data
    }
    fn meta_data_mut(&mut self) -> &mut RawJsonString {
        &mut self.meta_data
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinTimerEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinTimerEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinTimerEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinTimerEvent {
}

pub static TELEMETRYSDKPINTIMEREVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTimerEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinTimerEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "category",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, category),
            },
            FieldInfoData {
                name: "measure",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, measure),
            },
            FieldInfoData {
                name: "dur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, dur),
            },
            FieldInfoData {
                name: "start_time",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, start_time),
            },
            FieldInfoData {
                name: "end_time",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, end_time),
            },
            FieldInfoData {
                name: "meta_data",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, meta_data),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinTimerEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINTIMEREVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinTimerEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINTIMEREVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINTIMEREVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinTimerEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinTimerEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinMessageEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinMessageEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn service(&self) -> &String;
    fn service_mut(&mut self) -> &mut String;
    fn content_type(&self) -> &String;
    fn content_type_mut(&mut self) -> &mut String;
    fn format(&self) -> &String;
    fn format_mut(&mut self) -> &mut String;
    fn media(&self) -> &String;
    fn media_mut(&mut self) -> &mut String;
    fn campaign_id(&self) -> &String;
    fn campaign_id_mut(&mut self) -> &mut String;
    fn client_state(&self) -> &String;
    fn client_state_mut(&mut self) -> &mut String;
    fn msg_id(&self) -> &String;
    fn msg_id_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn option(&self) -> &String;
    fn option_mut(&mut self) -> &mut String;
    fn content(&self) -> &String;
    fn content_mut(&mut self) -> &mut String;
    fn destination_name(&self) -> &String;
    fn destination_name_mut(&mut self) -> &mut String;
    fn destination_id(&self) -> &String;
    fn destination_id_mut(&mut self) -> &mut String;
    fn place(&self) -> &String;
    fn place_mut(&mut self) -> &mut String;
    fn loc(&self) -> &super::core::Vec2;
    fn loc_mut(&mut self) -> &mut super::core::Vec2;
    fn size(&self) -> &super::core::Vec2;
    fn size_mut(&mut self) -> &mut super::core::Vec2;
    fn segment_id(&self) -> &String;
    fn segment_id_mut(&mut self) -> &mut String;
    fn count(&self) -> &i64;
    fn count_mut(&mut self) -> &mut i64;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl TelemetrySdkPinMessageEventTrait for TelemetrySdkPinMessageEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn service(&self) -> &String {
        &self.service
    }
    fn service_mut(&mut self) -> &mut String {
        &mut self.service
    }
    fn content_type(&self) -> &String {
        &self.content_type
    }
    fn content_type_mut(&mut self) -> &mut String {
        &mut self.content_type
    }
    fn format(&self) -> &String {
        &self.format
    }
    fn format_mut(&mut self) -> &mut String {
        &mut self.format
    }
    fn media(&self) -> &String {
        &self.media
    }
    fn media_mut(&mut self) -> &mut String {
        &mut self.media
    }
    fn campaign_id(&self) -> &String {
        &self.campaign_id
    }
    fn campaign_id_mut(&mut self) -> &mut String {
        &mut self.campaign_id
    }
    fn client_state(&self) -> &String {
        &self.client_state
    }
    fn client_state_mut(&mut self) -> &mut String {
        &mut self.client_state
    }
    fn msg_id(&self) -> &String {
        &self.msg_id
    }
    fn msg_id_mut(&mut self) -> &mut String {
        &mut self.msg_id
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn option(&self) -> &String {
        &self.option
    }
    fn option_mut(&mut self) -> &mut String {
        &mut self.option
    }
    fn content(&self) -> &String {
        &self.content
    }
    fn content_mut(&mut self) -> &mut String {
        &mut self.content
    }
    fn destination_name(&self) -> &String {
        &self.destination_name
    }
    fn destination_name_mut(&mut self) -> &mut String {
        &mut self.destination_name
    }
    fn destination_id(&self) -> &String {
        &self.destination_id
    }
    fn destination_id_mut(&mut self) -> &mut String {
        &mut self.destination_id
    }
    fn place(&self) -> &String {
        &self.place
    }
    fn place_mut(&mut self) -> &mut String {
        &mut self.place
    }
    fn loc(&self) -> &super::core::Vec2 {
        &self.loc
    }
    fn loc_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.loc
    }
    fn size(&self) -> &super::core::Vec2 {
        &self.size
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.size
    }
    fn segment_id(&self) -> &String {
        &self.segment_id
    }
    fn segment_id_mut(&mut self) -> &mut String {
        &mut self.segment_id
    }
    fn count(&self) -> &i64 {
        &self.count
    }
    fn count_mut(&mut self) -> &mut i64 {
        &mut self.count
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinMessageEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinMessageEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinMessageEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinMessageEvent {
}

pub static TELEMETRYSDKPINMESSAGEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMessageEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinMessageEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, r#type),
            },
            FieldInfoData {
                name: "service",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, service),
            },
            FieldInfoData {
                name: "content_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, content_type),
            },
            FieldInfoData {
                name: "format",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, format),
            },
            FieldInfoData {
                name: "media",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, media),
            },
            FieldInfoData {
                name: "campaign_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, campaign_id),
            },
            FieldInfoData {
                name: "client_state",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, client_state),
            },
            FieldInfoData {
                name: "msg_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, msg_id),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, status_code),
            },
            FieldInfoData {
                name: "option",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, option),
            },
            FieldInfoData {
                name: "content",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, content),
            },
            FieldInfoData {
                name: "destination_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, destination_name),
            },
            FieldInfoData {
                name: "destination_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, destination_id),
            },
            FieldInfoData {
                name: "place",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, place),
            },
            FieldInfoData {
                name: "loc",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, loc),
            },
            FieldInfoData {
                name: "size",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, size),
            },
            FieldInfoData {
                name: "segment_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, segment_id),
            },
            FieldInfoData {
                name: "count",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, count),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinMessageEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMESSAGEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinMessageEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINMESSAGEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINMESSAGEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMessageEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinMessageEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPageViewEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub r#type: String,
    pub pgid: String,
    pub pgdur: u32,
    pub fromid: String,
    pub toid: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinPageViewEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn pgid(&self) -> &String;
    fn pgid_mut(&mut self) -> &mut String;
    fn pgdur(&self) -> &u32;
    fn pgdur_mut(&mut self) -> &mut u32;
    fn fromid(&self) -> &String;
    fn fromid_mut(&mut self) -> &mut String;
    fn toid(&self) -> &String;
    fn toid_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinPageViewEventTrait for TelemetrySdkPinPageViewEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn pgid(&self) -> &String {
        &self.pgid
    }
    fn pgid_mut(&mut self) -> &mut String {
        &mut self.pgid
    }
    fn pgdur(&self) -> &u32 {
        &self.pgdur
    }
    fn pgdur_mut(&mut self) -> &mut u32 {
        &mut self.pgdur
    }
    fn fromid(&self) -> &String {
        &self.fromid
    }
    fn fromid_mut(&mut self) -> &mut String {
        &mut self.fromid
    }
    fn toid(&self) -> &String {
        &self.toid
    }
    fn toid_mut(&mut self) -> &mut String {
        &mut self.toid
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPageViewEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPageViewEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPageViewEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPageViewEvent {
}

pub static TELEMETRYSDKPINPAGEVIEWEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPageViewEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPageViewEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, r#type),
            },
            FieldInfoData {
                name: "pgid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, pgid),
            },
            FieldInfoData {
                name: "pgdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, pgdur),
            },
            FieldInfoData {
                name: "fromid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, fromid),
            },
            FieldInfoData {
                name: "toid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, toid),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinPageViewEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPAGEVIEWEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPageViewEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPAGEVIEWEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPAGEVIEWEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPageViewEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPageViewEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinAchievementEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinAchievementEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn achv_id(&self) -> &String;
    fn achv_id_mut(&mut self) -> &mut String;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn reqs(&self) -> &RawJsonString;
    fn reqs_mut(&mut self) -> &mut RawJsonString;
    fn reward(&self) -> &RawJsonString;
    fn reward_mut(&mut self) -> &mut RawJsonString;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn percent(&self) -> &u32;
    fn percent_mut(&mut self) -> &mut u32;
    fn diff(&self) -> &String;
    fn diff_mut(&mut self) -> &mut String;
    fn gdur(&self) -> &i64;
    fn gdur_mut(&mut self) -> &mut i64;
    fn sdur(&self) -> &i64;
    fn sdur_mut(&mut self) -> &mut i64;
    fn cdur(&self) -> &i64;
    fn cdur_mut(&mut self) -> &mut i64;
    fn tdur(&self) -> &i64;
    fn tdur_mut(&mut self) -> &mut i64;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinAchievementEventTrait for TelemetrySdkPinAchievementEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn achv_id(&self) -> &String {
        &self.achv_id
    }
    fn achv_id_mut(&mut self) -> &mut String {
        &mut self.achv_id
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn reqs(&self) -> &RawJsonString {
        &self.reqs
    }
    fn reqs_mut(&mut self) -> &mut RawJsonString {
        &mut self.reqs
    }
    fn reward(&self) -> &RawJsonString {
        &self.reward
    }
    fn reward_mut(&mut self) -> &mut RawJsonString {
        &mut self.reward
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn percent(&self) -> &u32 {
        &self.percent
    }
    fn percent_mut(&mut self) -> &mut u32 {
        &mut self.percent
    }
    fn diff(&self) -> &String {
        &self.diff
    }
    fn diff_mut(&mut self) -> &mut String {
        &mut self.diff
    }
    fn gdur(&self) -> &i64 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut i64 {
        &mut self.gdur
    }
    fn sdur(&self) -> &i64 {
        &self.sdur
    }
    fn sdur_mut(&mut self) -> &mut i64 {
        &mut self.sdur
    }
    fn cdur(&self) -> &i64 {
        &self.cdur
    }
    fn cdur_mut(&mut self) -> &mut i64 {
        &mut self.cdur
    }
    fn tdur(&self) -> &i64 {
        &self.tdur
    }
    fn tdur_mut(&mut self) -> &mut i64 {
        &mut self.tdur
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinAchievementEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinAchievementEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinAchievementEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinAchievementEvent {
}

pub static TELEMETRYSDKPINACHIEVEMENTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinAchievementEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinAchievementEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, r#type),
            },
            FieldInfoData {
                name: "achv_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, achv_id),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, instance_id),
            },
            FieldInfoData {
                name: "reqs",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, reqs),
            },
            FieldInfoData {
                name: "reward",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, reward),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, status),
            },
            FieldInfoData {
                name: "percent",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, percent),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, diff),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, gdur),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, sdur),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, cdur),
            },
            FieldInfoData {
                name: "tdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, tdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinAchievementEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINACHIEVEMENTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinAchievementEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINACHIEVEMENTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINACHIEVEMENTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinAchievementEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinAchievementEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinMileStoneEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinMileStoneEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn moment(&self) -> &String;
    fn moment_mut(&mut self) -> &mut String;
    fn mstid(&self) -> &String;
    fn mstid_mut(&mut self) -> &mut String;
    fn game_mode(&self) -> &String;
    fn game_mode_mut(&mut self) -> &mut String;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn diff(&self) -> &String;
    fn diff_mut(&mut self) -> &mut String;
    fn gdur(&self) -> &i64;
    fn gdur_mut(&mut self) -> &mut i64;
    fn sdur(&self) -> &i64;
    fn sdur_mut(&mut self) -> &mut i64;
    fn cdur(&self) -> &i64;
    fn cdur_mut(&mut self) -> &mut i64;
    fn tdur(&self) -> &i64;
    fn tdur_mut(&mut self) -> &mut i64;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinMileStoneEventTrait for TelemetrySdkPinMileStoneEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn moment(&self) -> &String {
        &self.moment
    }
    fn moment_mut(&mut self) -> &mut String {
        &mut self.moment
    }
    fn mstid(&self) -> &String {
        &self.mstid
    }
    fn mstid_mut(&mut self) -> &mut String {
        &mut self.mstid
    }
    fn game_mode(&self) -> &String {
        &self.game_mode
    }
    fn game_mode_mut(&mut self) -> &mut String {
        &mut self.game_mode
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn diff(&self) -> &String {
        &self.diff
    }
    fn diff_mut(&mut self) -> &mut String {
        &mut self.diff
    }
    fn gdur(&self) -> &i64 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut i64 {
        &mut self.gdur
    }
    fn sdur(&self) -> &i64 {
        &self.sdur
    }
    fn sdur_mut(&mut self) -> &mut i64 {
        &mut self.sdur
    }
    fn cdur(&self) -> &i64 {
        &self.cdur
    }
    fn cdur_mut(&mut self) -> &mut i64 {
        &mut self.cdur
    }
    fn tdur(&self) -> &i64 {
        &self.tdur
    }
    fn tdur_mut(&mut self) -> &mut i64 {
        &mut self.tdur
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinMileStoneEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinMileStoneEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinMileStoneEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinMileStoneEvent {
}

pub static TELEMETRYSDKPINMILESTONEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMileStoneEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinMileStoneEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, r#type),
            },
            FieldInfoData {
                name: "moment",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, moment),
            },
            FieldInfoData {
                name: "mstid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, mstid),
            },
            FieldInfoData {
                name: "game_mode",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, game_mode),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, instance_id),
            },
            FieldInfoData {
                name: "diff",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, diff),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, gdur),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, sdur),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, cdur),
            },
            FieldInfoData {
                name: "tdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, tdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinMileStoneEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINMILESTONEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinMileStoneEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINMILESTONEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINMILESTONEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinMileStoneEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinMileStoneEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinPlayerLevelEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinPlayerLevelEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn mode(&self) -> &String;
    fn mode_mut(&mut self) -> &mut String;
    fn instance_id(&self) -> &String;
    fn instance_id_mut(&mut self) -> &mut String;
    fn level(&self) -> &String;
    fn level_mut(&mut self) -> &mut String;
    fn level_name(&self) -> &String;
    fn level_name_mut(&mut self) -> &mut String;
    fn gdur(&self) -> &i64;
    fn gdur_mut(&mut self) -> &mut i64;
    fn sdur(&self) -> &i64;
    fn sdur_mut(&mut self) -> &mut i64;
    fn cdur(&self) -> &i64;
    fn cdur_mut(&mut self) -> &mut i64;
    fn tdur(&self) -> &i64;
    fn tdur_mut(&mut self) -> &mut i64;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl TelemetrySdkPinPlayerLevelEventTrait for TelemetrySdkPinPlayerLevelEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn mode(&self) -> &String {
        &self.mode
    }
    fn mode_mut(&mut self) -> &mut String {
        &mut self.mode
    }
    fn instance_id(&self) -> &String {
        &self.instance_id
    }
    fn instance_id_mut(&mut self) -> &mut String {
        &mut self.instance_id
    }
    fn level(&self) -> &String {
        &self.level
    }
    fn level_mut(&mut self) -> &mut String {
        &mut self.level
    }
    fn level_name(&self) -> &String {
        &self.level_name
    }
    fn level_name_mut(&mut self) -> &mut String {
        &mut self.level_name
    }
    fn gdur(&self) -> &i64 {
        &self.gdur
    }
    fn gdur_mut(&mut self) -> &mut i64 {
        &mut self.gdur
    }
    fn sdur(&self) -> &i64 {
        &self.sdur
    }
    fn sdur_mut(&mut self) -> &mut i64 {
        &mut self.sdur
    }
    fn cdur(&self) -> &i64 {
        &self.cdur
    }
    fn cdur_mut(&mut self) -> &mut i64 {
        &mut self.cdur
    }
    fn tdur(&self) -> &i64 {
        &self.tdur
    }
    fn tdur_mut(&mut self) -> &mut i64 {
        &mut self.tdur
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinPlayerLevelEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinPlayerLevelEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinPlayerLevelEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinPlayerLevelEvent {
}

pub static TELEMETRYSDKPINPLAYERLEVELEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerLevelEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinPlayerLevelEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, r#type),
            },
            FieldInfoData {
                name: "mode",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, mode),
            },
            FieldInfoData {
                name: "instance_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, instance_id),
            },
            FieldInfoData {
                name: "level",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, level),
            },
            FieldInfoData {
                name: "level_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, level_name),
            },
            FieldInfoData {
                name: "gdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, gdur),
            },
            FieldInfoData {
                name: "sdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, sdur),
            },
            FieldInfoData {
                name: "cdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, cdur),
            },
            FieldInfoData {
                name: "tdur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, tdur),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TelemetrySdkPinPlayerLevelEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINPLAYERLEVELEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinPlayerLevelEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINPLAYERLEVELEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINPLAYERLEVELEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinPlayerLevelEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinPlayerLevelEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinFavoriteEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub type1: String,
    pub type1_id: String,
    pub type1_name: String,
    pub type2: String,
    pub type2_id: String,
    pub type2_name: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinFavoriteEventTrait: TelemetrySDKPinEventTrait {
    fn type1(&self) -> &String;
    fn type1_mut(&mut self) -> &mut String;
    fn type1_id(&self) -> &String;
    fn type1_id_mut(&mut self) -> &mut String;
    fn type1_name(&self) -> &String;
    fn type1_name_mut(&mut self) -> &mut String;
    fn type2(&self) -> &String;
    fn type2_mut(&mut self) -> &mut String;
    fn type2_id(&self) -> &String;
    fn type2_id_mut(&mut self) -> &mut String;
    fn type2_name(&self) -> &String;
    fn type2_name_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinFavoriteEventTrait for TelemetrySdkPinFavoriteEvent {
    fn type1(&self) -> &String {
        &self.type1
    }
    fn type1_mut(&mut self) -> &mut String {
        &mut self.type1
    }
    fn type1_id(&self) -> &String {
        &self.type1_id
    }
    fn type1_id_mut(&mut self) -> &mut String {
        &mut self.type1_id
    }
    fn type1_name(&self) -> &String {
        &self.type1_name
    }
    fn type1_name_mut(&mut self) -> &mut String {
        &mut self.type1_name
    }
    fn type2(&self) -> &String {
        &self.type2
    }
    fn type2_mut(&mut self) -> &mut String {
        &mut self.type2
    }
    fn type2_id(&self) -> &String {
        &self.type2_id
    }
    fn type2_id_mut(&mut self) -> &mut String {
        &mut self.type2_id
    }
    fn type2_name(&self) -> &String {
        &self.type2_name
    }
    fn type2_name_mut(&mut self) -> &mut String {
        &mut self.type2_name
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinFavoriteEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinFavoriteEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinFavoriteEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinFavoriteEvent {
}

pub static TELEMETRYSDKPINFAVORITEEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFavoriteEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinFavoriteEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type1",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type1),
            },
            FieldInfoData {
                name: "type1_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type1_id),
            },
            FieldInfoData {
                name: "type1_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type1_name),
            },
            FieldInfoData {
                name: "type2",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type2),
            },
            FieldInfoData {
                name: "type2_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type2_id),
            },
            FieldInfoData {
                name: "type2_name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, type2_name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinFavoriteEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINFAVORITEEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinFavoriteEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINFAVORITEEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINFAVORITEEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinFavoriteEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinFavoriteEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinDownloadEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
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

pub trait TelemetrySdkPinDownloadEventTrait: TelemetrySDKPinEventTrait {
    fn item_id(&self) -> &String;
    fn item_id_mut(&mut self) -> &mut String;
    fn item_type(&self) -> &String;
    fn item_type_mut(&mut self) -> &mut String;
    fn item_platform(&self) -> &String;
    fn item_platform_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn dur(&self) -> &i32;
    fn dur_mut(&mut self) -> &mut i32;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn percent(&self) -> &f32;
    fn percent_mut(&mut self) -> &mut f32;
    fn download_id(&self) -> &i32;
    fn download_id_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinDownloadEventTrait for TelemetrySdkPinDownloadEvent {
    fn item_id(&self) -> &String {
        &self.item_id
    }
    fn item_id_mut(&mut self) -> &mut String {
        &mut self.item_id
    }
    fn item_type(&self) -> &String {
        &self.item_type
    }
    fn item_type_mut(&mut self) -> &mut String {
        &mut self.item_type
    }
    fn item_platform(&self) -> &String {
        &self.item_platform
    }
    fn item_platform_mut(&mut self) -> &mut String {
        &mut self.item_platform
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn dur(&self) -> &i32 {
        &self.dur
    }
    fn dur_mut(&mut self) -> &mut i32 {
        &mut self.dur
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn percent(&self) -> &f32 {
        &self.percent
    }
    fn percent_mut(&mut self) -> &mut f32 {
        &mut self.percent
    }
    fn download_id(&self) -> &i32 {
        &self.download_id
    }
    fn download_id_mut(&mut self) -> &mut i32 {
        &mut self.download_id
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinDownloadEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinDownloadEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinDownloadEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinDownloadEvent {
}

pub static TELEMETRYSDKPINDOWNLOADEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinDownloadEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinDownloadEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "item_id",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, item_id),
            },
            FieldInfoData {
                name: "item_type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, item_type),
            },
            FieldInfoData {
                name: "item_platform",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, item_platform),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, status),
            },
            FieldInfoData {
                name: "dur",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, dur),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, status_code),
            },
            FieldInfoData {
                name: "percent",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, percent),
            },
            FieldInfoData {
                name: "download_id",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, download_id),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinDownloadEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINDOWNLOADEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinDownloadEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINDOWNLOADEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINDOWNLOADEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinDownloadEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinDownloadEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinSettingsEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub r#type: String,
    pub status: String,
    pub status_code: String,
    pub settings: RawJsonString,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinSettingsEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn settings(&self) -> &RawJsonString;
    fn settings_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinSettingsEventTrait for TelemetrySdkPinSettingsEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn settings(&self) -> &RawJsonString {
        &self.settings
    }
    fn settings_mut(&mut self) -> &mut RawJsonString {
        &mut self.settings
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinSettingsEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinSettingsEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinSettingsEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinSettingsEvent {
}

pub static TELEMETRYSDKPINSETTINGSEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSettingsEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinSettingsEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSettingsEvent, r#type),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSettingsEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinSettingsEvent, status_code),
            },
            FieldInfoData {
                name: "settings",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinSettingsEvent, settings),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinSettingsEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINSETTINGSEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinSettingsEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINSETTINGSEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINSETTINGSEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinSettingsEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinSettingsEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinRegistrationEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub source: String,
    pub status: String,
    pub status_code: String,
    pub domain: String,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinRegistrationEventTrait: TelemetrySDKPinEventTrait {
    fn source(&self) -> &String;
    fn source_mut(&mut self) -> &mut String;
    fn status(&self) -> &String;
    fn status_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn domain(&self) -> &String;
    fn domain_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinRegistrationEventTrait for TelemetrySdkPinRegistrationEvent {
    fn source(&self) -> &String {
        &self.source
    }
    fn source_mut(&mut self) -> &mut String {
        &mut self.source
    }
    fn status(&self) -> &String {
        &self.status
    }
    fn status_mut(&mut self) -> &mut String {
        &mut self.status
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn domain(&self) -> &String {
        &self.domain
    }
    fn domain_mut(&mut self) -> &mut String {
        &mut self.domain
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinRegistrationEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinRegistrationEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinRegistrationEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinRegistrationEvent {
}

pub static TELEMETRYSDKPINREGISTRATIONEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRegistrationEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinRegistrationEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRegistrationEvent, source),
            },
            FieldInfoData {
                name: "status",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRegistrationEvent, status),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRegistrationEvent, status_code),
            },
            FieldInfoData {
                name: "domain",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinRegistrationEvent, domain),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinRegistrationEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINREGISTRATIONEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinRegistrationEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINREGISTRATIONEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINREGISTRATIONEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinRegistrationEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinRegistrationEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySdkPinAccountEvent {
    pub _glacier_base: TelemetrySDKPinEvent,
    pub r#type: String,
    pub status_code: String,
    pub acntid: RawJsonString,
    pub source: String,
    pub reason: String,
    pub duration: i64,
    pub metadata: RawJsonString,
    pub field_flag_changed0: u8,
}

pub trait TelemetrySdkPinAccountEventTrait: TelemetrySDKPinEventTrait {
    fn r#type(&self) -> &String;
    fn r#type_mut(&mut self) -> &mut String;
    fn status_code(&self) -> &String;
    fn status_code_mut(&mut self) -> &mut String;
    fn acntid(&self) -> &RawJsonString;
    fn acntid_mut(&mut self) -> &mut RawJsonString;
    fn source(&self) -> &String;
    fn source_mut(&mut self) -> &mut String;
    fn reason(&self) -> &String;
    fn reason_mut(&mut self) -> &mut String;
    fn duration(&self) -> &i64;
    fn duration_mut(&mut self) -> &mut i64;
    fn metadata(&self) -> &RawJsonString;
    fn metadata_mut(&mut self) -> &mut RawJsonString;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl TelemetrySdkPinAccountEventTrait for TelemetrySdkPinAccountEvent {
    fn r#type(&self) -> &String {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut String {
        &mut self.r#type
    }
    fn status_code(&self) -> &String {
        &self.status_code
    }
    fn status_code_mut(&mut self) -> &mut String {
        &mut self.status_code
    }
    fn acntid(&self) -> &RawJsonString {
        &self.acntid
    }
    fn acntid_mut(&mut self) -> &mut RawJsonString {
        &mut self.acntid
    }
    fn source(&self) -> &String {
        &self.source
    }
    fn source_mut(&mut self) -> &mut String {
        &mut self.source
    }
    fn reason(&self) -> &String {
        &self.reason
    }
    fn reason_mut(&mut self) -> &mut String {
        &mut self.reason
    }
    fn duration(&self) -> &i64 {
        &self.duration
    }
    fn duration_mut(&mut self) -> &mut i64 {
        &mut self.duration
    }
    fn metadata(&self) -> &RawJsonString {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut RawJsonString {
        &mut self.metadata
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

impl TelemetrySDKPinEventTrait for TelemetrySdkPinAccountEvent {
    fn pin_event_name(&self) -> &String {
        self._glacier_base.pin_event_name()
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        self._glacier_base.pin_event_name_mut()
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier()
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        self._glacier_base.event_header_modifier_mut()
    }
    fn track_changes(&self) -> &bool {
        self._glacier_base.track_changes()
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        self._glacier_base.track_changes_mut()
    }
}

impl TelemetryLogEventTrait for TelemetrySdkPinAccountEvent {
}

impl TelemetryRowDataTrait for TelemetrySdkPinAccountEvent {
}

impl super::core::DataContainerTrait for TelemetrySdkPinAccountEvent {
}

pub static TELEMETRYSDKPINACCOUNTEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinAccountEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYSDKPINEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySdkPinAccountEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, r#type),
            },
            FieldInfoData {
                name: "status_code",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, status_code),
            },
            FieldInfoData {
                name: "acntid",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, acntid),
            },
            FieldInfoData {
                name: "source",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, source),
            },
            FieldInfoData {
                name: "reason",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, reason),
            },
            FieldInfoData {
                name: "duration",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, duration),
            },
            FieldInfoData {
                name: "metadata",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, metadata),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetrySdkPinAccountEvent, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINACCOUNTEVENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySdkPinAccountEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINACCOUNTEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINACCOUNTEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySdkPinAccountEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySdkPinAccountEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetrySDKPinEvent {
    pub _glacier_base: TelemetryLogEvent,
    pub pin_event_name: String,
    pub event_header_modifier: TelemetrySDKPinEventHeaderModifier,
    pub track_changes: bool,
}

pub trait TelemetrySDKPinEventTrait: TelemetryLogEventTrait {
    fn pin_event_name(&self) -> &String;
    fn pin_event_name_mut(&mut self) -> &mut String;
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier;
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier;
    fn track_changes(&self) -> &bool;
    fn track_changes_mut(&mut self) -> &mut bool;
}

impl TelemetrySDKPinEventTrait for TelemetrySDKPinEvent {
    fn pin_event_name(&self) -> &String {
        &self.pin_event_name
    }
    fn pin_event_name_mut(&mut self) -> &mut String {
        &mut self.pin_event_name
    }
    fn event_header_modifier(&self) -> &TelemetrySDKPinEventHeaderModifier {
        &self.event_header_modifier
    }
    fn event_header_modifier_mut(&mut self) -> &mut TelemetrySDKPinEventHeaderModifier {
        &mut self.event_header_modifier
    }
    fn track_changes(&self) -> &bool {
        &self.track_changes
    }
    fn track_changes_mut(&mut self) -> &mut bool {
        &mut self.track_changes
    }
}

impl TelemetryLogEventTrait for TelemetrySDKPinEvent {
}

impl TelemetryRowDataTrait for TelemetrySDKPinEvent {
}

impl super::core::DataContainerTrait for TelemetrySDKPinEvent {
}

pub static TELEMETRYSDKPINEVENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEvent",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TELEMETRYLOGEVENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySDKPinEvent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PinEventName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEvent, pin_event_name),
            },
            FieldInfoData {
                name: "eventHeaderModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetrySDKPinEventHeaderModifier",
                rust_offset: offset_of!(TelemetrySDKPinEvent, event_header_modifier),
            },
            FieldInfoData {
                name: "TrackChanges",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetrySDKPinEvent, track_changes),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINEVENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TelemetrySDKPinEvent {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINEVENT_TYPE_INFO
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


pub static TELEMETRYSDKPINEVENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEvent-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinEvent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait TelemetrySDKPinEventHeaderModifierTrait: TypeObject {
    fn session_id(&self) -> &i64;
    fn session_id_mut(&mut self) -> &mut i64;
    fn game_id(&self) -> &i64;
    fn game_id_mut(&mut self) -> &mut i64;
    fn player_id_type(&self) -> &String;
    fn player_id_type_mut(&mut self) -> &mut String;
    fn player_id(&self) -> &String;
    fn player_id_mut(&mut self) -> &mut String;
    fn title_id_type(&self) -> &String;
    fn title_id_type_mut(&mut self) -> &mut String;
    fn title_id(&self) -> &String;
    fn title_id_mut(&mut self) -> &mut String;
    fn date_of_birth(&self) -> &String;
    fn date_of_birth_mut(&mut self) -> &mut String;
    fn current_level(&self) -> &String;
    fn current_level_mut(&mut self) -> &mut String;
    fn current_level_name(&self) -> &String;
    fn current_level_name_mut(&mut self) -> &mut String;
    fn release_type(&self) -> &String;
    fn release_type_mut(&mut self) -> &mut String;
    fn platform(&self) -> &String;
    fn platform_mut(&mut self) -> &mut String;
    fn player_id_map(&self) -> &String;
    fn player_id_map_mut(&mut self) -> &mut String;
    fn experiment_id(&self) -> &String;
    fn experiment_id_mut(&mut self) -> &mut String;
    fn mac_address(&self) -> &String;
    fn mac_address_mut(&mut self) -> &mut String;
    fn device_id_map(&self) -> &String;
    fn device_id_map_mut(&mut self) -> &mut String;
    fn custom_event_header(&self) -> &String;
    fn custom_event_header_mut(&mut self) -> &mut String;
    fn is_session(&self) -> &String;
    fn is_session_mut(&mut self) -> &mut String;
    fn is_player(&self) -> &String;
    fn is_player_mut(&mut self) -> &mut String;
    fn is_mlu(&self) -> &String;
    fn is_mlu_mut(&mut self) -> &mut String;
    fn subs(&self) -> &String;
    fn subs_mut(&mut self) -> &mut String;
    fn game_mode(&self) -> &String;
    fn game_mode_mut(&mut self) -> &mut String;
    fn game_type(&self) -> &String;
    fn game_type_mut(&mut self) -> &mut String;
    fn mode_type(&self) -> &String;
    fn mode_type_mut(&mut self) -> &mut String;
    fn map(&self) -> &String;
    fn map_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u32;
    fn field_flag_changed0_mut(&mut self) -> &mut u32;
}

impl TelemetrySDKPinEventHeaderModifierTrait for TelemetrySDKPinEventHeaderModifier {
    fn session_id(&self) -> &i64 {
        &self.session_id
    }
    fn session_id_mut(&mut self) -> &mut i64 {
        &mut self.session_id
    }
    fn game_id(&self) -> &i64 {
        &self.game_id
    }
    fn game_id_mut(&mut self) -> &mut i64 {
        &mut self.game_id
    }
    fn player_id_type(&self) -> &String {
        &self.player_id_type
    }
    fn player_id_type_mut(&mut self) -> &mut String {
        &mut self.player_id_type
    }
    fn player_id(&self) -> &String {
        &self.player_id
    }
    fn player_id_mut(&mut self) -> &mut String {
        &mut self.player_id
    }
    fn title_id_type(&self) -> &String {
        &self.title_id_type
    }
    fn title_id_type_mut(&mut self) -> &mut String {
        &mut self.title_id_type
    }
    fn title_id(&self) -> &String {
        &self.title_id
    }
    fn title_id_mut(&mut self) -> &mut String {
        &mut self.title_id
    }
    fn date_of_birth(&self) -> &String {
        &self.date_of_birth
    }
    fn date_of_birth_mut(&mut self) -> &mut String {
        &mut self.date_of_birth
    }
    fn current_level(&self) -> &String {
        &self.current_level
    }
    fn current_level_mut(&mut self) -> &mut String {
        &mut self.current_level
    }
    fn current_level_name(&self) -> &String {
        &self.current_level_name
    }
    fn current_level_name_mut(&mut self) -> &mut String {
        &mut self.current_level_name
    }
    fn release_type(&self) -> &String {
        &self.release_type
    }
    fn release_type_mut(&mut self) -> &mut String {
        &mut self.release_type
    }
    fn platform(&self) -> &String {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut String {
        &mut self.platform
    }
    fn player_id_map(&self) -> &String {
        &self.player_id_map
    }
    fn player_id_map_mut(&mut self) -> &mut String {
        &mut self.player_id_map
    }
    fn experiment_id(&self) -> &String {
        &self.experiment_id
    }
    fn experiment_id_mut(&mut self) -> &mut String {
        &mut self.experiment_id
    }
    fn mac_address(&self) -> &String {
        &self.mac_address
    }
    fn mac_address_mut(&mut self) -> &mut String {
        &mut self.mac_address
    }
    fn device_id_map(&self) -> &String {
        &self.device_id_map
    }
    fn device_id_map_mut(&mut self) -> &mut String {
        &mut self.device_id_map
    }
    fn custom_event_header(&self) -> &String {
        &self.custom_event_header
    }
    fn custom_event_header_mut(&mut self) -> &mut String {
        &mut self.custom_event_header
    }
    fn is_session(&self) -> &String {
        &self.is_session
    }
    fn is_session_mut(&mut self) -> &mut String {
        &mut self.is_session
    }
    fn is_player(&self) -> &String {
        &self.is_player
    }
    fn is_player_mut(&mut self) -> &mut String {
        &mut self.is_player
    }
    fn is_mlu(&self) -> &String {
        &self.is_mlu
    }
    fn is_mlu_mut(&mut self) -> &mut String {
        &mut self.is_mlu
    }
    fn subs(&self) -> &String {
        &self.subs
    }
    fn subs_mut(&mut self) -> &mut String {
        &mut self.subs
    }
    fn game_mode(&self) -> &String {
        &self.game_mode
    }
    fn game_mode_mut(&mut self) -> &mut String {
        &mut self.game_mode
    }
    fn game_type(&self) -> &String {
        &self.game_type
    }
    fn game_type_mut(&mut self) -> &mut String {
        &mut self.game_type
    }
    fn mode_type(&self) -> &String {
        &self.mode_type
    }
    fn mode_type_mut(&mut self) -> &mut String {
        &mut self.mode_type
    }
    fn map(&self) -> &String {
        &self.map
    }
    fn map_mut(&mut self) -> &mut String {
        &mut self.map
    }
    fn field_flag_changed0(&self) -> &u32 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u32 {
        &mut self.field_flag_changed0
    }
}

pub static TELEMETRYSDKPINEVENTHEADERMODIFIER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEventHeaderModifier",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySDKPinEventHeaderModifier as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "sessionId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, session_id),
            },
            FieldInfoData {
                name: "gameId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int64",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, game_id),
            },
            FieldInfoData {
                name: "playerIdType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, player_id_type),
            },
            FieldInfoData {
                name: "playerId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, player_id),
            },
            FieldInfoData {
                name: "titleIdType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, title_id_type),
            },
            FieldInfoData {
                name: "titleId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, title_id),
            },
            FieldInfoData {
                name: "dateOfBirth",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, date_of_birth),
            },
            FieldInfoData {
                name: "currentLevel",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, current_level),
            },
            FieldInfoData {
                name: "currentLevelName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, current_level_name),
            },
            FieldInfoData {
                name: "releaseType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, release_type),
            },
            FieldInfoData {
                name: "platform",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, platform),
            },
            FieldInfoData {
                name: "playerIdMap",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, player_id_map),
            },
            FieldInfoData {
                name: "experimentId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, experiment_id),
            },
            FieldInfoData {
                name: "macAddress",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, mac_address),
            },
            FieldInfoData {
                name: "deviceIdMap",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, device_id_map),
            },
            FieldInfoData {
                name: "customEventHeader",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, custom_event_header),
            },
            FieldInfoData {
                name: "isSession",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, is_session),
            },
            FieldInfoData {
                name: "isPlayer",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, is_player),
            },
            FieldInfoData {
                name: "isMlu",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, is_mlu),
            },
            FieldInfoData {
                name: "subs",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, subs),
            },
            FieldInfoData {
                name: "gameMode",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, game_mode),
            },
            FieldInfoData {
                name: "gameType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, game_type),
            },
            FieldInfoData {
                name: "modeType",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, mode_type),
            },
            FieldInfoData {
                name: "map",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, map),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetrySDKPinEventHeaderModifier, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(TELEMETRYSDKPINEVENTHEADERMODIFIER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetrySDKPinEventHeaderModifier {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSDKPINEVENTHEADERMODIFIER_TYPE_INFO
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


pub static TELEMETRYSDKPINEVENTHEADERMODIFIER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySDKPinEventHeaderModifier-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetrySDKPinEventHeaderModifier"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TransactionalTelemetryHookEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub stream: Option<Arc<Mutex<dyn TransactionalStreamDataTrait>>>,
}

pub trait TransactionalTelemetryHookEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn stream(&self) -> &Option<Arc<Mutex<dyn TransactionalStreamDataTrait>>>;
    fn stream_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TransactionalStreamDataTrait>>>;
}

impl TransactionalTelemetryHookEntityDataTrait for TransactionalTelemetryHookEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn stream(&self) -> &Option<Arc<Mutex<dyn TransactionalStreamDataTrait>>> {
        &self.stream
    }
    fn stream_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TransactionalStreamDataTrait>>> {
        &mut self.stream
    }
}

impl super::entity::EntityDataTrait for TransactionalTelemetryHookEntityData {
}

impl super::entity::GameObjectDataTrait for TransactionalTelemetryHookEntityData {
}

impl super::core::DataBusPeerTrait for TransactionalTelemetryHookEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TransactionalTelemetryHookEntityData {
}

impl super::core::DataContainerTrait for TransactionalTelemetryHookEntityData {
}

pub static TRANSACTIONALTELEMETRYHOOKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryHookEntityData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransactionalTelemetryHookEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(TransactionalTelemetryHookEntityData, realm),
            },
            FieldInfoData {
                name: "Stream",
                flags: MemberInfoFlags::new(0),
                field_type: "TransactionalStreamData",
                rust_offset: offset_of!(TransactionalTelemetryHookEntityData, stream),
            },
        ],
    }),
    array_type: Some(TRANSACTIONALTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TransactionalTelemetryHookEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSACTIONALTELEMETRYHOOKENTITYDATA_TYPE_INFO
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


pub static TRANSACTIONALTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransactionalTelemetryHookEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TransactionalTelemetryHookEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct VarStreamTelemetryHookEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub r#mod: String,
    pub grp: String,
    pub subgrp: String,
    pub stream: Option<Arc<Mutex<dyn VarEventStreamDataTrait>>>,
}

pub trait VarStreamTelemetryHookEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn r#mod(&self) -> &String;
    fn r#mod_mut(&mut self) -> &mut String;
    fn grp(&self) -> &String;
    fn grp_mut(&mut self) -> &mut String;
    fn subgrp(&self) -> &String;
    fn subgrp_mut(&mut self) -> &mut String;
    fn stream(&self) -> &Option<Arc<Mutex<dyn VarEventStreamDataTrait>>>;
    fn stream_mut(&mut self) -> &mut Option<Arc<Mutex<dyn VarEventStreamDataTrait>>>;
}

impl VarStreamTelemetryHookEntityDataTrait for VarStreamTelemetryHookEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn r#mod(&self) -> &String {
        &self.r#mod
    }
    fn r#mod_mut(&mut self) -> &mut String {
        &mut self.r#mod
    }
    fn grp(&self) -> &String {
        &self.grp
    }
    fn grp_mut(&mut self) -> &mut String {
        &mut self.grp
    }
    fn subgrp(&self) -> &String {
        &self.subgrp
    }
    fn subgrp_mut(&mut self) -> &mut String {
        &mut self.subgrp
    }
    fn stream(&self) -> &Option<Arc<Mutex<dyn VarEventStreamDataTrait>>> {
        &self.stream
    }
    fn stream_mut(&mut self) -> &mut Option<Arc<Mutex<dyn VarEventStreamDataTrait>>> {
        &mut self.stream
    }
}

impl super::entity::EntityDataTrait for VarStreamTelemetryHookEntityData {
}

impl super::entity::GameObjectDataTrait for VarStreamTelemetryHookEntityData {
}

impl super::core::DataBusPeerTrait for VarStreamTelemetryHookEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for VarStreamTelemetryHookEntityData {
}

impl super::core::DataContainerTrait for VarStreamTelemetryHookEntityData {
}

pub static VARSTREAMTELEMETRYHOOKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarStreamTelemetryHookEntityData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VarStreamTelemetryHookEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(VarStreamTelemetryHookEntityData, realm),
            },
            FieldInfoData {
                name: "mod",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VarStreamTelemetryHookEntityData, r#mod),
            },
            FieldInfoData {
                name: "grp",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VarStreamTelemetryHookEntityData, grp),
            },
            FieldInfoData {
                name: "subgrp",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VarStreamTelemetryHookEntityData, subgrp),
            },
            FieldInfoData {
                name: "Stream",
                flags: MemberInfoFlags::new(0),
                field_type: "VarEventStreamData",
                rust_offset: offset_of!(VarStreamTelemetryHookEntityData, stream),
            },
        ],
    }),
    array_type: Some(VARSTREAMTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VarStreamTelemetryHookEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VARSTREAMTELEMETRYHOOKENTITYDATA_TYPE_INFO
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


pub static VARSTREAMTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VarStreamTelemetryHookEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("VarStreamTelemetryHookEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FixedStreamTelemetryHookEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub stream: Option<Arc<Mutex<dyn FixedEventStreamDataTrait>>>,
    pub has_telemetry_sdk3_event_fields: bool,
    pub r#mod: String,
    pub grp: String,
    pub subgrp: String,
}

pub trait FixedStreamTelemetryHookEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn stream(&self) -> &Option<Arc<Mutex<dyn FixedEventStreamDataTrait>>>;
    fn stream_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FixedEventStreamDataTrait>>>;
    fn has_telemetry_sdk3_event_fields(&self) -> &bool;
    fn has_telemetry_sdk3_event_fields_mut(&mut self) -> &mut bool;
    fn r#mod(&self) -> &String;
    fn r#mod_mut(&mut self) -> &mut String;
    fn grp(&self) -> &String;
    fn grp_mut(&mut self) -> &mut String;
    fn subgrp(&self) -> &String;
    fn subgrp_mut(&mut self) -> &mut String;
}

impl FixedStreamTelemetryHookEntityDataTrait for FixedStreamTelemetryHookEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn stream(&self) -> &Option<Arc<Mutex<dyn FixedEventStreamDataTrait>>> {
        &self.stream
    }
    fn stream_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FixedEventStreamDataTrait>>> {
        &mut self.stream
    }
    fn has_telemetry_sdk3_event_fields(&self) -> &bool {
        &self.has_telemetry_sdk3_event_fields
    }
    fn has_telemetry_sdk3_event_fields_mut(&mut self) -> &mut bool {
        &mut self.has_telemetry_sdk3_event_fields
    }
    fn r#mod(&self) -> &String {
        &self.r#mod
    }
    fn r#mod_mut(&mut self) -> &mut String {
        &mut self.r#mod
    }
    fn grp(&self) -> &String {
        &self.grp
    }
    fn grp_mut(&mut self) -> &mut String {
        &mut self.grp
    }
    fn subgrp(&self) -> &String {
        &self.subgrp
    }
    fn subgrp_mut(&mut self) -> &mut String {
        &mut self.subgrp
    }
}

impl super::entity::EntityDataTrait for FixedStreamTelemetryHookEntityData {
}

impl super::entity::GameObjectDataTrait for FixedStreamTelemetryHookEntityData {
}

impl super::core::DataBusPeerTrait for FixedStreamTelemetryHookEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for FixedStreamTelemetryHookEntityData {
}

impl super::core::DataContainerTrait for FixedStreamTelemetryHookEntityData {
}

pub static FIXEDSTREAMTELEMETRYHOOKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedStreamTelemetryHookEntityData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FixedStreamTelemetryHookEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, realm),
            },
            FieldInfoData {
                name: "Stream",
                flags: MemberInfoFlags::new(0),
                field_type: "FixedEventStreamData",
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, stream),
            },
            FieldInfoData {
                name: "HasTelemetrySdk3EventFields",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, has_telemetry_sdk3_event_fields),
            },
            FieldInfoData {
                name: "mod",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, r#mod),
            },
            FieldInfoData {
                name: "grp",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, grp),
            },
            FieldInfoData {
                name: "subgrp",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(FixedStreamTelemetryHookEntityData, subgrp),
            },
        ],
    }),
    array_type: Some(FIXEDSTREAMTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FixedStreamTelemetryHookEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FIXEDSTREAMTELEMETRYHOOKENTITYDATA_TYPE_INFO
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


pub static FIXEDSTREAMTELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FixedStreamTelemetryHookEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("FixedStreamTelemetryHookEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub r#mod: String,
    pub grp: String,
    pub subgrp: String,
    pub params: Vec<TelemetryParameterDataProperty>,
    pub transports: Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>>,
}

pub trait TelemetryHookEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn r#mod(&self) -> &String;
    fn r#mod_mut(&mut self) -> &mut String;
    fn grp(&self) -> &String;
    fn grp_mut(&mut self) -> &mut String;
    fn subgrp(&self) -> &String;
    fn subgrp_mut(&mut self) -> &mut String;
    fn params(&self) -> &Vec<TelemetryParameterDataProperty>;
    fn params_mut(&mut self) -> &mut Vec<TelemetryParameterDataProperty>;
    fn transports(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>>;
    fn transports_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>>;
}

impl TelemetryHookEntityDataTrait for TelemetryHookEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn r#mod(&self) -> &String {
        &self.r#mod
    }
    fn r#mod_mut(&mut self) -> &mut String {
        &mut self.r#mod
    }
    fn grp(&self) -> &String {
        &self.grp
    }
    fn grp_mut(&mut self) -> &mut String {
        &mut self.grp
    }
    fn subgrp(&self) -> &String {
        &self.subgrp
    }
    fn subgrp_mut(&mut self) -> &mut String {
        &mut self.subgrp
    }
    fn params(&self) -> &Vec<TelemetryParameterDataProperty> {
        &self.params
    }
    fn params_mut(&mut self) -> &mut Vec<TelemetryParameterDataProperty> {
        &mut self.params
    }
    fn transports(&self) -> &Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        &self.transports
    }
    fn transports_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TelemetryTransportDataTrait>>>> {
        &mut self.transports
    }
}

impl super::entity::EntityDataTrait for TelemetryHookEntityData {
}

impl super::entity::GameObjectDataTrait for TelemetryHookEntityData {
}

impl super::core::DataBusPeerTrait for TelemetryHookEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TelemetryHookEntityData {
}

impl super::core::DataContainerTrait for TelemetryHookEntityData {
}

pub static TELEMETRYHOOKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookEntityData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(TelemetryHookEntityData, realm),
            },
            FieldInfoData {
                name: "mod",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookEntityData, r#mod),
            },
            FieldInfoData {
                name: "grp",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookEntityData, grp),
            },
            FieldInfoData {
                name: "subgrp",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookEntityData, subgrp),
            },
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(144),
                field_type: "TelemetryParameterDataProperty-Array",
                rust_offset: offset_of!(TelemetryHookEntityData, params),
            },
            FieldInfoData {
                name: "Transports",
                flags: MemberInfoFlags::new(144),
                field_type: "TelemetryTransportData-Array",
                rust_offset: offset_of!(TelemetryHookEntityData, transports),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKENTITYDATA_TYPE_INFO
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


pub static TELEMETRYHOOKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryGenericHookEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub stream: Option<Arc<Mutex<dyn EventStreamDataTrait>>>,
    pub log_event: Option<Arc<Mutex<dyn TelemetryLogEventTrait>>>,
}

pub trait TelemetryGenericHookEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn stream(&self) -> &Option<Arc<Mutex<dyn EventStreamDataTrait>>>;
    fn stream_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EventStreamDataTrait>>>;
    fn log_event(&self) -> &Option<Arc<Mutex<dyn TelemetryLogEventTrait>>>;
    fn log_event_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TelemetryLogEventTrait>>>;
}

impl TelemetryGenericHookEntityDataTrait for TelemetryGenericHookEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn stream(&self) -> &Option<Arc<Mutex<dyn EventStreamDataTrait>>> {
        &self.stream
    }
    fn stream_mut(&mut self) -> &mut Option<Arc<Mutex<dyn EventStreamDataTrait>>> {
        &mut self.stream
    }
    fn log_event(&self) -> &Option<Arc<Mutex<dyn TelemetryLogEventTrait>>> {
        &self.log_event
    }
    fn log_event_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TelemetryLogEventTrait>>> {
        &mut self.log_event
    }
}

impl super::entity::EntityDataTrait for TelemetryGenericHookEntityData {
}

impl super::entity::GameObjectDataTrait for TelemetryGenericHookEntityData {
}

impl super::core::DataBusPeerTrait for TelemetryGenericHookEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TelemetryGenericHookEntityData {
}

impl super::core::DataContainerTrait for TelemetryGenericHookEntityData {
}

pub static TELEMETRYGENERICHOOKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryGenericHookEntityData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryGenericHookEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(TelemetryGenericHookEntityData, realm),
            },
            FieldInfoData {
                name: "Stream",
                flags: MemberInfoFlags::new(0),
                field_type: "EventStreamData",
                rust_offset: offset_of!(TelemetryGenericHookEntityData, stream),
            },
            FieldInfoData {
                name: "LogEvent",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryLogEvent",
                rust_offset: offset_of!(TelemetryGenericHookEntityData, log_event),
            },
        ],
    }),
    array_type: Some(TELEMETRYGENERICHOOKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryGenericHookEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYGENERICHOOKENTITYDATA_TYPE_INFO
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


pub static TELEMETRYGENERICHOOKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryGenericHookEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryGenericHookEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryClearTelemetryTransactionMessageBase {
}

pub trait TelemetryClearTelemetryTransactionMessageBaseTrait: TypeObject {
}

impl TelemetryClearTelemetryTransactionMessageBaseTrait for TelemetryClearTelemetryTransactionMessageBase {
}

pub static TELEMETRYCLEARTELEMETRYTRANSACTIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryClearTelemetryTransactionMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryClearTelemetryTransactionMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetryClearTelemetryTransactionMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYCLEARTELEMETRYTRANSACTIONMESSAGEBASE_TYPE_INFO
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
pub struct TelemetryCommitTelemetryTransactionMessageBase {
}

pub trait TelemetryCommitTelemetryTransactionMessageBaseTrait: TypeObject {
}

impl TelemetryCommitTelemetryTransactionMessageBaseTrait for TelemetryCommitTelemetryTransactionMessageBase {
}

pub static TELEMETRYCOMMITTELEMETRYTRANSACTIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryCommitTelemetryTransactionMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryCommitTelemetryTransactionMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetryCommitTelemetryTransactionMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYCOMMITTELEMETRYTRANSACTIONMESSAGEBASE_TYPE_INFO
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
pub struct TelemetrySendTelemetryTransactionRowMessageBase {
}

pub trait TelemetrySendTelemetryTransactionRowMessageBaseTrait: TypeObject {
}

impl TelemetrySendTelemetryTransactionRowMessageBaseTrait for TelemetrySendTelemetryTransactionRowMessageBase {
}

pub static TELEMETRYSENDTELEMETRYTRANSACTIONROWMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySendTelemetryTransactionRowMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySendTelemetryTransactionRowMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetrySendTelemetryTransactionRowMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSENDTELEMETRYTRANSACTIONROWMESSAGEBASE_TYPE_INFO
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
pub struct TelemetrySendTelemetryRowMessageBase {
}

pub trait TelemetrySendTelemetryRowMessageBaseTrait: TypeObject {
}

impl TelemetrySendTelemetryRowMessageBaseTrait for TelemetrySendTelemetryRowMessageBase {
}

pub static TELEMETRYSENDTELEMETRYROWMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySendTelemetryRowMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySendTelemetryRowMessageBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for TelemetrySendTelemetryRowMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSENDTELEMETRYROWMESSAGEBASE_TYPE_INFO
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
pub struct TelemetrySendEventMessage {
}

pub trait TelemetrySendEventMessageTrait: TypeObject {
}

impl TelemetrySendEventMessageTrait for TelemetrySendEventMessage {
}

pub static TELEMETRYSENDEVENTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetrySendEventMessage",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetrySendEventMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for TelemetrySendEventMessage {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYSENDEVENTMESSAGE_TYPE_INFO
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
pub struct TelemetryHookParameterStringArray {
    pub data: Vec<String>,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterStringArrayTrait: TypeObject {
    fn data(&self) -> &Vec<String>;
    fn data_mut(&mut self) -> &mut Vec<String>;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterStringArrayTrait for TelemetryHookParameterStringArray {
    fn data(&self) -> &Vec<String> {
        &self.data
    }
    fn data_mut(&mut self) -> &mut Vec<String> {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERSTRINGARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterStringArray",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterStringArray as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(TelemetryHookParameterStringArray, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterStringArray, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterStringArray, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERSTRINGARRAY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterStringArray {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERSTRINGARRAY_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERSTRINGARRAY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterStringArray-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterStringArray"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterRawJsonString {
    pub data: RawJsonString,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterRawJsonStringTrait: TypeObject {
    fn data(&self) -> &RawJsonString;
    fn data_mut(&mut self) -> &mut RawJsonString;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterRawJsonStringTrait for TelemetryHookParameterRawJsonString {
    fn data(&self) -> &RawJsonString {
        &self.data
    }
    fn data_mut(&mut self) -> &mut RawJsonString {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERRAWJSONSTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterRawJsonString",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterRawJsonString as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "RawJsonString",
                rust_offset: offset_of!(TelemetryHookParameterRawJsonString, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterRawJsonString, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterRawJsonString, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERRAWJSONSTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterRawJsonString {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERRAWJSONSTRING_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERRAWJSONSTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterRawJsonString-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterRawJsonString"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterTransform {
    pub data: super::core::LinearTransform,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterTransformTrait: TypeObject {
    fn data(&self) -> &super::core::LinearTransform;
    fn data_mut(&mut self) -> &mut super::core::LinearTransform;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterTransformTrait for TelemetryHookParameterTransform {
    fn data(&self) -> &super::core::LinearTransform {
        &self.data
    }
    fn data_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERTRANSFORM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterTransform",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterTransform as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(TelemetryHookParameterTransform, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterTransform, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterTransform, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERTRANSFORM_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetryHookParameterTransform {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERTRANSFORM_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERTRANSFORM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterTransform-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterTransform"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterVec4 {
    pub data: super::core::Vec4,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterVec4Trait: TypeObject {
    fn data(&self) -> &super::core::Vec4;
    fn data_mut(&mut self) -> &mut super::core::Vec4;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterVec4Trait for TelemetryHookParameterVec4 {
    fn data(&self) -> &super::core::Vec4 {
        &self.data
    }
    fn data_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERVEC4_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec4",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterVec4 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(TelemetryHookParameterVec4, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterVec4, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterVec4, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERVEC4_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetryHookParameterVec4 {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERVEC4_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERVEC4_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec4-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterVec4"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterVec3 {
    pub data: super::core::Vec3,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterVec3Trait: TypeObject {
    fn data(&self) -> &super::core::Vec3;
    fn data_mut(&mut self) -> &mut super::core::Vec3;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterVec3Trait for TelemetryHookParameterVec3 {
    fn data(&self) -> &super::core::Vec3 {
        &self.data
    }
    fn data_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERVEC3_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec3",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterVec3 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(TelemetryHookParameterVec3, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterVec3, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterVec3, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERVEC3_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TelemetryHookParameterVec3 {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERVEC3_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERVEC3_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec3-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterVec3"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterVec2 {
    pub data: super::core::Vec2,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterVec2Trait: TypeObject {
    fn data(&self) -> &super::core::Vec2;
    fn data_mut(&mut self) -> &mut super::core::Vec2;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterVec2Trait for TelemetryHookParameterVec2 {
    fn data(&self) -> &super::core::Vec2 {
        &self.data
    }
    fn data_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERVEC2_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec2",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterVec2 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(TelemetryHookParameterVec2, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterVec2, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterVec2, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERVEC2_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterVec2 {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERVEC2_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERVEC2_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterVec2-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterVec2"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterBool {
    pub data: bool,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterBoolTrait: TypeObject {
    fn data(&self) -> &bool;
    fn data_mut(&mut self) -> &mut bool;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterBoolTrait for TelemetryHookParameterBool {
    fn data(&self) -> &bool {
        &self.data
    }
    fn data_mut(&mut self) -> &mut bool {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERBOOL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterBool",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterBool as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TelemetryHookParameterBool, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterBool, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterBool, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERBOOL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterBool {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERBOOL_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERBOOL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterBool-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterBool"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterUint64 {
    pub data: u64,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterUint64Trait: TypeObject {
    fn data(&self) -> &u64;
    fn data_mut(&mut self) -> &mut u64;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterUint64Trait for TelemetryHookParameterUint64 {
    fn data(&self) -> &u64 {
        &self.data
    }
    fn data_mut(&mut self) -> &mut u64 {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERUINT64_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterUint64",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterUint64 as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(TelemetryHookParameterUint64, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterUint64, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterUint64, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERUINT64_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterUint64 {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERUINT64_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERUINT64_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterUint64-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterUint64"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterUint {
    pub data: u32,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterUintTrait: TypeObject {
    fn data(&self) -> &u32;
    fn data_mut(&mut self) -> &mut u32;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterUintTrait for TelemetryHookParameterUint {
    fn data(&self) -> &u32 {
        &self.data
    }
    fn data_mut(&mut self) -> &mut u32 {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERUINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterUint",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterUint as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(TelemetryHookParameterUint, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterUint, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterUint, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERUINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterUint {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERUINT_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERUINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterUint-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterUint"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterString {
    pub data: String,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterStringTrait: TypeObject {
    fn data(&self) -> &String;
    fn data_mut(&mut self) -> &mut String;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterStringTrait for TelemetryHookParameterString {
    fn data(&self) -> &String {
        &self.data
    }
    fn data_mut(&mut self) -> &mut String {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERSTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterString",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterString as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterString, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterString, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterString, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERSTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterString {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERSTRING_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERSTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterString-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterString"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterFloat {
    pub data: f32,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterFloatTrait: TypeObject {
    fn data(&self) -> &f32;
    fn data_mut(&mut self) -> &mut f32;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterFloatTrait for TelemetryHookParameterFloat {
    fn data(&self) -> &f32 {
        &self.data
    }
    fn data_mut(&mut self) -> &mut f32 {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERFLOAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterFloat",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterFloat as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TelemetryHookParameterFloat, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterFloat, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterFloat, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERFLOAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterFloat {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERFLOAT_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERFLOAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterFloat-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterFloat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterInt {
    pub data: i32,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterIntTrait: TypeObject {
    fn data(&self) -> &i32;
    fn data_mut(&mut self) -> &mut i32;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterIntTrait for TelemetryHookParameterInt {
    fn data(&self) -> &i32 {
        &self.data
    }
    fn data_mut(&mut self) -> &mut i32 {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterInt",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterInt as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(TelemetryHookParameterInt, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterInt, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterInt, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterInt {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERINT_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterInt-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterInt"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameterChar {
    pub data: u8,
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterCharTrait: TypeObject {
    fn data(&self) -> &u8;
    fn data_mut(&mut self) -> &mut u8;
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterCharTrait for TelemetryHookParameterChar {
    fn data(&self) -> &u8 {
        &self.data
    }
    fn data_mut(&mut self) -> &mut u8 {
        &mut self.data
    }
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETERCHAR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterChar",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameterChar as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Data",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TelemetryHookParameterChar, data),
            },
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameterChar, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameterChar, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETERCHAR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameterChar {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETERCHAR_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETERCHAR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameterChar-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameterChar"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryHookParameter {
    pub parameter_type: TelemetryParameterType,
    pub parameter_name: String,
}

pub trait TelemetryHookParameterTrait: TypeObject {
    fn parameter_type(&self) -> &TelemetryParameterType;
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType;
    fn parameter_name(&self) -> &String;
    fn parameter_name_mut(&mut self) -> &mut String;
}

impl TelemetryHookParameterTrait for TelemetryHookParameter {
    fn parameter_type(&self) -> &TelemetryParameterType {
        &self.parameter_type
    }
    fn parameter_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.parameter_type
    }
    fn parameter_name(&self) -> &String {
        &self.parameter_name
    }
    fn parameter_name_mut(&mut self) -> &mut String {
        &mut self.parameter_name
    }
}

pub static TELEMETRYHOOKPARAMETER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameter",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryHookParameter as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ParameterType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryHookParameter, parameter_type),
            },
            FieldInfoData {
                name: "ParameterName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryHookParameter, parameter_name),
            },
        ],
    }),
    array_type: Some(TELEMETRYHOOKPARAMETER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryHookParameter {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYHOOKPARAMETER_TYPE_INFO
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


pub static TELEMETRYHOOKPARAMETER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryHookParameter-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryHookParameter"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TelemetryParameterDataProperty {
    pub stat_name: String,
    pub data_type: TelemetryParameterType,
}

pub trait TelemetryParameterDataPropertyTrait: TypeObject {
    fn stat_name(&self) -> &String;
    fn stat_name_mut(&mut self) -> &mut String;
    fn data_type(&self) -> &TelemetryParameterType;
    fn data_type_mut(&mut self) -> &mut TelemetryParameterType;
}

impl TelemetryParameterDataPropertyTrait for TelemetryParameterDataProperty {
    fn stat_name(&self) -> &String {
        &self.stat_name
    }
    fn stat_name_mut(&mut self) -> &mut String {
        &mut self.stat_name
    }
    fn data_type(&self) -> &TelemetryParameterType {
        &self.data_type
    }
    fn data_type_mut(&mut self) -> &mut TelemetryParameterType {
        &mut self.data_type
    }
}

pub static TELEMETRYPARAMETERDATAPROPERTY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryParameterDataProperty",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TelemetryParameterDataProperty as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StatName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(TelemetryParameterDataProperty, stat_name),
            },
            FieldInfoData {
                name: "DataType",
                flags: MemberInfoFlags::new(0),
                field_type: "TelemetryParameterType",
                rust_offset: offset_of!(TelemetryParameterDataProperty, data_type),
            },
        ],
    }),
    array_type: Some(TELEMETRYPARAMETERDATAPROPERTY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TelemetryParameterDataProperty {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYPARAMETERDATAPROPERTY_TYPE_INFO
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


pub static TELEMETRYPARAMETERDATAPROPERTY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryParameterDataProperty-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryParameterDataProperty"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RawJsonString {
    pub raw_json: String,
}

pub trait RawJsonStringTrait: TypeObject {
    fn raw_json(&self) -> &String;
    fn raw_json_mut(&mut self) -> &mut String;
}

impl RawJsonStringTrait for RawJsonString {
    fn raw_json(&self) -> &String {
        &self.raw_json
    }
    fn raw_json_mut(&mut self) -> &mut String {
        &mut self.raw_json
    }
}

pub static RAWJSONSTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RawJsonString",
    flags: MemberInfoFlags::new(73),
    module: "TelemetryShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RawJsonString as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RawJson",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(RawJsonString, raw_json),
            },
        ],
    }),
    array_type: Some(RAWJSONSTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RawJsonString {
    fn type_info(&self) -> &'static TypeInfo {
        RAWJSONSTRING_TYPE_INFO
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


pub static RAWJSONSTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RawJsonString-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("RawJsonString"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SpecialTypeData {
    pub _glacier_base: super::core::DataContainer,
    pub owns_value: bool,
}

pub trait SpecialTypeDataTrait: super::core::DataContainerTrait {
    fn owns_value(&self) -> &bool;
    fn owns_value_mut(&mut self) -> &mut bool;
}

impl SpecialTypeDataTrait for SpecialTypeData {
    fn owns_value(&self) -> &bool {
        &self.owns_value
    }
    fn owns_value_mut(&mut self) -> &mut bool {
        &mut self.owns_value
    }
}

impl super::core::DataContainerTrait for SpecialTypeData {
}

pub static SPECIALTYPEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpecialTypeData",
    flags: MemberInfoFlags::new(101),
    module: "TelemetryShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SpecialTypeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "OwnsValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SpecialTypeData, owns_value),
            },
        ],
    }),
    array_type: Some(SPECIALTYPEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SpecialTypeData {
    fn type_info(&self) -> &'static TypeInfo {
        SPECIALTYPEDATA_TYPE_INFO
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


pub static SPECIALTYPEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SpecialTypeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("SpecialTypeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static TELEMETRYPARAMETERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryParameterType",
    flags: MemberInfoFlags::new(49429),
    module: "TelemetryShared",
    data: TypeInfoData::Enum,
    array_type: Some(TELEMETRYPARAMETERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TelemetryParameterType {
    fn type_info(&self) -> &'static TypeInfo {
        TELEMETRYPARAMETERTYPE_TYPE_INFO
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


pub static TELEMETRYPARAMETERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TelemetryParameterType-Array",
    flags: MemberInfoFlags::new(145),
    module: "TelemetryShared",
    data: TypeInfoData::Array("TelemetryParameterType"),
    array_type: None,
    alignment: 8,
};


