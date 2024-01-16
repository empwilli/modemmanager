//! # DBus interface proxy for: `org.freedesktop.ModemManager1.Modem`

use zbus::dbus_proxy;

use modemmanager_sys::{
    MMBearerIpFamily, MMModemAccessTechnology, MMModemBand, MMModemCapability, MMModemLock,
    MMModemMode, MMModemPortType, MMModemPowerState, MMModemState, MMModemStateChangeReason,
    MMModemStateFailedReason,
};

#[dbus_proxy(
    interface = "org.freedesktop.ModemManager1.Modem",
    assume_defaults = true
)]
trait Modem {
    /// Command method
    fn command(&self, cmd: &str, timeout: u32) -> zbus::Result<String>;

    /// CreateBearer method
    fn create_bearer(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// DeleteBearer method
    fn delete_bearer(&self, bearer: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// Enable method
    fn enable(&self, enable: bool) -> zbus::Result<()>;

    /// FactoryReset method
    fn factory_reset(&self, code: &str) -> zbus::Result<()>;

    /// GetCellInfo method
    fn get_cell_info(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// ListBearers method
    fn list_bearers(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Reset method
    fn reset(&self) -> zbus::Result<()>;

    /// SetCurrentBands method
    fn set_current_bands(&self, bands: &[MMModemBand]) -> zbus::Result<()>;

    /// SetCurrentCapabilities method
    fn set_current_capabilities(&self, capabilities: MMModemCapability) -> zbus::Result<()>;

    /// SetCurrentModes method
    fn set_current_modes(&self, modes: &(MMModemMode, MMModemMode)) -> zbus::Result<()>;

    /// SetPowerState method
    fn set_power_state(&self, state: MMModemPowerState) -> zbus::Result<()>;

    /// SetPrimarySimSlot method
    fn set_primary_sim_slot(&self, sim_slot: u32) -> zbus::Result<()>;

    /// StateChanged signal
    #[dbus_proxy(signal, name = "state_changed")]
    fn state_changed_func(
        &self,
        old: MMModemState,
        new: MMModemState,
        reason: MMModemStateChangeReason,
    ) -> zbus::Result<()>;

    /// AccessTechnologies property
    #[dbus_proxy(property)]
    fn access_technologies(&self) -> zbus::Result<MMModemAccessTechnology>;

    /// Bearers property
    #[dbus_proxy(property)]
    fn bearers(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// CarrierConfiguration property
    #[dbus_proxy(property)]
    fn carrier_configuration(&self) -> zbus::Result<String>;

    /// CarrierConfigurationRevision property
    #[dbus_proxy(property)]
    fn carrier_configuration_revision(&self) -> zbus::Result<String>;

    /// CurrentBands property
    #[dbus_proxy(property)]
    fn current_bands(&self) -> zbus::Result<Vec<MMModemBand>>;

    /// CurrentCapabilities property
    #[dbus_proxy(property)]
    fn current_capabilities(&self) -> zbus::Result<MMModemCapability>;

    /// CurrentModes property
    #[dbus_proxy(property)]
    fn current_modes(&self) -> zbus::Result<(MMModemMode, MMModemMode)>;

    /// Device property
    #[dbus_proxy(property)]
    fn device(&self) -> zbus::Result<String>;

    /// DeviceIdentifier property
    #[dbus_proxy(property)]
    fn device_identifier(&self) -> zbus::Result<String>;

    /// Drivers property
    #[dbus_proxy(property)]
    fn drivers(&self) -> zbus::Result<Vec<String>>;

    /// EquipmentIdentifier property
    #[dbus_proxy(property)]
    fn equipment_identifier(&self) -> zbus::Result<String>;

    /// HardwareRevision property
    #[dbus_proxy(property)]
    fn hardware_revision(&self) -> zbus::Result<String>;

    /// Manufacturer property
    #[dbus_proxy(property)]
    fn manufacturer(&self) -> zbus::Result<String>;

    /// MaxActiveBearers property
    #[dbus_proxy(property)]
    fn max_active_bearers(&self) -> zbus::Result<u32>;

    /// MaxActiveMultiplexedBearers property
    #[dbus_proxy(property)]
    fn max_active_multiplexed_bearers(&self) -> zbus::Result<u32>;

    /// MaxBearers property
    #[dbus_proxy(property)]
    fn max_bearers(&self) -> zbus::Result<u32>;

    /// Model property
    #[dbus_proxy(property)]
    fn model(&self) -> zbus::Result<String>;

    /// OwnNumbers property
    #[dbus_proxy(property)]
    fn own_numbers(&self) -> zbus::Result<Vec<String>>;

    /// Physdev property
    #[dbus_proxy(property)]
    fn physdev(&self) -> zbus::Result<String>;

    /// Plugin property
    #[dbus_proxy(property)]
    fn plugin(&self) -> zbus::Result<String>;

    /// Ports property
    #[dbus_proxy(property)]
    fn ports(&self) -> zbus::Result<Vec<(String, MMModemPortType)>>;

    /// PowerState property
    #[dbus_proxy(property)]
    fn power_state(&self) -> zbus::Result<MMModemPowerState>;

    /// PrimaryPort property
    #[dbus_proxy(property)]
    fn primary_port(&self) -> zbus::Result<String>;

    /// PrimarySimSlot property
    #[dbus_proxy(property)]
    fn primary_sim_slot(&self) -> zbus::Result<u32>;

    /// Revision property
    #[dbus_proxy(property)]
    fn revision(&self) -> zbus::Result<String>;

    /// SignalQuality property
    #[dbus_proxy(property)]
    fn signal_quality(&self) -> zbus::Result<(u32, bool)>;

    /// Sim property
    #[dbus_proxy(property)]
    fn sim(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// SimSlots property
    #[dbus_proxy(property)]
    fn sim_slots(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// State property
    #[dbus_proxy(property)]
    fn state(&self) -> zbus::Result<MMModemState>;

    /// StateFailedReason property
    #[dbus_proxy(property)]
    fn state_failed_reason(&self) -> zbus::Result<MMModemStateFailedReason>;

    /// SupportedBands property
    #[dbus_proxy(property)]
    fn supported_bands(&self) -> zbus::Result<Vec<MMModemBand>>;

    /// SupportedCapabilities property
    #[dbus_proxy(property)]
    fn supported_capabilities(&self) -> zbus::Result<Vec<MMModemCapability>>;

    /// SupportedIpFamilies property
    #[dbus_proxy(property)]
    fn supported_ip_families(&self) -> zbus::Result<MMBearerIpFamily>;

    /// SupportedModes property
    #[dbus_proxy(property)]
    fn supported_modes(&self) -> zbus::Result<Vec<(MMModemMode, MMModemMode)>>;

    /// UnlockRequired property
    #[dbus_proxy(property)]
    fn unlock_required(&self) -> zbus::Result<MMModemLock>;

    /// UnlockRetries property
    #[dbus_proxy(property)]
    fn unlock_retries(&self) -> zbus::Result<std::collections::HashMap<u32, u32>>;
}
