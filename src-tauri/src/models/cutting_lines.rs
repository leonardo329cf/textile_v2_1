use super::cut_disposition::Vertex;

pub struct Line {
    pub first: Vertex,
    pub second: Vertex
}

pub struct CuttingLines {
    pub vertical_lines: Vec<Line>,
    pub horizontal_lines: Vec<Line>
}
