use crate::models::{cut_disposition::{PositionedRectangle, Vertex}, cutting_lines::{CuttingLines, Line, line_vertical_closest_to_left_comparator, line_horizontal_closest_to_top_comparator}};

pub fn define_cutting_lines(
    positioned_rectangle_list: Vec<PositionedRectangle>,
    textile_separation_line_width: Option<i32>

) -> CuttingLines {

    let mut vertical_lines: Vec<Line> = Vec::<Line>::new();

    let mut horizontal_lines: Vec<Line> = Vec::<Line>::new();

    if let Some(width) = textile_separation_line_width {
        if width > 0 {
            horizontal_lines.push(
                Line { 
                    start: Vertex { 
                        pos_x: 0, 
                        pos_y: 0 
                    }, 
                    end: Vertex { 
                        pos_x: width, 
                        pos_y: 0
                    }
                }
            )
        }
    };

    for positioned_rectangle in positioned_rectangle_list {
        vertical_lines.push(positioned_rectangle.get_left_line());
        vertical_lines.push(positioned_rectangle.get_rigth_line());

        horizontal_lines.push(positioned_rectangle.get_top_line());
        horizontal_lines.push(positioned_rectangle.get_bottom_line());
    }

    let mut vertical_lines_result_list = combine_lines(&vertical_lines, &horizontal_lines);

    let mut horizontal_lines_result_list = combine_lines(&horizontal_lines, &vertical_lines);
    
    vertical_lines_result_list.sort_by(line_vertical_closest_to_left_comparator);

    horizontal_lines_result_list.sort_by(line_horizontal_closest_to_top_comparator);

    CuttingLines {
        vertical_lines: vertical_lines_result_list,
        horizontal_lines: horizontal_lines_result_list
    }
}

fn combine_lines(lines_to_be_combined: &Vec<Line>, lines_not_to_be_crossed: &Vec<Line>) -> Vec<Line> {
    let mut lines_result_list: Vec<Line> = Vec::<Line>::new();

    for line in lines_to_be_combined {

        let mut can_combine = false;

        for existing_result_line in &lines_result_list {

            if line.is_same_level(existing_result_line) {

                let combined_line = line.combine(existing_result_line);
                let mut does_cross: bool = false;

                for perpendicular_line in lines_not_to_be_crossed {

                    if combined_line.cross_perpend(perpendicular_line) {
                        does_cross = true;
                        break;
                    }
                }

                if does_cross {
                    continue;
                } else {
                    can_combine = true;
                    lines_result_list.push(combined_line);
                    break;
                }

            }
        }

        if !can_combine {
            lines_result_list.push(line.clone());
        }
    }
    lines_result_list
}

#[cfg(test)]
mod tests {
    use crate::{models::{cut_disposition::{PositionedRectangle, Vertex}, cutting_lines::Line}, services::cutting_lines_service::define_cutting_lines};

    #[test]
    fn define_cutting_lines_test() {
        let pos_rect_1 = PositionedRectangle {
            id: 1,
            width: 150,
            length: 50,
            top_left_vertex: Vertex {
                pos_x: 0,
                pos_y: 0,
            },
        };

        let pos_rect_2 = PositionedRectangle {
            id: 2,
            width: 120,
            length: 40,
            top_left_vertex: Vertex {
                pos_x: 0,
                pos_y: 60,
            },
        };

        let pos_rect_3 = PositionedRectangle {
            id: 3,
            width: 40,
            length: 70,
            top_left_vertex: Vertex {
                pos_x: 160,
                pos_y: 0,
            },
        };

        let pos_rect_4 = PositionedRectangle {
            id: 4,
            width: 20,
            length: 40,
            top_left_vertex: Vertex {
                pos_x: 130,
                pos_y: 60,
            },
        };

        let pos_rect_5 = PositionedRectangle {
            id: 5,
            width: 20,
            length: 20,
            top_left_vertex: Vertex {
                pos_x: 160,
                pos_y: 80,
            },
        };

        let positioned_rectangle_list = vec![pos_rect_1, pos_rect_2, pos_rect_3, pos_rect_4, pos_rect_5];

        let cutting_lines = define_cutting_lines(positioned_rectangle_list, None);

        //expected vertical lines
        let v0 = Line { start: Vertex { pos_x: 0, pos_y: 0 },end: Vertex { pos_x: 0, pos_y: 100 } };
        let v1 = Line { start: Vertex { pos_x: 150, pos_y: 0 },end: Vertex { pos_x: 150, pos_y: 100 } };
        let v2 = Line { start: Vertex { pos_x: 120, pos_y: 60 },end: Vertex { pos_x: 120, pos_y: 100 } };
        let v3 = Line { start: Vertex { pos_x: 160, pos_y: 0 },end: Vertex { pos_x: 160, pos_y: 100 } };
        let v4 = Line { start: Vertex { pos_x: 200, pos_y: 0 },end: Vertex { pos_x: 200, pos_y: 70 } };
        let v5 = Line { start: Vertex { pos_x: 130, pos_y: 60 },end: Vertex { pos_x: 130, pos_y: 100 } };
        let v6 = Line { start: Vertex { pos_x: 180, pos_y: 80 },end: Vertex { pos_x: 180, pos_y: 100 } };

		//expected horizontal lines
        let h0 = Line { start: Vertex { pos_x: 0, pos_y: 0 },end: Vertex { pos_x: 200, pos_y: 0 } };
        let h1 = Line { start: Vertex { pos_x: 0, pos_y: 50 },end: Vertex { pos_x: 150, pos_y: 50 } };
        let h2 = Line { start: Vertex { pos_x: 0, pos_y: 60 },end: Vertex { pos_x: 150, pos_y: 60 } };
        let h3 = Line { start: Vertex { pos_x: 0, pos_y: 100 },end: Vertex { pos_x: 180, pos_y: 100 } };
        let h4 = Line { start: Vertex { pos_x: 160, pos_y: 70 },end: Vertex { pos_x: 200, pos_y: 70 } };
        let h5 = Line { start: Vertex { pos_x: 160, pos_y: 80 },end: Vertex { pos_x: 180, pos_y: 80 } };

        assert!(cutting_lines.vertical_lines.contains(&v0));
        assert!(cutting_lines.vertical_lines.contains(&v1));
        assert!(cutting_lines.vertical_lines.contains(&v2));
        assert!(cutting_lines.vertical_lines.contains(&v3));
        assert!(cutting_lines.vertical_lines.contains(&v4));
        assert!(cutting_lines.vertical_lines.contains(&v5));
        assert!(cutting_lines.vertical_lines.contains(&v6));

        assert!(cutting_lines.horizontal_lines.contains(&h0));
        assert!(cutting_lines.horizontal_lines.contains(&h1));
        assert!(cutting_lines.horizontal_lines.contains(&h2));
        assert!(cutting_lines.horizontal_lines.contains(&h3));
        assert!(cutting_lines.horizontal_lines.contains(&h4));
        assert!(cutting_lines.horizontal_lines.contains(&h5));

    }
}