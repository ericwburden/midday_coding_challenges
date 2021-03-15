#! /usr/bin/Rscript

#' In general programming, we receive a piece of data, and we do something with
#' the data we receive to make it useful. This could be for statistical
#' analysis, results presentation, gathering metrics to do something useful, or
#' many other reasons. One of the most common data objects I work with is the
#' file. Taking the following file as input, execute a program which compiles
#' the following useful metrics from the file:
#' 
#' The time the job started (Started at Mon Oct 12 16:12:22 2020.)
#' The time the job was Terminated (Terminated at Mon Oct 12 16:12:28 2020.)
#' the CPU time: (CPU time :                                   2.40 sec.)
#' The Max Memory used: (Max Memory :                                 2824 MB)
#' The walltime (Terminated time - Start Time)
#' Rough Estimate of the efficiency: CPU Time / walltime (this may require
#'   conversion of units)
#' 
#' The program should execute as follows:
#' 
#' gather_stats job_log.txt and it should print the following information:
#'       Walltime:   6s
#'       CPU Time:   2.40s
#'       Max Memory: 2.824G
#'       Efficiency: 0.4 (edited) 

# Libraries --------------------------------------------------------------------
library(stringr)  # string operations
library(magrittr) # pipe operator
library(glue)     # string interpolation

  
# Functions --------------------------------------------------------------------
# DateTime format: Mon Oct 12 16:12:20 2020
datetime_from_line <- function(s) {
  format <- '%b %d %H:%M:%S %Y'
  pattern <- '\\w{3}\\s\\d{2}\\s\\d{2}:\\d{2}:\\d{2}\\s\\d{4}'
  string_rep <- stringr::str_extract(s, pattern)
  as.POSIXct(string_rep, format=format)
}

cpu_time_from_line <- function(s) {
  pattern <- '\\d*\\.\\d*\\ssec'
  string_rep <- stringr::str_extract(s, pattern) %>% 
    stringr::str_remove('\\ssec$')
  round(as.numeric(string_rep), 2)
}

max_memory_from_line <- function(s) {
  pattern <- '\\d+\\sMB'
  string_rep <- stringr::str_extract(s, pattern) %>% 
    stringr::str_remove('\\sMB')
  as.numeric(string_rep)/1000
}

parse_line_by_search_string <- function(parse_fun, search_string, ss) {
  line <- ss[sapply(ss, stringr::str_detect, search_string)]
  if (length(line) > 1) {
    stop(glue('The search_string ({search_string}) did not identify a unique line.'))
  }
  parse_fun(line)
}

walltime <- function(start, end) {
  as.numeric(end - start)
}

efficiency <- function(start, end, cpu) {
  cpu/walltime(start, end)
}

# Script -----------------------------------------------------------------------

args <- commandArgs(trailingOnly=TRUE)
if (length(args) > 1) {
  stop('Expected only one argument.')
}
if (!stringr::str_detect(args[1], '\\.txt$')) {
  stop('Expected the name of a text file: "*.txt"')
}

# Read in the file, line by line
file_contents <- readr::read_lines(args[1])
if (length(file_contents) == 0) {
  stop('File contains no information.')
}

# search_strings are strings that uniquely identify a line
# parse_funs are functions that extract the desired information from a line
search_strings <- list(
  started    = '^\\s+(Started at).*\\.$', 
  terminated = '^\\s+(Terminated at).*\\.$', 
  cpu        = '^\\s+(CPU time).*\\.$', 
  max_mem    = '^\\s+(Max Memory).*\\MB$'
)

parse_funs <- list(
  started    = datetime_from_line, 
  terminated = datetime_from_line, 
  cpu        = cpu_time_from_line, 
  max_mem    = max_memory_from_line
)

parse_results <- mapply(
  parse_line_by_search_string, 
  parse_funs, 
  search_strings, 
  MoreArgs = list(ss=file_contents), 
  SIMPLIFY = F
)

attach(parse_results) # Make list elements available as variables
cat(
  paste(
    '\n',
    glue('Walltime:   {walltime(started, terminated)}s'),
    glue('CPU Time:   {cpu}s'),
    glue('Max Memory: {max_mem}G'),
    glue('Efficiency: {efficiency(started, terminated, cpu)}'),
    sep = '\n'
  ),
  '\n',
  sep = '\n'
)
