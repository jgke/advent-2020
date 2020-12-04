pub struct Grid<Cell> {
    pub elems: Vec<Vec<Cell>>,
}

#[allow(dead_code)]
impl<Cell> Grid<Cell> {
    pub fn new(elems: Vec<Vec<Cell>>) -> Grid<Cell> {
        Grid { elems }
    }

    pub fn col_size(&self) -> usize {
        self.elems.len()
    }

    pub fn row_size(&self) -> usize {
        self.elems[0].len()
    }
}
