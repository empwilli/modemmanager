//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem.Modem3gpp`

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.ModemManager1.Modem.Modem3gpp",
    assume_defaults = true
)]
trait Modem3gpp {
    /// DisableFacilityLock method
    fn disable_facility_lock(&self, properties: &(u32, &str)) -> zbus::Result<()>;

    /// Register method
    fn register(&self, operator_id: &str) -> zbus::Result<()>;

    /// Scan method
    fn scan(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// SetCarrierLock method
    fn set_carrier_lock(&self, data: &[u8]) -> zbus::Result<()>;

    /// SetEpsUeModeOperation method
    fn set_eps_ue_mode_operation(&self, mode: u32) -> zbus::Result<()>;

    /// SetInitialEpsBearerSettings method
    fn set_initial_eps_bearer_settings(
        &self,
        settings: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetNr5gRegistrationSettings method
    fn set_nr5g_registration_settings(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetPacketServiceState method
    fn set_packet_service_state(&self, state: u32) -> zbus::Result<()>;

    /// EnabledFacilityLocks property
    #[dbus_proxy(property)]
    fn enabled_facility_locks(&self) -> zbus::Result<u32>;

    /// EpsUeModeOperation property
    #[dbus_proxy(property)]
    fn eps_ue_mode_operation(&self) -> zbus::Result<u32>;

    /// Imei property
    #[dbus_proxy(property)]
    fn imei(&self) -> zbus::Result<String>;

    /// InitialEpsBearer property
    #[dbus_proxy(property)]
    fn initial_eps_bearer(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// InitialEpsBearerSettings property
    #[dbus_proxy(property)]
    fn initial_eps_bearer_settings(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Nr5gRegistrationSettings property
    #[dbus_proxy(property)]
    fn nr5g_registration_settings(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// OperatorCode property
    #[dbus_proxy(property)]
    fn operator_code(&self) -> zbus::Result<String>;

    /// OperatorName property
    #[dbus_proxy(property)]
    fn operator_name(&self) -> zbus::Result<String>;

    /// PacketServiceState property
    #[dbus_proxy(property)]
    fn packet_service_state(&self) -> zbus::Result<u32>;

    /// Pco property
    #[dbus_proxy(property)]
    fn pco(&self) -> zbus::Result<Vec<(u32, bool, Vec<u8>)>>;

    /// RegistrationState property
    #[dbus_proxy(property)]
    fn registration_state(&self) -> zbus::Result<u32>;

    /// SubscriptionState property
    #[dbus_proxy(property)]
    fn subscription_state(&self) -> zbus::Result<u32>;
}
