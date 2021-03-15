#' Staircase
#'
#' This is a staircase of size `4`:
#'
#' ```
#'     #
#'    ##
#'   ###
#'  ####
#' ```
#'
#' Its base and height are both equal to . It is drawn using # symbols and
#' spaces. The last line is not preceded by any spaces.
#'
#' Write a program that prints a staircase of size `n`.

library(tidyverse)

#' Helper function, produces a string of `char` characters of length `n`
rep_char <- function(char, n) {
  map(n, ~ rep(char, .x)) %>%
    lapply(paste, collapse = '') %>%
    unlist()
}

#' Helper function, pretty prints a staircase to the console
pprint <- function(s) {
  cat(paste(s, collapse = '\n'), '\n')
}

#' Helper function, applies decorations to a tree
christmas_spirit <- function(h) {
  chars <- unlist(str_split(h, ''))
  if (nchar(str_remove_all(h, ' ')) == 1) {
    return(str_replace(h, '#', crayon::yellow('\U2731')))
  }

  decorations <- character(0)
  for (char in chars) {
    if (char == ' ') {
      decorations <- c(decorations, char)
    } else {
      decorations <- c(decorations, decorate())
    }
  }
  paste(decorations, collapse = '')
}

#' Helper function, produces either a blue or red ornament
bauble <- function() {
  color <- sample(c(crayon::blue, crayon::red), 1)
  color[[1]]('@')
}

#' Helper function, randomly produces a green '#' or a colored ornament
decorate <- function() {
  randomizer <- runif(1)
  if (randomizer < .2) { bauble() } else { crayon::green('#') }
}

#' Produces a staircase and prints to console
hashtag_staircase <- function(n) {
  hashes <- rep_char('#', seq(n))
  staircase <- str_pad(hashes, n)
  pprint(staircase)
}

#' Produces a Christmas tree and prints to console
hashtag_xmas_tree <- function(n) {
  if (n %% 2 == 0) { n <- n + 1 }
  hashes <- rep_char('#', seq(1, n, 2))
  xmas_tree <- str_pad(hashes, n, 'both') %>%
    sapply(christmas_spirit)
  pprint(xmas_tree)
}

rockefeller <- function() {
  cat('\014', '\n')
  hashtag_xmas_tree(87)
  cat('\n', '\n')
}
