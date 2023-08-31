VOWELS <- c("a", "e", "i", "o", "u")

rotate <- function(vec, n) {
  n <- n %% length(vec)
  c(tail(vec, n), head(vec, -n))
}

shift_vowels <- function(string, n) {
  letters    <- string |> strsplit("") |> unlist()
  vowel_idxs <- which(tolower(letters) %in% VOWELS)
  rot_idxs   <- rotate(vowel_idxs, n)
  letters[vowel_idxs] <- letters[rot_idxs]
  paste(letters, collapse = "")
}


library(testthat)

test_that("We can shift vowel locations in the example", {
  input <- "This is a test!"
  expect_equal(shift_vowels(input, 1), "Thes is i tast!")
  expect_equal(shift_vowels(input, 3), "This as e tist!")
})
