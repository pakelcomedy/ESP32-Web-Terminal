// src/serial_bridge.rs

use anyhow::{Context, Result};
use esp_idf_svc::http::server::{EspHttpServer, Upgrade};
use esp_idf_svc::uart::{UartDriver, UartConfig, UartDataBits, UartFlowControl, UartParity, UartStopBits};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use embedded_svc::http::Method;
use log::*;
use std::sync::{Arc, Mutex};
use std::thread;

/// Bridges a WebSocket “terminal” endpoint to the ESP32’s UART.
///
/// - Reads lines from UART and broadcasts to all connected WS clients.
/// - Forwards text from WS clients into UART.
pub struct SerialBridge {
    uart: Arc<Mutex<UartDriver<'static>>>,
}

impl SerialBridge {
    /// Open the given UART port (e.g. "/dev/uart/0") at `baud` bps.
    pub fn new(port: &str, baud: u32) -> Result<Self> {
        let config = UartConfig {
            baudrate: baud,
            data_bits: UartDataBits::DataBits8,
            parity: UartParity::None,
            stop_bits: UartStopBits::Stop1,
            flow_control: UartFlowControl::None,
        };
        let uart = UartDriver::new(port, &config, &EspSystemEventLoop::take()?)
            .context("Failed to initialize UART")?;
        Ok(SerialBridge { uart: Arc::new(Mutex::new(uart)) })
    }

    /// Register a WebSocket handler at `path` (e.g. "/api/term") on the given HTTP server.
    pub fn spawn(self, server: &EspHttpServer, path: &str) -> Result<()> {
        let uart_tx = self.uart.clone();
        let uart_rx = self.uart;

        // KEEP track of connected sockets
        let clients = Arc::new(Mutex::new(Vec::new()));

        // 1) WebSocket upgrade endpoint
        server.fn_handler(path, Method::Get, move |request| {
            let mut upgrade = request.upgrade()?;
            let ws = upgrade.wait().context("WS upgrade failed")?;
            let mut socket = ws.accept().context("WS accept failed")?;

            info!("🖥️ New terminal client connected");
            let tx_clone = clients.clone();
            {
                let mut guard = tx_clone.lock().unwrap();
                guard.push(socket.clone());
            }

            // Spawn a task to forward UART -> this socket
            let uart_rx_clone = uart_rx.clone();
            let mut sock_clone = socket.clone();
            thread::spawn(move || {
                let mut buf = [0u8; 128];
                loop {
                    let n = uart_rx_clone.lock().unwrap().read(&mut buf).unwrap_or(0);
                    if n > 0 {
                        let slice = &buf[..n];
                        if sock_clone.send(slice).is_err() {
                            break;
                        }
                    }
                    thread::sleep(std::time::Duration::from_millis(10));
                }
                info!("🖥️ Terminal client disconnected");
            });

            // Spawn a task to forward this socket -> UART
            let uart_tx_clone = uart_tx.clone();
            thread::spawn(move || {
                while let Ok(msg) = socket.receive() {
                    if socket.is_closed() { break; }
                    let data = msg.into_data();
                    uart_tx_clone.lock().unwrap().write(&data).ok();
                }
            });

            Ok(esp_idf_svc::http::server::Response::empty(101))
        })?;

        // 2) Optionally: broadcast any UART output to all clients
        let clients_broad = clients.clone();
        thread::spawn(move || {
            let mut buf = [0u8; 128];
            loop {
                let n = uart_tx.lock().unwrap().read(&mut buf).unwrap_or(0);
                if n > 0 {
                    let slice = &buf[..n];
                    let mut guard = clients_broad.lock().unwrap();
                    guard.retain(|sock| sock.send(slice).is_ok());
                }
                thread::sleep(std::time::Duration::from_millis(50));
            }
        });

        Ok(())
    }
}
