use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::cut_disposition::Vertex;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub start: Vertex,
    pub end: Vertex
}

pub fn line_horizontal_closest_to_top_comparator(first: &Line, second: &Line) -> Ordering {
    if first.start.pos_y < second.start.pos_y || (first.start.pos_y == second.start.pos_y && first.start.pos_x < second.start.pos_x) {
        Ordering::Less
    } else if first.start.pos_y == second.start.pos_y && first.start.pos_x == second.start.pos_x {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

pub fn line_vertical_closest_to_left_comparator(first: &Line, second: &Line) -> Ordering {
    if first.start.pos_x < second.start.pos_x || (first.start.pos_x == second.start.pos_x && first.start.pos_y < second.start.pos_y) {
        Ordering::Less
    } else if first.start.pos_x == second.start.pos_x && first.start.pos_y == second.start.pos_y {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
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
        let mut end = self.end.clone();

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

#[cfg(test)]
mod tests {
    use crate::models::{cut_disposition::Vertex, cutting_lines::Line};

    #[test]
    fn cross_perpend_vertical_line_test() {

        // subject vertical line
		let vert_l = Line {
            start: Vertex {
                pos_x: 10,
                pos_y: 10
            },
            end: Vertex {
                pos_x: 10,
                pos_y: 20
            }
        };
		
		// no-crossing horizontal lines
		let no_crossing_h_1 = Line {
            start: Vertex {
                pos_x: 5,
                pos_y: 10
            },
            end: Vertex {
                pos_x: 15,
                pos_y: 10
            }
        };

        let no_crossing_h_2 = Line {
            start: Vertex {
                pos_x: 10,
                pos_y: 12
            },
            end: Vertex {
                pos_x: 15,
                pos_y: 12
            }
        }; 

        let no_crossing_h_3 = Line {
            start: Vertex {
                pos_x: 5,
                pos_y: 18
            },
            end: Vertex {
                pos_x: 10,
                pos_y: 18
            }
        }; 

        let no_crossing_h_4 = Line {
            start: Vertex {
                pos_x: 5,
                pos_y: 20
            },
            end: Vertex {
                pos_x: 15,
                pos_y: 20
            }
        }; 

		// crossing horizontal lines
        let crossing_h = Line {
            start: Vertex {
                pos_x: 5,
                pos_y: 15
            },
            end: Vertex {
                pos_x: 15,
                pos_y: 15
            }
        };

        assert!(!vert_l.cross_perpend(&no_crossing_h_1));
        assert!(!vert_l.cross_perpend(&no_crossing_h_2));
        assert!(!vert_l.cross_perpend(&no_crossing_h_3));
        assert!(!vert_l.cross_perpend(&no_crossing_h_4));

        assert_eq!(vert_l.cross_perpend(&crossing_h), true);
    }

    #[test]
    fn cross_perpend_horizontal_line_test() {
        // subject horizontal line
		let horiz_l = Line {
            start: Vertex {
                pos_x: 10,
                pos_y: 10
            },
            end: Vertex {
                pos_x: 20,
                pos_y: 10
            }
        };

        // no-crossing vertical lines
		let no_crossing_v_1 = Line {
            start: Vertex {
                pos_x: 10,
                pos_y: 5
            },
            end: Vertex {
                pos_x: 10,
                pos_y: 15
            }
        };

        let no_crossing_v_2 = Line {
            start: Vertex {
                pos_x: 12,
                pos_y: 5
            },
            end: Vertex {
                pos_x: 12,
                pos_y: 10
            }
        };

        let no_crossing_v_3 = Line {
            start: Vertex {
                pos_x: 18,
                pos_y: 10
            },
            end: Vertex {
                pos_x: 18,
                pos_y: 15
            }
        };

        let no_crossing_v_4 = Line {
            start: Vertex {
                pos_x: 20,
                pos_y: 5
            },
            end: Vertex {
                pos_x: 20,
                pos_y: 15
            }
        };

        // crossing vertical lines
        let crossing_v = Line {
            start: Vertex {
                pos_x: 15,
                pos_y: 5
            },
            end: Vertex {
                pos_x: 15,
                pos_y: 15
            }
        };

        assert!(!horiz_l.cross_perpend(&no_crossing_v_1));
        assert!(!horiz_l.cross_perpend(&no_crossing_v_2));
        assert!(!horiz_l.cross_perpend(&no_crossing_v_3));
        assert!(!horiz_l.cross_perpend(&no_crossing_v_4));

        assert!(horiz_l.cross_perpend(&crossing_v));
    }
}
