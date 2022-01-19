(input
  <- readLines("input.txt")
  |> as.numeric())

part_one <- function(input) {
  head <- input[1:length(input) - 1]
  tail <- input[2:length(input)]
  sum(tail > head)
}

part_two <- function(input) {
  head <- input[1:length(input) - 1]
  tail <- input[2:length(input)]
  increases <- 0
  for (idx in 4:length(input)) {
    curr_sum <- sum(input[(idx - 2):(idx)])
    prev_sum <- sum(input[(idx - 3):(idx - 1)])
    if (curr_sum > prev_sum) increases <- increases + 1
  }
  increases
}

part_one_start <- Sys.time()
part_one(input)
part_one_end <- Sys.time()

part_two_start <- Sys.time()
part_two(input)
part_two_end <- Sys.time()

print("- Day 01")
print((part_one_end - part_one_start)*1000)
print((part_two_end - part_two_start)*1000)


microbenchmark::microbenchmark(part_one(input))
microbenchmark::microbenchmark(part_two(input))
