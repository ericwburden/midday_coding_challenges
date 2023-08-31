#![allow(dead_code)]

use itertools::Itertools;
use rayon::prelude::*;

struct Bar {
    x: usize,
    y: usize,
}

fn max_rect_histogram(heights: &[usize]) -> usize {
    let mut stack: Vec<Bar> = Vec::with_capacity(heights.len());
    let mut max_area = 0;
    let mut idx = 0;

    while idx < heights.len() {
        let current_bar = Bar {
            x: idx,
            y: heights[idx],
        };

        let last_bar_height = stack.last().map(|bar| bar.y).unwrap_or(0);

        if last_bar_height <= current_bar.y {
            stack.push(current_bar);
            idx += 1;
        } else {
            // The only way to enter this branch is if the stack contains
            // at least one bar whose height is greater than the height
            // of the previous `current_bar`.
            let current_bar = stack.pop().unwrap();
            let width = stack.last().map(|bar| idx - bar.x - 1).unwrap_or(idx);
            let area = current_bar.y * width;
            max_area = std::cmp::max(max_area, area);
        }
    }

    while let Some(current_bar) = stack.pop() {
        let width = stack.last().map(|bar| idx - bar.x - 1).unwrap_or(idx);
        let area = current_bar.y * width;
        max_area = std::cmp::max(max_area, area);
    }

    max_area
}

fn parse_matrix(str_arr: &[&str]) -> Vec<Vec<usize>> {
    str_arr
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

/// Given a matrix of 1's and 0's, calculate the area of the largest rectangle that
/// contains the point (row_idx, col_idx) as its top-left inner point.
fn area_of_rectangle_at_idx(matrix: &[Vec<usize>], row_idx: usize, col_idx: usize) -> usize {
    // The width of the widest rectangle that can be made including the
    // top-left point on the current line being examined.
    let mut width_limit = usize::MAX;

    // The maximum area of a rectangle found so far.
    let mut max_area = 0;

    // Row and column indices for the point currently being examined.
    let mut row_cursor = row_idx;
    let mut col_cursor = col_idx;

    // For as long as our `row_cursor` index is still inside the matrix...
    'outer: while let Some(row) = matrix.get(row_cursor) {
        // For as long as our `col_cursor` index is also inside the matrix...
        while let Some(value) = row.get(col_cursor) {
            // If the value under the cursor is a zero _or_ we've moved to a column
            // past the widest rectangle we can make...
            if *value == 0 || col_cursor > width_limit {
                // If we're on the same column as the top left corner, we're done. we've
                // gone down as far as we can go.
                if col_cursor == col_idx {
                    break 'outer;
                }

                // Otherwise, update the `width_limit` to be the current column minus 1,
                // meaning the previous column was the last column with a 1 in it.
                width_limit = std::cmp::min(width_limit, col_cursor - 1);

                // Check the next row.
                break;
            }

            // If the value under the cursor is a 1 and we haven't gone past the rightmost
            // limit, then calculate the area of the rectangle with (row_idx, col_idx) as
            // the top-left corner and (row_cursor, col_cursor) as the bottom-right corner.
            let width = col_cursor - col_idx + 1;
            let height = row_cursor - row_idx + 1;

            // Update `max_area` if we've found a bigger rectangle.
            max_area = std::cmp::max(width * height, max_area);

            // Check the next column.
            col_cursor += 1;
        }

        // Once finished with the current row, reset the column cursor and move on to the
        // next row.
        col_cursor = col_idx;
        row_cursor += 1;
    }

    // Return the area of the largest rectangle found.
    max_area
}

pub fn matrix_challenge_linear(str_arr: &[&str]) -> usize {
    let mut matrix = parse_matrix(str_arr);
    let rows = matrix.len();
    let cols = matrix.first().map(|row| row.len()).unwrap_or(0);

    for col in 1..cols {
        let mut height = 0;
        for row in (1..rows).rev() {
            if matrix[row][col] == 0 {
                height = 0;
            } else {
                matrix[row][col] += height;
                height += 1;
            }
        }
    }

    matrix
        .iter()
        .map(|row| max_rect_histogram(row))
        .max()
        .unwrap_or(0)
}

pub fn matrix_challenge_quadratic(str_arr: &[&str]) -> usize {
    let matrix = parse_matrix(str_arr);
    let rows = matrix.len();
    let cols = matrix.first().map(|row| row.len()).unwrap_or(0);
    let mut max_area = 0;

    for row_idx in 1..rows {
        for col_idx in 1..cols {
            let area = area_of_rectangle_at_idx(&matrix, row_idx, col_idx);
            max_area = std::cmp::max(area, max_area);
        }
    }

    max_area
}

pub fn matrix_challenge_quad_par(str_arr: &[&str]) -> usize {
    let matrix = parse_matrix(str_arr);
    let rows = matrix.len();
    let cols = matrix.first().map(|row| row.len()).unwrap_or(0);
    (1..rows)
        .cartesian_product(1..cols)
        .par_bridge()
        .map(|(r, c)| area_of_rectangle_at_idx(&matrix, r, c))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    #[test]
    fn can_find_max_rect_in_histogram() {
        assert_eq!(max_rect_histogram(&[1, 2, 3,  4, 5]),           9);
        assert_eq!(max_rect_histogram(&[5, 4, 3,  2, 1]),           9);
        assert_eq!(max_rect_histogram(&[2, 5, 5,  5, 2, 2, 2]),    15);
        assert_eq!(max_rect_histogram(&[2, 2, 5,  5, 5, 2, 2, 2]), 16);
        assert_eq!(max_rect_histogram(&[5, 5, 2,  2, 5, 5]),       12);
        assert_eq!(max_rect_histogram(&[5, 5, 5, 50, 5, 5]),       50);
    }

    #[test]
    fn can_complete_matrix_challenge_with_hist_rect() {
        let str_arr = [
            "111110",
            "100000",
            "101110",
            "101110",
            "101110"
        ];
        assert_eq!(matrix_challenge_linear(&str_arr),    9);
        assert_eq!(matrix_challenge_quadratic(&str_arr), 9);
        assert_eq!(matrix_challenge_quad_par(&str_arr),  9);

        let str_arr = [
            "000000000000",
            "011111111110",
            "011110111110",
            "011110111110",
            "010000000010",
            "011110111110",
            "011110111110",
            "011111111110",
            "000000000000",
        ];
        assert_eq!(matrix_challenge_linear(&str_arr),    15);
        assert_eq!(matrix_challenge_quadratic(&str_arr), 15);
        assert_eq!(matrix_challenge_quad_par(&str_arr),  15);
    }

    #[test]
    fn can_get_area_of_largest_rect_originating_at_row_col() {
        let str_arr = [
            "000000000000",
            "011111111110",
            "011110111110",
            "011110111110",
            "010000000010",
            "011110111110",
            "011110111110",
            "011111111110",
            "000000000000",
        ];

        let matrix = str_arr
            .into_iter()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(area_of_rectangle_at_idx(&matrix, 1, 1), 12);
        assert_eq!(area_of_rectangle_at_idx(&matrix, 1, 5),  6);
        assert_eq!(area_of_rectangle_at_idx(&matrix, 3, 4),  1);
        assert_eq!(area_of_rectangle_at_idx(&matrix, 1, 6), 15);
        assert_eq!(area_of_rectangle_at_idx(&matrix, 5, 1), 12);
        assert_eq!(area_of_rectangle_at_idx(&matrix, 5, 6), 15);
    }
}
