#' Zip It, If You Can?
#' Given an array of women and an array of men, either:  
#' - Return "sizes don't match" if the two arrays have different sizes.
#' - If the sizes match, return an array of pairs, with the first woman 
#'   paired with the first man, second woman paired with the second man, etc.
#'   
#' Examples:
#' zipIt(["Elise", "Mary"], ["John", "Rick"]) ➞ [["Elise", "John"], ["Mary", "Rick"]]
#' zipIt(["Ana", "Amy", "Lisa"], ["Bob", "Josh"]) ➞ "sizes don't match" 
#' zipIt(["Ana", "Amy", "Lisa"], ["Bob", "Josh", "Tim"]) ➞ [["Ana", "Bob"], ["Amy", "Josh"],["Lisa", "Tim"]]

zip_it <- function(...) {
  #' Check the length of each argument to ensure they're all the same
  unique_lengths <- unique(sapply(list(...), length))
  if (length(unique_lengths) > 1) { return("sizes don\'t match") }
  
  #' Check to be sure all vectors are the same type
  unique_types <- unique(sapply(list(...), typeof))
  if (length(unique_types) > 1) { stop("All arguments must be of the same type") }
  
  #' Combine the vectors
  mapply(c, ..., SIMPLIFY = F, USE.NAMES = F)
}

#' Pretty printing for test cases
pprint <- function(c){
  paste0(
    '\n\t- input: [', paste(c$input, collapse = ', '),']',
    '\n\t- output: [', paste(c$output, collapse = ', '),']'
  )
}

#' List of valid test cases
test_cases <- list(
  list(
    input = list(c("Elise", "Mary"), c("John", "Rick")),
    output = list(c("Elise", "John"), c("Mary", "Rick"))
  ),
  list(
    input = list(c("Ana", "Amy", "Lisa"), c("Bob", "Josh")),
    output = "sizes don't match"
  ),
  list(
    input = list(c("Ana", "Amy", "Lisa"), c("Bob", "Josh", "Tim")),
    output = list(c("Ana", "Bob"), c("Amy", "Josh"), c("Lisa", "Tim"))
  ),
  list(
    input = list(c("Ana", "Amy", "Lisa"), c("Bob", "Josh", "Tim"), c("Car", "Baby", "House")),
    output = list(c("Ana", "Bob", "Car"), c("Amy", "Josh", "Baby"), c("Lisa", "Tim", "House"))
  )
)

#' Test runner function
run_test <- function(c, f) {
  result <- do.call(f, c$input)
  check <- mapply('==', result, c$output, SIMPLIFY = F)
  pass <- all(sapply(check, all))
  msg <- if (pass) {
    crayon::green(paste(substitute(f), 'PASSED test case:', pprint(c)))
  } else {
    crayon::red(paste('*', substitute(f), 'FAILED test case:', pprint(c)))
  }
  cat(msg, '\n\n')
}

#' Run all tests
sapply(test_cases, run_test, zip_it)


#' Results
#' zip_it PASSED test case: 
#'   - input: [c("Elise", "Mary"), c("John", "Rick")]
#'   - output: [c("Elise", "John"), c("Mary", "Rick")]
#'
#' zip_it PASSED test case: 
#'   - input: [c("Ana", "Amy", "Lisa"), c("Bob", "Josh")]
#'   - output: [sizes don't match] 
#'   
#'  zip_it PASSED test case: 
#'  	- input: [c("Ana", "Amy", "Lisa"), c("Bob", "Josh", "Tim")]
#'  	- output: [c("Ana", "Bob"), c("Amy", "Josh"), c("Lisa", "Tim")]
#'  	
#'  zip_it PASSED test case: 
#'    - input: [c("Ana", "Amy", "Lisa"), c("Bob", "Josh", "Tim"), c("Car", "Baby", "House")]
#'    - output: [c("Ana", "Bob", "Car"), c("Amy", "Josh", "Baby"), c("Lisa", "Tim", "House")] 

