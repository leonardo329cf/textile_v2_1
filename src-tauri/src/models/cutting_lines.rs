use serde::{Deserialize, Serialize};

use super::cut_disposition::Vertex;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub start: Vertex,
    pub end: Vertex
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.start.pos_x == self.end.pos_x
    }

    pub fn is_same_level(&self, line: &Line) -> bool {
        if self.is_vertical() == line.is_vertical() {
            if self.is_vertical() {
                return self.start.pos_x == line.start.pos_x;
            } else {
                return self.start.pos_y == line.start.pos_y;
            }
        }
        false
    }

    pub fn cross_perpend(&self, line: &Line) -> bool {
        (
                self.start.pos_y < line.start.pos_y && self.end.pos_y > line.start.pos_y && line.start.pos_x < self.start.pos_x && line.end.pos_x > self.start.pos_x
        ) || (
                self.start.pos_y > line.start.pos_y && self.start.pos_y < line.end.pos_y && line.start.pos_x > self.start.pos_x && line.start.pos_x < self.end.pos_x
        )
    }

    pub fn combine(&self, line: &Line) -> Line {
        let mut start = self.start.clone();
        let mut end = line.end.clone();

        if self.is_vertical() {
            if start.pos_y > line.start.pos_y {
                start = line.start.clone();
            }
            if end.pos_y < line.end.pos_y {
                end = line.end.clone();
            }
        } else {
            if start.pos_x > line.start.pos_x {
                start = line.start.clone();
            }
            if end.pos_x < line.end.pos_x {
                end = line.end.clone();
            }
        }

        Line {
            start,
            end
        }
    }
}

pub struct CuttingLines {
    pub vertical_lines: Vec<Line>,
    pub horizontal_lines: Vec<Line>
}