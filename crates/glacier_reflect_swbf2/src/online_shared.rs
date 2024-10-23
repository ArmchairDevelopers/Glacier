use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub assert_on_presence_request_failures: bool,
    pub client_is_presence_enabled: bool,
    pub server_is_presence_enabled: bool,
    pub environment: OnlineEnvironment,
    pub is_secure: bool,
    pub enable_qo_s: bool,
    pub wait_for_qo_s: bool,
    pub provider: Option<LockedTypeObject /* OnlineProviderAsset */>,
    pub platforms: Vec<BoxedTypeObject /* OnlinePlatformConfiguration */>,
    pub service_name_override: String,
    pub log_level: LogLevelType,
    pub blaze_log_level: i32,
    pub dirty_sock_log_level: i32,
    pub rich_presence_data: Option<LockedTypeObject /* OnlineRichPresenceData */>,
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

pub trait OnlineSettingsTrait: super::core::SystemSettingsTrait {
    fn assert_on_presence_request_failures(&self) -> &bool;
    fn assert_on_presence_request_failures_mut(&mut self) -> &mut bool;
    fn client_is_presence_enabled(&self) -> &bool;
    fn client_is_presence_enabled_mut(&mut self) -> &mut bool;
    fn server_is_presence_enabled(&self) -> &bool;
    fn server_is_presence_enabled_mut(&mut self) -> &mut bool;
    fn environment(&self) -> &OnlineEnvironment;
    fn environment_mut(&mut self) -> &mut OnlineEnvironment;
    fn is_secure(&self) -> &bool;
    fn is_secure_mut(&mut self) -> &mut bool;
    fn enable_qo_s(&self) -> &bool;
    fn enable_qo_s_mut(&mut self) -> &mut bool;
    fn wait_for_qo_s(&self) -> &bool;
    fn wait_for_qo_s_mut(&mut self) -> &mut bool;
    fn provider(&self) -> &Option<LockedTypeObject /* OnlineProviderAsset */>;
    fn provider_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineProviderAsset */>;
    fn platforms(&self) -> &Vec<BoxedTypeObject /* OnlinePlatformConfiguration */>;
    fn platforms_mut(&mut self) -> &mut Vec<BoxedTypeObject /* OnlinePlatformConfiguration */>;
    fn service_name_override(&self) -> &String;
    fn service_name_override_mut(&mut self) -> &mut String;
    fn log_level(&self) -> &LogLevelType;
    fn log_level_mut(&mut self) -> &mut LogLevelType;
    fn blaze_log_level(&self) -> &i32;
    fn blaze_log_level_mut(&mut self) -> &mut i32;
    fn dirty_sock_log_level(&self) -> &i32;
    fn dirty_sock_log_level_mut(&mut self) -> &mut i32;
    fn rich_presence_data(&self) -> &Option<LockedTypeObject /* OnlineRichPresenceData */>;
    fn rich_presence_data_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineRichPresenceData */>;
    fn region(&self) -> &String;
    fn region_mut(&mut self) -> &mut String;
    fn country(&self) -> &String;
    fn country_mut(&mut self) -> &mut String;
    fn ping_site(&self) -> &String;
    fn ping_site_mut(&mut self) -> &mut String;
    fn matchmaking_token(&self) -> &String;
    fn matchmaking_token_mut(&mut self) -> &mut String;
    fn server_is_reconfigurable(&self) -> &bool;
    fn server_is_reconfigurable_mut(&mut self) -> &mut bool;
    fn support_host_migration(&self) -> &bool;
    fn support_host_migration_mut(&mut self) -> &mut bool;
    fn negative_user_cache_refresh_period(&self) -> &u32;
    fn negative_user_cache_refresh_period_mut(&mut self) -> &mut u32;
    fn server_login_email(&self) -> &String;
    fn server_login_email_mut(&mut self) -> &mut String;
    fn server_login_password(&self) -> &String;
    fn server_login_password_mut(&mut self) -> &mut String;
    fn server_login_persona_name(&self) -> &String;
    fn server_login_persona_name_mut(&mut self) -> &mut String;
    fn server_login_project_tag(&self) -> &String;
    fn server_login_project_tag_mut(&mut self) -> &mut String;
    fn blaze_server_connection_timeout(&self) -> &i32;
    fn blaze_server_connection_timeout_mut(&mut self) -> &mut i32;
    fn blaze_server_timeout(&self) -> &i32;
    fn blaze_server_timeout_mut(&mut self) -> &mut i32;
    fn blaze_server_tunnel_socket_recv_buf_size(&self) -> &u32;
    fn blaze_server_tunnel_socket_recv_buf_size_mut(&mut self) -> &mut u32;
    fn blaze_server_tunnel_socket_send_buf_size(&self) -> &u32;
    fn blaze_server_tunnel_socket_send_buf_size_mut(&mut self) -> &mut u32;
    fn blaze_outgoing_buffer_size(&self) -> &u32;
    fn blaze_outgoing_buffer_size_mut(&mut self) -> &mut u32;
    fn blaze_client_connection_timeout(&self) -> &i32;
    fn blaze_client_connection_timeout_mut(&mut self) -> &mut i32;
    fn blaze_client_timeout(&self) -> &i32;
    fn blaze_client_timeout_mut(&mut self) -> &mut i32;
    fn blaze_client_tunnel_socket_recv_buf_size(&self) -> &u32;
    fn blaze_client_tunnel_socket_recv_buf_size_mut(&mut self) -> &mut u32;
    fn blaze_client_tunnel_socket_send_buf_size(&self) -> &u32;
    fn blaze_client_tunnel_socket_send_buf_size_mut(&mut self) -> &mut u32;
    fn server_allow_any_reputation(&self) -> &bool;
    fn server_allow_any_reputation_mut(&mut self) -> &mut bool;
    fn peer_port(&self) -> &i32;
    fn peer_port_mut(&mut self) -> &mut i32;
    fn enable_gamegroup_invites(&self) -> &bool;
    fn enable_gamegroup_invites_mut(&mut self) -> &mut bool;
    fn dirty_sock_server_packet_queue_capacity(&self) -> &i32;
    fn dirty_sock_server_packet_queue_capacity_mut(&mut self) -> &mut i32;
    fn dirty_sock_max_connection_count(&self) -> &u32;
    fn dirty_sock_max_connection_count_mut(&mut self) -> &mut u32;
    fn blaze_cached_user_refresh_interval(&self) -> &u32;
    fn blaze_cached_user_refresh_interval_mut(&mut self) -> &mut u32;
    fn trusted_login_path(&self) -> &String;
    fn trusted_login_path_mut(&mut self) -> &mut String;
    fn trusted_login_cert_filename(&self) -> &String;
    fn trusted_login_cert_filename_mut(&mut self) -> &mut String;
    fn trusted_login_key_filename(&self) -> &String;
    fn trusted_login_key_filename_mut(&mut self) -> &mut String;
    fn enable_nucleus_lt_override(&self) -> &bool;
    fn enable_nucleus_lt_override_mut(&mut self) -> &mut bool;
    fn min_player_capacity(&self) -> &u32;
    fn min_player_capacity_mut(&mut self) -> &mut u32;
    fn should_control_dirty_sock(&self) -> &bool;
    fn should_control_dirty_sock_mut(&mut self) -> &mut bool;
    fn debug_message_callstack_type_list(&self) -> &String;
    fn debug_message_callstack_type_list_mut(&mut self) -> &mut String;
    fn override_create_game_template(&self) -> &bool;
    fn override_create_game_template_mut(&mut self) -> &mut bool;
    fn override_create_game_template_name(&self) -> &String;
    fn override_create_game_template_name_mut(&mut self) -> &mut String;
    fn resettable_pool(&self) -> &String;
    fn resettable_pool_mut(&mut self) -> &mut String;
}

impl OnlineSettingsTrait for OnlineSettings {
    fn assert_on_presence_request_failures(&self) -> &bool {
        &self.assert_on_presence_request_failures
    }
    fn assert_on_presence_request_failures_mut(&mut self) -> &mut bool {
        &mut self.assert_on_presence_request_failures
    }
    fn client_is_presence_enabled(&self) -> &bool {
        &self.client_is_presence_enabled
    }
    fn client_is_presence_enabled_mut(&mut self) -> &mut bool {
        &mut self.client_is_presence_enabled
    }
    fn server_is_presence_enabled(&self) -> &bool {
        &self.server_is_presence_enabled
    }
    fn server_is_presence_enabled_mut(&mut self) -> &mut bool {
        &mut self.server_is_presence_enabled
    }
    fn environment(&self) -> &OnlineEnvironment {
        &self.environment
    }
    fn environment_mut(&mut self) -> &mut OnlineEnvironment {
        &mut self.environment
    }
    fn is_secure(&self) -> &bool {
        &self.is_secure
    }
    fn is_secure_mut(&mut self) -> &mut bool {
        &mut self.is_secure
    }
    fn enable_qo_s(&self) -> &bool {
        &self.enable_qo_s
    }
    fn enable_qo_s_mut(&mut self) -> &mut bool {
        &mut self.enable_qo_s
    }
    fn wait_for_qo_s(&self) -> &bool {
        &self.wait_for_qo_s
    }
    fn wait_for_qo_s_mut(&mut self) -> &mut bool {
        &mut self.wait_for_qo_s
    }
    fn provider(&self) -> &Option<LockedTypeObject /* OnlineProviderAsset */> {
        &self.provider
    }
    fn provider_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineProviderAsset */> {
        &mut self.provider
    }
    fn platforms(&self) -> &Vec<BoxedTypeObject /* OnlinePlatformConfiguration */> {
        &self.platforms
    }
    fn platforms_mut(&mut self) -> &mut Vec<BoxedTypeObject /* OnlinePlatformConfiguration */> {
        &mut self.platforms
    }
    fn service_name_override(&self) -> &String {
        &self.service_name_override
    }
    fn service_name_override_mut(&mut self) -> &mut String {
        &mut self.service_name_override
    }
    fn log_level(&self) -> &LogLevelType {
        &self.log_level
    }
    fn log_level_mut(&mut self) -> &mut LogLevelType {
        &mut self.log_level
    }
    fn blaze_log_level(&self) -> &i32 {
        &self.blaze_log_level
    }
    fn blaze_log_level_mut(&mut self) -> &mut i32 {
        &mut self.blaze_log_level
    }
    fn dirty_sock_log_level(&self) -> &i32 {
        &self.dirty_sock_log_level
    }
    fn dirty_sock_log_level_mut(&mut self) -> &mut i32 {
        &mut self.dirty_sock_log_level
    }
    fn rich_presence_data(&self) -> &Option<LockedTypeObject /* OnlineRichPresenceData */> {
        &self.rich_presence_data
    }
    fn rich_presence_data_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineRichPresenceData */> {
        &mut self.rich_presence_data
    }
    fn region(&self) -> &String {
        &self.region
    }
    fn region_mut(&mut self) -> &mut String {
        &mut self.region
    }
    fn country(&self) -> &String {
        &self.country
    }
    fn country_mut(&mut self) -> &mut String {
        &mut self.country
    }
    fn ping_site(&self) -> &String {
        &self.ping_site
    }
    fn ping_site_mut(&mut self) -> &mut String {
        &mut self.ping_site
    }
    fn matchmaking_token(&self) -> &String {
        &self.matchmaking_token
    }
    fn matchmaking_token_mut(&mut self) -> &mut String {
        &mut self.matchmaking_token
    }
    fn server_is_reconfigurable(&self) -> &bool {
        &self.server_is_reconfigurable
    }
    fn server_is_reconfigurable_mut(&mut self) -> &mut bool {
        &mut self.server_is_reconfigurable
    }
    fn support_host_migration(&self) -> &bool {
        &self.support_host_migration
    }
    fn support_host_migration_mut(&mut self) -> &mut bool {
        &mut self.support_host_migration
    }
    fn negative_user_cache_refresh_period(&self) -> &u32 {
        &self.negative_user_cache_refresh_period
    }
    fn negative_user_cache_refresh_period_mut(&mut self) -> &mut u32 {
        &mut self.negative_user_cache_refresh_period
    }
    fn server_login_email(&self) -> &String {
        &self.server_login_email
    }
    fn server_login_email_mut(&mut self) -> &mut String {
        &mut self.server_login_email
    }
    fn server_login_password(&self) -> &String {
        &self.server_login_password
    }
    fn server_login_password_mut(&mut self) -> &mut String {
        &mut self.server_login_password
    }
    fn server_login_persona_name(&self) -> &String {
        &self.server_login_persona_name
    }
    fn server_login_persona_name_mut(&mut self) -> &mut String {
        &mut self.server_login_persona_name
    }
    fn server_login_project_tag(&self) -> &String {
        &self.server_login_project_tag
    }
    fn server_login_project_tag_mut(&mut self) -> &mut String {
        &mut self.server_login_project_tag
    }
    fn blaze_server_connection_timeout(&self) -> &i32 {
        &self.blaze_server_connection_timeout
    }
    fn blaze_server_connection_timeout_mut(&mut self) -> &mut i32 {
        &mut self.blaze_server_connection_timeout
    }
    fn blaze_server_timeout(&self) -> &i32 {
        &self.blaze_server_timeout
    }
    fn blaze_server_timeout_mut(&mut self) -> &mut i32 {
        &mut self.blaze_server_timeout
    }
    fn blaze_server_tunnel_socket_recv_buf_size(&self) -> &u32 {
        &self.blaze_server_tunnel_socket_recv_buf_size
    }
    fn blaze_server_tunnel_socket_recv_buf_size_mut(&mut self) -> &mut u32 {
        &mut self.blaze_server_tunnel_socket_recv_buf_size
    }
    fn blaze_server_tunnel_socket_send_buf_size(&self) -> &u32 {
        &self.blaze_server_tunnel_socket_send_buf_size
    }
    fn blaze_server_tunnel_socket_send_buf_size_mut(&mut self) -> &mut u32 {
        &mut self.blaze_server_tunnel_socket_send_buf_size
    }
    fn blaze_outgoing_buffer_size(&self) -> &u32 {
        &self.blaze_outgoing_buffer_size
    }
    fn blaze_outgoing_buffer_size_mut(&mut self) -> &mut u32 {
        &mut self.blaze_outgoing_buffer_size
    }
    fn blaze_client_connection_timeout(&self) -> &i32 {
        &self.blaze_client_connection_timeout
    }
    fn blaze_client_connection_timeout_mut(&mut self) -> &mut i32 {
        &mut self.blaze_client_connection_timeout
    }
    fn blaze_client_timeout(&self) -> &i32 {
        &self.blaze_client_timeout
    }
    fn blaze_client_timeout_mut(&mut self) -> &mut i32 {
        &mut self.blaze_client_timeout
    }
    fn blaze_client_tunnel_socket_recv_buf_size(&self) -> &u32 {
        &self.blaze_client_tunnel_socket_recv_buf_size
    }
    fn blaze_client_tunnel_socket_recv_buf_size_mut(&mut self) -> &mut u32 {
        &mut self.blaze_client_tunnel_socket_recv_buf_size
    }
    fn blaze_client_tunnel_socket_send_buf_size(&self) -> &u32 {
        &self.blaze_client_tunnel_socket_send_buf_size
    }
    fn blaze_client_tunnel_socket_send_buf_size_mut(&mut self) -> &mut u32 {
        &mut self.blaze_client_tunnel_socket_send_buf_size
    }
    fn server_allow_any_reputation(&self) -> &bool {
        &self.server_allow_any_reputation
    }
    fn server_allow_any_reputation_mut(&mut self) -> &mut bool {
        &mut self.server_allow_any_reputation
    }
    fn peer_port(&self) -> &i32 {
        &self.peer_port
    }
    fn peer_port_mut(&mut self) -> &mut i32 {
        &mut self.peer_port
    }
    fn enable_gamegroup_invites(&self) -> &bool {
        &self.enable_gamegroup_invites
    }
    fn enable_gamegroup_invites_mut(&mut self) -> &mut bool {
        &mut self.enable_gamegroup_invites
    }
    fn dirty_sock_server_packet_queue_capacity(&self) -> &i32 {
        &self.dirty_sock_server_packet_queue_capacity
    }
    fn dirty_sock_server_packet_queue_capacity_mut(&mut self) -> &mut i32 {
        &mut self.dirty_sock_server_packet_queue_capacity
    }
    fn dirty_sock_max_connection_count(&self) -> &u32 {
        &self.dirty_sock_max_connection_count
    }
    fn dirty_sock_max_connection_count_mut(&mut self) -> &mut u32 {
        &mut self.dirty_sock_max_connection_count
    }
    fn blaze_cached_user_refresh_interval(&self) -> &u32 {
        &self.blaze_cached_user_refresh_interval
    }
    fn blaze_cached_user_refresh_interval_mut(&mut self) -> &mut u32 {
        &mut self.blaze_cached_user_refresh_interval
    }
    fn trusted_login_path(&self) -> &String {
        &self.trusted_login_path
    }
    fn trusted_login_path_mut(&mut self) -> &mut String {
        &mut self.trusted_login_path
    }
    fn trusted_login_cert_filename(&self) -> &String {
        &self.trusted_login_cert_filename
    }
    fn trusted_login_cert_filename_mut(&mut self) -> &mut String {
        &mut self.trusted_login_cert_filename
    }
    fn trusted_login_key_filename(&self) -> &String {
        &self.trusted_login_key_filename
    }
    fn trusted_login_key_filename_mut(&mut self) -> &mut String {
        &mut self.trusted_login_key_filename
    }
    fn enable_nucleus_lt_override(&self) -> &bool {
        &self.enable_nucleus_lt_override
    }
    fn enable_nucleus_lt_override_mut(&mut self) -> &mut bool {
        &mut self.enable_nucleus_lt_override
    }
    fn min_player_capacity(&self) -> &u32 {
        &self.min_player_capacity
    }
    fn min_player_capacity_mut(&mut self) -> &mut u32 {
        &mut self.min_player_capacity
    }
    fn should_control_dirty_sock(&self) -> &bool {
        &self.should_control_dirty_sock
    }
    fn should_control_dirty_sock_mut(&mut self) -> &mut bool {
        &mut self.should_control_dirty_sock
    }
    fn debug_message_callstack_type_list(&self) -> &String {
        &self.debug_message_callstack_type_list
    }
    fn debug_message_callstack_type_list_mut(&mut self) -> &mut String {
        &mut self.debug_message_callstack_type_list
    }
    fn override_create_game_template(&self) -> &bool {
        &self.override_create_game_template
    }
    fn override_create_game_template_mut(&mut self) -> &mut bool {
        &mut self.override_create_game_template
    }
    fn override_create_game_template_name(&self) -> &String {
        &self.override_create_game_template_name
    }
    fn override_create_game_template_name_mut(&mut self) -> &mut String {
        &mut self.override_create_game_template_name
    }
    fn resettable_pool(&self) -> &String {
        &self.resettable_pool
    }
    fn resettable_pool_mut(&mut self) -> &mut String {
        &mut self.resettable_pool
    }
}

impl super::core::SystemSettingsTrait for OnlineSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for OnlineSettings {
}

pub static ONLINESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineSettings",
    name_hash: 1463314319,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(OnlineSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineSettings as Default>::default())),
            create_boxed: || Box::new(<OnlineSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AssertOnPresenceRequestFailures",
                name_hash: 3469877437,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, assert_on_presence_request_failures),
            },
            FieldInfoData {
                name: "ClientIsPresenceEnabled",
                name_hash: 3531320058,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, client_is_presence_enabled),
            },
            FieldInfoData {
                name: "ServerIsPresenceEnabled",
                name_hash: 3840694566,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, server_is_presence_enabled),
            },
            FieldInfoData {
                name: "Environment",
                name_hash: 2480382480,
                flags: MemberInfoFlags::new(0),
                field_type: "OnlineEnvironment",
                rust_offset: offset_of!(OnlineSettings, environment),
            },
            FieldInfoData {
                name: "IsSecure",
                name_hash: 451767400,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, is_secure),
            },
            FieldInfoData {
                name: "EnableQoS",
                name_hash: 2899410345,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, enable_qo_s),
            },
            FieldInfoData {
                name: "WaitForQoS",
                name_hash: 827139512,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, wait_for_qo_s),
            },
            FieldInfoData {
                name: "Provider",
                name_hash: 3021915972,
                flags: MemberInfoFlags::new(0),
                field_type: "OnlineProviderAsset",
                rust_offset: offset_of!(OnlineSettings, provider),
            },
            FieldInfoData {
                name: "Platforms",
                name_hash: 1046011945,
                flags: MemberInfoFlags::new(144),
                field_type: "OnlinePlatformConfiguration-Array",
                rust_offset: offset_of!(OnlineSettings, platforms),
            },
            FieldInfoData {
                name: "ServiceNameOverride",
                name_hash: 2327369995,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, service_name_override),
            },
            FieldInfoData {
                name: "LogLevel",
                name_hash: 1867701815,
                flags: MemberInfoFlags::new(0),
                field_type: "LogLevelType",
                rust_offset: offset_of!(OnlineSettings, log_level),
            },
            FieldInfoData {
                name: "BlazeLogLevel",
                name_hash: 2299541703,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OnlineSettings, blaze_log_level),
            },
            FieldInfoData {
                name: "DirtySockLogLevel",
                name_hash: 4290716369,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OnlineSettings, dirty_sock_log_level),
            },
            FieldInfoData {
                name: "RichPresenceData",
                name_hash: 2013223484,
                flags: MemberInfoFlags::new(0),
                field_type: "OnlineRichPresenceData",
                rust_offset: offset_of!(OnlineSettings, rich_presence_data),
            },
            FieldInfoData {
                name: "Region",
                name_hash: 3293978493,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, region),
            },
            FieldInfoData {
                name: "Country",
                name_hash: 3685467405,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, country),
            },
            FieldInfoData {
                name: "PingSite",
                name_hash: 1333330622,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, ping_site),
            },
            FieldInfoData {
                name: "MatchmakingToken",
                name_hash: 2041898954,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, matchmaking_token),
            },
            FieldInfoData {
                name: "ServerIsReconfigurable",
                name_hash: 2078923466,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, server_is_reconfigurable),
            },
            FieldInfoData {
                name: "SupportHostMigration",
                name_hash: 1663175238,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, support_host_migration),
            },
            FieldInfoData {
                name: "NegativeUserCacheRefreshPeriod",
                name_hash: 1822528579,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OnlineSettings, negative_user_cache_refresh_period),
            },
            FieldInfoData {
                name: "ServerLoginEmail",
                name_hash: 539710543,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, server_login_email),
            },
            FieldInfoData {
                name: "ServerLoginPassword",
                name_hash: 2187554588,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, server_login_password),
            },
            FieldInfoData {
                name: "ServerLoginPersonaName",
                name_hash: 1272502416,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, server_login_persona_name),
            },
            FieldInfoData {
                name: "ServerLoginProjectTag",
                name_hash: 581897540,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, server_login_project_tag),
            },
            FieldInfoData {
                name: "BlazeServerConnectionTimeout",
                name_hash: 2728137725,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OnlineSettings, blaze_server_connection_timeout),
            },
            FieldInfoData {
                name: "BlazeServerTimeout",
                name_hash: 2514305131,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OnlineSettings, blaze_server_timeout),
            },
            FieldInfoData {
                name: "BlazeServerTunnelSocketRecvBufSize",
                name_hash: 4153439947,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OnlineSettings, blaze_server_tunnel_socket_recv_buf_size),
            },
            FieldInfoData {
                name: "BlazeServerTunnelSocketSendBufSize",
                name_hash: 3398256597,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OnlineSettings, blaze_server_tunnel_socket_send_buf_size),
            },
            FieldInfoData {
                name: "BlazeOutgoingBufferSize",
                name_hash: 2906553942,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OnlineSettings, blaze_outgoing_buffer_size),
            },
            FieldInfoData {
                name: "BlazeClientConnectionTimeout",
                name_hash: 1688792225,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OnlineSettings, blaze_client_connection_timeout),
            },
            FieldInfoData {
                name: "BlazeClientTimeout",
                name_hash: 882909239,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OnlineSettings, blaze_client_timeout),
            },
            FieldInfoData {
                name: "BlazeClientTunnelSocketRecvBufSize",
                name_hash: 3294488215,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OnlineSettings, blaze_client_tunnel_socket_recv_buf_size),
            },
            FieldInfoData {
                name: "BlazeClientTunnelSocketSendBufSize",
                name_hash: 3201215881,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OnlineSettings, blaze_client_tunnel_socket_send_buf_size),
            },
            FieldInfoData {
                name: "ServerAllowAnyReputation",
                name_hash: 766487988,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, server_allow_any_reputation),
            },
            FieldInfoData {
                name: "PeerPort",
                name_hash: 3664055806,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OnlineSettings, peer_port),
            },
            FieldInfoData {
                name: "EnableGamegroupInvites",
                name_hash: 3673903535,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, enable_gamegroup_invites),
            },
            FieldInfoData {
                name: "DirtySockServerPacketQueueCapacity",
                name_hash: 435256971,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(OnlineSettings, dirty_sock_server_packet_queue_capacity),
            },
            FieldInfoData {
                name: "DirtySockMaxConnectionCount",
                name_hash: 902234850,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OnlineSettings, dirty_sock_max_connection_count),
            },
            FieldInfoData {
                name: "BlazeCachedUserRefreshInterval",
                name_hash: 4271724590,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OnlineSettings, blaze_cached_user_refresh_interval),
            },
            FieldInfoData {
                name: "TrustedLoginPath",
                name_hash: 1318149342,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, trusted_login_path),
            },
            FieldInfoData {
                name: "TrustedLoginCertFilename",
                name_hash: 3519380594,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, trusted_login_cert_filename),
            },
            FieldInfoData {
                name: "TrustedLoginKeyFilename",
                name_hash: 3730027045,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, trusted_login_key_filename),
            },
            FieldInfoData {
                name: "EnableNucleusLtOverride",
                name_hash: 3460304063,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, enable_nucleus_lt_override),
            },
            FieldInfoData {
                name: "MinPlayerCapacity",
                name_hash: 1879110856,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OnlineSettings, min_player_capacity),
            },
            FieldInfoData {
                name: "ShouldControlDirtySock",
                name_hash: 2712276173,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, should_control_dirty_sock),
            },
            FieldInfoData {
                name: "DebugMessageCallstackTypeList",
                name_hash: 2961674377,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, debug_message_callstack_type_list),
            },
            FieldInfoData {
                name: "OverrideCreateGameTemplate",
                name_hash: 3945475115,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineSettings, override_create_game_template),
            },
            FieldInfoData {
                name: "OverrideCreateGameTemplateName",
                name_hash: 2819197036,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, override_create_game_template_name),
            },
            FieldInfoData {
                name: "ResettablePool",
                name_hash: 1026634738,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineSettings, resettable_pool),
            },
        ],
    }),
    array_type: Some(ONLINESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineSettings {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINESETTINGS_TYPE_INFO
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


pub static ONLINESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineSettings-Array",
    name_hash: 724108347,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static LOGLEVELTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogLevelType",
    name_hash: 2106472143,
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(LOGLEVELTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LogLevelType {
    fn type_info(&self) -> &'static TypeInfo {
        LOGLEVELTYPE_TYPE_INFO
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


pub static LOGLEVELTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LogLevelType-Array",
    name_hash: 2308768763,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("LogLevelType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineServicesAsset {
    pub _glacier_base: super::core::Asset,
    pub online_services: Vec<Option<LockedTypeObject /* PresenceServiceData */>>,
}

pub trait OnlineServicesAssetTrait: super::core::AssetTrait {
    fn online_services(&self) -> &Vec<Option<LockedTypeObject /* PresenceServiceData */>>;
    fn online_services_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* PresenceServiceData */>>;
}

impl OnlineServicesAssetTrait for OnlineServicesAsset {
    fn online_services(&self) -> &Vec<Option<LockedTypeObject /* PresenceServiceData */>> {
        &self.online_services
    }
    fn online_services_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* PresenceServiceData */>> {
        &mut self.online_services
    }
}

impl super::core::AssetTrait for OnlineServicesAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for OnlineServicesAsset {
}

pub static ONLINESERVICESASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineServicesAsset",
    name_hash: 3977090836,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(OnlineServicesAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineServicesAsset as Default>::default())),
            create_boxed: || Box::new(<OnlineServicesAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "OnlineServices",
                name_hash: 1539201604,
                flags: MemberInfoFlags::new(144),
                field_type: "PresenceServiceData-Array",
                rust_offset: offset_of!(OnlineServicesAsset, online_services),
            },
        ],
    }),
    array_type: Some(ONLINESERVICESASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineServicesAsset {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINESERVICESASSET_TYPE_INFO
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


pub static ONLINESERVICESASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineServicesAsset-Array",
    name_hash: 2987651872,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineServicesAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceProfanityFilterServiceData {
    pub _glacier_base: PresenceServiceData,
}

pub trait PresenceProfanityFilterServiceDataTrait: PresenceServiceDataTrait {
}

impl PresenceProfanityFilterServiceDataTrait for PresenceProfanityFilterServiceData {
}

impl PresenceServiceDataTrait for PresenceProfanityFilterServiceData {
}

impl super::core::AssetTrait for PresenceProfanityFilterServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceProfanityFilterServiceData {
}

pub static PRESENCEPROFANITYFILTERSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceProfanityFilterServiceData",
    name_hash: 3500101041,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceProfanityFilterServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceProfanityFilterServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceProfanityFilterServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEPROFANITYFILTERSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceProfanityFilterServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPROFANITYFILTERSERVICEDATA_TYPE_INFO
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


pub static PRESENCEPROFANITYFILTERSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceProfanityFilterServiceData-Array",
    name_hash: 1760264197,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceProfanityFilterServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceUserProfileServiceData {
    pub _glacier_base: PresenceServiceData,
}

pub trait PresenceUserProfileServiceDataTrait: PresenceServiceDataTrait {
}

impl PresenceUserProfileServiceDataTrait for PresenceUserProfileServiceData {
}

impl PresenceServiceDataTrait for PresenceUserProfileServiceData {
}

impl super::core::AssetTrait for PresenceUserProfileServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceUserProfileServiceData {
}

pub static PRESENCEUSERPROFILESERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserProfileServiceData",
    name_hash: 3953134763,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceUserProfileServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceUserProfileServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceUserProfileServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEUSERPROFILESERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceUserProfileServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEUSERPROFILESERVICEDATA_TYPE_INFO
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


pub static PRESENCEUSERPROFILESERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserProfileServiceData-Array",
    name_hash: 2221181983,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceUserProfileServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceInviteServiceData {
    pub _glacier_base: PresenceServiceData,
}

pub trait PresenceInviteServiceDataTrait: PresenceServiceDataTrait {
}

impl PresenceInviteServiceDataTrait for PresenceInviteServiceData {
}

impl PresenceServiceDataTrait for PresenceInviteServiceData {
}

impl super::core::AssetTrait for PresenceInviteServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceInviteServiceData {
}

pub static PRESENCEINVITESERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceInviteServiceData",
    name_hash: 3672184344,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceInviteServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceInviteServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceInviteServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEINVITESERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceInviteServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEINVITESERVICEDATA_TYPE_INFO
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


pub static PRESENCEINVITESERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceInviteServiceData-Array",
    name_hash: 3079299116,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceInviteServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresencePrivilegeServiceData {
    pub _glacier_base: PresenceServiceData,
}

pub trait PresencePrivilegeServiceDataTrait: PresenceServiceDataTrait {
}

impl PresencePrivilegeServiceDataTrait for PresencePrivilegeServiceData {
}

impl PresenceServiceDataTrait for PresencePrivilegeServiceData {
}

impl super::core::AssetTrait for PresencePrivilegeServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresencePrivilegeServiceData {
}

pub static PRESENCEPRIVILEGESERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePrivilegeServiceData",
    name_hash: 1770939118,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresencePrivilegeServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePrivilegeServiceData as Default>::default())),
            create_boxed: || Box::new(<PresencePrivilegeServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEPRIVILEGESERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresencePrivilegeServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPRIVILEGESERVICEDATA_TYPE_INFO
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


pub static PRESENCEPRIVILEGESERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePrivilegeServiceData-Array",
    name_hash: 3470965722,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresencePrivilegeServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceUserIdServiceData {
    pub _glacier_base: PresenceServiceData,
}

pub trait PresenceUserIdServiceDataTrait: PresenceServiceDataTrait {
}

impl PresenceUserIdServiceDataTrait for PresenceUserIdServiceData {
}

impl PresenceServiceDataTrait for PresenceUserIdServiceData {
}

impl super::core::AssetTrait for PresenceUserIdServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceUserIdServiceData {
}

pub static PRESENCEUSERIDSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserIdServiceData",
    name_hash: 3713473421,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceUserIdServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceUserIdServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceUserIdServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEUSERIDSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceUserIdServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEUSERIDSERVICEDATA_TYPE_INFO
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


pub static PRESENCEUSERIDSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserIdServiceData-Array",
    name_hash: 2823175993,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceUserIdServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceFriendsServiceData {
    pub _glacier_base: PresenceServiceData,
}

pub trait PresenceFriendsServiceDataTrait: PresenceServiceDataTrait {
}

impl PresenceFriendsServiceDataTrait for PresenceFriendsServiceData {
}

impl PresenceServiceDataTrait for PresenceFriendsServiceData {
}

impl super::core::AssetTrait for PresenceFriendsServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceFriendsServiceData {
}

pub static PRESENCEFRIENDSSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFriendsServiceData",
    name_hash: 1934359728,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceFriendsServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceFriendsServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceFriendsServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEFRIENDSSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceFriendsServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEFRIENDSSERVICEDATA_TYPE_INFO
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


pub static PRESENCEFRIENDSSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFriendsServiceData-Array",
    name_hash: 2538088964,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceFriendsServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceBlobServiceData {
    pub _glacier_base: PresenceServiceData,
}

pub trait PresenceBlobServiceDataTrait: PresenceServiceDataTrait {
}

impl PresenceBlobServiceDataTrait for PresenceBlobServiceData {
}

impl PresenceServiceDataTrait for PresenceBlobServiceData {
}

impl super::core::AssetTrait for PresenceBlobServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceBlobServiceData {
}

pub static PRESENCEBLOBSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlobServiceData",
    name_hash: 795589778,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceBlobServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBlobServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceBlobServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEBLOBSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceBlobServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEBLOBSERVICEDATA_TYPE_INFO
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


pub static PRESENCEBLOBSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlobServiceData-Array",
    name_hash: 3812934566,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceBlobServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceBlazeUserInfoServiceData {
    pub _glacier_base: PresenceServiceData,
    pub game_browser_config_name: String,
}

pub trait PresenceBlazeUserInfoServiceDataTrait: PresenceServiceDataTrait {
    fn game_browser_config_name(&self) -> &String;
    fn game_browser_config_name_mut(&mut self) -> &mut String;
}

impl PresenceBlazeUserInfoServiceDataTrait for PresenceBlazeUserInfoServiceData {
    fn game_browser_config_name(&self) -> &String {
        &self.game_browser_config_name
    }
    fn game_browser_config_name_mut(&mut self) -> &mut String {
        &mut self.game_browser_config_name
    }
}

impl PresenceServiceDataTrait for PresenceBlazeUserInfoServiceData {
}

impl super::core::AssetTrait for PresenceBlazeUserInfoServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceBlazeUserInfoServiceData {
}

pub static PRESENCEBLAZEUSERINFOSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlazeUserInfoServiceData",
    name_hash: 2144229822,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceBlazeUserInfoServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBlazeUserInfoServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceBlazeUserInfoServiceData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "GameBrowserConfigName",
                name_hash: 3592995882,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PresenceBlazeUserInfoServiceData, game_browser_config_name),
            },
        ],
    }),
    array_type: Some(PRESENCEBLAZEUSERINFOSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceBlazeUserInfoServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEBLAZEUSERINFOSERVICEDATA_TYPE_INFO
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


pub static PRESENCEBLAZEUSERINFOSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlazeUserInfoServiceData-Array",
    name_hash: 1666435594,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceBlazeUserInfoServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceAuthenticationServiceData {
    pub _glacier_base: PresenceServiceData,
}

pub trait PresenceAuthenticationServiceDataTrait: PresenceServiceDataTrait {
}

impl PresenceAuthenticationServiceDataTrait for PresenceAuthenticationServiceData {
}

impl PresenceServiceDataTrait for PresenceAuthenticationServiceData {
}

impl super::core::AssetTrait for PresenceAuthenticationServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceAuthenticationServiceData {
}

pub static PRESENCEAUTHENTICATIONSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAuthenticationServiceData",
    name_hash: 3128828369,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceAuthenticationServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAuthenticationServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceAuthenticationServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCEAUTHENTICATIONSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceAuthenticationServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEAUTHENTICATIONSERVICEDATA_TYPE_INFO
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


pub static PRESENCEAUTHENTICATIONSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAuthenticationServiceData-Array",
    name_hash: 95990757,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceAuthenticationServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceConnectionServiceData {
    pub _glacier_base: PresenceServiceData,
}

pub trait PresenceConnectionServiceDataTrait: PresenceServiceDataTrait {
}

impl PresenceConnectionServiceDataTrait for PresenceConnectionServiceData {
}

impl PresenceServiceDataTrait for PresenceConnectionServiceData {
}

impl super::core::AssetTrait for PresenceConnectionServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceConnectionServiceData {
}

pub static PRESENCECONNECTIONSERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceConnectionServiceData",
    name_hash: 1376652071,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCESERVICEDATA_TYPE_INFO),
        super_class_offset: offset_of!(PresenceConnectionServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceConnectionServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceConnectionServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCECONNECTIONSERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceConnectionServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCECONNECTIONSERVICEDATA_TYPE_INFO
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


pub static PRESENCECONNECTIONSERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceConnectionServiceData-Array",
    name_hash: 3971464339,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceConnectionServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceServiceData {
    pub _glacier_base: super::core::Asset,
}

pub trait PresenceServiceDataTrait: super::core::AssetTrait {
}

impl PresenceServiceDataTrait for PresenceServiceData {
}

impl super::core::AssetTrait for PresenceServiceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceServiceData {
}

pub static PRESENCESERVICEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceServiceData",
    name_hash: 1815865265,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(PresenceServiceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceServiceData as Default>::default())),
            create_boxed: || Box::new(<PresenceServiceData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PRESENCESERVICEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceServiceData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCESERVICEDATA_TYPE_INFO
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


pub static PRESENCESERVICEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceServiceData-Array",
    name_hash: 3524499973,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceServiceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineProviderAsset {
    pub _glacier_base: super::core::Asset,
    pub configurations: Vec<BoxedTypeObject /* OnlineProviderConfiguration */>,
}

pub trait OnlineProviderAssetTrait: super::core::AssetTrait {
    fn configurations(&self) -> &Vec<BoxedTypeObject /* OnlineProviderConfiguration */>;
    fn configurations_mut(&mut self) -> &mut Vec<BoxedTypeObject /* OnlineProviderConfiguration */>;
}

impl OnlineProviderAssetTrait for OnlineProviderAsset {
    fn configurations(&self) -> &Vec<BoxedTypeObject /* OnlineProviderConfiguration */> {
        &self.configurations
    }
    fn configurations_mut(&mut self) -> &mut Vec<BoxedTypeObject /* OnlineProviderConfiguration */> {
        &mut self.configurations
    }
}

impl super::core::AssetTrait for OnlineProviderAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for OnlineProviderAsset {
}

pub static ONLINEPROVIDERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineProviderAsset",
    name_hash: 329639163,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(OnlineProviderAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineProviderAsset as Default>::default())),
            create_boxed: || Box::new(<OnlineProviderAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Configurations",
                name_hash: 4211240070,
                flags: MemberInfoFlags::new(144),
                field_type: "OnlineProviderConfiguration-Array",
                rust_offset: offset_of!(OnlineProviderAsset, configurations),
            },
        ],
    }),
    array_type: Some(ONLINEPROVIDERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineProviderAsset {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEPROVIDERASSET_TYPE_INFO
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


pub static ONLINEPROVIDERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineProviderAsset-Array",
    name_hash: 374496463,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineProviderAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineProviderConfiguration {
    pub platform: super::core::GamePlatform,
    pub is_server: bool,
    pub service_name: String,
    pub client: String,
    pub s_k_u: String,
    pub version: String,
    pub server_socket_packet_size: u32,
}

pub trait OnlineProviderConfigurationTrait: TypeObject {
    fn platform(&self) -> &super::core::GamePlatform;
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform;
    fn is_server(&self) -> &bool;
    fn is_server_mut(&mut self) -> &mut bool;
    fn service_name(&self) -> &String;
    fn service_name_mut(&mut self) -> &mut String;
    fn client(&self) -> &String;
    fn client_mut(&mut self) -> &mut String;
    fn s_k_u(&self) -> &String;
    fn s_k_u_mut(&mut self) -> &mut String;
    fn version(&self) -> &String;
    fn version_mut(&mut self) -> &mut String;
    fn server_socket_packet_size(&self) -> &u32;
    fn server_socket_packet_size_mut(&mut self) -> &mut u32;
}

impl OnlineProviderConfigurationTrait for OnlineProviderConfiguration {
    fn platform(&self) -> &super::core::GamePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        &mut self.platform
    }
    fn is_server(&self) -> &bool {
        &self.is_server
    }
    fn is_server_mut(&mut self) -> &mut bool {
        &mut self.is_server
    }
    fn service_name(&self) -> &String {
        &self.service_name
    }
    fn service_name_mut(&mut self) -> &mut String {
        &mut self.service_name
    }
    fn client(&self) -> &String {
        &self.client
    }
    fn client_mut(&mut self) -> &mut String {
        &mut self.client
    }
    fn s_k_u(&self) -> &String {
        &self.s_k_u
    }
    fn s_k_u_mut(&mut self) -> &mut String {
        &mut self.s_k_u
    }
    fn version(&self) -> &String {
        &self.version
    }
    fn version_mut(&mut self) -> &mut String {
        &mut self.version
    }
    fn server_socket_packet_size(&self) -> &u32 {
        &self.server_socket_packet_size
    }
    fn server_socket_packet_size_mut(&mut self) -> &mut u32 {
        &mut self.server_socket_packet_size
    }
}

pub static ONLINEPROVIDERCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineProviderConfiguration",
    name_hash: 2373042011,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineProviderConfiguration as Default>::default())),
            create_boxed: || Box::new(<OnlineProviderConfiguration as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                name_hash: 942751002,
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(OnlineProviderConfiguration, platform),
            },
            FieldInfoData {
                name: "IsServer",
                name_hash: 452268730,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlineProviderConfiguration, is_server),
            },
            FieldInfoData {
                name: "ServiceName",
                name_hash: 2487476607,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineProviderConfiguration, service_name),
            },
            FieldInfoData {
                name: "Client",
                name_hash: 2721713788,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineProviderConfiguration, client),
            },
            FieldInfoData {
                name: "SKU",
                name_hash: 193467592,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineProviderConfiguration, s_k_u),
            },
            FieldInfoData {
                name: "Version",
                name_hash: 747123679,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineProviderConfiguration, version),
            },
            FieldInfoData {
                name: "ServerSocketPacketSize",
                name_hash: 2343662152,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OnlineProviderConfiguration, server_socket_packet_size),
            },
        ],
    }),
    array_type: Some(ONLINEPROVIDERCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineProviderConfiguration {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEPROVIDERCONFIGURATION_TYPE_INFO
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


pub static ONLINEPROVIDERCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineProviderConfiguration-Array",
    name_hash: 3547858543,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineProviderConfiguration"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlinePlatformConfiguration {
    pub platform: super::core::GamePlatform,
    pub is_fallback: bool,
    pub platform_data: Option<LockedTypeObject /* OnlinePlatformData */>,
    pub services: Option<LockedTypeObject /* OnlineServicesAsset */>,
    pub server_services: Option<LockedTypeObject /* OnlineServicesAsset */>,
    pub client_backends: Vec<Option<LockedTypeObject /* PresenceBackendData */>>,
    pub server_backends: Vec<Option<LockedTypeObject /* PresenceBackendData */>>,
    pub server_game_backends: Vec<Option<LockedTypeObject /* PresenceBackendData */>>,
}

pub trait OnlinePlatformConfigurationTrait: TypeObject {
    fn platform(&self) -> &super::core::GamePlatform;
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform;
    fn is_fallback(&self) -> &bool;
    fn is_fallback_mut(&mut self) -> &mut bool;
    fn platform_data(&self) -> &Option<LockedTypeObject /* OnlinePlatformData */>;
    fn platform_data_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlinePlatformData */>;
    fn services(&self) -> &Option<LockedTypeObject /* OnlineServicesAsset */>;
    fn services_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineServicesAsset */>;
    fn server_services(&self) -> &Option<LockedTypeObject /* OnlineServicesAsset */>;
    fn server_services_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineServicesAsset */>;
    fn client_backends(&self) -> &Vec<Option<LockedTypeObject /* PresenceBackendData */>>;
    fn client_backends_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* PresenceBackendData */>>;
    fn server_backends(&self) -> &Vec<Option<LockedTypeObject /* PresenceBackendData */>>;
    fn server_backends_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* PresenceBackendData */>>;
    fn server_game_backends(&self) -> &Vec<Option<LockedTypeObject /* PresenceBackendData */>>;
    fn server_game_backends_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* PresenceBackendData */>>;
}

impl OnlinePlatformConfigurationTrait for OnlinePlatformConfiguration {
    fn platform(&self) -> &super::core::GamePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        &mut self.platform
    }
    fn is_fallback(&self) -> &bool {
        &self.is_fallback
    }
    fn is_fallback_mut(&mut self) -> &mut bool {
        &mut self.is_fallback
    }
    fn platform_data(&self) -> &Option<LockedTypeObject /* OnlinePlatformData */> {
        &self.platform_data
    }
    fn platform_data_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlinePlatformData */> {
        &mut self.platform_data
    }
    fn services(&self) -> &Option<LockedTypeObject /* OnlineServicesAsset */> {
        &self.services
    }
    fn services_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineServicesAsset */> {
        &mut self.services
    }
    fn server_services(&self) -> &Option<LockedTypeObject /* OnlineServicesAsset */> {
        &self.server_services
    }
    fn server_services_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineServicesAsset */> {
        &mut self.server_services
    }
    fn client_backends(&self) -> &Vec<Option<LockedTypeObject /* PresenceBackendData */>> {
        &self.client_backends
    }
    fn client_backends_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* PresenceBackendData */>> {
        &mut self.client_backends
    }
    fn server_backends(&self) -> &Vec<Option<LockedTypeObject /* PresenceBackendData */>> {
        &self.server_backends
    }
    fn server_backends_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* PresenceBackendData */>> {
        &mut self.server_backends
    }
    fn server_game_backends(&self) -> &Vec<Option<LockedTypeObject /* PresenceBackendData */>> {
        &self.server_game_backends
    }
    fn server_game_backends_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* PresenceBackendData */>> {
        &mut self.server_game_backends
    }
}

pub static ONLINEPLATFORMCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePlatformConfiguration",
    name_hash: 3795310405,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlinePlatformConfiguration as Default>::default())),
            create_boxed: || Box::new(<OnlinePlatformConfiguration as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                name_hash: 942751002,
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(OnlinePlatformConfiguration, platform),
            },
            FieldInfoData {
                name: "IsFallback",
                name_hash: 72751187,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(OnlinePlatformConfiguration, is_fallback),
            },
            FieldInfoData {
                name: "PlatformData",
                name_hash: 977066282,
                flags: MemberInfoFlags::new(0),
                field_type: "OnlinePlatformData",
                rust_offset: offset_of!(OnlinePlatformConfiguration, platform_data),
            },
            FieldInfoData {
                name: "Services",
                name_hash: 270289867,
                flags: MemberInfoFlags::new(0),
                field_type: "OnlineServicesAsset",
                rust_offset: offset_of!(OnlinePlatformConfiguration, services),
            },
            FieldInfoData {
                name: "ServerServices",
                name_hash: 1886794158,
                flags: MemberInfoFlags::new(0),
                field_type: "OnlineServicesAsset",
                rust_offset: offset_of!(OnlinePlatformConfiguration, server_services),
            },
            FieldInfoData {
                name: "ClientBackends",
                name_hash: 3444245387,
                flags: MemberInfoFlags::new(144),
                field_type: "PresenceBackendData-Array",
                rust_offset: offset_of!(OnlinePlatformConfiguration, client_backends),
            },
            FieldInfoData {
                name: "ServerBackends",
                name_hash: 2837482711,
                flags: MemberInfoFlags::new(144),
                field_type: "PresenceBackendData-Array",
                rust_offset: offset_of!(OnlinePlatformConfiguration, server_backends),
            },
            FieldInfoData {
                name: "ServerGameBackends",
                name_hash: 3033436537,
                flags: MemberInfoFlags::new(144),
                field_type: "PresenceBackendData-Array",
                rust_offset: offset_of!(OnlinePlatformConfiguration, server_game_backends),
            },
        ],
    }),
    array_type: Some(ONLINEPLATFORMCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlinePlatformConfiguration {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEPLATFORMCONFIGURATION_TYPE_INFO
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


pub static ONLINEPLATFORMCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePlatformConfiguration-Array",
    name_hash: 267480945,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlinePlatformConfiguration"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Ps4OnlineData {
    pub _glacier_base: OnlinePlatformData,
    pub default_title_data: Ps4OnlineTitleData,
    pub title_data: Vec<BoxedTypeObject /* Ps4OnlineTitleData */>,
}

pub trait Ps4OnlineDataTrait: OnlinePlatformDataTrait {
    fn default_title_data(&self) -> &Ps4OnlineTitleData;
    fn default_title_data_mut(&mut self) -> &mut Ps4OnlineTitleData;
    fn title_data(&self) -> &Vec<BoxedTypeObject /* Ps4OnlineTitleData */>;
    fn title_data_mut(&mut self) -> &mut Vec<BoxedTypeObject /* Ps4OnlineTitleData */>;
}

impl Ps4OnlineDataTrait for Ps4OnlineData {
    fn default_title_data(&self) -> &Ps4OnlineTitleData {
        &self.default_title_data
    }
    fn default_title_data_mut(&mut self) -> &mut Ps4OnlineTitleData {
        &mut self.default_title_data
    }
    fn title_data(&self) -> &Vec<BoxedTypeObject /* Ps4OnlineTitleData */> {
        &self.title_data
    }
    fn title_data_mut(&mut self) -> &mut Vec<BoxedTypeObject /* Ps4OnlineTitleData */> {
        &mut self.title_data
    }
}

impl OnlinePlatformDataTrait for Ps4OnlineData {
}

impl super::core::AssetTrait for Ps4OnlineData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for Ps4OnlineData {
}

pub static PS4ONLINEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4OnlineData",
    name_hash: 3708529517,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ONLINEPLATFORMDATA_TYPE_INFO),
        super_class_offset: offset_of!(Ps4OnlineData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Ps4OnlineData as Default>::default())),
            create_boxed: || Box::new(<Ps4OnlineData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DefaultTitleData",
                name_hash: 2266845566,
                flags: MemberInfoFlags::new(0),
                field_type: "Ps4OnlineTitleData",
                rust_offset: offset_of!(Ps4OnlineData, default_title_data),
            },
            FieldInfoData {
                name: "TitleData",
                name_hash: 3651021941,
                flags: MemberInfoFlags::new(144),
                field_type: "Ps4OnlineTitleData-Array",
                rust_offset: offset_of!(Ps4OnlineData, title_data),
            },
        ],
    }),
    array_type: Some(PS4ONLINEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4OnlineData {
    fn type_info(&self) -> &'static TypeInfo {
        PS4ONLINEDATA_TYPE_INFO
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


pub static PS4ONLINEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4OnlineData-Array",
    name_hash: 1469296729,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("Ps4OnlineData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Ps4OnlineTitleData {
    pub title_id: String,
    pub title_secret: String,
}

pub trait Ps4OnlineTitleDataTrait: TypeObject {
    fn title_id(&self) -> &String;
    fn title_id_mut(&mut self) -> &mut String;
    fn title_secret(&self) -> &String;
    fn title_secret_mut(&mut self) -> &mut String;
}

impl Ps4OnlineTitleDataTrait for Ps4OnlineTitleData {
    fn title_id(&self) -> &String {
        &self.title_id
    }
    fn title_id_mut(&mut self) -> &mut String {
        &mut self.title_id
    }
    fn title_secret(&self) -> &String {
        &self.title_secret
    }
    fn title_secret_mut(&mut self) -> &mut String {
        &mut self.title_secret
    }
}

pub static PS4ONLINETITLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4OnlineTitleData",
    name_hash: 2457201133,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Ps4OnlineTitleData as Default>::default())),
            create_boxed: || Box::new(<Ps4OnlineTitleData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TitleId",
                name_hash: 3335995016,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(Ps4OnlineTitleData, title_id),
            },
            FieldInfoData {
                name: "TitleSecret",
                name_hash: 3865331987,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(Ps4OnlineTitleData, title_secret),
            },
        ],
    }),
    array_type: Some(PS4ONLINETITLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4OnlineTitleData {
    fn type_info(&self) -> &'static TypeInfo {
        PS4ONLINETITLEDATA_TYPE_INFO
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


pub static PS4ONLINETITLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4OnlineTitleData-Array",
    name_hash: 206753497,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("Ps4OnlineTitleData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlinePlatformData {
    pub _glacier_base: super::core::Asset,
}

pub trait OnlinePlatformDataTrait: super::core::AssetTrait {
}

impl OnlinePlatformDataTrait for OnlinePlatformData {
}

impl super::core::AssetTrait for OnlinePlatformData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for OnlinePlatformData {
}

pub static ONLINEPLATFORMDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePlatformData",
    name_hash: 3902197733,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(OnlinePlatformData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlinePlatformData as Default>::default())),
            create_boxed: || Box::new(<OnlinePlatformData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(ONLINEPLATFORMDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlinePlatformData {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEPLATFORMDATA_TYPE_INFO
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


pub static ONLINEPLATFORMDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePlatformData-Array",
    name_hash: 176740561,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlinePlatformData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineRichPresenceData {
    pub _glacier_base: super::core::Asset,
    pub rich_presence_strings: Vec<Option<LockedTypeObject /* OnlineRichPresenceString */>>,
    pub contexts: Vec<Option<LockedTypeObject /* OnlineRichPresenceContext */>>,
}

pub trait OnlineRichPresenceDataTrait: super::core::AssetTrait {
    fn rich_presence_strings(&self) -> &Vec<Option<LockedTypeObject /* OnlineRichPresenceString */>>;
    fn rich_presence_strings_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* OnlineRichPresenceString */>>;
    fn contexts(&self) -> &Vec<Option<LockedTypeObject /* OnlineRichPresenceContext */>>;
    fn contexts_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* OnlineRichPresenceContext */>>;
}

impl OnlineRichPresenceDataTrait for OnlineRichPresenceData {
    fn rich_presence_strings(&self) -> &Vec<Option<LockedTypeObject /* OnlineRichPresenceString */>> {
        &self.rich_presence_strings
    }
    fn rich_presence_strings_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* OnlineRichPresenceString */>> {
        &mut self.rich_presence_strings
    }
    fn contexts(&self) -> &Vec<Option<LockedTypeObject /* OnlineRichPresenceContext */>> {
        &self.contexts
    }
    fn contexts_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* OnlineRichPresenceContext */>> {
        &mut self.contexts
    }
}

impl super::core::AssetTrait for OnlineRichPresenceData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for OnlineRichPresenceData {
}

pub static ONLINERICHPRESENCEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceData",
    name_hash: 1271156723,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(OnlineRichPresenceData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineRichPresenceData as Default>::default())),
            create_boxed: || Box::new(<OnlineRichPresenceData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RichPresenceStrings",
                name_hash: 1876374026,
                flags: MemberInfoFlags::new(144),
                field_type: "OnlineRichPresenceString-Array",
                rust_offset: offset_of!(OnlineRichPresenceData, rich_presence_strings),
            },
            FieldInfoData {
                name: "Contexts",
                name_hash: 333666601,
                flags: MemberInfoFlags::new(144),
                field_type: "OnlineRichPresenceContext-Array",
                rust_offset: offset_of!(OnlineRichPresenceData, contexts),
            },
        ],
    }),
    array_type: Some(ONLINERICHPRESENCEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineRichPresenceData {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINERICHPRESENCEDATA_TYPE_INFO
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


pub static ONLINERICHPRESENCEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceData-Array",
    name_hash: 3252005831,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineRichPresenceData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineRichPresenceContextValuePair {
    pub _glacier_base: super::core::DataContainer,
    pub context: Option<LockedTypeObject /* OnlineRichPresenceContext */>,
    pub value: Option<LockedTypeObject /* OnlineRichPresenceContextValue */>,
}

pub trait OnlineRichPresenceContextValuePairTrait: super::core::DataContainerTrait {
    fn context(&self) -> &Option<LockedTypeObject /* OnlineRichPresenceContext */>;
    fn context_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineRichPresenceContext */>;
    fn value(&self) -> &Option<LockedTypeObject /* OnlineRichPresenceContextValue */>;
    fn value_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineRichPresenceContextValue */>;
}

impl OnlineRichPresenceContextValuePairTrait for OnlineRichPresenceContextValuePair {
    fn context(&self) -> &Option<LockedTypeObject /* OnlineRichPresenceContext */> {
        &self.context
    }
    fn context_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineRichPresenceContext */> {
        &mut self.context
    }
    fn value(&self) -> &Option<LockedTypeObject /* OnlineRichPresenceContextValue */> {
        &self.value
    }
    fn value_mut(&mut self) -> &mut Option<LockedTypeObject /* OnlineRichPresenceContextValue */> {
        &mut self.value
    }
}

impl super::core::DataContainerTrait for OnlineRichPresenceContextValuePair {
}

pub static ONLINERICHPRESENCECONTEXTVALUEPAIR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContextValuePair",
    name_hash: 3458760285,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(OnlineRichPresenceContextValuePair, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineRichPresenceContextValuePair as Default>::default())),
            create_boxed: || Box::new(<OnlineRichPresenceContextValuePair as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Context",
                name_hash: 3654325786,
                flags: MemberInfoFlags::new(0),
                field_type: "OnlineRichPresenceContext",
                rust_offset: offset_of!(OnlineRichPresenceContextValuePair, context),
            },
            FieldInfoData {
                name: "Value",
                name_hash: 225375086,
                flags: MemberInfoFlags::new(0),
                field_type: "OnlineRichPresenceContextValue",
                rust_offset: offset_of!(OnlineRichPresenceContextValuePair, value),
            },
        ],
    }),
    array_type: Some(ONLINERICHPRESENCECONTEXTVALUEPAIR_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineRichPresenceContextValuePair {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINERICHPRESENCECONTEXTVALUEPAIR_TYPE_INFO
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


pub static ONLINERICHPRESENCECONTEXTVALUEPAIR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContextValuePair-Array",
    name_hash: 2140502633,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineRichPresenceContextValuePair"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineRichPresenceString {
    pub _glacier_base: super::core::DataContainer,
    pub sid: String,
    pub xdp_name: String,
    pub index: u8,
}

pub trait OnlineRichPresenceStringTrait: super::core::DataContainerTrait {
    fn sid(&self) -> &String;
    fn sid_mut(&mut self) -> &mut String;
    fn xdp_name(&self) -> &String;
    fn xdp_name_mut(&mut self) -> &mut String;
    fn index(&self) -> &u8;
    fn index_mut(&mut self) -> &mut u8;
}

impl OnlineRichPresenceStringTrait for OnlineRichPresenceString {
    fn sid(&self) -> &String {
        &self.sid
    }
    fn sid_mut(&mut self) -> &mut String {
        &mut self.sid
    }
    fn xdp_name(&self) -> &String {
        &self.xdp_name
    }
    fn xdp_name_mut(&mut self) -> &mut String {
        &mut self.xdp_name
    }
    fn index(&self) -> &u8 {
        &self.index
    }
    fn index_mut(&mut self) -> &mut u8 {
        &mut self.index
    }
}

impl super::core::DataContainerTrait for OnlineRichPresenceString {
}

pub static ONLINERICHPRESENCESTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceString",
    name_hash: 1571188182,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(OnlineRichPresenceString, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineRichPresenceString as Default>::default())),
            create_boxed: || Box::new(<OnlineRichPresenceString as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Sid",
                name_hash: 193466587,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineRichPresenceString, sid),
            },
            FieldInfoData {
                name: "XdpName",
                name_hash: 1445548590,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineRichPresenceString, xdp_name),
            },
            FieldInfoData {
                name: "Index",
                name_hash: 214509467,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OnlineRichPresenceString, index),
            },
        ],
    }),
    array_type: Some(ONLINERICHPRESENCESTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineRichPresenceString {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINERICHPRESENCESTRING_TYPE_INFO
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


pub static ONLINERICHPRESENCESTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceString-Array",
    name_hash: 3256832866,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineRichPresenceString"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineRichPresenceContext {
    pub _glacier_base: super::core::DataContainer,
    pub name: String,
    pub xdp_name: String,
    pub values: Vec<Option<LockedTypeObject /* OnlineRichPresenceContextValue */>>,
    pub index: u8,
}

pub trait OnlineRichPresenceContextTrait: super::core::DataContainerTrait {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn xdp_name(&self) -> &String;
    fn xdp_name_mut(&mut self) -> &mut String;
    fn values(&self) -> &Vec<Option<LockedTypeObject /* OnlineRichPresenceContextValue */>>;
    fn values_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* OnlineRichPresenceContextValue */>>;
    fn index(&self) -> &u8;
    fn index_mut(&mut self) -> &mut u8;
}

impl OnlineRichPresenceContextTrait for OnlineRichPresenceContext {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn xdp_name(&self) -> &String {
        &self.xdp_name
    }
    fn xdp_name_mut(&mut self) -> &mut String {
        &mut self.xdp_name
    }
    fn values(&self) -> &Vec<Option<LockedTypeObject /* OnlineRichPresenceContextValue */>> {
        &self.values
    }
    fn values_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* OnlineRichPresenceContextValue */>> {
        &mut self.values
    }
    fn index(&self) -> &u8 {
        &self.index
    }
    fn index_mut(&mut self) -> &mut u8 {
        &mut self.index
    }
}

impl super::core::DataContainerTrait for OnlineRichPresenceContext {
}

pub static ONLINERICHPRESENCECONTEXT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContext",
    name_hash: 2216098524,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(OnlineRichPresenceContext, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineRichPresenceContext as Default>::default())),
            create_boxed: || Box::new(<OnlineRichPresenceContext as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineRichPresenceContext, name),
            },
            FieldInfoData {
                name: "XdpName",
                name_hash: 1445548590,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineRichPresenceContext, xdp_name),
            },
            FieldInfoData {
                name: "Values",
                name_hash: 3142410589,
                flags: MemberInfoFlags::new(144),
                field_type: "OnlineRichPresenceContextValue-Array",
                rust_offset: offset_of!(OnlineRichPresenceContext, values),
            },
            FieldInfoData {
                name: "Index",
                name_hash: 214509467,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OnlineRichPresenceContext, index),
            },
        ],
    }),
    array_type: Some(ONLINERICHPRESENCECONTEXT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineRichPresenceContext {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINERICHPRESENCECONTEXT_TYPE_INFO
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


pub static ONLINERICHPRESENCECONTEXT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContext-Array",
    name_hash: 2018280296,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineRichPresenceContext"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineRichPresenceContextValue {
    pub _glacier_base: super::core::DataContainer,
    pub sid: String,
    pub key: String,
    pub xdp_name: String,
    pub index: u8,
}

pub trait OnlineRichPresenceContextValueTrait: super::core::DataContainerTrait {
    fn sid(&self) -> &String;
    fn sid_mut(&mut self) -> &mut String;
    fn key(&self) -> &String;
    fn key_mut(&mut self) -> &mut String;
    fn xdp_name(&self) -> &String;
    fn xdp_name_mut(&mut self) -> &mut String;
    fn index(&self) -> &u8;
    fn index_mut(&mut self) -> &mut u8;
}

impl OnlineRichPresenceContextValueTrait for OnlineRichPresenceContextValue {
    fn sid(&self) -> &String {
        &self.sid
    }
    fn sid_mut(&mut self) -> &mut String {
        &mut self.sid
    }
    fn key(&self) -> &String {
        &self.key
    }
    fn key_mut(&mut self) -> &mut String {
        &mut self.key
    }
    fn xdp_name(&self) -> &String {
        &self.xdp_name
    }
    fn xdp_name_mut(&mut self) -> &mut String {
        &mut self.xdp_name
    }
    fn index(&self) -> &u8 {
        &self.index
    }
    fn index_mut(&mut self) -> &mut u8 {
        &mut self.index
    }
}

impl super::core::DataContainerTrait for OnlineRichPresenceContextValue {
}

pub static ONLINERICHPRESENCECONTEXTVALUE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContextValue",
    name_hash: 2799095703,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(OnlineRichPresenceContextValue, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineRichPresenceContextValue as Default>::default())),
            create_boxed: || Box::new(<OnlineRichPresenceContextValue as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Sid",
                name_hash: 193466587,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineRichPresenceContextValue, sid),
            },
            FieldInfoData {
                name: "Key",
                name_hash: 193457490,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineRichPresenceContextValue, key),
            },
            FieldInfoData {
                name: "XdpName",
                name_hash: 1445548590,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineRichPresenceContextValue, xdp_name),
            },
            FieldInfoData {
                name: "Index",
                name_hash: 214509467,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(OnlineRichPresenceContextValue, index),
            },
        ],
    }),
    array_type: Some(ONLINERICHPRESENCECONTEXTVALUE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineRichPresenceContextValue {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINERICHPRESENCECONTEXTVALUE_TYPE_INFO
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


pub static ONLINERICHPRESENCECONTEXTVALUE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineRichPresenceContextValue-Array",
    name_hash: 1862850595,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineRichPresenceContextValue"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineEnvironmentConsoleUrl {
    pub urls: Vec<BoxedTypeObject /* OnlineEnvironmentConsoleUrlData */>,
}

pub trait OnlineEnvironmentConsoleUrlTrait: TypeObject {
    fn urls(&self) -> &Vec<BoxedTypeObject /* OnlineEnvironmentConsoleUrlData */>;
    fn urls_mut(&mut self) -> &mut Vec<BoxedTypeObject /* OnlineEnvironmentConsoleUrlData */>;
}

impl OnlineEnvironmentConsoleUrlTrait for OnlineEnvironmentConsoleUrl {
    fn urls(&self) -> &Vec<BoxedTypeObject /* OnlineEnvironmentConsoleUrlData */> {
        &self.urls
    }
    fn urls_mut(&mut self) -> &mut Vec<BoxedTypeObject /* OnlineEnvironmentConsoleUrlData */> {
        &mut self.urls
    }
}

pub static ONLINEENVIRONMENTCONSOLEURL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentConsoleUrl",
    name_hash: 217316099,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineEnvironmentConsoleUrl as Default>::default())),
            create_boxed: || Box::new(<OnlineEnvironmentConsoleUrl as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Urls",
                name_hash: 2089048349,
                flags: MemberInfoFlags::new(144),
                field_type: "OnlineEnvironmentConsoleUrlData-Array",
                rust_offset: offset_of!(OnlineEnvironmentConsoleUrl, urls),
            },
        ],
    }),
    array_type: Some(ONLINEENVIRONMENTCONSOLEURL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineEnvironmentConsoleUrl {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEENVIRONMENTCONSOLEURL_TYPE_INFO
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


pub static ONLINEENVIRONMENTCONSOLEURL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentConsoleUrl-Array",
    name_hash: 3412471223,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineEnvironmentConsoleUrl"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineEnvironmentConsoleUrlData {
    pub platform: super::core::GamePlatform,
    pub url: OnlineEnvironmentUrl,
}

pub trait OnlineEnvironmentConsoleUrlDataTrait: TypeObject {
    fn platform(&self) -> &super::core::GamePlatform;
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform;
    fn url(&self) -> &OnlineEnvironmentUrl;
    fn url_mut(&mut self) -> &mut OnlineEnvironmentUrl;
}

impl OnlineEnvironmentConsoleUrlDataTrait for OnlineEnvironmentConsoleUrlData {
    fn platform(&self) -> &super::core::GamePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        &mut self.platform
    }
    fn url(&self) -> &OnlineEnvironmentUrl {
        &self.url
    }
    fn url_mut(&mut self) -> &mut OnlineEnvironmentUrl {
        &mut self.url
    }
}

pub static ONLINEENVIRONMENTCONSOLEURLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentConsoleUrlData",
    name_hash: 210794483,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineEnvironmentConsoleUrlData as Default>::default())),
            create_boxed: || Box::new(<OnlineEnvironmentConsoleUrlData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                name_hash: 942751002,
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(OnlineEnvironmentConsoleUrlData, platform),
            },
            FieldInfoData {
                name: "Url",
                name_hash: 193455022,
                flags: MemberInfoFlags::new(0),
                field_type: "OnlineEnvironmentUrl",
                rust_offset: offset_of!(OnlineEnvironmentConsoleUrlData, url),
            },
        ],
    }),
    array_type: Some(ONLINEENVIRONMENTCONSOLEURLDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineEnvironmentConsoleUrlData {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEENVIRONMENTCONSOLEURLDATA_TYPE_INFO
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


pub static ONLINEENVIRONMENTCONSOLEURLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentConsoleUrlData-Array",
    name_hash: 85971911,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineEnvironmentConsoleUrlData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineEnvironmentUrl {
    pub urls: Vec<BoxedTypeObject /* OnlineEnvironmentUrlData */>,
}

pub trait OnlineEnvironmentUrlTrait: TypeObject {
    fn urls(&self) -> &Vec<BoxedTypeObject /* OnlineEnvironmentUrlData */>;
    fn urls_mut(&mut self) -> &mut Vec<BoxedTypeObject /* OnlineEnvironmentUrlData */>;
}

impl OnlineEnvironmentUrlTrait for OnlineEnvironmentUrl {
    fn urls(&self) -> &Vec<BoxedTypeObject /* OnlineEnvironmentUrlData */> {
        &self.urls
    }
    fn urls_mut(&mut self) -> &mut Vec<BoxedTypeObject /* OnlineEnvironmentUrlData */> {
        &mut self.urls
    }
}

pub static ONLINEENVIRONMENTURL_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentUrl",
    name_hash: 152272084,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineEnvironmentUrl as Default>::default())),
            create_boxed: || Box::new(<OnlineEnvironmentUrl as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Urls",
                name_hash: 2089048349,
                flags: MemberInfoFlags::new(144),
                field_type: "OnlineEnvironmentUrlData-Array",
                rust_offset: offset_of!(OnlineEnvironmentUrl, urls),
            },
        ],
    }),
    array_type: Some(ONLINEENVIRONMENTURL_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineEnvironmentUrl {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEENVIRONMENTURL_TYPE_INFO
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


pub static ONLINEENVIRONMENTURL_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentUrl-Array",
    name_hash: 2594384224,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineEnvironmentUrl"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OnlineEnvironmentUrlData {
    pub url: String,
    pub environment: OnlineEnvironment,
}

pub trait OnlineEnvironmentUrlDataTrait: TypeObject {
    fn url(&self) -> &String;
    fn url_mut(&mut self) -> &mut String;
    fn environment(&self) -> &OnlineEnvironment;
    fn environment_mut(&mut self) -> &mut OnlineEnvironment;
}

impl OnlineEnvironmentUrlDataTrait for OnlineEnvironmentUrlData {
    fn url(&self) -> &String {
        &self.url
    }
    fn url_mut(&mut self) -> &mut String {
        &mut self.url
    }
    fn environment(&self) -> &OnlineEnvironment {
        &self.environment
    }
    fn environment_mut(&mut self) -> &mut OnlineEnvironment {
        &mut self.environment
    }
}

pub static ONLINEENVIRONMENTURLDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentUrlData",
    name_hash: 759833828,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OnlineEnvironmentUrlData as Default>::default())),
            create_boxed: || Box::new(<OnlineEnvironmentUrlData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Url",
                name_hash: 193455022,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(OnlineEnvironmentUrlData, url),
            },
            FieldInfoData {
                name: "Environment",
                name_hash: 2480382480,
                flags: MemberInfoFlags::new(0),
                field_type: "OnlineEnvironment",
                rust_offset: offset_of!(OnlineEnvironmentUrlData, environment),
            },
        ],
    }),
    array_type: Some(ONLINEENVIRONMENTURLDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OnlineEnvironmentUrlData {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEENVIRONMENTURLDATA_TYPE_INFO
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


pub static ONLINEENVIRONMENTURLDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironmentUrlData-Array",
    name_hash: 4173097168,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineEnvironmentUrlData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum OnlineEnvironment {
    #[default]
    OnlineEnvironment_Development = 0,
    OnlineEnvironment_Test = 1,
    OnlineEnvironment_Certification = 2,
    OnlineEnvironment_Production = 3,
    OnlineEnvironment_Count = 4,
}

pub static ONLINEENVIRONMENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironment",
    name_hash: 184653055,
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(ONLINEENVIRONMENT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OnlineEnvironment {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEENVIRONMENT_TYPE_INFO
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


pub static ONLINEENVIRONMENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlineEnvironment-Array",
    name_hash: 284277451,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlineEnvironment"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct OriginPresenceBackendData {
    pub _glacier_base: PresenceBackendData,
    pub commerce_categories: Vec<String>,
    pub invite_timeout: u32,
}

pub trait OriginPresenceBackendDataTrait: PresenceBackendDataTrait {
    fn commerce_categories(&self) -> &Vec<String>;
    fn commerce_categories_mut(&mut self) -> &mut Vec<String>;
    fn invite_timeout(&self) -> &u32;
    fn invite_timeout_mut(&mut self) -> &mut u32;
}

impl OriginPresenceBackendDataTrait for OriginPresenceBackendData {
    fn commerce_categories(&self) -> &Vec<String> {
        &self.commerce_categories
    }
    fn commerce_categories_mut(&mut self) -> &mut Vec<String> {
        &mut self.commerce_categories
    }
    fn invite_timeout(&self) -> &u32 {
        &self.invite_timeout
    }
    fn invite_timeout_mut(&mut self) -> &mut u32 {
        &mut self.invite_timeout
    }
}

impl PresenceBackendDataTrait for OriginPresenceBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for OriginPresenceBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for OriginPresenceBackendData {
}

pub static ORIGINPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPresenceBackendData",
    name_hash: 3701947260,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(OriginPresenceBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<OriginPresenceBackendData as Default>::default())),
            create_boxed: || Box::new(<OriginPresenceBackendData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CommerceCategories",
                name_hash: 1489746094,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(OriginPresenceBackendData, commerce_categories),
            },
            FieldInfoData {
                name: "InviteTimeout",
                name_hash: 1055902519,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(OriginPresenceBackendData, invite_timeout),
            },
        ],
    }),
    array_type: Some(ORIGINPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for OriginPresenceBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        ORIGINPRESENCEBACKENDDATA_TYPE_INFO
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


pub static ORIGINPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OriginPresenceBackendData-Array",
    name_hash: 172131144,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OriginPresenceBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlazeInProcServerBackendData {
    pub _glacier_base: PresenceBackendData,
}

pub trait BlazeInProcServerBackendDataTrait: PresenceBackendDataTrait {
}

impl BlazeInProcServerBackendDataTrait for BlazeInProcServerBackendData {
}

impl PresenceBackendDataTrait for BlazeInProcServerBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for BlazeInProcServerBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for BlazeInProcServerBackendData {
}

pub static BLAZEINPROCSERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeInProcServerBackendData",
    name_hash: 436945741,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(BlazeInProcServerBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlazeInProcServerBackendData as Default>::default())),
            create_boxed: || Box::new(<BlazeInProcServerBackendData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(BLAZEINPROCSERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlazeInProcServerBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        BLAZEINPROCSERVERBACKENDDATA_TYPE_INFO
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


pub static BLAZEINPROCSERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeInProcServerBackendData-Array",
    name_hash: 1847932793,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("BlazeInProcServerBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlazeServerBackendData {
    pub _glacier_base: PresenceBackendData,
    pub config_url: OnlineEnvironmentConsoleUrl,
    pub create_game_template_name: String,
}

pub trait BlazeServerBackendDataTrait: PresenceBackendDataTrait {
    fn config_url(&self) -> &OnlineEnvironmentConsoleUrl;
    fn config_url_mut(&mut self) -> &mut OnlineEnvironmentConsoleUrl;
    fn create_game_template_name(&self) -> &String;
    fn create_game_template_name_mut(&mut self) -> &mut String;
}

impl BlazeServerBackendDataTrait for BlazeServerBackendData {
    fn config_url(&self) -> &OnlineEnvironmentConsoleUrl {
        &self.config_url
    }
    fn config_url_mut(&mut self) -> &mut OnlineEnvironmentConsoleUrl {
        &mut self.config_url
    }
    fn create_game_template_name(&self) -> &String {
        &self.create_game_template_name
    }
    fn create_game_template_name_mut(&mut self) -> &mut String {
        &mut self.create_game_template_name
    }
}

impl PresenceBackendDataTrait for BlazeServerBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for BlazeServerBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for BlazeServerBackendData {
}

pub static BLAZESERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeServerBackendData",
    name_hash: 3235170340,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(BlazeServerBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlazeServerBackendData as Default>::default())),
            create_boxed: || Box::new(<BlazeServerBackendData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ConfigUrl",
                name_hash: 1873884036,
                flags: MemberInfoFlags::new(0),
                field_type: "OnlineEnvironmentConsoleUrl",
                rust_offset: offset_of!(BlazeServerBackendData, config_url),
            },
            FieldInfoData {
                name: "CreateGameTemplateName",
                name_hash: 2054672536,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(BlazeServerBackendData, create_game_template_name),
            },
        ],
    }),
    array_type: Some(BLAZESERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlazeServerBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        BLAZESERVERBACKENDDATA_TYPE_INFO
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


pub static BLAZESERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazeServerBackendData-Array",
    name_hash: 667450256,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("BlazeServerBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LanServerBackendData {
    pub _glacier_base: PresenceBackendData,
}

pub trait LanServerBackendDataTrait: PresenceBackendDataTrait {
}

impl LanServerBackendDataTrait for LanServerBackendData {
}

impl PresenceBackendDataTrait for LanServerBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for LanServerBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for LanServerBackendData {
}

pub static LANSERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanServerBackendData",
    name_hash: 2718772791,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(LanServerBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LanServerBackendData as Default>::default())),
            create_boxed: || Box::new(<LanServerBackendData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LANSERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LanServerBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        LANSERVERBACKENDDATA_TYPE_INFO
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


pub static LANSERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanServerBackendData-Array",
    name_hash: 98035587,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("LanServerBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LocalServerBackendData {
    pub _glacier_base: PresenceBackendData,
}

pub trait LocalServerBackendDataTrait: PresenceBackendDataTrait {
}

impl LocalServerBackendDataTrait for LocalServerBackendData {
}

impl PresenceBackendDataTrait for LocalServerBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for LocalServerBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for LocalServerBackendData {
}

pub static LOCALSERVERBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalServerBackendData",
    name_hash: 577280601,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(LocalServerBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalServerBackendData as Default>::default())),
            create_boxed: || Box::new(<LocalServerBackendData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOCALSERVERBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalServerBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALSERVERBACKENDDATA_TYPE_INFO
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


pub static LOCALSERVERBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalServerBackendData-Array",
    name_hash: 1094379629,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("LocalServerBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DurangoPresenceBackendData {
    pub _glacier_base: PresenceBackendData,
    pub title_id: u32,
    pub service_config_id: String,
    pub multiplayer_privilege_needed: bool,
}

pub trait DurangoPresenceBackendDataTrait: PresenceBackendDataTrait {
    fn title_id(&self) -> &u32;
    fn title_id_mut(&mut self) -> &mut u32;
    fn service_config_id(&self) -> &String;
    fn service_config_id_mut(&mut self) -> &mut String;
    fn multiplayer_privilege_needed(&self) -> &bool;
    fn multiplayer_privilege_needed_mut(&mut self) -> &mut bool;
}

impl DurangoPresenceBackendDataTrait for DurangoPresenceBackendData {
    fn title_id(&self) -> &u32 {
        &self.title_id
    }
    fn title_id_mut(&mut self) -> &mut u32 {
        &mut self.title_id
    }
    fn service_config_id(&self) -> &String {
        &self.service_config_id
    }
    fn service_config_id_mut(&mut self) -> &mut String {
        &mut self.service_config_id
    }
    fn multiplayer_privilege_needed(&self) -> &bool {
        &self.multiplayer_privilege_needed
    }
    fn multiplayer_privilege_needed_mut(&mut self) -> &mut bool {
        &mut self.multiplayer_privilege_needed
    }
}

impl PresenceBackendDataTrait for DurangoPresenceBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for DurangoPresenceBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for DurangoPresenceBackendData {
}

pub static DURANGOPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DurangoPresenceBackendData",
    name_hash: 1170640300,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(DurangoPresenceBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DurangoPresenceBackendData as Default>::default())),
            create_boxed: || Box::new(<DurangoPresenceBackendData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TitleId",
                name_hash: 3335995016,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DurangoPresenceBackendData, title_id),
            },
            FieldInfoData {
                name: "ServiceConfigId",
                name_hash: 3955217759,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(DurangoPresenceBackendData, service_config_id),
            },
            FieldInfoData {
                name: "MultiplayerPrivilegeNeeded",
                name_hash: 1357546411,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DurangoPresenceBackendData, multiplayer_privilege_needed),
            },
        ],
    }),
    array_type: Some(DURANGOPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DurangoPresenceBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        DURANGOPRESENCEBACKENDDATA_TYPE_INFO
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


pub static DURANGOPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DurangoPresenceBackendData-Array",
    name_hash: 769113368,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("DurangoPresenceBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Ps4PresenceBackendData {
    pub _glacier_base: PresenceBackendData,
    pub age_settings: Ps4AgeSettings,
    pub multiplayer_privilege_needed: bool,
    pub send_invite_without_first_party_dialog: bool,
    pub send_invite_without_custom_message: bool,
}

pub trait Ps4PresenceBackendDataTrait: PresenceBackendDataTrait {
    fn age_settings(&self) -> &Ps4AgeSettings;
    fn age_settings_mut(&mut self) -> &mut Ps4AgeSettings;
    fn multiplayer_privilege_needed(&self) -> &bool;
    fn multiplayer_privilege_needed_mut(&mut self) -> &mut bool;
    fn send_invite_without_first_party_dialog(&self) -> &bool;
    fn send_invite_without_first_party_dialog_mut(&mut self) -> &mut bool;
    fn send_invite_without_custom_message(&self) -> &bool;
    fn send_invite_without_custom_message_mut(&mut self) -> &mut bool;
}

impl Ps4PresenceBackendDataTrait for Ps4PresenceBackendData {
    fn age_settings(&self) -> &Ps4AgeSettings {
        &self.age_settings
    }
    fn age_settings_mut(&mut self) -> &mut Ps4AgeSettings {
        &mut self.age_settings
    }
    fn multiplayer_privilege_needed(&self) -> &bool {
        &self.multiplayer_privilege_needed
    }
    fn multiplayer_privilege_needed_mut(&mut self) -> &mut bool {
        &mut self.multiplayer_privilege_needed
    }
    fn send_invite_without_first_party_dialog(&self) -> &bool {
        &self.send_invite_without_first_party_dialog
    }
    fn send_invite_without_first_party_dialog_mut(&mut self) -> &mut bool {
        &mut self.send_invite_without_first_party_dialog
    }
    fn send_invite_without_custom_message(&self) -> &bool {
        &self.send_invite_without_custom_message
    }
    fn send_invite_without_custom_message_mut(&mut self) -> &mut bool {
        &mut self.send_invite_without_custom_message
    }
}

impl PresenceBackendDataTrait for Ps4PresenceBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for Ps4PresenceBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for Ps4PresenceBackendData {
}

pub static PS4PRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4PresenceBackendData",
    name_hash: 2658565599,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(Ps4PresenceBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Ps4PresenceBackendData as Default>::default())),
            create_boxed: || Box::new(<Ps4PresenceBackendData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "AgeSettings",
                name_hash: 1715149187,
                flags: MemberInfoFlags::new(0),
                field_type: "Ps4AgeSettings",
                rust_offset: offset_of!(Ps4PresenceBackendData, age_settings),
            },
            FieldInfoData {
                name: "MultiplayerPrivilegeNeeded",
                name_hash: 1357546411,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(Ps4PresenceBackendData, multiplayer_privilege_needed),
            },
            FieldInfoData {
                name: "SendInviteWithoutFirstPartyDialog",
                name_hash: 3993261888,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(Ps4PresenceBackendData, send_invite_without_first_party_dialog),
            },
            FieldInfoData {
                name: "SendInviteWithoutCustomMessage",
                name_hash: 2147019460,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(Ps4PresenceBackendData, send_invite_without_custom_message),
            },
        ],
    }),
    array_type: Some(PS4PRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4PresenceBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        PS4PRESENCEBACKENDDATA_TYPE_INFO
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


pub static PS4PRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4PresenceBackendData-Array",
    name_hash: 4008792811,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("Ps4PresenceBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Ps4AgeSettings {
    pub default_age_requirement: i32,
    pub age_overrides: Vec<BoxedTypeObject /* Ps4CountryAgeOverrides */>,
}

pub trait Ps4AgeSettingsTrait: TypeObject {
    fn default_age_requirement(&self) -> &i32;
    fn default_age_requirement_mut(&mut self) -> &mut i32;
    fn age_overrides(&self) -> &Vec<BoxedTypeObject /* Ps4CountryAgeOverrides */>;
    fn age_overrides_mut(&mut self) -> &mut Vec<BoxedTypeObject /* Ps4CountryAgeOverrides */>;
}

impl Ps4AgeSettingsTrait for Ps4AgeSettings {
    fn default_age_requirement(&self) -> &i32 {
        &self.default_age_requirement
    }
    fn default_age_requirement_mut(&mut self) -> &mut i32 {
        &mut self.default_age_requirement
    }
    fn age_overrides(&self) -> &Vec<BoxedTypeObject /* Ps4CountryAgeOverrides */> {
        &self.age_overrides
    }
    fn age_overrides_mut(&mut self) -> &mut Vec<BoxedTypeObject /* Ps4CountryAgeOverrides */> {
        &mut self.age_overrides
    }
}

pub static PS4AGESETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4AgeSettings",
    name_hash: 1217159732,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Ps4AgeSettings as Default>::default())),
            create_boxed: || Box::new(<Ps4AgeSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DefaultAgeRequirement",
                name_hash: 2776642642,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(Ps4AgeSettings, default_age_requirement),
            },
            FieldInfoData {
                name: "AgeOverrides",
                name_hash: 1200285313,
                flags: MemberInfoFlags::new(144),
                field_type: "Ps4CountryAgeOverrides-Array",
                rust_offset: offset_of!(Ps4AgeSettings, age_overrides),
            },
        ],
    }),
    array_type: Some(PS4AGESETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4AgeSettings {
    fn type_info(&self) -> &'static TypeInfo {
        PS4AGESETTINGS_TYPE_INFO
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


pub static PS4AGESETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4AgeSettings-Array",
    name_hash: 3034471808,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("Ps4AgeSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct Ps4CountryAgeOverrides {
    pub country_code: String,
    pub age_requirement: i32,
}

pub trait Ps4CountryAgeOverridesTrait: TypeObject {
    fn country_code(&self) -> &String;
    fn country_code_mut(&mut self) -> &mut String;
    fn age_requirement(&self) -> &i32;
    fn age_requirement_mut(&mut self) -> &mut i32;
}

impl Ps4CountryAgeOverridesTrait for Ps4CountryAgeOverrides {
    fn country_code(&self) -> &String {
        &self.country_code
    }
    fn country_code_mut(&mut self) -> &mut String {
        &mut self.country_code
    }
    fn age_requirement(&self) -> &i32 {
        &self.age_requirement
    }
    fn age_requirement_mut(&mut self) -> &mut i32 {
        &mut self.age_requirement
    }
}

pub static PS4COUNTRYAGEOVERRIDES_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4CountryAgeOverrides",
    name_hash: 1644317374,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Ps4CountryAgeOverrides as Default>::default())),
            create_boxed: || Box::new(<Ps4CountryAgeOverrides as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CountryCode",
                name_hash: 2803199296,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(Ps4CountryAgeOverrides, country_code),
            },
            FieldInfoData {
                name: "AgeRequirement",
                name_hash: 876349305,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(Ps4CountryAgeOverrides, age_requirement),
            },
        ],
    }),
    array_type: Some(PS4COUNTRYAGEOVERRIDES_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Ps4CountryAgeOverrides {
    fn type_info(&self) -> &'static TypeInfo {
        PS4COUNTRYAGEOVERRIDES_TYPE_INFO
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


pub static PS4COUNTRYAGEOVERRIDES_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Ps4CountryAgeOverrides-Array",
    name_hash: 1928887050,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("Ps4CountryAgeOverrides"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NucleusPresenceBackendData {
    pub _glacier_base: PresenceBackendData,
    pub platforms: Vec<BoxedTypeObject /* super::nucleus::NucleusPlatformConfiguration */>,
}

pub trait NucleusPresenceBackendDataTrait: PresenceBackendDataTrait {
    fn platforms(&self) -> &Vec<BoxedTypeObject /* super::nucleus::NucleusPlatformConfiguration */>;
    fn platforms_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::nucleus::NucleusPlatformConfiguration */>;
}

impl NucleusPresenceBackendDataTrait for NucleusPresenceBackendData {
    fn platforms(&self) -> &Vec<BoxedTypeObject /* super::nucleus::NucleusPlatformConfiguration */> {
        &self.platforms
    }
    fn platforms_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::nucleus::NucleusPlatformConfiguration */> {
        &mut self.platforms
    }
}

impl PresenceBackendDataTrait for NucleusPresenceBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for NucleusPresenceBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for NucleusPresenceBackendData {
}

pub static NUCLEUSPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusPresenceBackendData",
    name_hash: 285435327,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(NucleusPresenceBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NucleusPresenceBackendData as Default>::default())),
            create_boxed: || Box::new(<NucleusPresenceBackendData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Platforms",
                name_hash: 1046011945,
                flags: MemberInfoFlags::new(144),
                field_type: "NucleusPlatformConfiguration-Array",
                rust_offset: offset_of!(NucleusPresenceBackendData, platforms),
            },
        ],
    }),
    array_type: Some(NUCLEUSPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NucleusPresenceBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        NUCLEUSPRESENCEBACKENDDATA_TYPE_INFO
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


pub static NUCLEUSPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NucleusPresenceBackendData-Array",
    name_hash: 3623426827,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("NucleusPresenceBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LanPresenceBackendData {
    pub _glacier_base: PresenceBackendData,
}

pub trait LanPresenceBackendDataTrait: PresenceBackendDataTrait {
}

impl LanPresenceBackendDataTrait for LanPresenceBackendData {
}

impl PresenceBackendDataTrait for LanPresenceBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for LanPresenceBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for LanPresenceBackendData {
}

pub static LANPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanPresenceBackendData",
    name_hash: 866534603,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(LanPresenceBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LanPresenceBackendData as Default>::default())),
            create_boxed: || Box::new(<LanPresenceBackendData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LANPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LanPresenceBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        LANPRESENCEBACKENDDATA_TYPE_INFO
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


pub static LANPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LanPresenceBackendData-Array",
    name_hash: 4291290111,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("LanPresenceBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DirtySockPresenceBackendData {
    pub _glacier_base: PresenceBackendData,
}

pub trait DirtySockPresenceBackendDataTrait: PresenceBackendDataTrait {
}

impl DirtySockPresenceBackendDataTrait for DirtySockPresenceBackendData {
}

impl PresenceBackendDataTrait for DirtySockPresenceBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for DirtySockPresenceBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for DirtySockPresenceBackendData {
}

pub static DIRTYSOCKPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DirtySockPresenceBackendData",
    name_hash: 684755534,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(DirtySockPresenceBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DirtySockPresenceBackendData as Default>::default())),
            create_boxed: || Box::new(<DirtySockPresenceBackendData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DIRTYSOCKPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DirtySockPresenceBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        DIRTYSOCKPRESENCEBACKENDDATA_TYPE_INFO
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


pub static DIRTYSOCKPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DirtySockPresenceBackendData-Array",
    name_hash: 326722554,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("DirtySockPresenceBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BlazePresenceBackendData {
    pub _glacier_base: PresenceBackendData,
    pub use_demangler_service: bool,
    pub use_dirty_sock_voip: bool,
    pub fetch_licenses_on_login: bool,
}

pub trait BlazePresenceBackendDataTrait: PresenceBackendDataTrait {
    fn use_demangler_service(&self) -> &bool;
    fn use_demangler_service_mut(&mut self) -> &mut bool;
    fn use_dirty_sock_voip(&self) -> &bool;
    fn use_dirty_sock_voip_mut(&mut self) -> &mut bool;
    fn fetch_licenses_on_login(&self) -> &bool;
    fn fetch_licenses_on_login_mut(&mut self) -> &mut bool;
}

impl BlazePresenceBackendDataTrait for BlazePresenceBackendData {
    fn use_demangler_service(&self) -> &bool {
        &self.use_demangler_service
    }
    fn use_demangler_service_mut(&mut self) -> &mut bool {
        &mut self.use_demangler_service
    }
    fn use_dirty_sock_voip(&self) -> &bool {
        &self.use_dirty_sock_voip
    }
    fn use_dirty_sock_voip_mut(&mut self) -> &mut bool {
        &mut self.use_dirty_sock_voip
    }
    fn fetch_licenses_on_login(&self) -> &bool {
        &self.fetch_licenses_on_login
    }
    fn fetch_licenses_on_login_mut(&mut self) -> &mut bool {
        &mut self.fetch_licenses_on_login
    }
}

impl PresenceBackendDataTrait for BlazePresenceBackendData {
    fn backend_type(&self) -> &i32 {
        self._glacier_base.backend_type()
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.backend_type_mut()
    }
}

impl super::core::AssetTrait for BlazePresenceBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for BlazePresenceBackendData {
}

pub static BLAZEPRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazePresenceBackendData",
    name_hash: 2455870488,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PRESENCEBACKENDDATA_TYPE_INFO),
        super_class_offset: offset_of!(BlazePresenceBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BlazePresenceBackendData as Default>::default())),
            create_boxed: || Box::new(<BlazePresenceBackendData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "UseDemanglerService",
                name_hash: 1785911844,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlazePresenceBackendData, use_demangler_service),
            },
            FieldInfoData {
                name: "UseDirtySockVoip",
                name_hash: 2224476416,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlazePresenceBackendData, use_dirty_sock_voip),
            },
            FieldInfoData {
                name: "FetchLicensesOnLogin",
                name_hash: 964685523,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BlazePresenceBackendData, fetch_licenses_on_login),
            },
        ],
    }),
    array_type: Some(BLAZEPRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BlazePresenceBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        BLAZEPRESENCEBACKENDDATA_TYPE_INFO
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


pub static BLAZEPRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlazePresenceBackendData-Array",
    name_hash: 181888556,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("BlazePresenceBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PlatformFetchLicense {
    pub platform: super::core::GamePlatform,
    pub fetch_licenses_on_login: bool,
}

pub trait PlatformFetchLicenseTrait: TypeObject {
    fn platform(&self) -> &super::core::GamePlatform;
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform;
    fn fetch_licenses_on_login(&self) -> &bool;
    fn fetch_licenses_on_login_mut(&mut self) -> &mut bool;
}

impl PlatformFetchLicenseTrait for PlatformFetchLicense {
    fn platform(&self) -> &super::core::GamePlatform {
        &self.platform
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        &mut self.platform
    }
    fn fetch_licenses_on_login(&self) -> &bool {
        &self.fetch_licenses_on_login
    }
    fn fetch_licenses_on_login_mut(&mut self) -> &mut bool {
        &mut self.fetch_licenses_on_login
    }
}

pub static PLATFORMFETCHLICENSE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformFetchLicense",
    name_hash: 852492029,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlatformFetchLicense as Default>::default())),
            create_boxed: || Box::new(<PlatformFetchLicense as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Platform",
                name_hash: 942751002,
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(PlatformFetchLicense, platform),
            },
            FieldInfoData {
                name: "FetchLicensesOnLogin",
                name_hash: 964685523,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlatformFetchLicense, fetch_licenses_on_login),
            },
        ],
    }),
    array_type: Some(PLATFORMFETCHLICENSE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PlatformFetchLicense {
    fn type_info(&self) -> &'static TypeInfo {
        PLATFORMFETCHLICENSE_TYPE_INFO
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


pub static PLATFORMFETCHLICENSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlatformFetchLicense-Array",
    name_hash: 1589341129,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PlatformFetchLicense"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceBackendData {
    pub _glacier_base: super::core::Asset,
    pub backend_type: i32,
}

pub trait PresenceBackendDataTrait: super::core::AssetTrait {
    fn backend_type(&self) -> &i32;
    fn backend_type_mut(&mut self) -> &mut i32;
}

impl PresenceBackendDataTrait for PresenceBackendData {
    fn backend_type(&self) -> &i32 {
        &self.backend_type
    }
    fn backend_type_mut(&mut self) -> &mut i32 {
        &mut self.backend_type
    }
}

impl super::core::AssetTrait for PresenceBackendData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PresenceBackendData {
}

pub static PRESENCEBACKENDDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBackendData",
    name_hash: 1382808296,
    flags: MemberInfoFlags::new(101),
    module: "OnlineShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(PresenceBackendData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBackendData as Default>::default())),
            create_boxed: || Box::new(<PresenceBackendData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BackendType",
                name_hash: 385411929,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(PresenceBackendData, backend_type),
            },
        ],
    }),
    array_type: Some(PRESENCEBACKENDDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PresenceBackendData {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEBACKENDDATA_TYPE_INFO
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


pub static PRESENCEBACKENDDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBackendData-Array",
    name_hash: 44461276,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceBackendData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PresenceMode {
    #[default]
    PresenceMode_Offline = 0,
    PresenceMode_SilentOnline = 1,
    PresenceMode_Online = 2,
}

pub static PRESENCEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMode",
    name_hash: 2266879327,
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(PRESENCEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PresenceMode {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEMODE_TYPE_INFO
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


pub static PRESENCEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceMode-Array",
    name_hash: 10242667,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("PresenceMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkInviteToken {
    pub invite_platform: super::core::GamePlatform,
    pub invite_type: InviteType,
    pub join_method: InviteJoinMethod,
    pub game_id: u64,
    pub player: NetworkInviteTokenPlayer,
}

pub trait NetworkInviteTokenTrait: TypeObject {
    fn invite_platform(&self) -> &super::core::GamePlatform;
    fn invite_platform_mut(&mut self) -> &mut super::core::GamePlatform;
    fn invite_type(&self) -> &InviteType;
    fn invite_type_mut(&mut self) -> &mut InviteType;
    fn join_method(&self) -> &InviteJoinMethod;
    fn join_method_mut(&mut self) -> &mut InviteJoinMethod;
    fn game_id(&self) -> &u64;
    fn game_id_mut(&mut self) -> &mut u64;
    fn player(&self) -> &NetworkInviteTokenPlayer;
    fn player_mut(&mut self) -> &mut NetworkInviteTokenPlayer;
}

impl NetworkInviteTokenTrait for NetworkInviteToken {
    fn invite_platform(&self) -> &super::core::GamePlatform {
        &self.invite_platform
    }
    fn invite_platform_mut(&mut self) -> &mut super::core::GamePlatform {
        &mut self.invite_platform
    }
    fn invite_type(&self) -> &InviteType {
        &self.invite_type
    }
    fn invite_type_mut(&mut self) -> &mut InviteType {
        &mut self.invite_type
    }
    fn join_method(&self) -> &InviteJoinMethod {
        &self.join_method
    }
    fn join_method_mut(&mut self) -> &mut InviteJoinMethod {
        &mut self.join_method
    }
    fn game_id(&self) -> &u64 {
        &self.game_id
    }
    fn game_id_mut(&mut self) -> &mut u64 {
        &mut self.game_id
    }
    fn player(&self) -> &NetworkInviteTokenPlayer {
        &self.player
    }
    fn player_mut(&mut self) -> &mut NetworkInviteTokenPlayer {
        &mut self.player
    }
}

pub static NETWORKINVITETOKEN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkInviteToken",
    name_hash: 2485565161,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkInviteToken as Default>::default())),
            create_boxed: || Box::new(<NetworkInviteToken as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "InvitePlatform",
                name_hash: 2113690963,
                flags: MemberInfoFlags::new(0),
                field_type: "GamePlatform",
                rust_offset: offset_of!(NetworkInviteToken, invite_platform),
            },
            FieldInfoData {
                name: "InviteType",
                name_hash: 1968643572,
                flags: MemberInfoFlags::new(0),
                field_type: "InviteType",
                rust_offset: offset_of!(NetworkInviteToken, invite_type),
            },
            FieldInfoData {
                name: "JoinMethod",
                name_hash: 3350835320,
                flags: MemberInfoFlags::new(0),
                field_type: "InviteJoinMethod",
                rust_offset: offset_of!(NetworkInviteToken, join_method),
            },
            FieldInfoData {
                name: "GameId",
                name_hash: 2560094758,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(NetworkInviteToken, game_id),
            },
            FieldInfoData {
                name: "Player",
                name_hash: 3384765366,
                flags: MemberInfoFlags::new(0),
                field_type: "NetworkInviteTokenPlayer",
                rust_offset: offset_of!(NetworkInviteToken, player),
            },
        ],
    }),
    array_type: Some(NETWORKINVITETOKEN_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkInviteToken {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKINVITETOKEN_TYPE_INFO
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


pub static NETWORKINVITETOKEN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkInviteToken-Array",
    name_hash: 3546918365,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("NetworkInviteToken"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct NetworkInviteTokenPlayer {
    pub id_type: InviteIdType,
    pub id_number: u64,
}

pub trait NetworkInviteTokenPlayerTrait: TypeObject {
    fn id_type(&self) -> &InviteIdType;
    fn id_type_mut(&mut self) -> &mut InviteIdType;
    fn id_number(&self) -> &u64;
    fn id_number_mut(&mut self) -> &mut u64;
}

impl NetworkInviteTokenPlayerTrait for NetworkInviteTokenPlayer {
    fn id_type(&self) -> &InviteIdType {
        &self.id_type
    }
    fn id_type_mut(&mut self) -> &mut InviteIdType {
        &mut self.id_type
    }
    fn id_number(&self) -> &u64 {
        &self.id_number
    }
    fn id_number_mut(&mut self) -> &mut u64 {
        &mut self.id_number
    }
}

pub static NETWORKINVITETOKENPLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkInviteTokenPlayer",
    name_hash: 1127479002,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NetworkInviteTokenPlayer as Default>::default())),
            create_boxed: || Box::new(<NetworkInviteTokenPlayer as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "IdType",
                name_hash: 2795035312,
                flags: MemberInfoFlags::new(0),
                field_type: "InviteIdType",
                rust_offset: offset_of!(NetworkInviteTokenPlayer, id_type),
            },
            FieldInfoData {
                name: "IdNumber",
                name_hash: 2289878635,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(NetworkInviteTokenPlayer, id_number),
            },
        ],
    }),
    array_type: Some(NETWORKINVITETOKENPLAYER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for NetworkInviteTokenPlayer {
    fn type_info(&self) -> &'static TypeInfo {
        NETWORKINVITETOKENPLAYER_TYPE_INFO
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


pub static NETWORKINVITETOKENPLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NetworkInviteTokenPlayer-Array",
    name_hash: 3912190574,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("NetworkInviteTokenPlayer"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InviteIdStringConstants {
    #[default]
    InviteIdStringLength = 32,
}

pub static INVITEIDSTRINGCONSTANTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteIdStringConstants",
    name_hash: 3385210873,
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(INVITEIDSTRINGCONSTANTS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InviteIdStringConstants {
    fn type_info(&self) -> &'static TypeInfo {
        INVITEIDSTRINGCONSTANTS_TYPE_INFO
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


pub static INVITEIDSTRINGCONSTANTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteIdStringConstants-Array",
    name_hash: 1819850445,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("InviteIdStringConstants"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static INVITEIDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteIdType",
    name_hash: 4156440505,
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(INVITEIDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InviteIdType {
    fn type_info(&self) -> &'static TypeInfo {
        INVITEIDTYPE_TYPE_INFO
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


pub static INVITEIDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteIdType-Array",
    name_hash: 3195489805,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("InviteIdType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InviteJoinMethod {
    #[default]
    InviteJoinMethod_Unknown = 0,
    InviteJoinMethod_Invite = 1,
    InviteJoinMethod_Join = 2,
    InviteJoinMethod_Count = 3,
}

pub static INVITEJOINMETHOD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteJoinMethod",
    name_hash: 987196913,
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(INVITEJOINMETHOD_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InviteJoinMethod {
    fn type_info(&self) -> &'static TypeInfo {
        INVITEJOINMETHOD_TYPE_INFO
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


pub static INVITEJOINMETHOD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteJoinMethod-Array",
    name_hash: 748359877,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("InviteJoinMethod"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InviteType {
    #[default]
    InviteType_Invalid = 0,
    InviteType_Game = 1,
    InviteType_Player = 2,
    InviteType_Count = 3,
}

pub static INVITETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteType",
    name_hash: 1968643572,
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(INVITETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InviteType {
    fn type_info(&self) -> &'static TypeInfo {
        INVITETYPE_TYPE_INFO
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


pub static INVITETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InviteType-Array",
    name_hash: 1316153792,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("InviteType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static ONLINEPRIVILEGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePrivilege",
    name_hash: 1051368437,
    flags: MemberInfoFlags::new(49429),
    module: "OnlineShared",
    data: TypeInfoData::Enum,
    array_type: Some(ONLINEPRIVILEGE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OnlinePrivilege {
    fn type_info(&self) -> &'static TypeInfo {
        ONLINEPRIVILEGE_TYPE_INFO
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


pub static ONLINEPRIVILEGE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OnlinePrivilege-Array",
    name_hash: 2597874881,
    flags: MemberInfoFlags::new(145),
    module: "OnlineShared",
    data: TypeInfoData::Array("OnlinePrivilege"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PresencePlayTogetherRequestMessageBase {
}

pub trait PresencePlayTogetherRequestMessageBaseTrait: TypeObject {
}

impl PresencePlayTogetherRequestMessageBaseTrait for PresencePlayTogetherRequestMessageBase {
}

pub static PRESENCEPLAYTOGETHERREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePlayTogetherRequestMessageBase",
    name_hash: 3479218501,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePlayTogetherRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresencePlayTogetherRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePlayTogetherRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPLAYTOGETHERREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceGamegroupUpdatedMessageBase {
}

pub trait PresenceGamegroupUpdatedMessageBaseTrait: TypeObject {
}

impl PresenceGamegroupUpdatedMessageBaseTrait for PresenceGamegroupUpdatedMessageBase {
}

pub static PRESENCEGAMEGROUPUPDATEDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceGamegroupUpdatedMessageBase",
    name_hash: 4135850214,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceGamegroupUpdatedMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceGamegroupUpdatedMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceGamegroupUpdatedMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEGAMEGROUPUPDATEDMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceLoginLicenseRequestMessageBase {
}

pub trait PresenceLoginLicenseRequestMessageBaseTrait: TypeObject {
}

impl PresenceLoginLicenseRequestMessageBaseTrait for PresenceLoginLicenseRequestMessageBase {
}

pub static PRESENCELOGINLICENSEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLoginLicenseRequestMessageBase",
    name_hash: 2766545323,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceLoginLicenseRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceLoginLicenseRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceLoginLicenseRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCELOGINLICENSEREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceLoginLicenseMessageBase {
}

pub trait PresenceLoginLicenseMessageBaseTrait: TypeObject {
}

impl PresenceLoginLicenseMessageBaseTrait for PresenceLoginLicenseMessageBase {
}

pub static PRESENCELOGINLICENSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLoginLicenseMessageBase",
    name_hash: 4054803770,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceLoginLicenseMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceLoginLicenseMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceLoginLicenseMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCELOGINLICENSEMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceBlazeAutoAccountLoginMessage {
}

pub trait PresenceBlazeAutoAccountLoginMessageTrait: TypeObject {
}

impl PresenceBlazeAutoAccountLoginMessageTrait for PresenceBlazeAutoAccountLoginMessage {
}

pub static PRESENCEBLAZEAUTOACCOUNTLOGINMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlazeAutoAccountLoginMessage",
    name_hash: 1825111914,
    flags: MemberInfoFlags::new(73),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBlazeAutoAccountLoginMessage as Default>::default())),
            create_boxed: || Box::new(<PresenceBlazeAutoAccountLoginMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceBlazeAutoAccountLoginMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEBLAZEAUTOACCOUNTLOGINMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceJoinRemoteGameMessage {
}

pub trait PresenceJoinRemoteGameMessageTrait: TypeObject {
}

impl PresenceJoinRemoteGameMessageTrait for PresenceJoinRemoteGameMessage {
}

pub static PRESENCEJOINREMOTEGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceJoinRemoteGameMessage",
    name_hash: 4044322207,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceJoinRemoteGameMessage as Default>::default())),
            create_boxed: || Box::new(<PresenceJoinRemoteGameMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceJoinRemoteGameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEJOINREMOTEGAMEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceCommitPlayerToGameMessage {
}

pub trait PresenceCommitPlayerToGameMessageTrait: TypeObject {
}

impl PresenceCommitPlayerToGameMessageTrait for PresenceCommitPlayerToGameMessage {
}

pub static PRESENCECOMMITPLAYERTOGAMEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceCommitPlayerToGameMessage",
    name_hash: 642711648,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceCommitPlayerToGameMessage as Default>::default())),
            create_boxed: || Box::new(<PresenceCommitPlayerToGameMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for PresenceCommitPlayerToGameMessage {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCECOMMITPLAYERTOGAMEMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceUserProfileRequestMessageBase {
}

pub trait PresenceUserProfileRequestMessageBaseTrait: TypeObject {
}

impl PresenceUserProfileRequestMessageBaseTrait for PresenceUserProfileRequestMessageBase {
}

pub static PRESENCEUSERPROFILEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserProfileRequestMessageBase",
    name_hash: 2572985897,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceUserProfileRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceUserProfileRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceUserProfileRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEUSERPROFILEREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceUserProfileMessageBase {
}

pub trait PresenceUserProfileMessageBaseTrait: TypeObject {
}

impl PresenceUserProfileMessageBaseTrait for PresenceUserProfileMessageBase {
}

pub static PRESENCEUSERPROFILEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserProfileMessageBase",
    name_hash: 2922269176,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceUserProfileMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceUserProfileMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceUserProfileMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEUSERPROFILEMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceUserIdRequestMessageBase {
}

pub trait PresenceUserIdRequestMessageBaseTrait: TypeObject {
}

impl PresenceUserIdRequestMessageBaseTrait for PresenceUserIdRequestMessageBase {
}

pub static PRESENCEUSERIDREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserIdRequestMessageBase",
    name_hash: 831909967,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceUserIdRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceUserIdRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceUserIdRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEUSERIDREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceUserIdMessageBase {
}

pub trait PresenceUserIdMessageBaseTrait: TypeObject {
}

impl PresenceUserIdMessageBaseTrait for PresenceUserIdMessageBase {
}

pub static PRESENCEUSERIDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceUserIdMessageBase",
    name_hash: 2043438686,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceUserIdMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceUserIdMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceUserIdMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEUSERIDMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresencePSPlusRequestMessageBase {
}

pub trait PresencePSPlusRequestMessageBaseTrait: TypeObject {
}

impl PresencePSPlusRequestMessageBaseTrait for PresencePSPlusRequestMessageBase {
}

pub static PRESENCEPSPLUSREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePSPlusRequestMessageBase",
    name_hash: 3221536106,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePSPlusRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresencePSPlusRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePSPlusRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPSPLUSREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceProfanityFilterResponseMessageBase {
}

pub trait PresenceProfanityFilterResponseMessageBaseTrait: TypeObject {
}

impl PresenceProfanityFilterResponseMessageBaseTrait for PresenceProfanityFilterResponseMessageBase {
}

pub static PRESENCEPROFANITYFILTERRESPONSEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceProfanityFilterResponseMessageBase",
    name_hash: 2392703489,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceProfanityFilterResponseMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceProfanityFilterResponseMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceProfanityFilterResponseMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPROFANITYFILTERRESPONSEMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceProfanityFilterRequestMessageBase {
}

pub trait PresenceProfanityFilterRequestMessageBaseTrait: TypeObject {
}

impl PresenceProfanityFilterRequestMessageBaseTrait for PresenceProfanityFilterRequestMessageBase {
}

pub static PRESENCEPROFANITYFILTERREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceProfanityFilterRequestMessageBase",
    name_hash: 2163276531,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceProfanityFilterRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceProfanityFilterRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceProfanityFilterRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPROFANITYFILTERREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresencePrivilegeRequestResultMessageBase {
}

pub trait PresencePrivilegeRequestResultMessageBaseTrait: TypeObject {
}

impl PresencePrivilegeRequestResultMessageBaseTrait for PresencePrivilegeRequestResultMessageBase {
}

pub static PRESENCEPRIVILEGEREQUESTRESULTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePrivilegeRequestResultMessageBase",
    name_hash: 1577113573,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePrivilegeRequestResultMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresencePrivilegeRequestResultMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePrivilegeRequestResultMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPRIVILEGEREQUESTRESULTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresencePrivilegeRequestMessageBase {
}

pub trait PresencePrivilegeRequestMessageBaseTrait: TypeObject {
}

impl PresencePrivilegeRequestMessageBaseTrait for PresencePrivilegeRequestMessageBase {
}

pub static PRESENCEPRIVILEGEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresencePrivilegeRequestMessageBase",
    name_hash: 932396108,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresencePrivilegeRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresencePrivilegeRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresencePrivilegeRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEPRIVILEGEREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceOriginUserNameRequestMessageBase {
}

pub trait PresenceOriginUserNameRequestMessageBaseTrait: TypeObject {
}

impl PresenceOriginUserNameRequestMessageBaseTrait for PresenceOriginUserNameRequestMessageBase {
}

pub static PRESENCEORIGINUSERNAMEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOriginUserNameRequestMessageBase",
    name_hash: 1169124817,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceOriginUserNameRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceOriginUserNameRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceOriginUserNameRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEORIGINUSERNAMEREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceOriginUserNameMessageBase {
}

pub trait PresenceOriginUserNameMessageBaseTrait: TypeObject {
}

impl PresenceOriginUserNameMessageBaseTrait for PresenceOriginUserNameMessageBase {
}

pub static PRESENCEORIGINUSERNAMEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceOriginUserNameMessageBase",
    name_hash: 3938990976,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceOriginUserNameMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceOriginUserNameMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceOriginUserNameMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEORIGINUSERNAMEMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceLivePartyMessageBase {
}

pub trait PresenceLivePartyMessageBaseTrait: TypeObject {
}

impl PresenceLivePartyMessageBaseTrait for PresenceLivePartyMessageBase {
}

pub static PRESENCELIVEPARTYMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceLivePartyMessageBase",
    name_hash: 20586170,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceLivePartyMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceLivePartyMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceLivePartyMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCELIVEPARTYMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceInviteRequestMessageBase {
}

pub trait PresenceInviteRequestMessageBaseTrait: TypeObject {
}

impl PresenceInviteRequestMessageBaseTrait for PresenceInviteRequestMessageBase {
}

pub static PRESENCEINVITEREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceInviteRequestMessageBase",
    name_hash: 4201730618,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceInviteRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceInviteRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceInviteRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEINVITEREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceInviteMessageBase {
}

pub trait PresenceInviteMessageBaseTrait: TypeObject {
}

impl PresenceInviteMessageBaseTrait for PresenceInviteMessageBase {
}

pub static PRESENCEINVITEMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceInviteMessageBase",
    name_hash: 578573387,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceInviteMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceInviteMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceInviteMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEINVITEMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceFriendsListManagerSettingsMessageBase {
}

pub trait PresenceFriendsListManagerSettingsMessageBaseTrait: TypeObject {
}

impl PresenceFriendsListManagerSettingsMessageBaseTrait for PresenceFriendsListManagerSettingsMessageBase {
}

pub static PRESENCEFRIENDSLISTMANAGERSETTINGSMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFriendsListManagerSettingsMessageBase",
    name_hash: 434667703,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceFriendsListManagerSettingsMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceFriendsListManagerSettingsMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceFriendsListManagerSettingsMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEFRIENDSLISTMANAGERSETTINGSMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceFriendRequestMessageBase {
}

pub trait PresenceFriendRequestMessageBaseTrait: TypeObject {
}

impl PresenceFriendRequestMessageBaseTrait for PresenceFriendRequestMessageBase {
}

pub static PRESENCEFRIENDREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFriendRequestMessageBase",
    name_hash: 2127988193,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceFriendRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceFriendRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceFriendRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEFRIENDREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceFriendMessageBase {
}

pub trait PresenceFriendMessageBaseTrait: TypeObject {
}

impl PresenceFriendMessageBaseTrait for PresenceFriendMessageBase {
}

pub static PRESENCEFRIENDMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceFriendMessageBase",
    name_hash: 3182459440,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceFriendMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceFriendMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceFriendMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEFRIENDMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceConnectionRequestMessageBase {
}

pub trait PresenceConnectionRequestMessageBaseTrait: TypeObject {
}

impl PresenceConnectionRequestMessageBaseTrait for PresenceConnectionRequestMessageBase {
}

pub static PRESENCECONNECTIONREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceConnectionRequestMessageBase",
    name_hash: 1895039525,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceConnectionRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceConnectionRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceConnectionRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCECONNECTIONREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceConnectionMessageBase {
}

pub trait PresenceConnectionMessageBaseTrait: TypeObject {
}

impl PresenceConnectionMessageBaseTrait for PresenceConnectionMessageBase {
}

pub static PRESENCECONNECTIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceConnectionMessageBase",
    name_hash: 3964411508,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceConnectionMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceConnectionMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceConnectionMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCECONNECTIONMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceBlockListRequestMessageBase {
}

pub trait PresenceBlockListRequestMessageBaseTrait: TypeObject {
}

impl PresenceBlockListRequestMessageBaseTrait for PresenceBlockListRequestMessageBase {
}

pub static PRESENCEBLOCKLISTREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlockListRequestMessageBase",
    name_hash: 1472165784,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBlockListRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceBlockListRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlockListRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEBLOCKLISTREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceBlockListMessageBase {
}

pub trait PresenceBlockListMessageBaseTrait: TypeObject {
}

impl PresenceBlockListMessageBaseTrait for PresenceBlockListMessageBase {
}

pub static PRESENCEBLOCKLISTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlockListMessageBase",
    name_hash: 329692393,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBlockListMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceBlockListMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlockListMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEBLOCKLISTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceBlobRequestMessageBase {
}

pub trait PresenceBlobRequestMessageBaseTrait: TypeObject {
}

impl PresenceBlobRequestMessageBaseTrait for PresenceBlobRequestMessageBase {
}

pub static PRESENCEBLOBREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlobRequestMessageBase",
    name_hash: 1848750064,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBlobRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceBlobRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlobRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEBLOBREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceBlobMessageBase {
}

pub trait PresenceBlobMessageBaseTrait: TypeObject {
}

impl PresenceBlobMessageBaseTrait for PresenceBlobMessageBase {
}

pub static PRESENCEBLOBMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlobMessageBase",
    name_hash: 2065626049,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBlobMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceBlobMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlobMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEBLOBMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceBlazeUserRequestMessageBase {
}

pub trait PresenceBlazeUserRequestMessageBaseTrait: TypeObject {
}

impl PresenceBlazeUserRequestMessageBaseTrait for PresenceBlazeUserRequestMessageBase {
}

pub static PRESENCEBLAZEUSERREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlazeUserRequestMessageBase",
    name_hash: 2184223762,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBlazeUserRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceBlazeUserRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlazeUserRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEBLAZEUSERREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceBlazeUserMessageBase {
}

pub trait PresenceBlazeUserMessageBaseTrait: TypeObject {
}

impl PresenceBlazeUserMessageBaseTrait for PresenceBlazeUserMessageBase {
}

pub static PRESENCEBLAZEUSERMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceBlazeUserMessageBase",
    name_hash: 859361571,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceBlazeUserMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceBlazeUserMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceBlazeUserMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEBLAZEUSERMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceAuthenticationRequestMessageBase {
}

pub trait PresenceAuthenticationRequestMessageBaseTrait: TypeObject {
}

impl PresenceAuthenticationRequestMessageBaseTrait for PresenceAuthenticationRequestMessageBase {
}

pub static PRESENCEAUTHENTICATIONREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAuthenticationRequestMessageBase",
    name_hash: 912631955,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAuthenticationRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceAuthenticationRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAuthenticationRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEAUTHENTICATIONREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceAuthenticationMessageBase {
}

pub trait PresenceAuthenticationMessageBaseTrait: TypeObject {
}

impl PresenceAuthenticationMessageBaseTrait for PresenceAuthenticationMessageBase {
}

pub static PRESENCEAUTHENTICATIONMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAuthenticationMessageBase",
    name_hash: 3947184002,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAuthenticationMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceAuthenticationMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAuthenticationMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEAUTHENTICATIONMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceAccountRequestMessageBase {
}

pub trait PresenceAccountRequestMessageBaseTrait: TypeObject {
}

impl PresenceAccountRequestMessageBaseTrait for PresenceAccountRequestMessageBase {
}

pub static PRESENCEACCOUNTREQUESTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAccountRequestMessageBase",
    name_hash: 797682226,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAccountRequestMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceAccountRequestMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAccountRequestMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEACCOUNTREQUESTMESSAGEBASE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct PresenceAccountMessageBase {
}

pub trait PresenceAccountMessageBaseTrait: TypeObject {
}

impl PresenceAccountMessageBaseTrait for PresenceAccountMessageBase {
}

pub static PRESENCEACCOUNTMESSAGEBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PresenceAccountMessageBase",
    name_hash: 429106499,
    flags: MemberInfoFlags::new(36937),
    module: "OnlineShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PresenceAccountMessageBase as Default>::default())),
            create_boxed: || Box::new(<PresenceAccountMessageBase as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 255,
};

impl TypeObject for PresenceAccountMessageBase {
    fn type_info(&self) -> &'static TypeInfo {
        PRESENCEACCOUNTMESSAGEBASE_TYPE_INFO
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

