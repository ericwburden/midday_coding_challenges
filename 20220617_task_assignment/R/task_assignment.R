assign_tasks <- function(k, tasks) {
  # Vector of indexes from `tasks` in sort order. Subtract 1 because
  # R is 1-indexed.
  indices    <- order(tasks) - 1

  small_half <- indices[1:k]
  large_half <- indices[(k + 1):(k * 2)]
  mapply(c, small_half, rev(large_half), SIMPLIFY = F)
}

# Tests -----------------------------------------------------------------------

require(testthat)

sort_answer <- function(x) {
  inner_sorted_x <- lapply(x, sort)
  inner_sorted_x[order(sapply(inner_sorted_x, "[[", 1))]
}

compare_answers <- function(x, y) {
  sorted_x <- sort_answer(x)
  sorted_y <- sort_answer(y)
  all(mapply(`==`, sorted_x, sorted_y))
}

expect_answer_match <- function(x, y) {
  x <- sort_answer(x) |> paste()
  y <- sort_answer(y) |> paste()
  testthat::expect_equal(x, y)
}

test_that("Correctly solves example 1", {
  possible_answers <- list(
    list(c(0, 2), c(4, 5), c(1, 3)),
    list(c(2, 4), c(0, 5), c(1, 3))
  )

  tasks   <- c(1, 3, 5, 3, 1, 4)
  answer  <- assign_tasks(3, tasks)
  matches <- sapply(possible_answers, compare_answers, answer)

  if (!any(matches)) lapply(possible_answers, expect_answer_match, answer)
  succeed("A matching answer was found!")
})

test_that("Correctly solves example 2", {
  expected <- list(c(0, 7), c(1, 6), c(2, 5), c(3, 4))
  tasks    <- c(1, 2, 3, 4, 5, 6, 7, 8)
  answer   <- assign_tasks(4, tasks)
  expect_answer_match(expected, answer)
})

test_that("Correctly solves example 3", {
  expected <- list(c(0, 1))
  tasks    <- c(3, 5)
  answer   <- assign_tasks(1, tasks)
  expect_answer_match(expected, answer)
})

test_that("Correctly solves example 4", {
  expected <- list(
    c(1, 12), c(0, 5),  c(2, 6), c(3, 8),
    c(4, 9),  c(10, 7), c(11, 13)
  )
  tasks    <- c(2, 1, 3, 4, 5, 13, 12, 9, 11, 10, 6, 7, 14, 8)
  answer   <- assign_tasks(7, tasks)
  expect_answer_match(expected, answer)
})

test_that("Correctly solves example 5", {
  expected <- list(c(4, 5), c(3, 0), c(2, 1))
  tasks    <- c(87, 65, 43, 32, 31, 320)
  answer   <- assign_tasks(3, tasks)
  expect_answer_match(expected, answer)
})

test_that("Correctly solves example 6", {
  expected <- list(c(0, 3), c(1, 2))
  tasks    <- c(1, 8, 9, 10)
  answer   <- assign_tasks(2, tasks)
  expect_answer_match(expected, answer)
})
