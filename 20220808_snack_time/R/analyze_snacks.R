library(tibble)
library(dplyr)
library(stringr)

(best_snack_list
  <- readLines("snack_record.txt")
  |> str_subset("\\d+ \\w+")
  |> str_extract("\\d+ \\w+")
  |> str_split(" ")
  |> lapply(setNames, c("num", "snack"))
  |> bind_rows()
  |> group_by(snack)
  |> mutate(across(num, as.numeric))
  |> summarise(total = sum(num))
  |> arrange(desc(total)))


