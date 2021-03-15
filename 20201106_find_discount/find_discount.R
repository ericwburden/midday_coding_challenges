#' *Find the Discount*
#' Create a function that takes two arguments: the final price and the discount 
#' percentage as integers and returns the final price after the discount.
#' 
#' *Examples*
#'   dis(1500, 50) ➞ 750
#'   dis(89, 20) ➞ 71.2
#'   dis(100, 75) ➞ 25
#'   
#' *Notes*
#' Your answer should be rounded to two decimal places.

#' Create a set of generic functions to handle inputs of different types, 
#' which can parse character strings to numbers
type <- function(x) {
  UseMethod('type', x)
}

type.numeric <- function(x) x
type.character <- function(x) {
  pre_num_chars <- regmatches(x, regexpr("([\\d,]+\\.?\\d*)", x, perl = T))
  num_chars <- gsub(',', '', pre_num_chars)
  as.numeric(num_chars)
}

#' Calculate discount, accepts arguments as numbers or strings
discount <- function(list_price, discount_rate) {
  list_price <- type(list_price)        # Ensure numeric type
  discount_rate <- type(discount_rate)  # Ensure numeric type
  final_pct <- 1 - (discount_rate/100)  # Invert percent off for calculation
  sprintf("$%.2f", list_price * final_pct)  # Return formatted result
}

#' Pretty printing for test cases
pprint <- function(c){
  paste0(
    'input: list_price: ', c$input[[1]], 
    ', discount_rate: ', c$input[[2]],
    '; output: ', c$output, '\n'
  )
}

#' List of valid test cases
test_cases <- list(
  list(input = list(     1500,    50), output = '$750.00'),
  list(input = list(       89,    20), output = '$71.20'),
  list(input = list(      100,    75), output = '$25.00'),
  list(input = list(   '$100',    75), output = '$25.00'),
  list(input = list(      100, '75%'), output = '$25.00'),
  list(input = list('$100.00', '75%'), output = '$25.00')
)

#' Test runner function
run_test <- function(c, f) {
  pass <- do.call(f, c$input) == c$output
  msg <- if (pass) {
    crayon::green(paste(substitute(f), 'PASSED test case:', pprint(c)))
  } else {
    crayon::red(paste('*', substitute(f), 'FAILED test case:', pprint(c)))
  }
  cat(msg, '\n')
}

#' Run all tests
sapply(test_cases, run_test, discount)

#' discount PASSED test case: input: list_price: 1500, discount_rate: 50; output: $750.00
#' discount PASSED test case: input: list_price: 89, discount_rate: 20; output: $71.20
#' discount PASSED test case: input: list_price: 100, discount_rate: 75; output: $25.00
#' discount PASSED test case: input: list_price: $100, discount_rate: 75; output: $25.00
#' discount PASSED test case: input: list_price: 100, discount_rate: 75%; output: $25.00
#' discount PASSED test case: input: list_price: $100.00, discount_rate: 75%; output: $25.00
 

#' One neat byproduct of this implementation is the ability to chain calls to 
#' `discount()` like so: `discount(10000, 10) %>% discount(10) -> "$8100.00"`
