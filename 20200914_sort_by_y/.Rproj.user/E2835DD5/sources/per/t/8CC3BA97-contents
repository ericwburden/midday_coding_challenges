#' Sort y value of array from least to greatest! In today's challenge, we will 
#' sort an array of (x, y) coordinate pairs by their y values from least to 
#' greatest. Example:
#' 
#' arr1 = [(10.1, 15.4), (100.6, 9.8), (8.8, 100.2), (15.6, 15.2)]
#' sort_by_y(arr1) -> [(100.6, 9.8), (15.6, 15.2), (10.1, 15.4), (8.8, 100.2)]
#' 
#' arr2 = [(10, 1), (15, 1), (20, 0.5), (11, 12)]
#' sort_by_y(arr2) -> [(20, 0.5), (10, 1), (15, 1), (11, 12)] # both are ok
#' sort_by_y(arr2) -> [(20, 0.5), (15, 1), (10, 1), (11, 12)] # both are ok
#' 
#' We do not care about x values, so if y values are equal order does not 
#' matter!

sort_by_y <- function(arr) {
  #' sapply(arr, "[[", 2) returns a vector of the second item in each pair
  #' order() returns the *indexes* of the input vector in sort order
  arr[order(sapply(arr, '[[', 2))]
}

test_cases <- list(
  list(
    input =  list(c(10.1, 15.4), c(100.6, 9.8), c(8.8, 100.2), c(15.6, 15.2)),
    output = list(c(100.6, 9.8), c(15.6, 15.2), c(10.1, 15.4), c(8.8, 100.2))
  ),
  list(
    input =  list(c(10, 1),  c(15, 1),  c(20, 0.5), c(11, 12)),
    output = list(c(20, 0.5), c(10, 1), c(15, 1),   c(11, 12))
  )
)

run_test <- function(case) {
  all(mapply('==', sort_by_y(case$input), case$output))
}


all(sapply(test_cases, run_test)) # TRUE
