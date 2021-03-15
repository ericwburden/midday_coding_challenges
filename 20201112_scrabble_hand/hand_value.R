#' **Scrabble Hand**
#' Given an array of scrabble tiles, create a function that outputs the maximum 
#' possible score a player can achieve by summing up the total number of points 
#' for all the tiles in their hand. Each hand contains 7 scrabble tiles.
#' 
#' **Examples**
#' maximumScore([
#'    { tile: "N", score: 1 } , { tile: "K", score: 5 },
#'    { tile: "Z", score: 10 }, { tile: "X", score: 8 },
#'    { tile: "D", score: 2 } , { tile: "A", score: 1 },
#'    { tile: "E", score: 1 }
#' ]) ➞ 28
#' 
#' maximumScore([
#'    { tile: "B", score: 2 }, { tile: "V", score: 4 },
#'    { tile: "F", score: 4 }, { tile: "U", score: 1 },
#'    { tile: "D", score: 2 }, { tile: "O", score: 1 },
#'    { tile: "U", score: 1 }
#' ]) ➞ 15

library(testthat)

# Functions --------------------------------------------------------------------

#' Source of Scrabble tile names and values
#'
#' @return a data.frame with a `tile` column of uppercase letters and a `score`
#' column of values for each tile.
scrabble_data <- function() {
  
  #' It's possible to construct a data.frame directly, but I find this syntax
  #' of building up a list of named lists and converting to a data.frame to
  #' be a lot more readable and easier to bug-fix.
  values <- list(
    list(tile = "A", score = 1),
    list(tile = "B", score = 3),
    list(tile = "C", score = 3),
    list(tile = "D", score = 2),
    list(tile = "E", score = 1),
    list(tile = "F", score = 4),
    list(tile = "G", score = 2),
    list(tile = "H", score = 4),
    list(tile = "I", score = 1),
    list(tile = "J", score = 8),
    list(tile = "K", score = 5),
    list(tile = "L", score = 1),
    list(tile = "M", score = 3),
    list(tile = "N", score = 1),
    list(tile = "O", score = 1),
    list(tile = "P", score = 3),
    list(tile = "Q", score = 10),
    list(tile = "R", score = 1),
    list(tile = "S", score = 1),
    list(tile = "T", score = 1),
    list(tile = "U", score = 1),
    list(tile = "V", score = 4),
    list(tile = "W", score = 4),
    list(tile = "X", score = 8),
    list(tile = "Y", score = 4),
    list(tile = "Z", score = 10)
  )

  do.call(rbind, lapply(values, as.data.frame))
}

#' Calculate the value of a Scrabble hand
#' 
#' @description Given a 7-long list of letters, return the total value of that
#' list in Scrabble score values.
#'
#' @param ... 7 single letter character arguments
#' @return a numeric maximum score for that hand
#' @examples hand_value('A', 'C', 'J', 'K', 'O', 'Z', 'E')
hand_value <- function(..., data_source) {
  # data_source includes a table of letter/score value pairs
  if (missing(data_source)) data_source <- scrabble_data()
  
  # convert arguments to a single, uppercase vector and check to ensure
  # correct arguments have been supplied
  tiles_in_hand <- toupper(c(...))
  if (!(length(tiles_in_hand) == 7)) stop('A valid hand has only 7 tiles.')
  if (any(nchar(tiles_in_hand) > 1)) stop('Each argument must be a single letter.')
  if (!all(tiles_in_hand %in% data_source$tile)) stop('Each argument must be a character.')
  
  # Literally the entire calculation logic
  sum(data_source$score[data_source$tile %in% tiles_in_hand])
}

# Tests ------------------------------------------------------------------------

test_that("correct hand value is calculated", {
  expect_equal(hand_value('N', 'K', 'Z', 'X', 'D', 'A', 'E'), 28)
  expect_equal(hand_value('B', 'V', 'F', 'U', 'D', 'O', 'U'), 15)
})

test_that("error messages are correctly raised", {
  e1 <- 'A valid hand has only 7 tiles.'
  e2 <- 'Each argument must be a single letter.'
  e3 <- 'Each argument must be a character.'
  
  expect_error(hand_value('N', 'K', 'Z', 'X', 'D', 'A'), e1)
  expect_error(hand_value('N', 'K', 'Z', 'X', 'D', 'A', 'E', 'B'), e1)
  expect_error(hand_value('N', 'K', 'Z', 'X', 'D', 'Apple', 'E'), e2)
  expect_error(hand_value('N', 'K', 'Z', 'X', 'D', 5, 'E'), e3)
})
