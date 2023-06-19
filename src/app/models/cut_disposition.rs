/*
    Coordinates system
     -----------------> X(width)
    |
    |
    |
    |
    |
    V

    Y(length)
 */

use serde::{Serialize, Deserialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CutDispositionInput {
    pub rectangles_list: Vec<Rectangle>,
    pub prohibited_area_list: Vec<PositionedRectangle>,
    pub showcase: Option<Rectangle>,
    pub spacing: Option<i32>,
    pub max_length: i32,
    pub defined_length: Option<i32>,
    pub defined_width: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ConfigCutDispositionInput {
    pub spacing: Option<i32>,
    pub max_length: i32,
    pub defined_length: Option<i32>,
    pub defined_width: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Rectangle {
    pub id: u32,
    pub width: i32,
    pub length: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PositionedRectangle {
    pub id: u32,
    pub width: i32,
    pub length: i32,
    pub top_left_vertex: Vertex,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Vertex {
    pub pos_x: i32,
    pub pos_y: i32,
}