use std::time::{SystemTime, UNIX_EPOCH};

pub use my_module::*;

pub mod doc;
pub mod error;
pub mod filter;
pub mod mongo;
mod my_module {
    pub use mongodb;
}

/// 获取当前时间戳
fn timestamp() -> u64 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let timestamp = duration.as_secs() * 1000 + u64::from(duration.subsec_nanos()) / 1_000_000;
    timestamp
}
