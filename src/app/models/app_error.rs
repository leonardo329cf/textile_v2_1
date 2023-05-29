use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppError {
    pub status: u32,
    pub message: String,
    pub timestamp: i64,
}
