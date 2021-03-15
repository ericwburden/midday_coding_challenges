#' How Many Vowels?
#' 
#' Create a function that takes a string and returns the number (count) of 
#' vowels contained within it.
#' 
#' Examples
#' - countVowels("Celebration") ➞ 5
#' - countVowels("Palm") ➞ 1
#' - countVowels("Prediction") ➞ 4
#' 
#' Notes
#' a, e, i, o, u are considered vowels (not y).
#' All test cases are one word and only contain letters.

#' There are MUCH better ways to do this, but I wanted to show off some
#' more R-specific syntax on this one

library(tidyverse) # Collection of AWESOME data manipulation libraries

count_vowels <- function(string) {
  #' Convert the string to a lowercase character vector
  lower_string <- tolower(string)
  char_vec <- stringr::str_split(lower_string, '')[[1]]
  vowels <- c('a', 'e', 'i', 'o', 'u')
  
  #' You can reference a function in a library using library::function syntax
  #' Since I used the library() call above, it's not strictly necessary,
  #' but this way it's explicit which libraries from the tidyverse package
  #' I'm using
  #' tidyr::expand_grid creates a dataframe consisting of all possible
  #' combinations of the two input vectors
  df <- tidyr::expand_grid(
    original_character = char_vec,
    vowel = vowels
  )
  
  #' The %>% (pipe) operator passes the results of the left hand side
  #' as the first argument of the function on the right hand side, a lot
  #' like pipes in a bash shell
  df <- df %>% 
    filter(original_character == vowel) %>% # Keep only vowels
    group_by(vowel) %>%                     # For each represented vowel
    tally()                                 # Count how many times it appears
  
  #' Lots of stuff going on here, making a bar chart
  #' https://ggplot2.tidyverse.org/
  bar_chart <- ggplot(df, aes(vowel, n)) +
    geom_col(aes(fill = vowel)) +
    geom_text(aes(label = n), size = 12, color = 'white', vjust = 1.2) +
    scale_fill_brewer(palette = 'Dark2') +
    theme_minimal() +
    labs(
      title = glue::glue("Vowels in {string}"),
      x = 'Vowel',
      y = '#',
      fill = 'Vowel'
    )
  
  #' Return a list containing the desired sum, the resulting dataframe
  #' and a bar chart
  list(sum = sum(df$n), df = df, plot = bar_chart)
}

celebration <-  count_vowels('Celebration')
palm <- count_vowels('Palm')
prediction <- count_vowels('prediction')
flex <- count_vowels('Supercalifragilisticexpialidocious')

print(flex$sum) # [1] 16

glimpse(flex$df)
#' Observations: 5
#' Variables: 2
#' $ vowel <chr> "a", "e", "i", "o", "u"
#' $ n     <int> 3, 2, 7, 2, 2

plot(flex$plot)
# 