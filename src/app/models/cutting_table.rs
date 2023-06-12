use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CuttingTable {
    pub id: i32,
    pub name: String,
    pub width: i32,
    pub length: i32,
}
