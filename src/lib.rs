use std::{
    time::{SystemTime, UNIX_EPOCH},
};

pub mod doc;
pub mod error;
pub mod mongo;
pub mod filter;
mod my_module {
    pub use mongodb;
}

pub use my_module::*;
fn timestamp() -> u64 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let timestamp = duration.as_secs() * 1000 + u64::from(duration.subsec_nanos()) / 1_000_000;
    timestamp
}

fn camel2snake<'a>(camel: &str) -> String {

    let mut output = String::new();

    for (i, c) in camel.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                output.push('_');
            }
            output.push(c.to_ascii_lowercase());
        } else {
            output.push(c);
        }
    }
    output
}

