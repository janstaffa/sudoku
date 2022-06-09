use std::fmt;

use crate::cell::{self, Cell, CellValue};

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(cells: Vec<Vec<char>>) -> Result<Self, String> {
        let cells: Result<Vec<Vec<Cell>>, String> = cells
            .iter()
            .map(|r| {
                r.iter()
                    .map(|&c| {
                        let cell_value = match c {
                            '.' => CellValue::Empty,
                            '0'..='9' => {
                                let d = c.to_digit(10).unwrap();
                                let cell_value = CellValue::Number(d as u8);
                                cell_value
                            }
                            _ => return Err("Invalid character in input file".into()),
                        };

                        Ok(Cell::new(
                            cell_value,
                            !matches!(cell_value, CellValue::Empty),
                        ))
                    })
                    .collect()
            })
            .collect();

        let cells = cells?;

        Ok(Board { cells })
    }

    pub fn is_solved(&self) -> bool {
        for (i, row) in self.cells.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if let CellValue::Empty = c.value {
                    return false;
                }

                let c = j / 3;
                let r = i / 3;

                let square = r * 3 + c;
                let to_check: [Vec<Cell>; 3] = [
                    self.get_row(i).unwrap(),
                    self.get_col(j).unwrap(),
                    self.get_square(square).unwrap(),
                ];

                for arr in to_check {
                    let mut remaining: Vec<u8> = (1..=9).collect();
                    for c in arr {
                        if let CellValue::Number(n) = c.value {
                            if let Some(idx) = remaining.iter().position(|&x| x == n) {
                                remaining.remove(idx);
                            }
                        }
                    }
                    if remaining.len() != 0 {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    //  fn validate_move(&mut self, r: usize, c: usize) -> Option<()> {
    //      let row = self.cells.get_mut(r)?;
    //      let row = row.iter_mut().enumerate();
    //      for (i, c) in row.iter_mut().enumerate() {
    //          for (j, x) in row.iter().enumerate() {
    //              if i != j {
    //                  if let CellValue::Number(c1) = c.value {
    //                      if let CellValue::Number(c2) = x.value {
    //                          if c1 == c2 {
    //                              c.is_invalid = true;
    //                          }
    //                      }
    //                  }
    //              }
    //          }
    //      }
    //      Some(())
    //  }

    fn get_row(&self, idx: usize) -> Option<Vec<Cell>> {
        Some(self.cells.get(idx)?.to_vec())
    }
    fn get_col(&self, idx: usize) -> Option<Vec<Cell>> {
        if idx > self.cells.first()?.len() {
            return None;
        }
        let mut result: Vec<Cell> = Vec::new();
        for row in &self.cells {
            for (i, c) in row.iter().enumerate() {
                if i == idx {
                    result.push(c.clone());
                }
            }
        }
        Some(result)
    }

    fn get_square(&self, idx: usize) -> Option<Vec<Cell>> {
        if idx > 8 {
            return None;
        }
        let row = idx / 3;
        let col = idx % 3;

        let cx = (col * 3) + 1;
        let cy = (row * 3) + 1;

        let mut result = Vec::new();
        for i in -1_i32..=1_i32 {
            for j in -1_i32..=1_i32 {
                let cell = self
                    .cells
                    .get((cy as i32 + i) as usize)
                    .unwrap()
                    .get((cx as i32 + j) as usize)
                    .unwrap();
                result.push(cell.clone());
            }
        }
        Some(result)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let pipe = std::iter::repeat('-')
            .take(self.cells.first().unwrap().len() + 4)
            .collect::<String>();
        result.push_str(&pipe);
        result.push('\n');

        for (i, row) in self.cells.iter().enumerate() {
            result.push('|');
            for (j, c) in row.iter().enumerate() {
                let ch = match c.value {
                    CellValue::Number(n) => char::from_digit(n as u32, 10).unwrap(),
                    CellValue::Empty => '.',
                };
                result.push(ch);
                if (j + 1) % 3 == 0 {
                    result.push('|');
                }
            }
            if (i + 1) % 3 == 0 {
                result.push('\n');
                result.push_str(&pipe);
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}
