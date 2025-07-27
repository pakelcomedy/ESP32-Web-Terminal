// src/spiffs.rs

use anyhow::{Context, Result};
use esp_idf_svc::fs::{EspFs, EspFsOptions, MountedFilesystem};

/// Manages mounting and unmounting SPIFFS on ESP32.
pub struct SpiffsManager {
    _fs: MountedFilesystem,
}

impl SpiffsManager {
    /// Mounts SPIFFS at the given `mount_point` (e.g. "/spiffs").
    pub fn mount(mount_point: &str) -> Result<Self> {
        let opts = EspFsOptions {
            partition: "spiffs",
            mount_point,
            ..Default::default()
        };
        let fs = EspFs::new(opts)
            .with_context(|| format!("Failed to mount SPIFFS at {}", mount_point))?;
        Ok(SpiffsManager { _fs: fs })
    }
}
