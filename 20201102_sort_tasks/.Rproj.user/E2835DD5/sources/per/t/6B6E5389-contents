#' Given a list of numbers and a number `k`, return whether any two numbers
#' from the list add up to `k`. For example, given `[10, 15, 3, 7]` and `k`
#' of `17`, return `TRUE` since `10 + 7` is `17`.
#' Bonus: Can you do this in one pass?
#' 
#' Examples:
#' - l = `[10, 15, 3, 7]`, k = `17` -> `TRUE`
#' - l = `[5, 4, 9, 21, 16]`, k = `3` -> `FALSE`
#' - l = `[4, 6, 8, 10, 6]`, k = `8` -> `FALSE`
#' - l = `[4, 6, 8, 10, 4]`, k = `8` -> `TRUE`

addends_in_list <- function(l, k) {
  #' The strategy is to construct a new list, `diffs`, subtracting every 
  #' element in list `l` from `k`, then checking whether any item in `diffs`
  #' is in `l`. There is an edge case where, if the item in list `l` is exactly
  #' `k/2`, then it will satisfy this check on its own. If `k/2` is in `l` 
  #' twice, then return true, otherwise remove `k/2` from `l` and proceed.
  if (length(l[l == k/2]) > 1) { return(TRUE) }
  diffs <- k - l[l != k/2]
  any(sapply(diffs, '%in%', l))
}

pprint <- function(c){
  paste0(
    'input: l = [', paste(c$input$l, collapse = ', '),'], k = ', c$input$k,
    '; output: ', c$output
  )
}

test_cases <- list(
  list(input = list(l = c(10, 15, 3, 7),    k = 17), output = TRUE),
  list(input = list(l = c(5, 4, 9, 21, 16), k = 3),  output = FALSE),
  list(input = list(l = c(4, 6, 8, 10, 6),  k = 8),  output = FALSE),
  list(input = list(l = c(4, 6, 8, 10, 4),  k = 8),  output = TRUE)
)

run_test <- function(c, f) {
  pass <- do.call(f, c$input) == c$output
  msg <- if (pass) {
    crayon::green(paste(substitute(f), 'PASSED test case:', pprint(c)))
  } else {
    crayon::red(paste('*', substitute(f), 'FAILED test case:', pprint(c)))
  }
  cat(msg, '\n')
}

sapply(test_cases, run_test, addends_in_list)
