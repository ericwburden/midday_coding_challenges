# Functions --------------------------------------------------------------------

## Initializing ----

#' Returns an x/y coordinate pair, ensures type safety
coordinate <- function(x, y) {
  if (!is.numeric(c(x, y))) { stop('Coordinates should be numeric') }
  c(x = unname(x), y = unname(y))
}

#' Checks to be sure a coordinate is on the board
coord_on_board <- function(coord, board) {
  dims <- dim(board)
  x <- coord['x']
  y <- coord['y']
  x <= dims[2] & x > 0 & y <= dims[1] & y > 0
}

#' Given a coordinate and board, return the value at that coordinate
status <- function(coord, board) {
  board[coord['y'], coord['x']]
}

#' Initializes the game board with a size of `d` x `d` with an initial set
#' of live cells given by `live_coords`
create_board <- function(d, live_coords) {
  board <- matrix(rep('.', d*d), ncol = d, byrow = T)
  for (coord in live_coords) {
    if (!coord_on_board(coord, board)) {
      stop(glue::glue('{coord} is not on the board'))
    }
    board[coord['y'], coord['x']] <- '#'
  }
  board
}

## Progressing ----

#' Get the coordinates on the board for the neighbors of a given cell
neighbor_coordinates <- function(coord, board) {
  neighbors <- list(
    coordinate(coord['x'] - 1, coord['y'] + 1),
    coordinate(coord['x'] - 1, coord['y']),
    coordinate(coord['x'] - 1, coord['y'] - 1),
    coordinate(coord['x']    , coord['y'] - 1),
    coordinate(coord['x'] + 1, coord['y'] - 1),
    coordinate(coord['x'] + 1, coord['y']),
    coordinate(coord['x'] + 1, coord['y'] + 1),
    coordinate(coord['x']    , coord['y'] + 1)
  )
  on_board <- sapply(neighbors, coord_on_board, board)
  neighbors[on_board]
}

#' Count the number of neighboring cells that are 'alive' given a coordinate
count_live_neighbors <- function(coord, board) {
  neighbors <- neighbor_coordinates(coord, board)
  neighbor_vals <- sapply(neighbors, status, board)
  sum(neighbor_vals == '#')
}

#' Determine the state of a given cell based on the current board configuration
advance_cell_operation <- function(x, y, board) {
  coord <- coordinate(x, y)
  cell_state <- status(coord, board)
  live_neighbors <- count_live_neighbors(coord, board)
  if (cell_state == '#' & live_neighbors < 2) { cell_state <- '.' }
  if (cell_state == '#' & live_neighbors > 3) { cell_state <- '.' }
  if (cell_state == '.' & live_neighbors == 3) { cell_state <- '#' }
  cell_state
}

#' Advance the board to the next state
advance_board <- function(board) {
  board_width <- dim(board)[2]
  board_height <- dim(board)[1]
  new_state_list <- mapply(
    advance_cell_operation,
    row(board),
    col(board),
    MoreArgs = list(board = board)
  )
  matrix(new_state_list, nrow = board_height, byrow = T)
}

## Display ----

#' Colors by state
pretty_state <- function(state) {
  if (state == '.') {
    crayon::silver(state)
  } else {
    crayon::blue(state)
  }
}

pretty_board <- function(board) {
  apply(board, 1:2, pretty_state)
}

#' Pretty print the board
cat_board <- function(board) {
  rows <- dim(board)[1]
  cols <- dim(board)[2]
  pb <- pretty_board(board)
  cat("\014")
  sapply(seq(rows), function(x) { cat(pb[1:cols, x], '\n') } )
  cat('\n')
}

#' Pretty print the board `n` times with a delay of `delay` seconds in
#' between, advancing the board each time
display_n_frames <- function(board, n, delay = 0.5) {
  for (i in seq(n)) {
    cat_board(board)
    board <- advance_board(board)
    Sys.sleep(delay)
  }
  cat_board(board)
}

# Examples ---------------------------------------------------------------------

#' Display a 'pulsar'
cogl_pulsar <- function(reps = 100) {
  board_size <- 15
  starting_coords <- list(
    coordinate(4 , 2) , coordinate(5 , 2) , coordinate(6 , 2) ,
    coordinate(10, 2) , coordinate(11, 2) , coordinate(12, 2) ,
    coordinate(2 , 4) , coordinate(2 , 5) , coordinate(2 , 6) ,
    coordinate(7 , 4) , coordinate(7 , 5) , coordinate(7 , 6) ,
    coordinate(9 , 4) , coordinate(9 , 5) , coordinate(9 , 6) ,
    coordinate(14, 4) , coordinate(14, 5) , coordinate(14, 6) ,
    coordinate(4 , 7) , coordinate(5 , 7) , coordinate(6 , 7) ,
    coordinate(10, 7) , coordinate(11, 7) , coordinate(12, 7) ,
    coordinate(4 , 9) , coordinate(5 , 9) , coordinate(6 , 9) ,
    coordinate(10, 9) , coordinate(11, 9) , coordinate(12, 9) ,
    coordinate(2 , 10), coordinate(2 , 11), coordinate(2 , 12),
    coordinate(7 , 10), coordinate(7 , 11), coordinate(7 , 12),
    coordinate(9 , 10), coordinate(9 , 11), coordinate(9 , 12),
    coordinate(14, 10), coordinate(14, 11), coordinate(14, 12),
    coordinate(4 , 14), coordinate(5 , 14), coordinate(6 , 14),
    coordinate(10, 14), coordinate(11, 14), coordinate(12, 14)
  )
  board <- create_board(board_size, starting_coords)
  display_n_frames(board, reps)
}

random_seed <- function(board_size, pct_alive, reps = 100) {
  randomizer <- matrix(runif(board_size*board_size) <= pct_alive, nrow = board_size)
  board <- apply(randomizer, 1:2, function(x) { ifelse(x, '#', '.') })
  display_n_frames(board, reps)
}
