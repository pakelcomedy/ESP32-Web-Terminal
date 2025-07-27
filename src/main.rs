// src/main.rs

#![no_std]
#![no_main]

use esp_idf_sys as _; // link ESP‑IDF native libs
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    fs::EspFs,
    log::EspLogger,
    nvs::EspDefaultNvsPartition,
    wifi::EspWifi,
    http::server::{Configuration as HttpConfig, EspHttpServer},
};
use embedded_svc::wifi::{Configuration as WifiConfig, ClientConfiguration};
use log::*;
use anyhow::Result;
use core::panic::PanicInfo;

mod wifi;
mod spiffs;
mod http;
mod serial_bridge;
mod api;

use wifi::WifiClient;
use spiffs::SpiffsManager;
use http::HttpServerManager;
use serial_bridge::SerialBridge;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    EspLogger::println(format_args!("Panic: {:?}", info)).unwrap();
    loop {}
}

#[entry]
fn main() -> Result<()> {
    // Initialize logger
    EspLogger::initialize_default();
    info!("🚀 Starting ESP32 Web Terminal + File Explorer…");

    // System event loop
    let sysloop = EspSystemEventLoop::take()?;

    // 1. Connect Wi-Fi
    let mut wifi = WifiClient::new(sysloop.clone(), EspDefaultNvsPartition::take()?)?;
    wifi.connect("YOUR_SSID", "YOUR_PASS")?;
    let ip = wifi.get_ip_info()?;
    info!("✅ Wi-Fi connected, IP = {}", ip.ip);

    // 2. Mount SPIFFS
    let mut fs = SpiffsManager::mount("/spiffs")?;
    info!("✅ SPIFFS mounted at /spiffs");

    // 3. Start HTTP server
    let server = EspHttpServer::new(&HttpConfig::default())?;
    let mut http_mgr = HttpServerManager::new(server, "/spiffs")?;
    http_mgr.mount_static("")?;              // serve index.html, style, script
    http_mgr.mount_file_api("/api/files")?;  // list, upload, download, delete
    http_mgr.mount_terminal_api("/api/term")?; // websocket terminal
    info!("🌐 HTTP server running");

    // 4. Start serial bridge task
    let mut bridge = SerialBridge::new("/dev/uart/0", 115200)?;
    bridge.spawn("/api/term")?;
    info!("🔗 Serial bridge active");

    // 5. Serve forever
    loop {
        // nothing: HTTP & serial bridge run on IDF threads
    }
}