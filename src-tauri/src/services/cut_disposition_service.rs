use std::cmp::Ordering;

use crate::models::cut_disposition::{CutDispositionInput, Vertex, PositionedRectangle, PositionedRectangleVertices, Rectangle, CutDispositionOutput, self};


fn organized_disposition(cut_disposition_input: &CutDispositionInput) {
    let max_length = match cut_disposition_input.defined_length {
        Some(defined_length) => defined_length,
        None => cut_disposition_input.max_length,
    };

    let spacing = cut_disposition_input.spacing.unwrap_or(0);

    let max_width = cut_disposition_input.defined_width;

    let rectangles_list = cut_disposition_input.rectangles_list.clone();

    let prohibited_area_list = cut_disposition_input.prohibited_area_list.clone();


    let MainRectangleOrganized { 
        mut possible_vertex_for_rectangle_list, 
        unused_rectangles_list, 
        positioned_rectangles_list, 
        length_used 
    } = organize_main_rectangles( 
        max_length, 
        spacing, 
        max_width, 
        &rectangles_list, 
        &prohibited_area_list
    );

    let mut positioned_showcase_list = Vec::<PositionedRectangle>::new();
    
    if let Some(showcase) = &cut_disposition_input.showcase {
        positioned_showcase_list.append(
            &mut organize_showcase_rectangles(
                showcase.clone(), 
                spacing, 
                max_width, 
                &prohibited_area_list, 
                &possible_vertex_for_rectangle_list, 
                &positioned_rectangles_list,
                length_used
            )
        )
    }

    
}

struct MainRectangleOrganized{
    pub possible_vertex_for_rectangle_list : Vec<Vertex>,
    pub unused_rectangles_list: Vec<Rectangle>,
    pub positioned_rectangles_list: Vec<PositionedRectangle>,
    pub length_used: i32,
}

fn organize_main_rectangles(
    max_length: i32, 
    spacing: i32, 
    max_width: i32, 
    rectangles_list: &[Rectangle], 
    prohibited_area_list: &[PositionedRectangle]
) -> MainRectangleOrganized {
    let mut possible_vertex_for_rectangle_list = Vec::<Vertex>::new();

    let mut unused_rectangles_list = Vec::<Rectangle>::new();

    let mut positioned_rectangles_list = Vec::<PositionedRectangle>::new();

    // Creates a vertex at the origin
    possible_vertex_for_rectangle_list.push(Vertex { pos_x: 0, pos_y: 0 });

    // creates vertices at the top left, bottom right and bottom left for prohibted areas
        
    possible_vertex_for_rectangle_list.append(
        &mut prohibited_area_list.iter().fold(
            Vec::<Vertex>::new(),
            |mut list, prohibited_area| {
                list.append(
                    &mut create_available_vertices_for_prohibited_area(prohibited_area)
                );
                list
            }
        )
    );

    let mut rectangles_list_sorted = Vec::from(rectangles_list);

    rectangles_list_sorted.sort_by(rectangle_wider_and_longer_comparator);
    
    for rectangle in rectangles_list_sorted {
        possible_vertex_for_rectangle_list.sort_by(vertex_closest_to_top_and_left_comparator);

        let positioned_rectangle_option = find_position_for_rectangle(
            &rectangle, 
            max_width, 
            max_length, 
            spacing, 
            &positioned_rectangles_list, 
            &possible_vertex_for_rectangle_list, 
            prohibited_area_list
        );

        match positioned_rectangle_option {
            Some(positioned_rectangle) => {
                positioned_rectangles_list.push(positioned_rectangle.clone());
                if let Some(used_vertex_index) = get_vertex_index_in_list(
                    &positioned_rectangle.top_left_vertex,
                    &possible_vertex_for_rectangle_list
                ) {
                    possible_vertex_for_rectangle_list.remove(used_vertex_index);
                }
                possible_vertex_for_rectangle_list.append(&mut get_vertices_for_positioning(positioned_rectangle, spacing));
            },
            None => {
                unused_rectangles_list.push(rectangle.clone());
            },
        };
    }

    let length_used = 
    match positioned_rectangles_list.iter().max_by(
        |first, second| rectangle_maximum_y_comparator(first, second)
    ) {
        Some(rect) => rect.top_left_vertex.pos_y + rect.length,
        None => 0,
    };

    MainRectangleOrganized {
        possible_vertex_for_rectangle_list,
        unused_rectangles_list,
        positioned_rectangles_list,
        length_used
    }
}

fn organize_showcase_rectangles(
    showcase: Rectangle,
    spacing: i32, 
    max_width: i32, 
    prohibited_area_list: &[PositionedRectangle],
    possible_vertex_for_rectangle_list: &[Vertex], 
    positioned_rectangles_list: &[PositionedRectangle], 
    length_used: i32
) -> Vec<PositionedRectangle> {
    let mut possible_vertex_for_rectangle_list_sorted: Vec<Vertex> = Vec::from(possible_vertex_for_rectangle_list);

    possible_vertex_for_rectangle_list_sorted.sort_by(vertex_closest_to_top_and_left_comparator);

    let mut positioned_showcase_list = Vec::<PositionedRectangle>::new();

    while !&possible_vertex_for_rectangle_list_sorted.is_empty() {
        possible_vertex_for_rectangle_list_sorted.sort_by(vertex_closest_to_top_and_left_comparator);

        if let Some(vertex) = possible_vertex_for_rectangle_list_sorted.get(0) {
            let subject = PositionedRectangle {
                width: showcase.width,
                length: showcase.length,
                top_left_vertex: vertex.clone(),
            };

            if 
            !is_within_boundaries(
                &subject, 
                max_width, 
                length_used)
            ||
            subject_intesect_with_positioned_rectangles_list(
                &subject, 
                positioned_rectangles_list, 
                spacing
            ) 
            ||
            subject_intesect_with_positioned_rectangles_list(
                &subject, 
                prohibited_area_list,
                0
            ) 
            ||
            subject_intesect_with_positioned_rectangles_list(
                &subject, 
                &positioned_showcase_list, 
                spacing
            ) {
                possible_vertex_for_rectangle_list_sorted.remove(0);
                continue;
            }

            possible_vertex_for_rectangle_list_sorted.remove(0);
            possible_vertex_for_rectangle_list_sorted.append(
                &mut create_available_vertices_for_positioning(&subject, spacing)
            );
            positioned_showcase_list.push(subject);
        } else {
            break;
        }
    };

    positioned_showcase_list
}

fn get_vertex_index_in_list(vertex: &Vertex, possible_vertex_for_rectangle_list: &Vec<Vertex>) -> Option<usize> {
    possible_vertex_for_rectangle_list
    .iter()
    .position(
        |used_vertex| 
        used_vertex.equals(vertex)
    )
}

fn find_position_for_rectangle(
    rectangle: &Rectangle,
    max_width: i32, 
    max_length: i32,
    spacing: i32, 
    positioned_rectangles_list: &[PositionedRectangle], 
    possible_vertex_for_rectangle_list: &[Vertex],
    prohibited_area_list: &[PositionedRectangle]
) -> Option<PositionedRectangle> {
    let positioned_rectangle_option = 
    possible_vertex_for_rectangle_list
    .iter()
    .find_map(
        |vertex| {
            let subject = PositionedRectangle {
                width: rectangle.width,
                length: rectangle.length,
                top_left_vertex: vertex.clone(),
            };
    
            if 
            !is_within_boundaries(
                &subject, 
                max_width, 
                max_length
            )
            ||
            subject_intesect_with_positioned_rectangles_list(
                &subject, 
                positioned_rectangles_list, 
                spacing
            ) 
            ||
            subject_intesect_with_positioned_rectangles_list(
                &subject, 
                prohibited_area_list,
                0
                ) {
                return Option::None;
            }

            Option::Some(subject)
        }
    );
    positioned_rectangle_option
}

fn get_vertices_for_positioning(positioned_rectangle: PositionedRectangle, spacing: i32) -> Vec<Vertex> {
    let positioned_rectangle_vertices = positioned_rectangle.get_vertices();
    let vertices_to_be_added = 
        &mut vec![
            Vertex { 
                pos_x: positioned_rectangle_vertices.bottom_left_vertex.pos_x, 
                pos_y:  positioned_rectangle_vertices.bottom_left_vertex.pos_y + spacing 
            },
            Vertex { 
                pos_x: positioned_rectangle_vertices.top_rigth_vertex.pos_x + spacing, 
                pos_y:  positioned_rectangle_vertices.top_rigth_vertex.pos_y 
            },
        ];
    vertices_to_be_added.to_vec()
}

fn subject_intesect_with_positioned_rectangles_list(
    subject: &PositionedRectangle,
    positioned_rectangles_list: &[PositionedRectangle], 
    spacing: i32
) -> bool {
    positioned_rectangles_list
    .iter()
    .any(
        |rect| {
            intersect(subject, rect, spacing)
        }
    )
}


fn create_available_vertices_for_prohibited_area(
    prohibited_area: &PositionedRectangle,
) -> Vec<Vertex> {
        let mut vertex_list: Vec<Vertex> = Vec::<Vertex>::new();
        vertex_list.append(&mut create_available_vertices_for_positioning(prohibited_area, 0));
        
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
    spacing: i32
) -> Vec<Vertex> {
    let mut vertex_list: Vec<Vertex> = Vec::<Vertex>::new();
    
    let PositionedRectangleVertices {top_rigth_vertex, bottom_left_vertex, ..} = positioned_rectangle.get_vertices();

    vertex_list.push(
        Vertex { 
            pos_x: top_rigth_vertex.pos_x + spacing, 
            pos_y: top_rigth_vertex.pos_y,
        }
    );
    vertex_list.push(
        Vertex { 
            pos_x: bottom_left_vertex.pos_x, 
            pos_y: bottom_left_vertex.pos_y + spacing,
        }
    );
    vertex_list
}

fn rectangle_wider_and_longer_comparator(first: &Rectangle, second: &Rectangle) -> Ordering {
    if first.width < second.width || (first.width == second.width && first.length < second.length) {
        Ordering::Greater
    } else if first.width == second.width && first.length == second.length {
        Ordering::Equal
    } else {
        Ordering::Less
    }
}

fn vertex_closest_to_top_and_left_comparator(first: &Vertex, second: &Vertex) -> Ordering {
    if first.pos_y < second.pos_y || (first.pos_y == second.pos_y && first.pos_x < second.pos_x) {
        Ordering::Less
    } else if first.pos_y == second.pos_y && first.pos_x == second.pos_x {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

fn is_within_boundaries(
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

fn intersect(
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

fn rectangle_maximum_y_comparator(first: &PositionedRectangle, second: &PositionedRectangle) -> Ordering {
    (first.top_left_vertex.pos_y + first.length).cmp(&(second.top_left_vertex.pos_y + second.length))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn organize_main_rectangles_test() {

        let prohibited_area = PositionedRectangle {
            width: 160,
            length: 60,
            top_left_vertex:
                Vertex { 
                    pos_x: 0, 
                    pos_y: 0 
                }
        };
        let rect1 = Rectangle {
            width: 120,
            length: 40
        };
        let rect2 = Rectangle {
            width: 40,
            length: 70
        };
        let rect3 = Rectangle {
            width: 20,
            length: 40
        };
        let rect_no_fit = Rectangle {
            width: 20,
            length: 40
        };

        let rectangles_list = vec![
            rect1.clone(),
            rect2.clone(),
            rect3.clone(),
            rect_no_fit.clone()
        ];

        let prohibited_area_list = vec![
            prohibited_area
        ];

        let spacing = 10;
        let max_length = 110;
        let max_width= 200;

        // action
        let main_rectangle_organized = organize_main_rectangles(
            max_length,
            spacing, 
            max_width, 
            &rectangles_list, 
            &prohibited_area_list
        );

        // assertion
        let p_rect1 = PositionedRectangle::new_from_rectangle_and_vertex(&rect1, &Vertex { pos_x: 0, pos_y: 60 });
        let p_rect2 = PositionedRectangle::new_from_rectangle_and_vertex(&rect2, &Vertex { pos_x: 160, pos_y: 0 });
        let p_rect3 = PositionedRectangle::new_from_rectangle_and_vertex(&rect3, &Vertex { pos_x: 130, pos_y: 60 });


        assert!(main_rectangle_organized.unused_rectangles_list.iter().any(|item| item.equals(&rect_no_fit)));
        assert!(main_rectangle_organized.positioned_rectangles_list.iter().any(|item| item.equals(&p_rect1)));
        assert!(main_rectangle_organized.positioned_rectangles_list.iter().any(|item| item.equals(&p_rect2)));
        assert!(main_rectangle_organized.positioned_rectangles_list.iter().any(|item| item.equals(&p_rect3)));

        assert_eq!(100, main_rectangle_organized.length_used);

    }


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

        // expect
        let vertices = vec![
            Vertex { 
                pos_x: 6, 
                pos_y: 3, 
            },
            Vertex { 
                pos_x: 1, 
                pos_y: 10, 
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
        assert_eq!(vertices, create_available_vertices_for_prohibited_area(&prohibited_area));
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

        let spacing = 3;

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
