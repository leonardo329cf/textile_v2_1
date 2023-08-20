use crate::models::{cut_disposition::PositionedRectangle, cutting_table::CuttingTable, cutting_lines::{CuttingLines, Line}};


pub fn define_cutting_lines(
    cutting_table: CuttingTable,
    positioned_rectangle_list: Vec<PositionedRectangle>
) -> CuttingLines {

    let mut vertical_lines: Vec<Line> = Vec::<Line>::new();

    let mut horizontal_lines: Vec<Line> = Vec::<Line>::new();

    for positioned_rectangle in positioned_rectangle_list {
        vertical_lines.push(positioned_rectangle.get_left_line());
        vertical_lines.push(positioned_rectangle.get_rigth_line());

        horizontal_lines.push(positioned_rectangle.get_top_line());
        horizontal_lines.push(positioned_rectangle.get_bottom_line());
    }

    let vertical_lines_result_list = combine_lines(&vertical_lines, &horizontal_lines);

    let horizontal_lines_result_list = combine_lines(&horizontal_lines, &vertical_lines);
    
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