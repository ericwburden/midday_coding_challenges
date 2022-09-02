kid_names <- c("Abigail",  "Becky",  "Charlie", "David",   "Erin",    "Frankie",
               "Gregory",  "Hannah", "Isaac",   "Jenny",   "Kerry",   "Lonnie",
               "Margaret", "Norman", "Ophelia", "Patsy",   "Quentin", "Rich",
               "Sandie",   "Thomas", "Ursula",  "Victory", "Walter",  "Xavier",
               "Yusuf",    "Zazie")

snack_counts <- seq(0, 10, 2)

snack_types <- c("cookies",  "goldfish", "marshmallows", 
                 "broccoli", "grahams",  "apples",
                 "gummies",  "pretzels", "cheesesticks")

alternatives <- c("got stuck on the monkey bars",
                  "wandered away",
                  "cried about trains",
                  "took a nap")

LINES <- 10000

permutations <- data.frame(
  name  = sample(kid_names,    LINES, replace = T),
  num   = sample(snack_counts, LINES, replace = T),
  type  = sample(snack_types,  LINES, replace = T),
  alt   = sample(alternatives, LINES, replace = T),
  use_alt = sample(c(T, F),    LINES, replace = T, prob = c(1, 9))
)

lines <- with(permutations,
              ifelse(use_alt,
                     paste(name, alt),
                     paste(name, "ate", num, type)))

conn <- file("snack_record.txt")
writeLines(lines, conn)
close(conn)
