use serde::{Serialize, Deserialize};

use super::cut_disposition::{Rectangle, PositionedRectangle};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RectangleType {
    Piece(Rectangle),
    Showcase(Rectangle),
    ProhibitedArea(PositionedRectangle)
}
