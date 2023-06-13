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
pub struct CutDispositionOutput {
    pub vertical_line_list: Vec<Line>,
    pub horizontal_line_list: Vec<Line>,
    pub positioned_rectangles_list: Vec<PositionedRectangle>,
    pub showcase_rectangles_located_list: Vec<PositionedRectangle>,
    pub unused_rectangles_list: Vec<PositionedRectangle>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Rectangle {
    pub width: i32,
    pub length: i32,
}
impl Rectangle {
    pub fn equals(&self, rectangle: &Rectangle) -> bool {
        self.width == rectangle.width &&
        self.length == rectangle.length
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PositionedRectangle {
    pub width: i32,
    pub length: i32,
    pub top_left_vertex: Vertex,
}

impl PositionedRectangle {
    pub fn new_from_rectangle_and_vertex(rectangle: &Rectangle, top_left_vertex: &Vertex) -> PositionedRectangle {
        PositionedRectangle {
            width: rectangle.width,
            length: rectangle.length,
            top_left_vertex: top_left_vertex.clone(),
        }
    }

    pub fn get_vertices(&self) -> PositionedRectangleVertices {
        PositionedRectangleVertices {
            top_left_vertex: self.top_left_vertex.clone(),
            top_rigth_vertex: Vertex { 
                pos_x: self.top_left_vertex.pos_x + self.width, 
                pos_y: self.top_left_vertex.pos_y, 
            },
            bottom_left_vertex: Vertex { 
                pos_x: self.top_left_vertex.pos_x, 
                pos_y: self.top_left_vertex.pos_y + self.length, 
            },
            bottom_rigth_vertex: Vertex { 
                pos_x: self.top_left_vertex.pos_x + self.width,
                pos_y: self.top_left_vertex.pos_y + self.length, 
            },
        }
    }

    pub fn equals(&self, positioned_rectangle: &PositionedRectangle) -> bool {
        self.width == positioned_rectangle.width &&
        self.length == positioned_rectangle.length &&
        self.top_left_vertex == positioned_rectangle.top_left_vertex
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PositionedRectangleVertices {
    pub top_left_vertex: Vertex,
    pub top_rigth_vertex: Vertex,
    pub bottom_left_vertex: Vertex,
    pub bottom_rigth_vertex: Vertex,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub first_vertex: Vertex,
    pub last_vertex: Vertex,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Vertex {
    pub pos_x: i32,
    pub pos_y: i32,
}

impl Vertex {
    pub fn equals(&self, vertex: &Vertex) -> bool {
        self.pos_x == vertex.pos_x &&
        self.pos_y == vertex.pos_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* 
    Assert that vertices are generated correctly
    for (x, y)
    top-left    top-right
    (1, 3)      (6, 3)
        | --------- |
        |     5     |
        |           |
        | 7         |
        |           |
        |           |
        |-----------|
    (1, 10)      (6, 10)
    bottom-left  bottom-right
    */
    #[test]
    fn get_vertices_test() {
        
        // input
        let top_left_vertex = Vertex {
            pos_x: 1,
            pos_y: 3,
        };

        let rectangle = 
        PositionedRectangle { 
            width: 5, 
            length: 7, 
            top_left_vertex: top_left_vertex.clone(),
        };

        // expect
        let vertices = PositionedRectangleVertices {
            top_left_vertex,
            top_rigth_vertex: Vertex { 
                pos_x: 6, 
                pos_y: 3, 
            },
            bottom_left_vertex: Vertex { 
                pos_x: 1, 
                pos_y: 10, 
            },
            bottom_rigth_vertex:  Vertex { 
                pos_x: 6, 
                pos_y: 10, 
            },
        };

        // assert
        assert_eq!(vertices, rectangle.get_vertices());
    }
}