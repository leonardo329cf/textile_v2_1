use serde::{Deserialize, Serialize};

pub const DEFAULT_ERROR_CODE: u32 = 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppError {
    pub status: u32,
    pub message: String,
    pub timestamp: i64,
}

impl AppError {
    pub fn new(status: u32, message: &str) -> Self {
        Self {
            status,
            message: message.to_owned(),
            timestamp: chrono::offset::Utc::now().timestamp_millis(),
        }
    }
}
