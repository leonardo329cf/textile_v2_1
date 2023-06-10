use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Fabric {
    pub id: i32,
    pub name: String,
    pub manufacturer: String,
    pub width: i32,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FabricCreate {
    pub name: String,
    pub manufacturer: String,
    pub width: i32,
    pub code: String,
}