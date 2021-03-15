random_phone_number <- function() {
  paste0('(', round(runif(1, 200, 999)), ') ') %>% 
    paste(round(runif(1, 212, 919)), '- ') %>% 
    paste(paste(round(runif(4, 0, 9)), collapse = ''))
}

random_email_address <- function() {
  runif(32, 0, 255) %>% 
    round() %>% 
    as.hexmode() %>% 
    paste(collapse = '') %>% 
    paste0('@fake.email')
}

replace_missing_names <- function(names, name_list) {
  replacements <- replicate(
    length(names), 
    {
      randi <- round(runif(1, 1, length(name_list)))
      name_list[randi]
    }
  )
  coalesce(names, replacements)
}

load_library <- function(lib_name) {
  if (!(require(lib_name, character.only = T))) {
    stop(paste(lib_name, 'is a required library for this script.'))
  }
}

complete_csv <- function(file_path) {
  libraries <- c('magrittr', 'dplyr', 'readr')
  lapply(libraries, load_library)
  
  # Name lists derived from https://github.com/smashew/NameDatabases
  first_name_list <- readRDS('first_names.rds')
  last_name_list <- readRDS('last_names.rds')
  
  readr::read_csv(file_path) %>% 
    mutate(
      FirstName = replace_missing_names(FirstName, first_name_list),
      LastName = replace_missing_names(LastName, last_name_list),
      phone_number = replicate(nrow(.), random_phone_number()),
      email_address = replicate(nrow(.), random_email_address())
    )
}

result <- complete_csv('people.csv') %T>%
  write_csv('completed_people.csv')
summary(result)

#  PersonNumber    FirstName           LastName         phone_number       email_address     
# Min.   :  0.0   Length:317         Length:317         Length:317         Length:317        
# 1st Qu.:618.0   Class :character   Class :character   Class :character   Class :character  
# Median :720.0   Mode  :character   Mode  :character   Mode  :character   Mode  :character  
# Mean   :630.2                                                                              
# 3rd Qu.:811.0                                                                              
# Max.   :961.0                          

# If there were any NA's left in the column, it would be indicated above.


glimpse(result)

# Rows: 317
# Columns: 5
# $ PersonNumber  <dbl> 0, 1, 2, 3, 4, 32, 103, 105, 106, 107, 108, 109, 133, 210, 211, 212, 213, 214, 215, 23…
# $ FirstName     <chr> "Georgeanna", "Anna", "Armando", "Alicia", "Mary", "Monica", "Dave", "Ginger", "Clara"…
# $ LastName      <chr> "Consolver", "Louden", "Desmarais", "Pio", "Mccan", "Koterba", "Kimbley", "Dreben", "H…
# $ phone_number  <chr> "(655)  724 -  5618", "(287)  299 -  5438", "(803)  821 -  6311", "(590)  525 -  1482"…
# $ email_address <chr> "01275a3d0b60594f4d2245135c114d4827170e0a183823505b2f3a2e611b3854@fake.email", "40425a…


nrow(distinct(result))  # Number of unique rows: 317
length(result$phone_number) == length(unique(result$phone_number))  # TRUE
length(result$email_address) == length(unique(result$email_address))  # TRUE

