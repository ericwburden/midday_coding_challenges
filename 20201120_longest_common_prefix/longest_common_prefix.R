#' **Longest Common Prefix**
#' 
#' Write a function to find the longest common prefix string among an array of 
#' strings.
#' If there is no common prefix, return an empty string "".
#' 
#' Example 1:
#'   Input: strs = ["flower","flow","flight"]
#'   Output: "fl"
#'   
#' Example 2:
#'   Input: strs = ["dog","racecar","car"]
#'   Output: ""
#'   Explanation: There is no common prefix among the input strings.

library(testthat)

# Functions --------------------------------------------------------------------

#' Returns true if all passed arguments are equivalent
all_equal <- function(...) {
  args <- list(...)
  if (length(args) == 0) { stop('all_equal must be called with arguments') }
  length(unique(args)) == 1
}

#' Accepts any number of string arguments and returns the longest common
#' prefix among all strings. Any vector or list arguments will be unpacked
#' to a single character vector for comparison.
longest_common_prefix <- function(...) {
  args <- list(...)
  
  #' Convert all arguments to a single list of vectors, where each vector
  #' contains all the individual characters from each string
  char_vecs <- strsplit(as.character(unlist(args)), '')
  
  #' Normalize all the vectors in the list to be the length of the shortest
  #' string, only exists to keep `mapply()` from throwing a warning about
  #' mismatched argument lengths
  char_vecs <- lapply(char_vecs, '[', seq(min(lengths(char_vecs))))

  #' Attach the `all_equal` function to the char_vecs list, will be parsed by
  #' do.call as an additional named argument to the `mapply` function
  char_vecs$FUN <- all_equal
  
  #' So, this passes all the items from `char_vecs` to `mapply` as individual
  #' arguments. This is why `FUN` was attached to `char_vecs` as a list element.
  #' The end result is that `all_equal()` will be called with the first element
  #' of each character vector, then the second, then the third, etc. The
  #' result of these repeated calls will be a named logical vector where the 
  #' values indicate which indices matched across vectors and the names 
  #' indicate the value at each index of the first vector
  pre_indices <- do.call(mapply, char_vecs)
  
  #' Get the index of the first `FALSE` value in `pre_indices`
  first_false <- min(which(!pre_indices))
  
  #' Collapse the names of the elements in `pre_indices` up to
  #' (but not including) the first `FALSE` value, unless there are
  #' no matches
  if (first_false == 1) { "" } else {
    paste(names(pre_indices[c(1:first_false-1)]), collapse = '')
  }
}

# Tests ------------------------------------------------------------------------

test_that("longest_common_prefix returns expected results", {
  expect_equal(longest_common_prefix("flower","flow","flight"), "fl")
  expect_equal(longest_common_prefix("dog","racecar","car"), "")
  expect_equal(longest_common_prefix(112, 113, 111), "11")
})

test_that("longest_common_prefix correctly handles heterogeneous args", {
  arg1 <- c("flower","flow","flight")
  arg2 <- "flattery"
  arg3 <- c("dog", "flyer", "flippant")
  arg4 <- list("flashy", "flagrant")
  
  expect_equal(longest_common_prefix(arg1, arg2), "fl")
  expect_equal(longest_common_prefix(arg2, arg3), "")
  expect_equal(longest_common_prefix(arg1, arg2, arg4), "fl")
})
