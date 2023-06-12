use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FabricCutPiece {
    pub id: i32,
    pub width: i32,
    pub length: i32,
    pub pos_x: i32,
    pub pos_y: i32,
    pub status: i32,
}
