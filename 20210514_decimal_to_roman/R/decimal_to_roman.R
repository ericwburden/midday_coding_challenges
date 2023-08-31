require(dplyr)
require(tidyr)
require(tibble)

mappings <- list(
  list(decimal = 1000, roman =  "M"),
  list(decimal =  900, roman = "CM"),
  list(decimal =  500, roman =  "D"),
  list(decimal =  400, roman = "CD"),
  list(decimal =  100, roman =  "C"),
  list(decimal =   90, roman = "XC"),
  list(decimal =   50, roman =  "L"),
  list(decimal =   40, roman = "XL"),
  list(decimal =   10, roman =  "X"),
  list(decimal =    9, roman = "IX"),
  list(decimal =    5, roman =  "V"),
  list(decimal =    4, roman = "IV"),
  list(decimal =    1, roman =  "I")
)

decimal_to_roman <- function(number) {
  out   <- ""
  iters <- 0
  for (mapping in mappings) {
    reps <- number %/% mapping$decimal
    out <- paste0(out, rep(mapping$roman, reps))
    number <- number - (mapping$decimal * reps)
    iters <- iters + 1
  }

  tibble(roman = out, iters = iters)
}

(test_frame
  <- tibble(input = seq(1, 3999))
  |> mutate(results = lapply(input, decimal_to_roman))
  |> unnest(results))
