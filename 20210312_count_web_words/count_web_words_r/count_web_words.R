library(dplyr)
library(ggplot2)
library(magrittr)
library(rvest)
library(stringr)
library(stringi)
library(tibble)

html_text_collapse <- function(x, collapse = " ", trim = TRUE) {
  text <- html_text(html_nodes(x, xpath = ".//text()[normalize-space()]"))
  if (trim) { text <- stri_trim_both(text) }
  paste(text, collapse = collapse)
}


words <- read_html("http://paulgraham.com/simply.html") %>% 
  html_nodes("body") %>% 
  html_text_collapse() %>% 
  str_replace_all(r"{[^a-zA-Z0-9'\-\s\n]}", " ") %>% 
  str_replace_all(r"(\n)", " ") %>%
  str_split(" ", simplify = T) %>% 
  as.character() %>% 
  tolower()

table <- tibble(word = words, length = nchar(words)) %>% 
  filter(length > 0) %>% 
  count(word, length) %>% 
  group_by(length) %>% 
  mutate(label = ifelse(n == max(n), word, NA))

  
ggplot(table, aes(length, n)) + 
  geom_point() +
  geom_label(aes(label = label)) +
  theme_minimal() +
  labs(
    title = "Word Counts in 'Write Simply'",
    subtitle = "http://paulgraham.com/simply.html",
    x = "Word Length",
    y = "Word Count",
    caption = "The word of each length that appeared the most times is labeled."
  )
