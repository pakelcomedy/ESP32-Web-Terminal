// src/api/file_api.rs

use anyhow::{Context, Result};
use esp_idf_svc::http::server::{EspHttpServer, Response};
use embedded_svc::http::{Method, server::HandlerResult};
use serde_json::json;
use std::{
    fs::{read_dir, remove_file, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

/// Mount this at e.g. `register_file_api(&server, "/api/files")`
pub fn register_file_api(server: &EspHttpServer, base_url: &str) -> Result<()> {
    let base = base_url.trim_end_matches('/').to_string();
    let fs_root = "/spiffs"; // adjust to your mount point

    // LIST FILES: GET {base}/
    server.fn_handler(&base, Method::Get, move |_req| {
        let mut files = Vec::new();
        for entry in read_dir(fs_root).context("Failed to read SPIFFS directory")? {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                files.push(name.to_string());
            }
        }
        let body = json!({ "files": files }).to_string();
        let mut resp = Response::new(200);
        resp.set_header("Content-Type", "application/json");
        resp.send_str(&body);
        Ok(resp)
    })?;

    // DOWNLOAD FILE: GET {base}/{filename}
    server.fn_handler(&format!("{}/<file>", base), Method::Get, move |req| {
        let file = req.param("file").unwrap_or("");
        let path = format!("{}/{}", fs_root, file);
        let mut f = File::open(&path)
            .with_context(|| format!("Failed to open file {}", path))?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)
            .context("Failed to read file")?;
        let mut resp = Response::new(200);
        resp.set_header("Content-Type", "application/octet-stream");
        resp.set_header("Content-Disposition", &format!("attachment; filename=\"{}\"", file));
        resp.send_data(&buf);
        Ok(resp)
    })?;

    // UPLOAD FILE: POST {base}/upload
    server.fn_handler(&format!("{}/upload", base), Method::Post, move |req| {
        let fname = req
            .header("X-Filename")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("upload.bin");
        let path = format!("{}/{}", fs_root, fname);
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .with_context(|| format!("Failed to create file {}", path))?;
        let mut body = Vec::new();
        req.into_reader().read_to_end(&mut body).context("Read request body")?;
        f.write_all(&body).context("Write to file")?;
        let mut resp = Response::new(201);
        resp.send_str("OK");
        Ok(resp)
    })?;

    // DELETE FILE: DELETE {base}/{filename}
    server.fn_handler(&format!("{}/<file>", base), Method::Delete, move |req| {
        let file = req.param("file").unwrap_or("");
        let path = format!("{}/{}", fs_root, file);
        remove_file(&path).with_context(|| format!("Failed to delete {}", path))?;
        let mut resp = Response::new(200);
        resp.send_str("Deleted");
        Ok(resp)
    })?;

    Ok(())
}
