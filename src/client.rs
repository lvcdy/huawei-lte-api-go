//! [`Client`] — the top-level aggregator of all API groups.
//!
//! Mirrors the Python library's `Client` class. It is constructed from a
//! [`Connection`] and exposes every API group as a getter that creates a
//! fresh group instance bound to that connection.

use crate::api::app::App;
use crate::api::bluetooth::Bluetooth;
use crate::api::cradle::Cradle;
use crate::api::cwmp::Cwmp;
use crate::api::d_dns::DDns;
use crate::api::developer::Developer;
use crate::api::developermode::Developermode;
use crate::api::device::Device;
use crate::api::dhcp::Dhcp;
use crate::api::diagnosis::Diagnosis;
use crate::api::dial_up::DialUp;
use crate::api::file_manager::FileManager;
use crate::api::global::Global;
use crate::api::host::Host;
use crate::api::lan::Lan;
use crate::api::language::Language;
use crate::api::led::Led;
use crate::api::log::Log;
use crate::api::m_log::MLog;
use crate::api::monitoring::Monitoring;
use crate::api::net::Net;
use crate::api::ntwk::Ntwk;
use crate::api::online_update::OnlineUpdate;
use crate::api::ota::Ota;
use crate::api::pb::Pb;
use crate::api::pin::Pin;
use crate::api::redirection::Redirection;
use crate::api::s_ntp::SNtp;
use crate::api::sd_card::SdCard;
use crate::api::security::Security;
use crate::api::sms::Sms;
use crate::api::staticroute::Staticroute;
use crate::api::statistic::Statistic;
use crate::api::syslog::Syslog;
use crate::api::system::System;
use crate::api::time::Time;
use crate::api::time_rule::TimeRule;
use crate::api::usb_printer::UsbPrinter;
use crate::api::usb_storage::UsbStorage;
use crate::api::ussd::Ussd;
use crate::api::v_sim::VSim;
use crate::api::voice::Voice;
use crate::api::vpn::Vpn;
use crate::api::web_server::WebServer;
use crate::api::wlan::WLan;
use crate::config::device::DeviceConfig;
use crate::config::device_information::DeviceInformationConfig;
use crate::config::dial_up::DialUpConfig;
use crate::config::fast_boot::FastBootConfig;
use crate::config::firewall::FirewallConfig;
use crate::config::global::GlobalConfig;
use crate::config::ipv6::Ipv6Config;
use crate::config::lan::LanConfig;
use crate::config::network::NetworkConfig;
use crate::config::ota::OtaConfig;
use crate::config::pb::PbConfig;
use crate::config::pc_assistant::PcAssistantConfig;
use crate::config::pincode::PincodeConfig;
use crate::config::sms::SmsConfig;
use crate::config::sntp::SntpConfig;
use crate::config::statistic::StatisticConfig;
use crate::config::stk::StkConfig;
use crate::config::u_pnp::UPnPConfig;
use crate::config::update::UpdateConfig;
use crate::config::ussd::UssdConfig;
use crate::config::voice::VoiceConfig;
use crate::config::web_sd::WebSdConfig;
use crate::config::web_ui_cfg::WebUICfgConfig;
use crate::config::wifi::WifiConfig;
use crate::connection::Connection;
use crate::user::User;
use crate::usermanual::public_sys_resources::PublicSysResources;

/// Top-level aggregator of all API groups.
///
/// Construct once from a [`Connection`] (or via [`Connection::login`] to log
/// in first), then call any group getter:
///
/// ```no_run
/// let conn = huawei_lte_api::Connection::new("http://192.168.8.1/", None, None)?;
/// let client = huawei_lte_api::Client::new(&conn);
/// let device = client.device().information()?;
/// # Ok::<(), huawei_lte_api::Error>(())
/// ```
pub struct Client<'a> {
    conn: &'a Connection,
}

impl<'a> Client<'a> {
    /// Build a client bound to `conn`.
    pub fn new(conn: &'a Connection) -> Self {
        Client { conn }
    }

    /// The underlying connection.
    pub fn connection(&self) -> &'a Connection {
        self.conn
    }

    pub fn monitoring(&self) -> Monitoring<'a> {
        Monitoring::new(self.conn)
    }
    pub fn security(&self) -> Security<'a> {
        Security::new(self.conn)
    }
    pub fn webserver(&self) -> WebServer<'a> {
        WebServer::new(self.conn)
    }
    pub fn global(&self) -> Global<'a> {
        Global::new(self.conn)
    }
    pub fn wlan(&self) -> WLan<'a> {
        WLan::new(self.conn)
    }
    pub fn cradle(&self) -> Cradle<'a> {
        Cradle::new(self.conn)
    }
    pub fn pin(&self) -> Pin<'a> {
        Pin::new(self.conn)
    }
    pub fn ota(&self) -> Ota<'a> {
        Ota::new(self.conn)
    }
    pub fn net(&self) -> Net<'a> {
        Net::new(self.conn)
    }
    pub fn dial_up(&self) -> DialUp<'a> {
        DialUp::new(self.conn)
    }
    pub fn sms(&self) -> Sms<'a> {
        Sms::new(self.conn)
    }
    pub fn redirection(&self) -> Redirection<'a> {
        Redirection::new(self.conn)
    }
    pub fn v_sim(&self) -> VSim<'a> {
        VSim::new(self.conn)
    }
    pub fn file_manager(&self) -> FileManager<'a> {
        FileManager::new(self.conn)
    }
    pub fn dhcp(&self) -> Dhcp<'a> {
        Dhcp::new(self.conn)
    }
    pub fn d_dns(&self) -> DDns<'a> {
        DDns::new(self.conn)
    }
    pub fn diagnosis(&self) -> Diagnosis<'a> {
        Diagnosis::new(self.conn)
    }
    pub fn s_ntp(&self) -> SNtp<'a> {
        SNtp::new(self.conn)
    }
    pub fn user(&self) -> User<'a> {
        User::new(self.conn)
    }
    pub fn device(&self) -> Device<'a> {
        Device::new(self.conn)
    }
    pub fn online_update(&self) -> OnlineUpdate<'a> {
        OnlineUpdate::new(self.conn)
    }
    pub fn log(&self) -> Log<'a> {
        Log::new(self.conn)
    }
    pub fn time(&self) -> Time<'a> {
        Time::new(self.conn)
    }
    pub fn sd_card(&self) -> SdCard<'a> {
        SdCard::new(self.conn)
    }
    pub fn usb_storage(&self) -> UsbStorage<'a> {
        UsbStorage::new(self.conn)
    }
    pub fn usb_printer(&self) -> UsbPrinter<'a> {
        UsbPrinter::new(self.conn)
    }
    pub fn vpn(&self) -> Vpn<'a> {
        Vpn::new(self.conn)
    }
    pub fn ntwk(&self) -> Ntwk<'a> {
        Ntwk::new(self.conn)
    }
    pub fn pb(&self) -> Pb<'a> {
        Pb::new(self.conn)
    }
    pub fn host(&self) -> Host<'a> {
        Host::new(self.conn)
    }
    pub fn language(&self) -> Language<'a> {
        Language::new(self.conn)
    }
    pub fn syslog(&self) -> Syslog<'a> {
        Syslog::new(self.conn)
    }
    pub fn voice(&self) -> Voice<'a> {
        Voice::new(self.conn)
    }
    pub fn cwmp(&self) -> Cwmp<'a> {
        Cwmp::new(self.conn)
    }
    pub fn lan(&self) -> Lan<'a> {
        Lan::new(self.conn)
    }
    pub fn led(&self) -> Led<'a> {
        Led::new(self.conn)
    }
    pub fn statistic(&self) -> Statistic<'a> {
        Statistic::new(self.conn)
    }
    pub fn timerule(&self) -> TimeRule<'a> {
        TimeRule::new(self.conn)
    }
    pub fn bluetooth(&self) -> Bluetooth<'a> {
        Bluetooth::new(self.conn)
    }
    pub fn mlog(&self) -> MLog<'a> {
        MLog::new(self.conn)
    }
    pub fn ussd(&self) -> Ussd<'a> {
        Ussd::new(self.conn)
    }
    pub fn staticroute(&self) -> Staticroute<'a> {
        Staticroute::new(self.conn)
    }
    pub fn system(&self) -> System<'a> {
        System::new(self.conn)
    }
    pub fn app(&self) -> App<'a> {
        App::new(self.conn)
    }
    pub fn developer(&self) -> Developer<'a> {
        Developer::new(self.conn)
    }
    pub fn developermode(&self) -> Developermode<'a> {
        Developermode::new(self.conn)
    }

    pub fn config_dialup(&self) -> DialUpConfig<'a> {
        DialUpConfig::new(self.conn)
    }
    pub fn config_global(&self) -> GlobalConfig<'a> {
        GlobalConfig::new(self.conn)
    }
    pub fn config_lan(&self) -> LanConfig<'a> {
        LanConfig::new(self.conn)
    }
    pub fn config_network(&self) -> NetworkConfig<'a> {
        NetworkConfig::new(self.conn)
    }
    pub fn config_pincode(&self) -> PincodeConfig<'a> {
        PincodeConfig::new(self.conn)
    }
    pub fn config_sms(&self) -> SmsConfig<'a> {
        SmsConfig::new(self.conn)
    }
    pub fn config_voice(&self) -> VoiceConfig<'a> {
        VoiceConfig::new(self.conn)
    }
    pub fn config_wifi(&self) -> WifiConfig<'a> {
        WifiConfig::new(self.conn)
    }
    pub fn config_pc_assistant(&self) -> PcAssistantConfig<'a> {
        PcAssistantConfig::new(self.conn)
    }
    pub fn config_device_information(&self) -> DeviceInformationConfig<'a> {
        DeviceInformationConfig::new(self.conn)
    }
    pub fn config_web_ui_cfg(&self) -> WebUICfgConfig<'a> {
        WebUICfgConfig::new(self.conn)
    }
    pub fn config_device(&self) -> DeviceConfig<'a> {
        DeviceConfig::new(self.conn)
    }
    pub fn config_fast_boot(&self) -> FastBootConfig<'a> {
        FastBootConfig::new(self.conn)
    }
    pub fn config_firewall(&self) -> FirewallConfig<'a> {
        FirewallConfig::new(self.conn)
    }
    pub fn config_ipv6(&self) -> Ipv6Config<'a> {
        Ipv6Config::new(self.conn)
    }
    pub fn config_ota(&self) -> OtaConfig<'a> {
        OtaConfig::new(self.conn)
    }
    pub fn config_pb(&self) -> PbConfig<'a> {
        PbConfig::new(self.conn)
    }
    pub fn config_sntp(&self) -> SntpConfig<'a> {
        SntpConfig::new(self.conn)
    }
    pub fn config_statistic(&self) -> StatisticConfig<'a> {
        StatisticConfig::new(self.conn)
    }
    pub fn config_stk(&self) -> StkConfig<'a> {
        StkConfig::new(self.conn)
    }
    pub fn config_update(&self) -> UpdateConfig<'a> {
        UpdateConfig::new(self.conn)
    }
    pub fn config_u_pnp(&self) -> UPnPConfig<'a> {
        UPnPConfig::new(self.conn)
    }
    pub fn config_ussd(&self) -> UssdConfig<'a> {
        UssdConfig::new(self.conn)
    }
    pub fn config_web_sd(&self) -> WebSdConfig<'a> {
        WebSdConfig::new(self.conn)
    }

    pub fn usermanual_public_sys_resources(&self) -> PublicSysResources<'a> {
        PublicSysResources::new(self.conn)
    }
}
