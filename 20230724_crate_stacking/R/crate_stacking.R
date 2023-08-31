# Parse the input file into usable data structures
parse_input <- function(input_text) {
  # Since the input file has two parts, top and bottom, we need to
  # split the input file lines on the empty line.
  empty_line_idx <- which(input_text == "")

  # The part of the input with the crates is before the empty line,
  # the part with instructions is after the empty line.
  crate_spec     <- head(input_text, empty_line_idx - 2)
  instructions   <- tail(input_text, -empty_line_idx)

  # Return the parsed input as a list containing the parsed crate stack
  # and the parsed list of instructions.
  list(parse_crate_stacks(crate_spec),
       parse_instructions(instructions))
}


# Parse the crates from the input file into a more usable data structure
parse_crate_stacks <- function(crate_spec) {
  # For each line in the input, extract the character where each crate
  # letter _should_ be. Unless all the stacks are the same height, there
  # will be empty spaces on top of some of the stacks. We'll replace these
  # spaces with NA's for simplicity. Since we need to read the input
  # line-by-line (row-by-row), we  transpose the result to switch the rows
  # into columns to make the input easier to reason about.
  crate_stacks <- t(sapply(
    crate_spec,
    \(line) {
      chars  <- line |> strsplit("") |> unlist()
      idxs   <- seq(2, length(chars), by = 4)
      crates <- chars[idxs]
      crates[crates == " "] <- NA_character_
      crates
    },
    USE.NAMES = FALSE
  ))

  # Since we'll end up moving crates around, we want to make enough space
  # at the top of the columns to allow for all crates to be in one stack,
  # theoretically.
  total_crates <- sum(!is.na(crate_stacks))
  row_padding  <- total_crates - nrow(crate_stacks)
  extra_rows   <- matrix(NA_character_,
                         nrow = row_padding,
                         ncol = ncol(crate_stacks))

  # Return a matrix of crate stacks with extra space at the top.
  rbind(extra_rows, crate_stacks)
}


# Parse the instructions from the input file into a more usable data structure
parse_instructions <- function(instructions) {
  # For each instruction, extract the numbers and return each line as a
  # vector of the three numbers, in a list.
  (instructions
   |> strsplit("")
   |> lapply(\(x) grep("\\d+", x, value = TRUE))
   |> lapply(as.numeric))
}


# Move one crate from one stack to another
move_crate <- function(crate_stacks, from_col, to_col) {
  # Identify which row in the `crate_stacks` matrix contains the top
  # letter for each column involved.
  from_row <- (crate_stacks[, from_col] |> is.na() |> which() |> max()) + 1
  to_row   <- (crate_stacks[, to_col] |> is.na() |> which() |> max())

  # Move the top letter from the `from` stack to the `to` stack
  crate_stacks[to_row, to_col]     <- crate_stacks[from_row, from_col]
  crate_stacks[from_row, from_col] <- NA_character_

  # Return the updated crate stacks matrix
  crate_stacks
}


# Process a single instruction, moving as many crates as specified
process_instruction <- function(crate_stacks, instruction) {
  # Get the numbers from the instruction
  times    <- instruction[1]
  from_col <- instruction[2]
  to_col   <- instruction[3]

  # Move as many crates as specified
  for (n in seq_len(times)) {
    crate_stacks <- move_crate(crate_stacks, from_col, to_col)
  }

  # Return the update crate stacks
  crate_stacks
}


# Process all the instructions, one after the other
move_crates <- function(crate_stacks, instructions) {
  # Follow each instruction, updating the crate stacks each time
  for (instruction in instructions) {
    crate_stacks <- process_instruction(crate_stacks, instruction)
  }

  # Return the updated crate stacks
  crate_stacks
}


# Solve the puzzle! Read the file at `input_path`, move the crates around
# then return a string consisting of the top letter in each stack.
solve <- function(input_path) {
  (input
   <- input_path
   |> readLines()
   |> parse_input())

  (move_crates(input[[1]], input[[2]])
   |> apply(2, \(x) head(na.omit(x), 1))
   |> paste(collapse = ""))
}
