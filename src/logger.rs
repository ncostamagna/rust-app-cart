// src/logger.rs
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};

pub fn init() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
}