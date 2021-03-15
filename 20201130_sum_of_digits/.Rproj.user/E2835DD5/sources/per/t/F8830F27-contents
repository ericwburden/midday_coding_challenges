library(magrittr) # For the pipe
library(stringr)  # String manipulation

str_sum_of_digits <- function(n) {
  n %>% 
    as.character() %>%               # To string
    str_replace('-', '') %>%         # Remove '-' sign, if exists
    str_split('', simplify = T) %>%  # Split string on every character
    as.numeric() %>%                 # Back to numbers
    sum()                            # Sum
}