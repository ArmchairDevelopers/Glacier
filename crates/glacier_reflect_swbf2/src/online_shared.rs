use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_online_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(ONLINESETTINGS_TYPE_INFO);
    registry.register_type(ONLINESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(LOGLEVELTYPE_TYPE_INFO);
    registry.register_type(LOGLEVELTYPE_ARRAY_TYPE_INFO);
    registry.register_type(ONLINESERVICESASSET_TYPE_INFO);
    registry.register_type(ONLINESERVICESASSET_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEPROFANITYFILTERSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEPROFANITYFILTERSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEUSERPROFILESERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEUSERPROFILESERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEINVITESERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEINVITESERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEPRIVILEGESERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEPRIVILEGESERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEUSERIDSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEUSERIDSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEFRIENDSSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEFRIENDSSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEBLOBSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEBLOBSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEBLAZEUSERINFOSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEBLAZEUSERINFOSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEAUTHENTICATIONSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCEAUTHENTICATIONSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCECONNECTIONSERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCECONNECTIONSERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCESERVICEDATA_TYPE_INFO);
    registry.register_type(PRESENCESERVICEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEPROVIDERASSET_TYPE_INFO);
    registry.register_type(ONLINEPROVIDERASSET_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEPROVIDERCONFIGURATION_TYPE_INFO);
    registry.register_type(ONLINEPROVIDERCONFIGURATION_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEPLATFORMCONFIGURATION_TYPE_INFO);
    registry.register_type(ONLINEPLATFORMCONFIGURATION_ARRAY_TYPE_INFO);
    registry.register_type(PS4ONLINEDATA_TYPE_INFO);
    registry.register_type(PS4ONLINEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PS4ONLINETITLEDATA_TYPE_INFO);
    registry.register_type(PS4ONLINETITLEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEPLATFORMDATA_TYPE_INFO);
    registry.register_type(ONLINEPLATFORMDATA_ARRAY_TYPE_INFO);
    registry.register_type(ONLINERICHPRESENCEDATA_TYPE_INFO);
    registry.register_type(ONLINERICHPRESENCEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ONLINERICHPRESENCECONTEXTVALUEPAIR_TYPE_INFO);
    registry.register_type(ONLINERICHPRESENCECONTEXTVALUEPAIR_ARRAY_TYPE_INFO);
    registry.register_type(ONLINERICHPRESENCESTRING_TYPE_INFO);
    registry.register_type(ONLINERICHPRESENCESTRING_ARRAY_TYPE_INFO);
    registry.register_type(ONLINERICHPRESENCECONTEXT_TYPE_INFO);
    registry.register_type(ONLINERICHPRESENCECONTEXT_ARRAY_TYPE_INFO);
    registry.register_type(ONLINERICHPRESENCECONTEXTVALUE_TYPE_INFO);
    registry.register_type(ONLINERICHPRESENCECONTEXTVALUE_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEENVIRONMENTCONSOLEURL_TYPE_INFO);
    registry.register_type(ONLINEENVIRONMENTCONSOLEURL_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEENVIRONMENTCONSOLEURLDATA_TYPE_INFO);
    registry.register_type(ONLINEENVIRONMENTCONSOLEURLDATA_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEENVIRONMENTURL_TYPE_INFO);
    registry.register_type(ONLINEENVIRONMENTURL_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEENVIRONMENTURLDATA_TYPE_INFO);
    registry.register_type(ONLINEENVIRONMENTURLDATA_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEENVIRONMENT_TYPE_INFO);
    registry.register_type(ONLINEENVIRONMENT_ARRAY_TYPE_INFO);
    registry.register_type(ORIGINPRESENCEBACKENDDATA_TYPE_INFO);
    registry.register_type(ORIGINPRESENCEBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(BLAZEINPROCSERVERBACKENDDATA_TYPE_INFO);
    registry.register_type(BLAZEINPROCSERVERBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(BLAZESERVERBACKENDDATA_TYPE_INFO);
    registry.register_type(BLAZESERVERBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(LANSERVERBACKENDDATA_TYPE_INFO);
    registry.register_type(LANSERVERBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALSERVERBACKENDDATA_TYPE_INFO);
    registry.register_type(LOCALSERVERBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(DURANGOPRESENCEBACKENDDATA_TYPE_INFO);
    registry.register_type(DURANGOPRESENCEBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PS4PRESENCEBACKENDDATA_TYPE_INFO);
    registry.register_type(PS4PRESENCEBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PS4AGESETTINGS_TYPE_INFO);
    registry.register_type(PS4AGESETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(PS4COUNTRYAGEOVERRIDES_TYPE_INFO);
    registry.register_type(PS4COUNTRYAGEOVERRIDES_ARRAY_TYPE_INFO);
    registry.register_type(NUCLEUSPRESENCEBACKENDDATA_TYPE_INFO);
    registry.register_type(NUCLEUSPRESENCEBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(LANPRESENCEBACKENDDATA_TYPE_INFO);
    registry.register_type(LANPRESENCEBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(DIRTYSOCKPRESENCEBACKENDDATA_TYPE_INFO);
    registry.register_type(DIRTYSOCKPRESENCEBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(BLAZEPRESENCEBACKENDDATA_TYPE_INFO);
    registry.register_type(BLAZEPRESENCEBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PLATFORMFETCHLICENSE_TYPE_INFO);
    registry.register_type(PLATFORMFETCHLICENSE_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEBACKENDDATA_TYPE_INFO);
    registry.register_type(PRESENCEBACKENDDATA_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEMODE_TYPE_INFO);
    registry.register_type(PRESENCEMODE_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKINVITETOKEN_TYPE_INFO);
    registry.register_type(NETWORKINVITETOKEN_ARRAY_TYPE_INFO);
    registry.register_type(NETWORKINVITETOKENPLAYER_TYPE_INFO);
    registry.register_type(NETWORKINVITETOKENPLAYER_ARRAY_TYPE_INFO);
    registry.register_type(INVITEIDSTRINGCONSTANTS_TYPE_INFO);
    registry.register_type(INVITEIDSTRINGCONSTANTS_ARRAY_TYPE_INFO);
    registry.register_type(INVITEIDTYPE_TYPE_INFO);
    registry.register_type(INVITEIDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(INVITEJOINMETHOD_TYPE_INFO);
    registry.register_type(INVITEJOINMETHOD_ARRAY_TYPE_INFO);
    registry.register_type(INVITETYPE_TYPE_INFO);
    registry.register_type(INVITETYPE_ARRAY_TYPE_INFO);
    registry.register_type(ONLINEPRIVILEGE_TYPE_INFO);
    registry.register_type(ONLINEPRIVILEGE_ARRAY_TYPE_INFO);
    registry.register_type(PRESENCEPLAYTOGETHERREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEGAMEGROUPUPDATEDMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCELOGINLICENSEREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCELOGINLICENSEMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEBLAZEAUTOACCOUNTLOGINMESSAGE_TYPE_INFO);
    registry.register_type(PRESENCEJOINREMOTEGAMEMESSAGE_TYPE_INFO);
    registry.register_type(PRESENCECOMMITPLAYERTOGAMEMESSAGE_TYPE_INFO);
    registry.register_type(PRESENCEUSERPROFILEREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEUSERPROFILEMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEUSERIDREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEUSERIDMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEPSPLUSREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEPROFANITYFILTERRESPONSEMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEPROFANITYFILTERREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEPRIVILEGEREQUESTRESULTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEPRIVILEGEREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEORIGINUSERNAMEREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEORIGINUSERNAMEMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCELIVEPARTYMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEINVITEREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEINVITEMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEFRIENDSLISTMANAGERSETTINGSMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEFRIENDREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEFRIENDMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCECONNECTIONREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCECONNECTIONMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEBLOCKLISTREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEBLOCKLISTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEBLOBREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEBLOBMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEBLAZEUSERREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEBLAZEUSERMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEAUTHENTICATIONREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEAUTHENTICATIONMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEACCOUNTREQUESTMESSAGEBASE_TYPE_INFO);
    registry.register_type(PRESENCEACCOUNTMESSAGEBASE_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineSettings {
    pub assert_on_presence_request_failures: bool,
    pub client_is_presence_enabled: bool,
    pub server_is_presence_enabled: bool,
    pub environment: OnlineEnvironment,
    pub is_secure: bool,
    pub enable_qo_s: bool,
    pub wait_for_qo_s: bool,
    pub provider: OnlineProviderAsset,
    pub platforms: Vec<OnlinePlatformConfiguration>,
    pub service_name_override: String,
    pub log_level: LogLevelType,
    pub blaze_log_level: i32,
    pub dirty_sock_log_level: i32,
    pub rich_presence_data: OnlineRichPresenceData,
    pub region: String,
    pub country: String,
    pub ping_site: String,
    pub matchmaking_token: String,
    pub server_is_reconfigurable: bool,
    pub support_host_migration: bool,
    pub negative_user_cache_refresh_period: u32,
    pub server_login_email: String,
    pub server_login_password: String,
    pub server_login_persona_name: String,
    pub server_login_project_tag: String,
    pub blaze_server_connection_timeout: i32,
    pub blaze_server_timeout: i32,
    pub blaze_server_tunnel_socket_recv_buf_size: u32,
    pub blaze_server_tunnel_socket_send_buf_size: u32,
    pub blaze_outgoing_buffer_size: u32,
    pub blaze_client_connection_timeout: i32,
    pub blaze_client_timeout: i32,
    pub blaze_client_tunnel_socket_recv_buf_size: u32,
    pub blaze_client_tunnel_socket_send_buf_size: u32,
    pub server_allow_any_reputation: bool,
    pub peer_port: i32,
    pub enable_gamegroup_invites: bool,
    pub dirty_sock_server_packet_queue_capacity: i32,
    pub dirty_sock_max_connection_count: u32,
    pub blaze_cached_user_refresh_interval: u32,
    pub trusted_login_path: String,
    pub trusted_login_cert_filename: String,
    pub trusted_login_key_filename: String,
    pub enable_nucleus_lt_override: bool,
    pub min_player_capacity: u32,
    pub should_control_dirty_sock: bool,
    pub debug_message_callstack_type_list: String,
    pub override_create_game_template: bool,
    pub override_create_game_template_name: String,
    pub resettable_pool: String,
}

pub const ONLINESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineSettings",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AssertOnPresenceRequestFailures",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, assert_on_presence_request_failures),
            },
            FieldInfoData {
                name: "ClientIsPresenceEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, client_is_presence_enabled),
            },
            FieldInfoData {
                name: "ServerIsPresenceEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, server_is_presence_enabled),
            },
            FieldInfoData {
                name: "Environment",
                flags: MemberInfoFlags::new(0),
                field_type: ONLINEENVIRONMENT_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, environment),
            },
            FieldInfoData {
                name: "IsSecure",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, is_secure),
            },
            FieldInfoData {
                name: "EnableQoS",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, enable_qo_s),
            },
            FieldInfoData {
                name: "WaitForQoS",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, wait_for_qo_s),
            },
            FieldInfoData {
                name: "Provider",
                flags: MemberInfoFlags::new(0),
                field_type: ONLINEPROVIDERASSET_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, provider),
            },
            FieldInfoData {
                name: "Platforms",
                flags: MemberInfoFlags::new(144),
                field_type: ONLINEPLATFORMCONFIGURATION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, platforms),
            },
            FieldInfoData {
                name: "ServiceNameOverride",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, service_name_override),
            },
            FieldInfoData {
                name: "LogLevel",
                flags: MemberInfoFlags::new(0),
                field_type: LOGLEVELTYPE_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, log_level),
            },
            FieldInfoData {
                name: "BlazeLogLevel",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, blaze_log_level),
            },
            FieldInfoData {
                name: "DirtySockLogLevel",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, dirty_sock_log_level),
            },
            FieldInfoData {
                name: "RichPresenceData",
                flags: MemberInfoFlags::new(0),
                field_type: ONLINERICHPRESENCEDATA_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, rich_presence_data),
            },
            FieldInfoData {
                name: "Region",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, region),
            },
            FieldInfoData {
                name: "Country",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, country),
            },
            FieldInfoData {
                name: "PingSite",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, ping_site),
            },
            FieldInfoData {
                name: "MatchmakingToken",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, matchmaking_token),
            },
            FieldInfoData {
                name: "ServerIsReconfigurable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, server_is_reconfigurable),
            },
            FieldInfoData {
                name: "SupportHostMigration",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, support_host_migration),
            },
            FieldInfoData {
                name: "NegativeUserCacheRefreshPeriod",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, negative_user_cache_refresh_period),
            },
            FieldInfoData {
                name: "ServerLoginEmail",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, server_login_email),
            },
            FieldInfoData {
                name: "ServerLoginPassword",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, server_login_password),
            },
            FieldInfoData {
                name: "ServerLoginPersonaName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, server_login_persona_name),
            },
            FieldInfoData {
                name: "ServerLoginProjectTag",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, server_login_project_tag),
            },
            FieldInfoData {
                name: "BlazeServerConnectionTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, blaze_server_connection_timeout),
            },
            FieldInfoData {
                name: "BlazeServerTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, blaze_server_timeout),
            },
            FieldInfoData {
                name: "BlazeServerTunnelSocketRecvBufSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, blaze_server_tunnel_socket_recv_buf_size),
            },
            FieldInfoData {
                name: "BlazeServerTunnelSocketSendBufSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, blaze_server_tunnel_socket_send_buf_size),
            },
            FieldInfoData {
                name: "BlazeOutgoingBufferSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, blaze_outgoing_buffer_size),
            },
            FieldInfoData {
                name: "BlazeClientConnectionTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, blaze_client_connection_timeout),
            },
            FieldInfoData {
                name: "BlazeClientTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, blaze_client_timeout),
            },
            FieldInfoData {
                name: "BlazeClientTunnelSocketRecvBufSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, blaze_client_tunnel_socket_recv_buf_size),
            },
            FieldInfoData {
                name: "BlazeClientTunnelSocketSendBufSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, blaze_client_tunnel_socket_send_buf_size),
            },
            FieldInfoData {
                name: "ServerAllowAnyReputation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, server_allow_any_reputation),
            },
            FieldInfoData {
                name: "PeerPort",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, peer_port),
            },
            FieldInfoData {
                name: "EnableGamegroupInvites",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, enable_gamegroup_invites),
            },
            FieldInfoData {
                name: "DirtySockServerPacketQueueCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, dirty_sock_server_packet_queue_capacity),
            },
            FieldInfoData {
                name: "DirtySockMaxConnectionCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, dirty_sock_max_connection_count),
            },
            FieldInfoData {
                name: "BlazeCachedUserRefreshInterval",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, blaze_cached_user_refresh_interval),
            },
            FieldInfoData {
                name: "TrustedLoginPath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, trusted_login_path),
            },
            FieldInfoData {
                name: "TrustedLoginCertFilename",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, trusted_login_cert_filename),
            },
            FieldInfoData {
                name: "TrustedLoginKeyFilename",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, trusted_login_key_filename),
            },
            FieldInfoData {
                name: "EnableNucleusLtOverride",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, enable_nucleus_lt_override),
            },
            FieldInfoData {
                name: "MinPlayerCapacity",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, min_player_capacity),
            },
            FieldInfoData {
                name: "ShouldControlDirtySock",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, should_control_dirty_sock),
            },
            FieldInfoData {
                name: "DebugMessageCallstackTypeList",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, debug_message_callstack_type_list),
            },
            FieldInfoData {
                name: "OverrideCreateGameTemplate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, override_create_game_template),
            },
            FieldInfoData {
                name: "OverrideCreateGameTemplateName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, override_create_game_template_name),
            },
            FieldInfoData {
                name: "ResettablePool",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineSettings, resettable_pool),
            },
        ],
    }),
    array_type: Some(ONLINESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineSettings {
    fn type_info() -> &'static TypeInfo {
        ONLINESETTINGS_TYPE_INFO
    }
}


pub const ONLINESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LogLevelType {
    #[default]
    LogLevel_Default = 0,
    LogLevel_Fatal = 1,
    LogLevel_Error = 2,
    LogLevel_Warn = 3,
    LogLevel_Info = 4,
    LogLevel_Debug = 5,
    LogLevel_Trace = 6,
}

pub const LOGLEVELTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogLevelType",
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(LOGLEVELTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LogLevelType {
    fn type_info() -> &'static TypeInfo {
        LOGLEVELTYPE_TYPE_INFO
    }
}


pub const LOGLEVELTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogLevelType-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("LogLevelType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineServicesAsset {
    pub online_services: Vec<PresenceServiceData>,
}

pub const ONLINESERVICESASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineServicesAsset",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "OnlineServices",
                flags: MemberInfoFlags::new(144),
                field_type: PRESENCESERVICEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OnlineServicesAsset, online_services),
            },
        ],
    }),
    array_type: Some(ONLINESERVICESASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineServicesAsset {
    fn type_info() -> &'static TypeInfo {
        ONLINESERVICESASSET_TYPE_INFO
    }
}


pub const ONLINESERVICESASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineServicesAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineServicesAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceProfanityFilterServiceData {
}

pub const PRESENCEPROFANITYFILTERSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceProfanityFilterServiceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEPROFANITYFILTERSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceProfanityFilterServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPROFANITYFILTERSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEPROFANITYFILTERSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceProfanityFilterServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceProfanityFilterServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceUserProfileServiceData {
}

pub const PRESENCEUSERPROFILESERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserProfileServiceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEUSERPROFILESERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceUserProfileServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEUSERPROFILESERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEUSERPROFILESERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserProfileServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceUserProfileServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceInviteServiceData {
}

pub const PRESENCEINVITESERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceInviteServiceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEINVITESERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceInviteServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEINVITESERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEINVITESERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceInviteServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceInviteServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresencePrivilegeServiceData {
}

pub const PRESENCEPRIVILEGESERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePrivilegeServiceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEPRIVILEGESERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresencePrivilegeServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPRIVILEGESERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEPRIVILEGESERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePrivilegeServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresencePrivilegeServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceUserIdServiceData {
}

pub const PRESENCEUSERIDSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserIdServiceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEUSERIDSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceUserIdServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEUSERIDSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEUSERIDSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserIdServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceUserIdServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceFriendsServiceData {
}

pub const PRESENCEFRIENDSSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFriendsServiceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEFRIENDSSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceFriendsServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEFRIENDSSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEFRIENDSSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFriendsServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceFriendsServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceBlobServiceData {
}

pub const PRESENCEBLOBSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlobServiceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEBLOBSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceBlobServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEBLOBSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEBLOBSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlobServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceBlobServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceBlazeUserInfoServiceData {
    pub game_browser_config_name: String,
}

pub const PRESENCEBLAZEUSERINFOSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlazeUserInfoServiceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "GameBrowserConfigName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PresenceBlazeUserInfoServiceData, game_browser_config_name),
            },
        ],
    }),
    array_type: Some(PRESENCEBLAZEUSERINFOSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceBlazeUserInfoServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEBLAZEUSERINFOSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEBLAZEUSERINFOSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlazeUserInfoServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceBlazeUserInfoServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceAuthenticationServiceData {
}

pub const PRESENCEAUTHENTICATIONSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAuthenticationServiceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEAUTHENTICATIONSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceAuthenticationServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEAUTHENTICATIONSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCEAUTHENTICATIONSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAuthenticationServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceAuthenticationServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceConnectionServiceData {
}

pub const PRESENCECONNECTIONSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceConnectionServiceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCECONNECTIONSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceConnectionServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCECONNECTIONSERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCECONNECTIONSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceConnectionServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceConnectionServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceServiceData {
}

pub const PRESENCESERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceServiceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceServiceData {
    fn type_info() -> &'static TypeInfo {
        PRESENCESERVICEDATA_TYPE_INFO
    }
}


pub const PRESENCESERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceServiceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceServiceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineProviderAsset {
    pub configurations: Vec<OnlineProviderConfiguration>,
}

pub const ONLINEPROVIDERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineProviderAsset",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Configurations",
                flags: MemberInfoFlags::new(144),
                field_type: ONLINEPROVIDERCONFIGURATION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OnlineProviderAsset, configurations),
            },
        ],
    }),
    array_type: Some(ONLINEPROVIDERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineProviderAsset {
    fn type_info() -> &'static TypeInfo {
        ONLINEPROVIDERASSET_TYPE_INFO
    }
}


pub const ONLINEPROVIDERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineProviderAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineProviderAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineProviderConfiguration {
    pub platform: super::core::GamePlatform,
    pub is_server: bool,
    pub service_name: String,
    pub client: String,
    pub s_k_u: String,
    pub version: String,
    pub server_socket_packet_size: u32,
}

pub const ONLINEPROVIDERCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineProviderConfiguration",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(OnlineProviderConfiguration, platform),
            },
            FieldInfoData {
                name: "IsServer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlineProviderConfiguration, is_server),
            },
            FieldInfoData {
                name: "ServiceName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineProviderConfiguration, service_name),
            },
            FieldInfoData {
                name: "Client",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineProviderConfiguration, client),
            },
            FieldInfoData {
                name: "SKU",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineProviderConfiguration, s_k_u),
            },
            FieldInfoData {
                name: "Version",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineProviderConfiguration, version),
            },
            FieldInfoData {
                name: "ServerSocketPacketSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OnlineProviderConfiguration, server_socket_packet_size),
            },
        ],
    }),
    array_type: Some(ONLINEPROVIDERCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineProviderConfiguration {
    fn type_info() -> &'static TypeInfo {
        ONLINEPROVIDERCONFIGURATION_TYPE_INFO
    }
}


pub const ONLINEPROVIDERCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineProviderConfiguration-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineProviderConfiguration-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlinePlatformConfiguration {
    pub platform: super::core::GamePlatform,
    pub is_fallback: bool,
    pub platform_data: OnlinePlatformData,
    pub services: OnlineServicesAsset,
    pub server_services: OnlineServicesAsset,
    pub client_backends: Vec<PresenceBackendData>,
    pub server_backends: Vec<PresenceBackendData>,
    pub server_game_backends: Vec<PresenceBackendData>,
}

pub const ONLINEPLATFORMCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePlatformConfiguration",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(OnlinePlatformConfiguration, platform),
            },
            FieldInfoData {
                name: "IsFallback",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(OnlinePlatformConfiguration, is_fallback),
            },
            FieldInfoData {
                name: "PlatformData",
                flags: MemberInfoFlags::new(0),
                field_type: ONLINEPLATFORMDATA_TYPE_INFO,
                rust_offset: offset_of!(OnlinePlatformConfiguration, platform_data),
            },
            FieldInfoData {
                name: "Services",
                flags: MemberInfoFlags::new(0),
                field_type: ONLINESERVICESASSET_TYPE_INFO,
                rust_offset: offset_of!(OnlinePlatformConfiguration, services),
            },
            FieldInfoData {
                name: "ServerServices",
                flags: MemberInfoFlags::new(0),
                field_type: ONLINESERVICESASSET_TYPE_INFO,
                rust_offset: offset_of!(OnlinePlatformConfiguration, server_services),
            },
            FieldInfoData {
                name: "ClientBackends",
                flags: MemberInfoFlags::new(144),
                field_type: PRESENCEBACKENDDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OnlinePlatformConfiguration, client_backends),
            },
            FieldInfoData {
                name: "ServerBackends",
                flags: MemberInfoFlags::new(144),
                field_type: PRESENCEBACKENDDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OnlinePlatformConfiguration, server_backends),
            },
            FieldInfoData {
                name: "ServerGameBackends",
                flags: MemberInfoFlags::new(144),
                field_type: PRESENCEBACKENDDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OnlinePlatformConfiguration, server_game_backends),
            },
        ],
    }),
    array_type: Some(ONLINEPLATFORMCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlinePlatformConfiguration {
    fn type_info() -> &'static TypeInfo {
        ONLINEPLATFORMCONFIGURATION_TYPE_INFO
    }
}


pub const ONLINEPLATFORMCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePlatformConfiguration-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlinePlatformConfiguration-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ps4OnlineData {
    pub default_title_data: Ps4OnlineTitleData,
    pub title_data: Vec<Ps4OnlineTitleData>,
}

pub const PS4ONLINEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4OnlineData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ONLINEPLATFORMDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DefaultTitleData",
                flags: MemberInfoFlags::new(0),
                field_type: PS4ONLINETITLEDATA_TYPE_INFO,
                rust_offset: offset_of!(Ps4OnlineData, default_title_data),
            },
            FieldInfoData {
                name: "TitleData",
                flags: MemberInfoFlags::new(144),
                field_type: PS4ONLINETITLEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Ps4OnlineData, title_data),
            },
        ],
    }),
    array_type: Some(PS4ONLINEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4OnlineData {
    fn type_info() -> &'static TypeInfo {
        PS4ONLINEDATA_TYPE_INFO
    }
}


pub const PS4ONLINEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4OnlineData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("Ps4OnlineData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ps4OnlineTitleData {
    pub title_id: String,
    pub title_secret: String,
}

pub const PS4ONLINETITLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4OnlineTitleData",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TitleId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(Ps4OnlineTitleData, title_id),
            },
            FieldInfoData {
                name: "TitleSecret",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(Ps4OnlineTitleData, title_secret),
            },
        ],
    }),
    array_type: Some(PS4ONLINETITLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4OnlineTitleData {
    fn type_info() -> &'static TypeInfo {
        PS4ONLINETITLEDATA_TYPE_INFO
    }
}


pub const PS4ONLINETITLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4OnlineTitleData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("Ps4OnlineTitleData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlinePlatformData {
}

pub const ONLINEPLATFORMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePlatformData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ONLINEPLATFORMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlinePlatformData {
    fn type_info() -> &'static TypeInfo {
        ONLINEPLATFORMDATA_TYPE_INFO
    }
}


pub const ONLINEPLATFORMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePlatformData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlinePlatformData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineRichPresenceData {
    pub rich_presence_strings: Vec<OnlineRichPresenceString>,
    pub contexts: Vec<OnlineRichPresenceContext>,
}

pub const ONLINERICHPRESENCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RichPresenceStrings",
                flags: MemberInfoFlags::new(144),
                field_type: ONLINERICHPRESENCESTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceData, rich_presence_strings),
            },
            FieldInfoData {
                name: "Contexts",
                flags: MemberInfoFlags::new(144),
                field_type: ONLINERICHPRESENCECONTEXT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceData, contexts),
            },
        ],
    }),
    array_type: Some(ONLINERICHPRESENCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineRichPresenceData {
    fn type_info() -> &'static TypeInfo {
        ONLINERICHPRESENCEDATA_TYPE_INFO
    }
}


pub const ONLINERICHPRESENCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineRichPresenceData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineRichPresenceContextValuePair {
    pub context: OnlineRichPresenceContext,
    pub value: OnlineRichPresenceContextValue,
}

pub const ONLINERICHPRESENCECONTEXTVALUEPAIR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContextValuePair",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Context",
                flags: MemberInfoFlags::new(0),
                field_type: ONLINERICHPRESENCECONTEXT_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceContextValuePair, context),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: ONLINERICHPRESENCECONTEXTVALUE_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceContextValuePair, value),
            },
        ],
    }),
    array_type: Some(ONLINERICHPRESENCECONTEXTVALUEPAIR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineRichPresenceContextValuePair {
    fn type_info() -> &'static TypeInfo {
        ONLINERICHPRESENCECONTEXTVALUEPAIR_TYPE_INFO
    }
}


pub const ONLINERICHPRESENCECONTEXTVALUEPAIR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContextValuePair-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineRichPresenceContextValuePair-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineRichPresenceString {
    pub sid: String,
    pub xdp_name: String,
    pub index: u8,
}

pub const ONLINERICHPRESENCESTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceString",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Sid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceString, sid),
            },
            FieldInfoData {
                name: "XdpName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceString, xdp_name),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceString, index),
            },
        ],
    }),
    array_type: Some(ONLINERICHPRESENCESTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineRichPresenceString {
    fn type_info() -> &'static TypeInfo {
        ONLINERICHPRESENCESTRING_TYPE_INFO
    }
}


pub const ONLINERICHPRESENCESTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceString-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineRichPresenceString-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineRichPresenceContext {
    pub name: String,
    pub xdp_name: String,
    pub values: Vec<OnlineRichPresenceContextValue>,
    pub index: u8,
}

pub const ONLINERICHPRESENCECONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContext",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceContext, name),
            },
            FieldInfoData {
                name: "XdpName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceContext, xdp_name),
            },
            FieldInfoData {
                name: "Values",
                flags: MemberInfoFlags::new(144),
                field_type: ONLINERICHPRESENCECONTEXTVALUE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceContext, values),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceContext, index),
            },
        ],
    }),
    array_type: Some(ONLINERICHPRESENCECONTEXT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineRichPresenceContext {
    fn type_info() -> &'static TypeInfo {
        ONLINERICHPRESENCECONTEXT_TYPE_INFO
    }
}


pub const ONLINERICHPRESENCECONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContext-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineRichPresenceContext-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineRichPresenceContextValue {
    pub sid: String,
    pub key: String,
    pub xdp_name: String,
    pub index: u8,
}

pub const ONLINERICHPRESENCECONTEXTVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContextValue",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Sid",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceContextValue, sid),
            },
            FieldInfoData {
                name: "Key",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceContextValue, key),
            },
            FieldInfoData {
                name: "XdpName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceContextValue, xdp_name),
            },
            FieldInfoData {
                name: "Index",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(OnlineRichPresenceContextValue, index),
            },
        ],
    }),
    array_type: Some(ONLINERICHPRESENCECONTEXTVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineRichPresenceContextValue {
    fn type_info() -> &'static TypeInfo {
        ONLINERICHPRESENCECONTEXTVALUE_TYPE_INFO
    }
}


pub const ONLINERICHPRESENCECONTEXTVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContextValue-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineRichPresenceContextValue-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineEnvironmentConsoleUrl {
    pub urls: Vec<OnlineEnvironmentConsoleUrlData>,
}

pub const ONLINEENVIRONMENTCONSOLEURL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentConsoleUrl",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Urls",
                flags: MemberInfoFlags::new(144),
                field_type: ONLINEENVIRONMENTCONSOLEURLDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OnlineEnvironmentConsoleUrl, urls),
            },
        ],
    }),
    array_type: Some(ONLINEENVIRONMENTCONSOLEURL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineEnvironmentConsoleUrl {
    fn type_info() -> &'static TypeInfo {
        ONLINEENVIRONMENTCONSOLEURL_TYPE_INFO
    }
}


pub const ONLINEENVIRONMENTCONSOLEURL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentConsoleUrl-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineEnvironmentConsoleUrl-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineEnvironmentConsoleUrlData {
    pub platform: super::core::GamePlatform,
    pub url: OnlineEnvironmentUrl,
}

pub const ONLINEENVIRONMENTCONSOLEURLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentConsoleUrlData",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(OnlineEnvironmentConsoleUrlData, platform),
            },
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: ONLINEENVIRONMENTURL_TYPE_INFO,
                rust_offset: offset_of!(OnlineEnvironmentConsoleUrlData, url),
            },
        ],
    }),
    array_type: Some(ONLINEENVIRONMENTCONSOLEURLDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineEnvironmentConsoleUrlData {
    fn type_info() -> &'static TypeInfo {
        ONLINEENVIRONMENTCONSOLEURLDATA_TYPE_INFO
    }
}


pub const ONLINEENVIRONMENTCONSOLEURLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentConsoleUrlData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineEnvironmentConsoleUrlData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineEnvironmentUrl {
    pub urls: Vec<OnlineEnvironmentUrlData>,
}

pub const ONLINEENVIRONMENTURL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentUrl",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Urls",
                flags: MemberInfoFlags::new(144),
                field_type: ONLINEENVIRONMENTURLDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OnlineEnvironmentUrl, urls),
            },
        ],
    }),
    array_type: Some(ONLINEENVIRONMENTURL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineEnvironmentUrl {
    fn type_info() -> &'static TypeInfo {
        ONLINEENVIRONMENTURL_TYPE_INFO
    }
}


pub const ONLINEENVIRONMENTURL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentUrl-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineEnvironmentUrl-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OnlineEnvironmentUrlData {
    pub url: String,
    pub environment: OnlineEnvironment,
}

pub const ONLINEENVIRONMENTURLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentUrlData",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(OnlineEnvironmentUrlData, url),
            },
            FieldInfoData {
                name: "Environment",
                flags: MemberInfoFlags::new(0),
                field_type: ONLINEENVIRONMENT_TYPE_INFO,
                rust_offset: offset_of!(OnlineEnvironmentUrlData, environment),
            },
        ],
    }),
    array_type: Some(ONLINEENVIRONMENTURLDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineEnvironmentUrlData {
    fn type_info() -> &'static TypeInfo {
        ONLINEENVIRONMENTURLDATA_TYPE_INFO
    }
}


pub const ONLINEENVIRONMENTURLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentUrlData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineEnvironmentUrlData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum OnlineEnvironment {
    #[default]
    OnlineEnvironment_Development = 0,
    OnlineEnvironment_Test = 1,
    OnlineEnvironment_Certification = 2,
    OnlineEnvironment_Production = 3,
    OnlineEnvironment_Count = 4,
}

pub const ONLINEENVIRONMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironment",
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(ONLINEENVIRONMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OnlineEnvironment {
    fn type_info() -> &'static TypeInfo {
        ONLINEENVIRONMENT_TYPE_INFO
    }
}


pub const ONLINEENVIRONMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironment-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineEnvironment-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OriginPresenceBackendData {
    pub commerce_categories: Vec<String>,
    pub invite_timeout: u32,
}

pub const ORIGINPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPresenceBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CommerceCategories",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(OriginPresenceBackendData, commerce_categories),
            },
            FieldInfoData {
                name: "InviteTimeout",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(OriginPresenceBackendData, invite_timeout),
            },
        ],
    }),
    array_type: Some(ORIGINPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OriginPresenceBackendData {
    fn type_info() -> &'static TypeInfo {
        ORIGINPRESENCEBACKENDDATA_TYPE_INFO
    }
}


pub const ORIGINPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPresenceBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OriginPresenceBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlazeInProcServerBackendData {
}

pub const BLAZEINPROCSERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeInProcServerBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BLAZEINPROCSERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlazeInProcServerBackendData {
    fn type_info() -> &'static TypeInfo {
        BLAZEINPROCSERVERBACKENDDATA_TYPE_INFO
    }
}


pub const BLAZEINPROCSERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeInProcServerBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("BlazeInProcServerBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlazeServerBackendData {
    pub config_url: OnlineEnvironmentConsoleUrl,
    pub create_game_template_name: String,
}

pub const BLAZESERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeServerBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ConfigUrl",
                flags: MemberInfoFlags::new(0),
                field_type: ONLINEENVIRONMENTCONSOLEURL_TYPE_INFO,
                rust_offset: offset_of!(BlazeServerBackendData, config_url),
            },
            FieldInfoData {
                name: "CreateGameTemplateName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(BlazeServerBackendData, create_game_template_name),
            },
        ],
    }),
    array_type: Some(BLAZESERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlazeServerBackendData {
    fn type_info() -> &'static TypeInfo {
        BLAZESERVERBACKENDDATA_TYPE_INFO
    }
}


pub const BLAZESERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeServerBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("BlazeServerBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LanServerBackendData {
}

pub const LANSERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanServerBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LANSERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LanServerBackendData {
    fn type_info() -> &'static TypeInfo {
        LANSERVERBACKENDDATA_TYPE_INFO
    }
}


pub const LANSERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanServerBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("LanServerBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalServerBackendData {
}

pub const LOCALSERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalServerBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALSERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalServerBackendData {
    fn type_info() -> &'static TypeInfo {
        LOCALSERVERBACKENDDATA_TYPE_INFO
    }
}


pub const LOCALSERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalServerBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("LocalServerBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DurangoPresenceBackendData {
    pub title_id: u32,
    pub service_config_id: String,
    pub multiplayer_privilege_needed: bool,
}

pub const DURANGOPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DurangoPresenceBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TitleId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DurangoPresenceBackendData, title_id),
            },
            FieldInfoData {
                name: "ServiceConfigId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(DurangoPresenceBackendData, service_config_id),
            },
            FieldInfoData {
                name: "MultiplayerPrivilegeNeeded",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DurangoPresenceBackendData, multiplayer_privilege_needed),
            },
        ],
    }),
    array_type: Some(DURANGOPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DurangoPresenceBackendData {
    fn type_info() -> &'static TypeInfo {
        DURANGOPRESENCEBACKENDDATA_TYPE_INFO
    }
}


pub const DURANGOPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DurangoPresenceBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("DurangoPresenceBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ps4PresenceBackendData {
    pub age_settings: Ps4AgeSettings,
    pub multiplayer_privilege_needed: bool,
    pub send_invite_without_first_party_dialog: bool,
    pub send_invite_without_custom_message: bool,
}

pub const PS4PRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4PresenceBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "AgeSettings",
                flags: MemberInfoFlags::new(0),
                field_type: PS4AGESETTINGS_TYPE_INFO,
                rust_offset: offset_of!(Ps4PresenceBackendData, age_settings),
            },
            FieldInfoData {
                name: "MultiplayerPrivilegeNeeded",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4PresenceBackendData, multiplayer_privilege_needed),
            },
            FieldInfoData {
                name: "SendInviteWithoutFirstPartyDialog",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4PresenceBackendData, send_invite_without_first_party_dialog),
            },
            FieldInfoData {
                name: "SendInviteWithoutCustomMessage",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(Ps4PresenceBackendData, send_invite_without_custom_message),
            },
        ],
    }),
    array_type: Some(PS4PRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4PresenceBackendData {
    fn type_info() -> &'static TypeInfo {
        PS4PRESENCEBACKENDDATA_TYPE_INFO
    }
}


pub const PS4PRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4PresenceBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("Ps4PresenceBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ps4AgeSettings {
    pub default_age_requirement: i32,
    pub age_overrides: Vec<Ps4CountryAgeOverrides>,
}

pub const PS4AGESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4AgeSettings",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "DefaultAgeRequirement",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Ps4AgeSettings, default_age_requirement),
            },
            FieldInfoData {
                name: "AgeOverrides",
                flags: MemberInfoFlags::new(144),
                field_type: PS4COUNTRYAGEOVERRIDES_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(Ps4AgeSettings, age_overrides),
            },
        ],
    }),
    array_type: Some(PS4AGESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4AgeSettings {
    fn type_info() -> &'static TypeInfo {
        PS4AGESETTINGS_TYPE_INFO
    }
}


pub const PS4AGESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4AgeSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("Ps4AgeSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ps4CountryAgeOverrides {
    pub country_code: String,
    pub age_requirement: i32,
}

pub const PS4COUNTRYAGEOVERRIDES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4CountryAgeOverrides",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CountryCode",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(Ps4CountryAgeOverrides, country_code),
            },
            FieldInfoData {
                name: "AgeRequirement",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(Ps4CountryAgeOverrides, age_requirement),
            },
        ],
    }),
    array_type: Some(PS4COUNTRYAGEOVERRIDES_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4CountryAgeOverrides {
    fn type_info() -> &'static TypeInfo {
        PS4COUNTRYAGEOVERRIDES_TYPE_INFO
    }
}


pub const PS4COUNTRYAGEOVERRIDES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4CountryAgeOverrides-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("Ps4CountryAgeOverrides-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NucleusPresenceBackendData {
    pub platforms: Vec<super::nucleus::NucleusPlatformConfiguration>,
}

pub const NUCLEUSPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusPresenceBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Platforms",
                flags: MemberInfoFlags::new(144),
                field_type: NUCLEUSPLATFORMCONFIGURATION_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(NucleusPresenceBackendData, platforms),
            },
        ],
    }),
    array_type: Some(NUCLEUSPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NucleusPresenceBackendData {
    fn type_info() -> &'static TypeInfo {
        NUCLEUSPRESENCEBACKENDDATA_TYPE_INFO
    }
}


pub const NUCLEUSPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusPresenceBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("NucleusPresenceBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LanPresenceBackendData {
}

pub const LANPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanPresenceBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LANPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LanPresenceBackendData {
    fn type_info() -> &'static TypeInfo {
        LANPRESENCEBACKENDDATA_TYPE_INFO
    }
}


pub const LANPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanPresenceBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("LanPresenceBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DirtySockPresenceBackendData {
}

pub const DIRTYSOCKPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DirtySockPresenceBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DIRTYSOCKPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DirtySockPresenceBackendData {
    fn type_info() -> &'static TypeInfo {
        DIRTYSOCKPRESENCEBACKENDDATA_TYPE_INFO
    }
}


pub const DIRTYSOCKPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DirtySockPresenceBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("DirtySockPresenceBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlazePresenceBackendData {
    pub use_demangler_service: bool,
    pub use_dirty_sock_voip: bool,
    pub fetch_licenses_on_login: bool,
}

pub const BLAZEPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazePresenceBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "UseDemanglerService",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlazePresenceBackendData, use_demangler_service),
            },
            FieldInfoData {
                name: "UseDirtySockVoip",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlazePresenceBackendData, use_dirty_sock_voip),
            },
            FieldInfoData {
                name: "FetchLicensesOnLogin",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BlazePresenceBackendData, fetch_licenses_on_login),
            },
        ],
    }),
    array_type: Some(BLAZEPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlazePresenceBackendData {
    fn type_info() -> &'static TypeInfo {
        BLAZEPRESENCEBACKENDDATA_TYPE_INFO
    }
}


pub const BLAZEPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazePresenceBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("BlazePresenceBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlatformFetchLicense {
    pub platform: super::core::GamePlatform,
    pub fetch_licenses_on_login: bool,
}

pub const PLATFORMFETCHLICENSE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformFetchLicense",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Platform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(PlatformFetchLicense, platform),
            },
            FieldInfoData {
                name: "FetchLicensesOnLogin",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlatformFetchLicense, fetch_licenses_on_login),
            },
        ],
    }),
    array_type: Some(PLATFORMFETCHLICENSE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PlatformFetchLicense {
    fn type_info() -> &'static TypeInfo {
        PLATFORMFETCHLICENSE_TYPE_INFO
    }
}


pub const PLATFORMFETCHLICENSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformFetchLicense-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PlatformFetchLicense-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceBackendData {
    pub backend_type: i32,
}

pub const PRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBackendData",
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BackendType",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(PresenceBackendData, backend_type),
            },
        ],
    }),
    array_type: Some(PRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceBackendData {
    fn type_info() -> &'static TypeInfo {
        PRESENCEBACKENDDATA_TYPE_INFO
    }
}


pub const PRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBackendData-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceBackendData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PresenceMode {
    #[default]
    PresenceMode_Offline = 0,
    PresenceMode_SilentOnline = 1,
    PresenceMode_Online = 2,
}

pub const PRESENCEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMode",
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(PRESENCEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PresenceMode {
    fn type_info() -> &'static TypeInfo {
        PRESENCEMODE_TYPE_INFO
    }
}


pub const PRESENCEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkInviteToken {
    pub invite_platform: super::core::GamePlatform,
    pub invite_type: InviteType,
    pub join_method: InviteJoinMethod,
    pub game_id: u64,
    pub player: NetworkInviteTokenPlayer,
}

pub const NETWORKINVITETOKEN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkInviteToken",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "InvitePlatform",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLATFORM_TYPE_INFO,
                rust_offset: offset_of!(NetworkInviteToken, invite_platform),
            },
            FieldInfoData {
                name: "InviteType",
                flags: MemberInfoFlags::new(0),
                field_type: INVITETYPE_TYPE_INFO,
                rust_offset: offset_of!(NetworkInviteToken, invite_type),
            },
            FieldInfoData {
                name: "JoinMethod",
                flags: MemberInfoFlags::new(0),
                field_type: INVITEJOINMETHOD_TYPE_INFO,
                rust_offset: offset_of!(NetworkInviteToken, join_method),
            },
            FieldInfoData {
                name: "GameId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(NetworkInviteToken, game_id),
            },
            FieldInfoData {
                name: "Player",
                flags: MemberInfoFlags::new(0),
                field_type: NETWORKINVITETOKENPLAYER_TYPE_INFO,
                rust_offset: offset_of!(NetworkInviteToken, player),
            },
        ],
    }),
    array_type: Some(NETWORKINVITETOKEN_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkInviteToken {
    fn type_info() -> &'static TypeInfo {
        NETWORKINVITETOKEN_TYPE_INFO
    }
}


pub const NETWORKINVITETOKEN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkInviteToken-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("NetworkInviteToken-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NetworkInviteTokenPlayer {
    pub id_type: InviteIdType,
    pub id_number: u64,
}

pub const NETWORKINVITETOKENPLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkInviteTokenPlayer",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "IdType",
                flags: MemberInfoFlags::new(0),
                field_type: INVITEIDTYPE_TYPE_INFO,
                rust_offset: offset_of!(NetworkInviteTokenPlayer, id_type),
            },
            FieldInfoData {
                name: "IdNumber",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(NetworkInviteTokenPlayer, id_number),
            },
        ],
    }),
    array_type: Some(NETWORKINVITETOKENPLAYER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkInviteTokenPlayer {
    fn type_info() -> &'static TypeInfo {
        NETWORKINVITETOKENPLAYER_TYPE_INFO
    }
}


pub const NETWORKINVITETOKENPLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkInviteTokenPlayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("NetworkInviteTokenPlayer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum InviteIdStringConstants {
    #[default]
    InviteIdStringLength = 32,
}

pub const INVITEIDSTRINGCONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteIdStringConstants",
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(INVITEIDSTRINGCONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InviteIdStringConstants {
    fn type_info() -> &'static TypeInfo {
        INVITEIDSTRINGCONSTANTS_TYPE_INFO
    }
}


pub const INVITEIDSTRINGCONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteIdStringConstants-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("InviteIdStringConstants-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum InviteIdType {
    #[default]
    IdType_Invalid = 0,
    IdType_Ps4 = 1,
    IdType_Xb1 = 2,
    IdType_EAUserId = 3,
    IdType_PersonaId = 4,
    IdType_PersonaName = 5,
    IdType_Count = 6,
}

pub const INVITEIDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteIdType",
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(INVITEIDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InviteIdType {
    fn type_info() -> &'static TypeInfo {
        INVITEIDTYPE_TYPE_INFO
    }
}


pub const INVITEIDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteIdType-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("InviteIdType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum InviteJoinMethod {
    #[default]
    InviteJoinMethod_Unknown = 0,
    InviteJoinMethod_Invite = 1,
    InviteJoinMethod_Join = 2,
    InviteJoinMethod_Count = 3,
}

pub const INVITEJOINMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteJoinMethod",
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(INVITEJOINMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InviteJoinMethod {
    fn type_info() -> &'static TypeInfo {
        INVITEJOINMETHOD_TYPE_INFO
    }
}


pub const INVITEJOINMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteJoinMethod-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("InviteJoinMethod-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum InviteType {
    #[default]
    InviteType_Invalid = 0,
    InviteType_Game = 1,
    InviteType_Player = 2,
    InviteType_Count = 3,
}

pub const INVITETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteType",
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(INVITETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InviteType {
    fn type_info() -> &'static TypeInfo {
        INVITETYPE_TYPE_INFO
    }
}


pub const INVITETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteType-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("InviteType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum OnlinePrivilege {
    #[default]
    OnlinePrivilege_Unknown = 0,
    OnlinePrivilege_CommunicationVoiceIngame = 1,
    OnlinePrivilege_CommunicationVoiceSkype = 2,
    OnlinePrivilege_VideoCommunications = 3,
    OnlinePrivilege_Communications = 4,
    OnlinePrivilege_UserCreatedContent = 5,
    OnlinePrivilege_MultiplayerSessionsRealtime = 6,
    OnlinePrivilege_MultiplayerSessionsAsync = 7,
    OnlinePrivilege_EASubscription = 8,
    OnlinePrivilege_Count = 9,
}

pub const ONLINEPRIVILEGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePrivilege",
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(ONLINEPRIVILEGE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OnlinePrivilege {
    fn type_info() -> &'static TypeInfo {
        ONLINEPRIVILEGE_TYPE_INFO
    }
}


pub const ONLINEPRIVILEGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePrivilege-Array",
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlinePrivilege-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresencePlayTogetherRequestMessageBase {
}

pub const PRESENCEPLAYTOGETHERREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePlayTogetherRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePlayTogetherRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPLAYTOGETHERREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceGamegroupUpdatedMessageBase {
}

pub const PRESENCEGAMEGROUPUPDATEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGamegroupUpdatedMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGamegroupUpdatedMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEGAMEGROUPUPDATEDMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceLoginLicenseRequestMessageBase {
}

pub const PRESENCELOGINLICENSEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLoginLicenseRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceLoginLicenseRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCELOGINLICENSEREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceLoginLicenseMessageBase {
}

pub const PRESENCELOGINLICENSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLoginLicenseMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceLoginLicenseMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCELOGINLICENSEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceBlazeAutoAccountLoginMessage {
}

pub const PRESENCEBLAZEAUTOACCOUNTLOGINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlazeAutoAccountLoginMessage",
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceBlazeAutoAccountLoginMessage {
    fn type_info() -> &'static TypeInfo {
        PRESENCEBLAZEAUTOACCOUNTLOGINMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceJoinRemoteGameMessage {
}

pub const PRESENCEJOINREMOTEGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceJoinRemoteGameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceJoinRemoteGameMessage {
    fn type_info() -> &'static TypeInfo {
        PRESENCEJOINREMOTEGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceCommitPlayerToGameMessage {
}

pub const PRESENCECOMMITPLAYERTOGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCommitPlayerToGameMessage",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceCommitPlayerToGameMessage {
    fn type_info() -> &'static TypeInfo {
        PRESENCECOMMITPLAYERTOGAMEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceUserProfileRequestMessageBase {
}

pub const PRESENCEUSERPROFILEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserProfileRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceUserProfileRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEUSERPROFILEREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceUserProfileMessageBase {
}

pub const PRESENCEUSERPROFILEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserProfileMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceUserProfileMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEUSERPROFILEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceUserIdRequestMessageBase {
}

pub const PRESENCEUSERIDREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserIdRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceUserIdRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEUSERIDREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceUserIdMessageBase {
}

pub const PRESENCEUSERIDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserIdMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceUserIdMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEUSERIDMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresencePSPlusRequestMessageBase {
}

pub const PRESENCEPSPLUSREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePSPlusRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePSPlusRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPSPLUSREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceProfanityFilterResponseMessageBase {
}

pub const PRESENCEPROFANITYFILTERRESPONSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceProfanityFilterResponseMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceProfanityFilterResponseMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPROFANITYFILTERRESPONSEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceProfanityFilterRequestMessageBase {
}

pub const PRESENCEPROFANITYFILTERREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceProfanityFilterRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceProfanityFilterRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPROFANITYFILTERREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresencePrivilegeRequestResultMessageBase {
}

pub const PRESENCEPRIVILEGEREQUESTRESULTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePrivilegeRequestResultMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePrivilegeRequestResultMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPRIVILEGEREQUESTRESULTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresencePrivilegeRequestMessageBase {
}

pub const PRESENCEPRIVILEGEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePrivilegeRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePrivilegeRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEPRIVILEGEREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceOriginUserNameRequestMessageBase {
}

pub const PRESENCEORIGINUSERNAMEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOriginUserNameRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceOriginUserNameRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEORIGINUSERNAMEREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceOriginUserNameMessageBase {
}

pub const PRESENCEORIGINUSERNAMEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOriginUserNameMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceOriginUserNameMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEORIGINUSERNAMEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceLivePartyMessageBase {
}

pub const PRESENCELIVEPARTYMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLivePartyMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceLivePartyMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCELIVEPARTYMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceInviteRequestMessageBase {
}

pub const PRESENCEINVITEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceInviteRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceInviteRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEINVITEREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceInviteMessageBase {
}

pub const PRESENCEINVITEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceInviteMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceInviteMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEINVITEMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceFriendsListManagerSettingsMessageBase {
}

pub const PRESENCEFRIENDSLISTMANAGERSETTINGSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFriendsListManagerSettingsMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceFriendsListManagerSettingsMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEFRIENDSLISTMANAGERSETTINGSMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceFriendRequestMessageBase {
}

pub const PRESENCEFRIENDREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFriendRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceFriendRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEFRIENDREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceFriendMessageBase {
}

pub const PRESENCEFRIENDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFriendMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceFriendMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEFRIENDMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceConnectionRequestMessageBase {
}

pub const PRESENCECONNECTIONREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceConnectionRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceConnectionRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCECONNECTIONREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceConnectionMessageBase {
}

pub const PRESENCECONNECTIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceConnectionMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceConnectionMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCECONNECTIONMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceBlockListRequestMessageBase {
}

pub const PRESENCEBLOCKLISTREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlockListRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlockListRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEBLOCKLISTREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceBlockListMessageBase {
}

pub const PRESENCEBLOCKLISTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlockListMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlockListMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEBLOCKLISTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceBlobRequestMessageBase {
}

pub const PRESENCEBLOBREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlobRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlobRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEBLOBREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceBlobMessageBase {
}

pub const PRESENCEBLOBMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlobMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlobMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEBLOBMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceBlazeUserRequestMessageBase {
}

pub const PRESENCEBLAZEUSERREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlazeUserRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlazeUserRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEBLAZEUSERREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceBlazeUserMessageBase {
}

pub const PRESENCEBLAZEUSERMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlazeUserMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlazeUserMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEBLAZEUSERMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceAuthenticationRequestMessageBase {
}

pub const PRESENCEAUTHENTICATIONREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAuthenticationRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAuthenticationRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEAUTHENTICATIONREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceAuthenticationMessageBase {
}

pub const PRESENCEAUTHENTICATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAuthenticationMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAuthenticationMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEAUTHENTICATIONMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceAccountRequestMessageBase {
}

pub const PRESENCEACCOUNTREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAccountRequestMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAccountRequestMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEACCOUNTREQUESTMESSAGEBASE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PresenceAccountMessageBase {
}

pub const PRESENCEACCOUNTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAccountMessageBase",
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAccountMessageBase {
    fn type_info() -> &'static TypeInfo {
        PRESENCEACCOUNTMESSAGEBASE_TYPE_INFO
    }
}

