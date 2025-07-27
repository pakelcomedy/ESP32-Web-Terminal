// src/wifi.rs

use anyhow::Result;
use embedded_svc::wifi::{ClientConfiguration, Configuration as WifiConfiguration, Wifi};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::EspWifi,
    netif::EspNetifIpInfo,
};
use log::info;

/// Simple Wi‑Fi client for ESP32
pub struct WifiClient {
    wifi: EspWifi,
}

impl WifiClient {
    /// Create a new WifiClient.
    /// 
    /// # Arguments
    /// * `sysloop` – the ESP‑IDF event loop
    /// * `nvs` – NVS partition for storing Wi‑Fi credentials
    pub fn new(
        sysloop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
    ) -> Result<Self> {
        let wifi = EspWifi::new_default(sysloop, Some(nvs))?;
        Ok(WifiClient { wifi })
    }

    /// Connect to an AP, blocking until an IP is obtained.
    pub fn connect(&mut self, ssid: &str, password: &str) -> Result<()> {
        info!("🔌 Configuring Wi‑Fi SSID “{}” …", ssid);
        let client_conf = ClientConfiguration {
            ssid: ssid.into(),
            password: password.into(),
            ..Default::default()
        };
        self.wifi.set_configuration(&WifiConfiguration::Client(client_conf))?;
        self.wifi.start()?;
        self.wifi.connect()?;

        info!("⏳ Waiting for Wi‑Fi connection...");
        while !self.wifi.is_connected()? {
            // spin until connected
        }
        info!("✅ Connected to Wi‑Fi");
        Ok(())
    }

    /// Get the acquired IP information (IP, gateway, netmask).
    pub fn get_ip_info(&self) -> Result<EspNetifIpInfo<'_>> {
        let ip_info = self.wifi.sta_netif().get_ip_info()?;
        info!(
            "📶 IP acquired: {}  gateway: {}  netmask: {}",
            ip_info.ip, ip_info.gateway, ip_info.netmask
        );
        Ok(ip_info)
    }
}
