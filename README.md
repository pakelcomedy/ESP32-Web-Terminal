```
esp32-web-terminal/
├── Cargo.toml
├── memory.x
├── .cargo/
│   └── config.toml
├── src/
│   ├── main.rs             # Entry point firmware ESP32
│   ├── wifi.rs             # Wi-Fi connection logic
│   ├── http.rs             # Embedded HTTP server (serve static + API)
│   ├── spiffs.rs           # SPIFFS init, read/write file API
│   ├── serial_bridge.rs    # Handle serial terminal (stdin/stdout passthrough)
│   └── api/
│       ├── mod.rs
│       ├── file_api.rs     # Upload, download, delete, list SPIFFS
│       └── terminal_api.rs # Handle WebSocket or HTTP endpoint for terminal
└── website/                # Frontend folder (served by ESP32)
    ├── index.html          # Main HTML interface: terminal + file explorer
    ├── style.css           # Optional styles
    └── script.js           # WebSocket terminal + file APIs
```
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/68ed8230-f7aa-4ae1-9feb-43cfd4be1601" />
