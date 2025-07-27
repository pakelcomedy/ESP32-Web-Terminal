// src/api/terminal_api.rs

use anyhow::Result;
use esp_idf_svc::http::server::EspHttpServer;

use crate::serial_bridge::SerialBridge;

/// Mounts the web‐terminal API at `url` (e.g. "/api/term").
/// Internally creates a SerialBridge (UART → WebSocket) on the default UART port.
pub fn register_terminal_api(server: &EspHttpServer, url: &str) -> Result<()> {
    // Initialize the serial bridge on UART0 ("/dev/uart/0") at 115200 bps
    let bridge = SerialBridge::new("/dev/uart/0", 115200)?;
    // Register and spawn the WebSocket terminal endpoint
    bridge.spawn(server, url)?;
    Ok(())
}
