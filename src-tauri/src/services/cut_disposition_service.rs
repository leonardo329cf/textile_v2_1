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
    for prohibited_area in &cut_disposition_input.prohibited_area_list {
        possible_vertex_for_rectangle_list.append(&mut create_available_vertices_for_prohibited_area(&prohibited_area, cut_disposition_input.spacing));
    }

    let mut rectangles_list = cut_disposition_input.rectangles_list;

    rectangles_list.sort_by(rectangle_wider_and_longer_comparator);

    let mut positioned_rectangle = Vec::<PositionedRectangle>::new();

    for rectangle in rectangles_list {
        possible_vertex_for_rectangle_list.sort_by(vertex_closest_to_top_and_left_comparator);

        let mut fitted = false;

        for vertex in &possible_vertex_for_rectangle_list {

            let subject = PositionedRectangle {
                width: rectangle.width,
                length: rectangle.length,
                top_left_vertex: vertex.clone(),
            };

            if !is_within_boundaries(&subject, cut_disposition_input.defined_width, max_length) {
                break;
            }
            let intersect: bool = 
                positioned_rectangle
                .iter()
                .any(
                    |rect| 
                    intersect(&subject, rect, cut_disposition_input.spacing.unwrap_or(0))
                )
                ||
                cut_disposition_input.prohibited_area_list
                .iter()
                .any(
                    |rect| 
                    intersect(&subject, rect, 0)
                );
            
            if intersect {
                break;
            }

            positioned_rectangle.push(subject);
        }
    }

}


fn create_available_vertices_for_prohibited_area(
    prohibited_area: &PositionedRectangle,
    spacing: Option<i32>
) -> Vec<Vertex> {
        let mut vertex_list: Vec<Vertex> = Vec::<Vertex>::new();
        vertex_list.append(&mut create_available_vertices_for_positioning(prohibited_area, spacing));
        
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
    positioned_rectangle: &PositionedRectangle,
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

pub fn is_within_boundaries(
    subject: &PositionedRectangle,
    max_width: i32,
    max_length: i32
) -> bool {
        let subject_vertices = subject.get_vertices();
         
        subject_vertices.bottom_rigth_vertex.pos_x <= max_width &&
        subject_vertices.bottom_rigth_vertex.pos_y <= max_length &&
        subject_vertices.top_left_vertex.pos_x >= 0 &&
        subject_vertices.top_left_vertex.pos_y >= 0
}

pub fn intersect(
    first: &PositionedRectangle,
    second: &PositionedRectangle,
    spacing: i32
) -> bool {
        let first_vertices = first.get_vertices();
        let second_vertices = second.get_vertices();

        // top side of the second rectangle is below bottom side of the first
        second_vertices.top_left_vertex.pos_y < first_vertices.bottom_rigth_vertex.pos_y + spacing
        &&
        // rigth side of the second rectangle is at left of the left side of the first
        second_vertices.bottom_rigth_vertex.pos_x + spacing > first_vertices.top_left_vertex.pos_x 
        &&
        // bottom side of the second rectangle is above top side of the first
        second_vertices.bottom_rigth_vertex.pos_y + spacing > first_vertices.top_left_vertex.pos_y 
        &&
        // left side of the second rectangle is at rigth side of the first
        second_vertices.top_left_vertex.pos_x < first_vertices.bottom_rigth_vertex.pos_x + spacing
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
        assert_eq!(vertices, create_available_vertices_for_prohibited_area(&prohibited_area, spacing));
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
        assert_eq!(vertices, create_available_vertices_for_positioning(&positioned_rectangle, spacing));
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

    #[test]
    fn is_within_boundaries_test() {
        // input
        let max_width = 5;
        let max_length = 10;

        let partially_outside_left = 
        PositionedRectangle {
            width: 2,
            length: 2,
            top_left_vertex: 
                Vertex { 
                    pos_x: -1, 
                    pos_y: 0 
                }
        };
        
        let partially_outside_top = 
        PositionedRectangle {
            width: 2,
            length: 2,
            top_left_vertex: 
                Vertex { 
                    pos_x: 0, 
                    pos_y: -1 
                }
        };

        let partially_outside_rigth = 
        PositionedRectangle {
            width: 2,
            length: 2,
            top_left_vertex: 
                Vertex { 
                    pos_x: 4, 
                    pos_y: 0 
                }
        };

        let partially_outside_bottom = 
        PositionedRectangle {
            width: 2,
            length: 2,
            top_left_vertex: 
                Vertex { 
                    pos_x: 0, 
                    pos_y: 9
                }
        };

        
        let partially_outside_rigth_bottom = 
        PositionedRectangle {
            width: 2,
            length: 2,
            top_left_vertex: 
                Vertex { 
                    pos_x: 4, 
                    pos_y: 9 
                }
        };

        
        let inside = 
        PositionedRectangle {
            width: 2,
            length: 2,
            top_left_vertex: 
                Vertex { 
                    pos_x: 1, 
                    pos_y: 4 
                }
        };

        let inside_left_top_corner = 
        PositionedRectangle {
            width: 2,
            length: 2,
            top_left_vertex: 
                Vertex { 
                    pos_x: 0, 
                    pos_y: 0 
                }
        };

        let inside_rigth_bottom_corner = 
        PositionedRectangle {
            width: 2,
            length: 2,
            top_left_vertex: 
                Vertex { 
                    pos_x: 3, 
                    pos_y: 8 
                }
        };

        // assert
        assert!(!is_within_boundaries(&partially_outside_left, max_width, max_length));
        assert!(!is_within_boundaries(&partially_outside_top, max_width, max_length));
        assert!(!is_within_boundaries(&partially_outside_rigth, max_width, max_length));
        assert!(!is_within_boundaries(&partially_outside_bottom, max_width, max_length));
        assert!(!is_within_boundaries(&partially_outside_rigth_bottom, max_width, max_length));

        assert!(is_within_boundaries(&inside, max_width, max_length));
        assert!(is_within_boundaries(&inside_left_top_corner, max_width, max_length));
        assert!(is_within_boundaries(&inside_rigth_bottom_corner, max_width, max_length));
    }

    #[test]
    fn intersect_no_spacing_test() {
        // input
        let spacing = 0;

        // intersect

        // sides

        let subject = 
        PositionedRectangle {
            width: 30,
            length: 50,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 50 
                }
        };

        let intersect_left = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 20, 
                    pos_y: 50
                }
        };
        
        let intersect_top = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 30 
                }
        };

        let intersect_rigth = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 50, 
                    pos_y: 50 
                }
        };

        let intersect_bottom = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 90
                }
        };

        // inside

        let intersect_inside_middle = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 35, 
                    pos_y: 60
                }
        };

        let intersect_inside_top_left = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 50
                }
        };

        let intersect_inside_bottom_rigth = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 40, 
                    pos_y: 70
                }
        };


        // no intersect

        // sides no touch

        let left = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 0, 
                    pos_y: 50
                }
        };

        let top = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 10 
                }
        };

        let rigth = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 70, 
                    pos_y: 50 
                }
        };

        let bottom = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 110
                }
        };
        

        // sides touch

        let left_touch = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 10, 
                    pos_y: 50
                }
        };
        
        let top_touch = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 20 
                }
        };

        let rigth_touch = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 60, 
                    pos_y: 50 
                }
        };

        let bottom_touch = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 100
                }
        };

        

        // assertion
        assert!(intersect(&subject, &intersect_left, spacing));
        assert!(intersect(&subject, &intersect_top, spacing));
        assert!(intersect(&subject, &intersect_rigth, spacing));
        assert!(intersect(&subject, &intersect_bottom, spacing));

        assert!(intersect(&subject, &intersect_inside_middle, spacing));
        assert!(intersect(&subject, &intersect_inside_top_left, spacing));
        assert!(intersect(&subject, &intersect_inside_bottom_rigth, spacing));
        
        assert!(!intersect(&subject, &left, spacing));
        assert!(!intersect(&subject, &top, spacing));
        assert!(!intersect(&subject, &rigth, spacing));
        assert!(!intersect(&subject, &bottom, spacing));

        assert!(!intersect(&subject, &left_touch, spacing));
        assert!(!intersect(&subject, &top_touch, spacing));
        assert!(!intersect(&subject, &rigth_touch, spacing));
        assert!(!intersect(&subject, &bottom_touch, spacing));

    }

    #[test]
    fn intersect_with_spacing_test() {
        // input
        let spacing = 5;

        // intersect

        // sides

        let subject = 
        PositionedRectangle {
            width: 30,
            length: 50,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 50 
                }
        };

        let intersect_left = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 15, 
                    pos_y: 50
                }
        };
        
        let intersect_top = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 25
                }
        };

        let intersect_rigth = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 55, 
                    pos_y: 50 
                }
        };

        let intersect_bottom = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 95
                }
        };

        // inside

        let intersect_inside_middle = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 35, 
                    pos_y: 60
                }
        };

        let intersect_inside_top_left = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 25, 
                    pos_y: 45
                }
        };

        let intersect_inside_bottom_rigth = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 45, 
                    pos_y: 75
                }
        };


        // no intersect

        // sides no touch

        let left = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 0, 
                    pos_y: 50
                }
        };

        let top = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 10 
                }
        };

        let rigth = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 70, 
                    pos_y: 50 
                }
        };

        let bottom = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 110
                }
        };
        

        // sides touch

        let left_touch = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 5, 
                    pos_y: 50
                }
        };
        
        let top_touch = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 15 
                }
        };

        let rigth_touch = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 65, 
                    pos_y: 50 
                }
        };

        let bottom_touch = 
        PositionedRectangle {
            width: 20,
            length: 30,
            top_left_vertex: 
                Vertex { 
                    pos_x: 30, 
                    pos_y: 105
                }
        };

        

        // assertion
        assert!(intersect(&subject, &intersect_left, spacing));
        assert!(intersect(&subject, &intersect_top, spacing));
        assert!(intersect(&subject, &intersect_rigth, spacing));
        assert!(intersect(&subject, &intersect_bottom, spacing));

        assert!(intersect(&subject, &intersect_inside_middle, spacing));
        assert!(intersect(&subject, &intersect_inside_top_left, spacing));
        assert!(intersect(&subject, &intersect_inside_bottom_rigth, spacing));
        
        assert!(!intersect(&subject, &left, spacing));
        assert!(!intersect(&subject, &top, spacing));
        assert!(!intersect(&subject, &rigth, spacing));
        assert!(!intersect(&subject, &bottom, spacing));

        assert!(!intersect(&subject, &left_touch, spacing));
        assert!(!intersect(&subject, &top_touch, spacing));
        assert!(!intersect(&subject, &rigth_touch, spacing));
        assert!(!intersect(&subject, &bottom_touch, spacing));

    }
}
