use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FabricCutPiece {
    pub id: i32,
    pub width: i32,
    pub length: i32,
    pub status: PieceStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PieceStatus {
    Fit {
        position: Vertex
    },
    DidNotFit,
    ProhibitedArea {
        position: Vertex
    },
    Showcase {
        position_list: Vec<Vertex>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Vertex {
    pub pos_x: i32,
    pub pos_y: i32
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PositionedRectangle {
    pub id: i32,
    pub width: i32,
    pub length: i32,
    pub pos_x: i32,
    pub pos_y: i32
}

pub fn get_optional_pos_x(piece: &FabricCutPiece) -> Option<i32>{
    match &piece.status {
        PieceStatus::Fit { position } => Some(position.pos_x),
        PieceStatus::DidNotFit => None,
        PieceStatus::ProhibitedArea { position } => Some(position.pos_x),
        PieceStatus::Showcase { position_list } => None,
    }
}

pub fn get_optional_pos_y(piece: &FabricCutPiece) -> Option<i32>{
    match &piece.status {
        PieceStatus::Fit { position } => Some(position.pos_y),
        PieceStatus::DidNotFit => None,
        PieceStatus::ProhibitedArea { position } => Some(position.pos_y),
        PieceStatus::Showcase { position_list } => None,
    }
}