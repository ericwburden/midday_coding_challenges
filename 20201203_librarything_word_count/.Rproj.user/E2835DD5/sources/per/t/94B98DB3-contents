#' # The LibraryThing programming quiz!
#'
#' Want to apply for one of the technical jobs at LibraryThing? Take the test
#' below. If you can do it in under five minutes, let’s talk!
#'
#' **Background**. I’ve been spending a lot of my time going through (read:
#' “drowning in”) applications for our 2-3 open technical jobs. And I’ve been
#' conducting a few interviews. The latter has demonstrated to me again the
#' value of asking applicants to write code, especially simple code, during
#' the interviews.
#'
#' That interviews should involve simple code is now common wisdom in
#' programming circles. The story is that a high percentage of programmers,
#' even people with impressive resumes, “just can’t code.” Asked to do the
#' simplest things—problems a good coders could solve as fast they could
#' write—some spend ten or twenty minutes before they get an answer, or fail
#' entirely. (See discussion by Jeff Attwood and Joel Spolsky.) I don’t go as
#' far as others here. I think a lot of “slow coders” are probably excellent
#' employees, making up for it in other areas. Some projects don’t need speed.
#' Some people just need to spend more time programming; everyone was a slow
#' programmer some time. But I know from experience that slow coders don’t work
#' at LibraryThing. They don’t fit the LibraryThing development culture.
#'
#' “Simple code” is critical. When I first started interviewing I’d ask people
#' to solve hard problems. This didn’t work. As Spolsky’s famous “The Guerrilla
#' Guide to Interviewing” argues, however, simple problems are just as good, or
#' better. Simple problems test simple skills, and simple skills are the basis
#' of complex ones. A programmer who struggles to loop through a bunch of words
#' will be at sea performing far more complex tasks. If you can’t boil water
#' your Toad in the Hole is likely to come out wrong.
#'
#' Coding tests irritate a minority of people. One applicant said his resume
#' should speak for itself. I have no time for that attitude—though I’m happy
#' for the weeding help. Good programmers should welcome the opportunity to
#' demonstrate they’re good at what they do. Heck, most programmers I know
#' enjoy brain teasers anyway. They test themselves when no one’s looking.
#'
#' The rules.
#'
#' - Time yourself.
#' - Write this code in any language you want. Use your best language. If you
#' can do many, PHP, Python and Javascript are preferred.
#' - Write it out longhand or in a simple text editor.
#' - I don’t care much about syntax. You can fix whether it’s isArray or
#' is_Array later. If it has some obvious bug you’d fix in a second, fine.
#' - This is not a trick question.
#' - I’m not asking for some imaginary “best answer” that does everything in
#' one line of self-writing code, or whatever.
#'
#' The problem.
#'
#' Input is a string—a paragraph of text. One of the paragraphs above would be fine.
#'
#' Output is a report listing how many words there are with X letters, like:
#'
#' - 10 words with 1 letter
#' - 20 words with 2 letters
#' - 7 words with 3 letters
#' - 15 words with 4 letters, etc.

library(tidyverse)

input <- "That interviews should involve simple code is now common wisdom in
          programming circles. The story is that a high percentage of programmers,
          even people with impressive resumes, “just can’t code.” Asked to do
          the simplest things—problems a good coders could solve as fast they
          could write—some spend ten or twenty minutes before they get an answer,
          or fail entirely. (See discussion by Jeff Attwood and Joel Spolsky.)
          I don’t go as far as others here. I think a lot of “slow coders” are
          probably excellent employees, making up for it in other areas. Some
          projects don’t need speed. Some people just need to spend more time
          programming; everyone was a slow programmer some time. But I know
          from experience that slow coders don’t work at LibraryThing. They
          don’t fit the LibraryThing development culture."

t <- tibble(words = str_split(str_remove_all(input, '[^\\w ]'), ' ')[[1]]) %>%
  mutate(l = nchar(words)) %>%
  group_by(l) %>%
  tally() %>%
  arrange(l)

for (i in seq(nrow(t))) {
  print(glue::glue('{t$n[[i]]} word(s) of length {t$l[[i]]}'))
}


