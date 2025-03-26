pub mod config;
pub mod models;
pub mod orderbook;
mod parsing;
pub mod primitives;

// Initialize logger
pub fn init_logger() {
    env_logger::Builder::from_default_env().format_timestamp(None).init();
}
