#' Dashed Vowels: Create a function that takes a string and returns dashes on 
#' the left and right side of every vowel (a e i o u).
#' 
#' Examples
#' - dashed("Edabit") ➞ "-E-d-a-b-i-t"
#' - dashed("Carpe Diem") ➞ "C-a-rp-e- D-i--e-m"
#' - dashed("Fight for your right to party!") ➞ "F-i-ght f-o-r y-o--u-r r-i-ght t-o- p-a-rty!"
#' 
#' Notes
#' - A string can contain uppercase and lowercase vowels.
#' - Y is not considered a vowel.

dashed <- function(input_str) {
  #' Replace each vowel listed in the regex with itself surrounded by dashes
  #' in the input string
  gsub("([AEIOUaeiou])", "-\\1-", input_str)
}

test_cases <- list(
  list(input = "Edabit",                         output = "-E-d-a-b-i-t"),
  list(input = "Carpe Diem",                     output = "C-a-rp-e- D-i--e-m"),
  list(input = "Fight for your right to party!", output = "F-i-ght f-o-r y-o--u-r r-i-ght t-o- p-a-rty!")
)


run_test <- function(case) {
  dashed(case$input) == case$output
}

all(sapply(test_cases, run_test)) # TRUE

