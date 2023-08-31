# The string "PAYPALISHIRING" is written in a zigzag pattern on a given number
# of rows like this: (you may want to display this pattern in a fixed font for
# better legibility)
#
# ```
# P   A   H   N
# A P L S I I G
# Y   I   R
# ```
#
# And then read line by line: "PAHNAPLSIIGYIR"
#
# Write the code that will take a string and make this conversion given a number
# of rows:
#
# ```
# string convert(string s, int numRows);
# ```

convert <- function(str, num_rows) {
  # Create an empty 2D matrix to store characters in zigzag order.
  str_chars  <- strsplit(str, "") |> unlist()
  num_cols   <- ceiling(length(str_chars) / 2)
  out_mtx    <- matrix("", nrow = num_rows, ncol = num_cols)

  # These three variables constitute a cursor over the spaces in `out_mtx`.
  layout_dir <- "down"
  layout_row <- 1
  layout_col <- 1

  # Move the cursor about, placing letters into the appropriate space on
  # the 2D matrix.
  for (char in str_chars) {
    out_mtx[layout_row, layout_col] <- char

    if (layout_dir == "down" && layout_row == num_rows) {
      layout_dir <- "up"
      layout_row <- layout_row - 1
      layout_col <- layout_col + 1
    } else if (layout_dir == "down") {
      layout_row <- layout_row + 1
    } else if (layout_dir == "up" && layout_row == 1) {
      layout_row <- layout_row + 1
      layout_dir <- "down"
    } else if (layout_dir == "up") {
      layout_row <- layout_row - 1
      layout_col <- layout_col + 1
    }
  }

  # Transpose the matrix, convert it to a vector with all the empty spaces
  # removed, then join the characters into a single string.
  (out_mtx
  |> t()
  |> (\(x) x[x != ""])()
  |> paste0(collapse = ""))
}

multi_seq <- function(from, to, by) {

}

convert2 <- function(str, num_rows) {
  str_chars  <- strsplit(str, "") |> unlist()
  num_chars  <- length(str_chars)
  periods    <- c((num_chars * 2) - 2, 0)
  sort_idxs  <- vector("numeric", num_chars)

  while (periods[1] >= 0) {
    round <- 1


  }
}
