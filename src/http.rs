// src/http.rs

use anyhow::{Context, Result};
use esp_idf_svc::http::server::{EspHttpServer, Response};
use embedded_svc::http::Method;
use std::fs;

use crate::api::{file_api::register_file_api, terminal_api::register_terminal_api};

/// Manages the embedded HTTP server: serves static files and mounts APIs.
pub struct HttpServerManager {
    server: EspHttpServer,
    fs_mount: String,
}

impl HttpServerManager {
    /// Create a new manager from an existing EspHttpServer and SPIFFS mount point.
    pub fn new(server: EspHttpServer, fs_mount: &str) -> Result<Self> {
        Ok(HttpServerManager {
            server,
            fs_mount: fs_mount.trim_end_matches('/').to_string(),
        })
    }

    /// Serve static files from SPIFFS at `fs_mount`.
    ///  
    /// - “/” → “/spiffs/index.html”  
    /// - “/foo.js” → “/spiffs/foo.js”, etc.
    pub fn mount_static(&mut self) -> Result<()> {
        let mount_point = self.fs_mount.clone();
        // catch-all GET handler
        self.server.fn_handler("/*", Method::Get, move |req| {
            let uri = req.uri();
            // map “/” → “/index.html”
            let rel = if uri == "/" { "/index.html" } else { uri };
            let path = format!("{}{}", mount_point, rel);
            let data = fs::read(&path)
                .with_context(|| format!("Failed to read static file {}", path))?;
            // simple mime sniff
            let content_type = if path.ends_with(".html") {
                "text/html; charset=utf-8"
            } else if path.ends_with(".css") {
                "text/css"
            } else if path.ends_with(".js") {
                "application/javascript"
            } else if path.ends_with(".png") {
                "image/png"
            } else {
                "application/octet-stream"
            };
            let mut resp = Response::new(200);
            resp.set_header("Content-Type", content_type);
            resp.send_data(&data);
            Ok(resp)
        })?;
        Ok(())
    }

    /// Mount the file‐explorer API at the given URL prefix (e.g. “/api/files”).
    pub fn mount_file_api(&mut self, url: &str) -> Result<()> {
        register_file_api(&self.server, url)
    }

    /// Mount the web‐terminal API (WebSocket or HTTP) at the given URL prefix.
    pub fn mount_terminal_api(&mut self, url: &str) -> Result<()> {
        register_terminal_api(&self.server, url)
    }
}
