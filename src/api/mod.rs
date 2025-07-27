// src/api/mod.rs

pub mod file_api;
pub mod terminal_api;

pub use file_api::register_file_api;
pub use terminal_api::register_terminal_api;
