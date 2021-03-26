most_frequent_digit <- function(n) {
  d <- unlist(strsplit(as.character(n), ""))
  as.integer(names(table(d)[table(d) == max(table(d))]))
}

most_frequent_digit_golf <- function(n) {
  n=c(25,2,3,57,38,41);d=unlist(strsplit(as.character(n),""));t=table;td=t(d);as.integer(names(td[td==max(td)]))
}



n=c(25,2,3,57,38,41);d=unlist(strsplit(c(n,""),""));t=table(d);as.numeric(names(t[t==max(t)]))
