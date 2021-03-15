#' **Closing Brackets** 
#' 
#' Given a string containing round, curly, and square open and closing 
#' brackets, return whether the brackets are balanced (well-formed). The string 
#' may also contain other characters. Raise an error (or otherwise alert the 
#' user) if the string doesn't contain any round, curly, or square brackets.
#' 
#' *Example*
#' 
#'   closed_brackets("([])[]({})") -> TRUE
#'   closed_brackets("(([[))]]") -> FALSE
#'   closed_brackets("(x + (y * z))") -> TRUE
#'   closed_brackets("no brackets") -> Raise an error

library(testthat)

# Functions --------------------------------------------------------------------

#' Pre-formats the strings and checks to be sure all the strings have brackets
closed_brackets <- function(s) {
  char_vecs <- strsplit(s, '')  # Split each string into a character vector
  
  # Remove all the non-bracket characters from the character vectors
  bracket_list <- lapply(char_vecs, function(x) x[grep("\\[|\\]|\\(|\\)|\\{|\\}", x)])
  only_brackets <- lapply(bracket_list, paste, collapse = "")
  
  # If any of the `only_brackets` strings is empty, raise an error
  if (any(sapply(only_brackets, "==", ""))) {
    stop("`closed_brackets` cannot accept arguments without any brackets.")
  }
  
  # For each `only_brackets` string, determine if the string is well-formed
  sapply(only_brackets, check_brackets)
}


# Function to check strings containing only brackets to ensure every opening
# bracket has a corresponding closing bracket
check_brackets <- function(bs) {
  # If all chars are removed, the string was well-formed
  if (nchar(bs) == 0) { return(TRUE) }  
  
  # Remove all the matching pairs
  pairs_out <- gsub("\\[\\]|\\(\\)|\\{\\}", '', bs)
  
  # If no matching pairs found, the string was not well-formed
  if (nchar(bs) == nchar(pairs_out)) { return(FALSE) }
  
  check_brackets(pairs_out)  # Try again
}

# Tests ------------------------------------------------------------------------

test_that("`closed_brackets` returns expected results", {
  s1 <- "([])[]({})"
  s2 <- "(x + (y * z))"
  s3 <- "({hi}[hello])[({bye})]"
  s4 <- "(([[))]]"
  s5 <- "no brackets"
  
  expect_equal(closed_brackets(s1), TRUE)
  expect_equal(closed_brackets(c(s1, s2)), c(TRUE, TRUE))
  expect_equal(closed_brackets(s4), FALSE)
  expect_equal(closed_brackets(c(s1, s2, s3, s4)), c(TRUE, TRUE, TRUE, FALSE))
  expect_error(closed_brackets(s5))
  expect_error(closed_bracets(c(s1, s4, s5)))
  
})

