use std::cmp::Ordering;

use crate::models::cut_disposition::{CutDispositionInput, Vertex, PositionedRectangle, PositionedRectangleVertices, Rectangle};

pub fn organize_pieces(cut_disposition_input: CutDispositionInput) {
    let max_length = match cut_disposition_input.defined_length {
        Some(defined_length) => defined_length,
        None => cut_disposition_input.max_length,
    };

    let mut possible_vertex_for_rectangle_list = Vec::<Vertex>::new();

    // Creates a vertex at the origin
    possible_vertex_for_rectangle_list.push(Vertex { pos_x: 0, pos_y: 0 });

    // creates vertices at the top left, bottom right and bottom left for prohibted areas
    for prohibited_area in cut_disposition_input.prohibited_area_list {
        possible_vertex_for_rectangle_list.append(&mut create_available_vertices_for_prohibited_area(prohibited_area, cut_disposition_input.spacing));
    }

    let mut rectangles_list = cut_disposition_input.rectangles_list;

    rectangles_list.sort_by(rectangle_wider_and_longer_comparator);

    for rectangle in rectangles_list {
        possible_vertex_for_rectangle_list.sort_by(vertex_closest_to_top_and_left_comparator);
    }
}

fn create_available_vertices_for_prohibited_area(
    prohibited_area: PositionedRectangle,
    spacing: Option<i32>
) -> Vec<Vertex> {
        let mut vertex_list: Vec<Vertex> = Vec::<Vertex>::new();
        vertex_list.append(&mut create_available_vertices_for_positioning(prohibited_area.clone(), spacing));
        
        // add vertex in the same column of the prohibited, in the zero vertical position, this avoid wasting the space on top of the prohibited area
        vertex_list.push(
            Vertex { 
                pos_x: prohibited_area.top_left_vertex.pos_x, 
                pos_y: 0 }
        );

        // add vertex in the same row of the prohibited, in the zero horizontal position, this avoid wasting the space on left of the prohibited area
        vertex_list.push(
            Vertex { 
                pos_x: 0, 
                pos_y: prohibited_area.top_left_vertex.pos_y }
        );

        vertex_list
}

fn create_available_vertices_for_positioning(
    positioned_rectangle: PositionedRectangle,
    spacing: Option<i32>
) -> Vec<Vertex> {
    let mut vertex_list: Vec<Vertex> = Vec::<Vertex>::new();
    
    let PositionedRectangleVertices {top_rigth_vertex, bottom_left_vertex, ..} = positioned_rectangle.get_vertices();

    match spacing {
        Some(space) => {
            vertex_list.push(
                Vertex { 
                    pos_x: top_rigth_vertex.pos_x + space, 
                    pos_y: top_rigth_vertex.pos_y,
                }
            );
            vertex_list.push(
                Vertex { 
                    pos_x: bottom_left_vertex.pos_x, 
                    pos_y: bottom_left_vertex.pos_y + space,
                }
            );
        },
        None => {
            vertex_list.push(top_rigth_vertex);
            vertex_list.push(bottom_left_vertex);
        },
    }
    vertex_list
}

pub fn rectangle_wider_and_longer_comparator(first: &Rectangle, second: &Rectangle) -> Ordering {
    if first.width < second.width || (first.width == second.width && first.length < second.length) {
        Ordering::Greater
    } else if first.width == second.width && first.length == second.length {
        Ordering::Equal
    } else {
        Ordering::Less
    }
}

pub fn vertex_closest_to_top_and_left_comparator(first: &Vertex, second: &Vertex) -> Ordering {
    if first.pos_y < second.pos_y || (first.pos_y == second.pos_y && first.pos_x < second.pos_x) {
        Ordering::Less
    } else if first.pos_y == second.pos_y && first.pos_x == second.pos_x {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /* 
    Assert that vertices are generated correctly
    for spacing = 3
        (1, 0)
            X

          (1, 3)    (6, 3)      (9, 3)
        X   | --------- |       X
     (0, 3) |     5     |
            |           |
            | 7         |
            |           |
            |           |
            |-----------|
        (1, 10)      (6, 10)

            X
        (1, 13)
    */
    #[test]
    fn create_available_vertices_for_prohibited_area_test() {
        // input
        let top_left_vertex = Vertex {
            pos_x: 1,
            pos_y: 3,
        };

        let prohibited_area: PositionedRectangle = 
        PositionedRectangle { 
            width: 5, 
            length: 7, 
            top_left_vertex,
        };

        let spacing = Some(3);

        // expect
        let vertices = vec![
            Vertex { 
                pos_x: 9, 
                pos_y: 3, 
            },
            Vertex { 
                pos_x: 1, 
                pos_y: 13, 
            },
            Vertex { 
                pos_x: 1, 
                pos_y: 0
            },
            Vertex { 
                pos_x: 0, 
                pos_y: 3
            }
        ];

        // assert
        assert_eq!(vertices, create_available_vertices_for_prohibited_area(prohibited_area, spacing));
    }


    /* 
    Assert that vertices are generated correctly
    for spacing = 3
    (1, 3)      (6, 3)      (9, 3)
        | --------- |       X
        |     5     |
        |           |
        | 7         |
        |           |
        |           |
        |-----------|
    (1, 10)      (6, 10)

        X
    (1, 13)
    */
    #[test]
    fn create_available_vertices_for_positioning_test() {
        // input
        let top_left_vertex = Vertex {
            pos_x: 1,
            pos_y: 3,
        };

        let positioned_rectangle: PositionedRectangle = 
        PositionedRectangle { 
            width: 5, 
            length: 7, 
            top_left_vertex,
        };

        let spacing = Some(3);

        // expect
        let vertices = vec![
            Vertex { 
                pos_x: 9, 
                pos_y: 3, 
            },
            Vertex { 
                pos_x: 1, 
                pos_y: 13, 
            }
        ];

        // assert
        assert_eq!(vertices, create_available_vertices_for_positioning(positioned_rectangle, spacing));
    }

    #[test]
    fn rectangle_wider_and_longer_comparator_test() {
        // input
        let mut rectangle_list_unordered = vec![
            Rectangle {
                width: 10,
                length: 2
            },
            Rectangle {
                width: 10,
                length: 5
            },
            Rectangle {
                width: 3,
                length: 2
            },
            Rectangle {
                width: 5,
                length: 10
            },
            Rectangle {
                width: 10,
                length: 2
            }
        ];

        // expect
        let rectangle_list_ordered = vec![
            Rectangle {
                width: 10,
                length: 5
            },
            Rectangle {
                width: 10,
                length: 2
            },
            Rectangle {
                width: 10,
                length: 2
            },
            Rectangle {
                width: 5,
                length: 10
            },
            Rectangle {
                width: 3,
                length: 2
            }
        ];

        // action
        rectangle_list_unordered.sort_by(rectangle_wider_and_longer_comparator);

        // assert
        assert_eq!(rectangle_list_ordered, rectangle_list_unordered);
        
    }

    /*

        (3,5)       (7,5)
        X           X
            (5,7)
            X
        (3, 9)      (7, 9)
        X           X
    */
    #[test]
    fn vertex_closest_to_top_and_left_comparator_test() {
        // input
        let mut vertex_list_unordered = vec![
            Vertex {
                pos_x: 3,
                pos_y: 5,
            },
            Vertex {
                pos_x: 5,
                pos_y: 7,
            },
            Vertex {
                pos_x: 7,
                pos_y: 9,
            },
            Vertex {
                pos_x: 3,
                pos_y: 9,
            },
            Vertex {
                pos_x: 7,
                pos_y: 5,
            }
        ];

        // expect
        let vertex_list_ordered = vec![
            Vertex {
                pos_x: 3,
                pos_y: 5,
            },
            Vertex {
                pos_x: 7,
                pos_y: 5,
            },
            Vertex {
                pos_x: 5,
                pos_y: 7,
            },
            Vertex {
                pos_x: 3,
                pos_y: 9,
            },
            Vertex {
                pos_x: 7,
                pos_y: 9,
            }
        ];

        // action
        vertex_list_unordered.sort_by(vertex_closest_to_top_and_left_comparator);

        // assert
        assert_eq!(vertex_list_ordered, vertex_list_unordered);
        
    }
}
