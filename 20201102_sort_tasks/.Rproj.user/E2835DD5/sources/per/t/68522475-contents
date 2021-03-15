#' Sort Data
#' You are given a list of longest run times of different tasks (`data.csv`). 
#' You need to order the tasks from longest running to shortest running so that 
#' you can prioritize efficiency improvements.

#' Provide a set of generic methods to handle fetching the dataframe from
#' either a `*.csv` file or from a `data.frame`. This is important for 
#' testing, so I can mock the `data.csv` file.
get_df <- function(x, ...) {
  UseMethod('get_df', x)
}

get_df.data.frame <- function(x, ...) x
get_df.character <- function(x, ...) {
  read.csv(
    x, 
    header = F, 
    col.names = c('job', 'run_time'), 
    stringsAsFactors = F
  )
}

#' The sorting function. Takes either a .csv file or a data frame, 
#' sorts by `run_time`, writes out the sorted dataframe (if the input was
#' a data file), then returns the sorted dataframe.
sort_tasks <- function(x) {
  df <- get_df(x)  # Read in the `*.csv` file and sort
  sorted_df <- df[order(df$run_time, decreasing = T), ]
  
  #' Write out the results to a file, `sorted_{filename}.csv`, if the input
  #' was a file path
  if (class(x) == 'character') {
    filename <- basename(x)
    sorted_filename <- paste('sorted', filename, sep = '_')
    write.csv(sorted_df, sorted_filename, row.names = F)
  }
 
  sorted_df   # Return the sorted dataframe
}

#' Pretty printing for test cases
pprint <- function(c){
  paste0(
    'input: jobs = [', paste(c$input$job, collapse = ', '),'], ',
    'run_time = [', paste(c$input$run_time, collapse = ', '),']; ',
    'output: ', c$output, '\n'
  )
}

#' List of valid test cases
test_cases <- list(
  list(
    input = data.frame(job = c('job1', 'job2', 'job3'), run_time = c(1, 2, 3)),
    output = 'job3'
  ),
  list(
    input = data.frame(job = c('job1', 'job2', 'job3'), run_time = c(1, 3, 2)),
    output = 'job2'
  ),
  list(
    input = data.frame(job = c('job1', 'job2', 'job3'), run_time = c(3, 2, 1)),
    output = 'job1'
  )
)

#' Test runner function
run_test <- function(c, f) {
  pass <- f(c$input)$job[[1]] == c$output
  msg <- if (pass) {
    crayon::green(paste(substitute(f), 'PASSED test case:', pprint(c)))
  } else {
    crayon::red(paste('*', substitute(f), 'FAILED test case:', pprint(c)))
  }
  cat(msg, '\n')
}

#' Run all tests
sapply(test_cases, run_test, sort_tasks)
