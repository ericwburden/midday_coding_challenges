#[derive(Default)]
pub struct SubSection([bool; 9]);

impl SubSection {
    pub fn add(&mut self, val: char) -> Result<(), &'static str> {
        let idx = match val {
            '.' => return Ok(()), // Just skip '.'
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            _ => return Err("Tried to add an invalid number.")
        };
        if self.0[idx] { return Err("Number already used!"); }
        self.0[idx] = true;
        Ok(())
    }
}

#[derive(Default)]
pub struct SudokuChecker {
    rows: [SubSection; 9],
    cols: [SubSection; 9],
    mins: [SubSection; 9],
}

impl SudokuChecker {
    pub fn add(&mut self, row: usize, col: usize, val: char) -> Result<(), &'static str> {
        // Add the number to the subsection for the row
        self.rows.get_mut(row).ok_or("Invalid row index")?.add(val)?;

        // Add the number to the subsection for the column
        self.cols.get_mut(col).ok_or("Invalid col index")?.add(val)?;

        // Figure out which mini-square to check
        let mini_square = match row {
            0..=2 => match col {
                0..=2 => 0,
                3..=5 => 1,
                6..=8 => 2,
                _ => unreachable!(),
            }
            3..=5 => match col {
                0..=2 => 3,
                3..=5 => 4,
                6..=8 => 5,
                _ => unreachable!(),
            }
            6..=8 => match col {
                0..=2 => 6,
                3..=5 => 7,
                6..=8 => 8,
                _ => unreachable!(),
            }
            _ => unreachable!(),
        };
        self.mins.get_mut(mini_square).unwrap().add(val)?;

        // If value is added successfully to all valid subsections, then
        // it must be the first time. Return OK.
        Ok(())
    }
}



pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut sudoku_checker = SudokuChecker::default();
    for (ridx, row) in board.iter().enumerate().take(9) {
        for (cidx, value) in row.iter().enumerate().take(9) {
            if sudoku_checker.add(ridx, cidx, *value).is_err() {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let board = [
            ['5', '3', ]
        ]
        assert_eq!(result, 4);
    }
}
