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
pub struct CutDispositionState {
    pub spacing: Option<i32>,
    pub max_length: i32,
    pub defined_length: Option<i32>,
    pub defined_width: i32,
    pub rectangles_list: Vec<Rectangle>,
    pub prohibited_area_list: Vec<PositionedRectangle>,
    pub showcase: Option<Rectangle>,
    pub last_id: u32
}
impl CutDispositionState {
    pub fn new() -> CutDispositionState {
        CutDispositionState {
            rectangles_list: Vec::<Rectangle>::new(),
            prohibited_area_list: Vec::<PositionedRectangle>::new(),
            showcase: None,
            last_id: 0,
            spacing: None,
            max_length: 0,
            defined_length: None,
            defined_width: 0,
        }
    }

    pub fn reset(&mut self) {
        self.rectangles_list = Vec::<Rectangle>::new();
        self.prohibited_area_list = Vec::<PositionedRectangle>::new();
        self.showcase = None;
        self.last_id = 0;
        self.spacing = None;
        self.max_length = 0;
        self.defined_length = None;
        self.defined_width = 0;
    }

    fn generate_next_id(&mut self) -> u32 {
        self.last_id += 1;
        self.last_id
    }

    pub fn get_pieces(&self) -> Vec<Rectangle> {
        self.rectangles_list.clone()
    }

    pub fn add_piece(&mut self, piece: &Rectangle) -> Rectangle {
        let piece_with_new_id = Rectangle {
            id: self.generate_next_id(),
            width: piece.width,
            length: piece.length,
        };
        self.rectangles_list.push(piece_with_new_id.clone());
        piece_with_new_id
    }

    pub fn remove_piece(&mut self, id: u32) -> Result<Rectangle, AppError> {
        if let Some(index) = self.rectangles_list.iter().position(|item| item.id == id) {
            Ok(self.rectangles_list.remove(index))
        } else {
            Err(
                AppError::new(
                    1, 
                    format!("Falha ao encontrar peça de id = {}", id).as_str()
                )
            )
        }
    }

    pub fn edit_piece(&mut self, piece: Rectangle) -> Result<Rectangle, AppError> {
        self.remove_piece(piece.id)?;
        self.rectangles_list.push(piece.clone());
        Ok(piece)
    }

    pub fn get_showcase(&self) -> Option<Rectangle> {
        self.showcase.clone()
    }

    pub fn add_showcase(&mut self, showcase: &Rectangle) -> Rectangle{
        let showcase_with_new_id = Rectangle {
            id: self.generate_next_id(),
            width: showcase.width,
            length: showcase.length,
        };
        self.rectangles_list.push(showcase_with_new_id.clone());
        showcase_with_new_id
    }

    pub fn remove_showcase(&mut self, id: u32) -> Result<Rectangle, AppError> {
        if let Some(showcase) = self.showcase.clone()  {
            if showcase.id != id {
                return Err(
                    AppError::new(
                        1, 
                        format!("Falha ao encontrar mostruário de id = {}", id).as_str()
                    )
                );
            }
            self.showcase = None;
            Ok(showcase)
        } else {
            Err(
                AppError::new(
                    1, 
                    "Não existe mostruário a ser deletado"
                )
            )
        }
    }

    pub fn edit_showcase(&mut self, showcase: Rectangle) -> Result<Rectangle, AppError> {
        self.remove_showcase(showcase.id)?;
        self.showcase = Some(showcase.clone());
        Ok(showcase)
    }

    pub fn get_prohibited_area(&self) -> Vec<PositionedRectangle> {
        self.prohibited_area_list.clone()
    }

    pub fn add_prohibited_area(&mut self, prohibited_area: &PositionedRectangle)  -> PositionedRectangle {
        let prohibited_area_with_new_id = PositionedRectangle {
            id: self.generate_next_id(),
            width: prohibited_area.width,
            length: prohibited_area.length,
            top_left_vertex: prohibited_area.top_left_vertex.clone(),
        };
        self.prohibited_area_list.push(prohibited_area_with_new_id.clone());
        prohibited_area_with_new_id
    }

    pub fn remove_prohibited_area(&mut self, id: u32) -> Result<PositionedRectangle, AppError> {
        if let Some(index) = self.prohibited_area_list.iter().position(|item| item.id == id) {
            Ok(self.prohibited_area_list.remove(index))
        } else {
            Err(
                AppError::new(
                    1, 
                    format!("Falha ao encontrar área proibida de id = {}", id).as_str()
                )
            )
        }
    }
    
    pub fn edit_prohibited_area(&mut self, prohibited_area: PositionedRectangle) -> Result<PositionedRectangle, AppError> {
        self.remove_prohibited_area(prohibited_area.id)?;
        self.prohibited_area_list.push(prohibited_area.clone());
        Ok(prohibited_area)
    }

    pub fn get_cut_disposition_input(&self) -> CutDispositionInput {
        CutDispositionInput {
            rectangles_list: self.rectangles_list.clone(),
            prohibited_area_list: self.prohibited_area_list.clone(),
            showcase: self.showcase.clone(),
            spacing: self.spacing,
            max_length: self.max_length,
            defined_length: self.defined_length,
            defined_width: self.defined_width,
        }
    }

    pub fn get_config_cut_disposition_input(&self) -> ConfigCutDispositionInput {
        ConfigCutDispositionInput {
            spacing: self.spacing,
            max_length: self.max_length,
            defined_length: self.defined_length,
            defined_width: self.defined_width,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ConfigCutDispositionInput {
    pub spacing: Option<i32>,
    pub max_length: i32,
    pub defined_length: Option<i32>,
    pub defined_width: i32,
}

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
    pub positioned_rectangles_list: Vec<PositionedRectangle>,
    pub showcase_rectangles_located_list: Vec<PositionedRectangle>,
    pub unused_rectangles_list: Vec<Rectangle>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Rectangle {
    pub id: u32,
    pub width: i32,
    pub length: i32,
}
impl Rectangle {
    pub fn equals(&self, rectangle: &Rectangle) -> bool {
        self.id == rectangle.id &&
        self.width == rectangle.width &&
        self.length == rectangle.length
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PositionedRectangle {
    pub id: u32,
    pub width: i32,
    pub length: i32,
    pub top_left_vertex: Vertex,
}

impl PositionedRectangle {
    pub fn new_from_rectangle_and_vertex(rectangle: &Rectangle, top_left_vertex: &Vertex) -> PositionedRectangle {
        PositionedRectangle {
            id: rectangle.id,
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
        self.id == positioned_rectangle.id &&
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
            id: 1,
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