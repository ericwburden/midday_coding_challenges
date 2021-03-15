library(tidyverse)
library(future)
library(furrr)

plan(multisession)
options(future.rng.onMisuse = "ignore")

# Functions --------------------------------------------------------------------

#' Produces a vector that meets the requirements given
valid_vector <- function(l = 1000) {
  vec <- runif(l, -l, l)
  whole_vec <- round(vec)
  sorted_vec <- sort(whole_vec)
  unique(sorted_vec)
}

#' Ensures a given vector has a magic index
force_mi <- function(v) {
  gte_mi <- v >= seq(length(v))
  mi <- min(which(gte_mi))
  v[mi] <- mi
  v
}

# A startup function for the recursion, provides default values for the first
# round
magic_index <- function(v, part = 0.5) {
  magic_index_recurse(v, 1, length(v), part, 0)
}

#' The workhorse. Checks the value at the test index and adjusts the range to 
#' search each round. If the result is found, it bubbles back up through the
#' call stack. Also tracks the number of rounds
magic_index_recurse <- function(v, low, high, part, rounds, result) {
  if (!missing(result)) { return(list(rounds = rounds, result = result)) }

  #' Use the partition value to identify the next index to check. For a 
  #' partition value of 0.5, it will be the index halfway between low
  #' and high
  i <- low + round((high - low) * part)

  # Check to see if it's the magic index, if so, set the result to the index.
  # If it's not the magic index and the search range is down to one value,
  # set the result to -1
  if (v[i] == i) { result <- i } else if (low >= high) { result <- -1 }
  
  #' If the value at the checked index is less than the index, the new
  #' range should be low...(i - 1). If the value at the checked index is greater
  #' than the index, the new range should be (i + 1)...high.
  if (i < v[i]) { high <- i - 1 }
  if (i > v[i]) { low <- i + 1 }
  
  # Try again
  rounds <- rounds + 1
  magic_index_recurse(v, low, high, part, rounds, result)
}

# Speed Tests! -----------------------------------------------------------------

#' So, given the difficulty of measuring timing (like, what happens if a 
#' background tasks runs during the test?), I want to track the number of
#' rounds it takes to find the answer, on average, for different partition
#' values. My gut feeling is that 0.75 should give the best performance on
#' small vectors where the 'magic index' exists, given that it starts searching
#' closer to where the answer is likely to lie. For larger vectors, this will 
#' likely wash out very quickly, and may even yield poorer performance. So, 
#' let's test this hypothesis!

#' Given a single vector and partition, find the magic index (or determine 
#' there isn't one) and report back the number of rounds it took
test_run_rounds <- function(v, part, pb) {
  r <- magic_index(v, part)
  r$part <- part
  r
}

#' Given a range of partitions and a vector, find the magic index (or determine
#' there isn't one) for each partition and report the results as a table
test_partition_range <- function(v, part_range, pb) {
  r <- map(part_range, ~ test_run_rounds(v, .x))
  bind_rows(map(r, as_tibble))
}

#' Given a range of partitions, a magnitude, and a number of repetitions, find
#' the magic index of a vector `l` items long, `reps` times, and report
#' the average number of rounds needed to find the magic index (or determine
#' there isn't one) along with the length and number of reps
test_length <- function(l, reps, part_range, pb) {
  vec_seeds <- rep(l, reps)
  r <- future_map(vec_seeds, ~ test_partition_range(valid_vector(.x), part_range))
  
  bind_rows(r) %>% 
    mutate(found_mi = result != -1) %>% 
    group_by(found_mi, part) %>% 
    summarise(mean_rounds = mean(rounds)) %>% 
    mutate(length = l, reps = reps)
}


