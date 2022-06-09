#[derive(Clone, Copy, Debug)]
pub enum CellValue {
    Empty,
    Number(u8),
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub value: CellValue,
    pub hints: Vec<u8>,
    pub is_original: bool,
    pub is_invalid: bool,
}

impl Cell {
    pub fn new(value: CellValue, is_original: bool) -> Self {
        Cell {
            value,
            hints: Vec::new(),
            is_original,
            is_invalid: false,
        }
    }

    pub fn add_hint(&mut self, hint: u8) {
        let idx = self.hints.iter().position(|&h| h == hint);
        if let Some(idx) = idx {
            self.hints.remove(idx);
            return;
        }

        self.hints.push(hint);
        self.hints.sort();
    }
}
