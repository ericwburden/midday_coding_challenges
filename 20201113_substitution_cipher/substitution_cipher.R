#' **Substitution Ciphers**
#' In a substitution cipher characters or groups of bits are replaced by other
#' characters or groups of bits. Julius Caesar used a shift of the alphabet by
#' 3 positions to communicate with the commanders of his garrison.
#'
#' Write one function that will take the plaintext string and perform the
#' encoding by this cipher. A second function should be created to decode the
#' same.
#'
#' The letter plus 3 cipher will encode A to D, B to E, C to F and so on with
#' X to A, Y to B, and Z to C.
#'
#' **Example:**
#'   encode("hello world") should return "khoor zruog"
#'   decode("khoor zruog") should return "hello world"
#'
#'   As an added difficulty, attempt this without an array/dict/map to directly
#'   map values or even make the function accept different ciphers.

library(testthat)

# Functions --------------------------------------------------------------------

#' The default `utf8ToInt` and `intToUtf8` functions only work on vectors with
#' a single value, which is honestly a bit odd in R. The following helper
#' functions just adapt them to work on a vector of any size.
utf8sToInts <- function(x) { sapply(x, utf8ToInt, simplify = T) }
intsToUtf8s <- function(x) { sapply(x, intToUtf8, simplify = T) }

#' The workhorse. This function takes a vector of characters and shifts them
#' by the specified amount. Uppercase characters are converted to lowercase,
#' shifted, then converted back. Non-alpha characters are noted, shifted, then
#' replaced with their original values.
shift_chars <- function(x, shift) {
  upper_locs <- grep('[A-Z]', x)         # Indices of uppercase chars
  nonalpha_locs <- grep('[^A-Za-z]', x)  # Indices of non-alpha chars

  # Lowercase all so there's only one range to wrap around (97 - 122)
  ascii_chars <- utf8sToInts(tolower(x))
  shifted <- ascii_chars + shift
  shifted[shifted > 122] <- shifted[shifted > 122] - 26
  shifted[shifted < 97] <- shifted[shifted < 97] + 26
  shifted <- intsToUtf8s(shifted)

  shifted[upper_locs] <- toupper(shifted[upper_locs])  # Replace uppercase
  shifted[nonalpha_locs] <- x[nonalpha_locs]           # Replace non-alpha
  shifted
}

#' Encoding function, x can be a single string or a vector of strings
encode <- function(x, shift = 3) {
  encoded_strs <- lapply(strsplit(x, ''), shift_chars, shift)
  sapply(encoded_strs, paste, collapse = '')
}

#' Decoding function, x can be a single string or a vector of strings
decode <- function(x, shift = 3) {
  decoded_strs <- lapply(strsplit(x, ''), shift_chars, -shift)
  sapply(decoded_strs, paste, collapse = '')
}

# Tests ------------------------------------------------------------------------

test_that('example is correctly encoded/decoded', {
  expect_equal(encode('hello world'), 'khoor zruog')
  expect_equal(decode('khoor zruog'), 'hello world')
})

test_that('encode/decode work on vectors longer than one', {
  v1 <- c('hello world', 'hello world', 'hello world')
  v2 <- c('khoor zruog', 'khoor zruog', 'khoor zruog')

  expect_equal(encode(v1), v2)
  expect_equal(decode(v2), v1)
})

test_that('arbitrary coding/decoding returns original vector', {
  v <- rep(intsToUtf8s(round(runif(10, 65, 90))), 5)
  encoded <- encode(v, 10)
  decoded <- decode(encoded, 10)

  expect_equal(decoded, v)
})
