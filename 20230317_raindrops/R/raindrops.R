# The Basic Function ----------------------------------------------------------

rain_factor <- function(factor, sound) {
  result <- list(factor = factor, sound = sound)
  class(result) <- c("rain_factor", class(result))
  result
}

RAIN_FACTORS <- list(
  rain_factor(3, "Pling"),
  rain_factor(5, "Plang"),
  rain_factor(7, "Plong")
)

convert <- function(number, rain_factors = RAIN_FACTORS) {
  rain_sounds <- ""
  for (rf in rain_factors) {
    if (number %% rf$factor > 0) next
    rain_sounds <- paste0(rain_sounds, rf$sound)
  }

  if (nchar(rain_sounds) == 0) as.character(number) else rain_sounds
}


# Evaluate Additional Factors -------------------------------------------------

library(broom)
library(dplyr)
library(ggplot2)
library(purrr)
library(stringr)
library(tibble)
library(tidyr)

rain_factors_table <- function(rain_factors) {
  map(rain_factors, as_tibble) |> bind_rows()
}

rain_sound_count_table <- function(n, rain_factors) {
  numbers <- list()
  for (number in seq(n)) {
    numbers[[number]] <- tibble(number = number,
                                sound = convert(number, rain_factors))
  }

  sound_list <- map(rain_factors, \(rf) rf$sound)
  
  (numbers
    |> bind_rows()
    |> filter(str_detect(sound, "Pl"))
    |> separate_longer_delim(sound, regex("(?<=[a-z])(?=P)"))
    |> count(sound)
    |> arrange(desc(n))
    |> left_join(rain_factors_table(rain_factors))
    |> mutate(
        pct = round(n / sum(n), 2),
        sound = factor(sound, levels = sound_list)
    ))
}

plot_rain_sounds <- function(rain_factors) {
  count_table <- rain_sound_count_table(10000, rain_factors)

  ggplot(count_table, aes(sound, pct)) +
    geom_point() +
    geom_smooth(aes(x = as.numeric(sound)), method = "lm") +
    geom_label(aes(label = factor)) +
    labs(title = "Rain Sounds from 1 - 10K")

  ggsave("rain_sound_count_plot.png")
}

rain_sound_count_slope <- function(rain_factors) {
  count_table <- rain_sound_count_table(10000, rain_factors)
  lm(pct ~ as.numeric(sound), count_table) |> coef() |> (\(x) x[2])()
}

find_next_rain_factor <- function(rain_factors = RAIN_FACTORS) {
  try_with_new_sound <- \(n) {
    for (rf in rain_factors) {
      if (n <= rf$factor) return(list())
      if (n %% rf$factor == 0) return(list())
    }
    new_factor   <- rain_factor(n, "Plung")
    test_factors <- append(rain_factors, list(new_factor))
    tibble(n = n, slope = rain_sound_count_slope(test_factors))
  }

  basis_slope <- rain_sound_count_slope(rain_factors)

  (result
    <- map(seq(100), try_with_new_sound)
    |> bind_rows()
    |> mutate(
        diff  = abs(basis_slope - slope),
        ideal = diff == min(diff),
        label = ifelse(ideal, n, NA_integer_)
    ))

  ggplot(result, aes(n, slope)) +
    geom_point() +
    geom_label(aes(label = label)) +
    geom_hline(yintercept = -0.14)

  ggsave("find_ideal_rain_factor_plot.png")
}
