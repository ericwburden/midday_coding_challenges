#define R_NO_REMAP
#define STRICT_R_HEADERS
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <Rinternals.h>

// Import C headers for rust API
#include "rustlib/api.h"

// Passing integers/integer vectors --------------------------------------------

SEXP pass_one_integer_wrapper(SEXP n){
  // Relatively straightforward, call the Rust function with the result of
  // Rf_asInteger (converts an INTSXP to an integer), returning a int32_t
  int32_t rust_n =  pass_one_integer(Rf_asInteger(n));
  return Rf_ScalarInteger(rust_n); // Convert back to an R length-one vector and return
}

SEXP pass_many_integers_wrapper(SEXP integers){
  // Build an IntegerSlice from the `integers` vector
  IntegerSlice c_integers = {INTEGER(integers), Rf_length(integers)};
  IntegerSlice rust_integers = pass_many_integers(c_integers); // Call the Rust fn

  // Create a protected, empty vector of length `rust_integers.len`
  SEXP out = PROTECT(Rf_allocVector(INTSXP, rust_integers.len));

  // Copy the ints from `rust_integers` to the outgoing vector
  memcpy(INTEGER(out), rust_integers.data, rust_integers.len * sizeof(int));
  free_integer_slice(rust_integers); // Free the IntegerSlice memory in Rust
  UNPROTECT(1); // Drop the GC protection

  return out;
}


// Passing logical/logical vectors ---------------------------------------------
SEXP pass_one_logical_wrapper(SEXP b){
  // Relatively straightforward, call the Rust function with the result of
  // Rf_asLogical (converts an LGLSXP to an integer), returning a int32_t
  int32_t rust_b =  pass_one_logical(Rf_asLogical(b));
  return Rf_ScalarLogical(rust_b); // Convert back to an R length-one vector and return
}

SEXP pass_many_logicals_wrapper(SEXP logicals){
  // Build a LogicalSlice from the `logicals` vector
  LogicalSlice c_logicals = {LOGICAL(logicals), Rf_length(logicals)};
  LogicalSlice rust_logicals = pass_many_logicals(c_logicals); // Call the Rust fn

  // Create a protected, empty vector of length `rust_integers.len`
  SEXP out = PROTECT(Rf_allocVector(LGLSXP, rust_logicals.len));

  // Copy the ints from `rust_logicals` to the outgoing vector
  memcpy(LOGICAL(out), rust_logicals.data, rust_logicals.len * sizeof(int));
  free_logical_slice(rust_logicals); // Free the LogicalSlice memory in Rust
  UNPROTECT(1); // Drop the GC protection

  return out;
}


// Passing double values/vectors -----------------------------------------------

SEXP pass_one_double_wrapper(SEXP d){
  // Note: Rust receives and treats NA's and NaN's from R exactly the same. The
  // major difference is in the last bit word of an NA, which is 1954 instead of
  // zero. It appears that attempts to transform the NA in Rust do not modify
  // this last bit word, so any NA passed to Rust should come back as an NA,
  // and not a generic NaN. So...no need to do anything special to keep track
  // of NA values.

  double rust_d = pass_one_double(Rf_asReal(d)); // Pass the double to Rust
  return Rf_ScalarReal(rust_d); // Convert back to an R length-one vector and return
}

SEXP pass_many_doubles_wrapper(SEXP doubles){
  // Build a DoubleSlice from the `doubles` vector
  DoubleSlice c_doubles = {REAL(doubles), Rf_length(doubles)};
  DoubleSlice rust_doubles = pass_many_doubles(c_doubles); // Call the Rust fn

  // Create a protected, empty vector of length `rust_doubles.len`
  SEXP out = PROTECT(Rf_allocVector(REALSXP, rust_doubles.len));

  // Copy the doubles from `rust_doubles` to the outgoing vector
  memcpy(REAL(out), rust_doubles.data, rust_doubles.len * sizeof(double));
  free_double_slice(rust_doubles); // Free the DoubleSlice memory in Rust
  UNPROTECT(1); // Drop the GC protection

  return out;
}


// Passing strings/string vectors ----------------------------------------------

SEXP pass_one_string_wrapper(SEXP string){
  // Check to see if R passed a string or a NA_character_. If `string` was
  // passed as an NA, set `r_string` to a null pointer, otherwise just convert
  // to a UTF8-translated character pointer
  const char *r_string;
  if (STRING_ELT(string, 0) != R_NaString) {
    r_string = Rf_translateCharUTF8(STRING_ELT(string, 0));
  } else {
    r_string = (void *) 0;
  }

  // Pass `r_string` to the Rust function
  char *c_string = pass_one_string(r_string);

  // If Rust returns a null pointer, convert it back to an NA_character_,
  // otherwise convert the string pointer back to an R string.
  if (c_string) {
    return Rf_ScalarString(Rf_mkCharCE(c_string, CE_UTF8));
  } else {
    return Rf_ScalarString(R_NaString);
  }
}

SEXP pass_many_strings_wrapper(SEXP strings){
  // Need to package the strings from the R vector into an array of pointers
  // pointing to the strings in memory
  int n, i;
  n = Rf_length(strings);  // Length of the vector
  // Allocate memory to store the pointers, a chunk of memory equal to n * the
  // size of a character pointer
  const char **c_strings = (const char**) malloc(n * sizeof(char*));
  if (c_strings != NULL) {
    for (i = 0; i < n; i++) {
      // For each string in the vector, store a pointer to a UTF8-translated
      // string at an offset from the beginning of `c_strings`. If the vector
      // contained one or more NA_character_'s, insert null pointers for each
      // NA.
      if (STRING_ELT(strings, i) != R_NaString) {
        c_strings[i] = Rf_translateCharUTF8(STRING_ELT(strings, i));
      } else {
        c_strings[i] = (void *) 0;
      }
    }

    // Call the Rust function and receive a StringSlice in return. StringSlice
    // contains a char** for the strings and the length of the return vector
    StringSlice rust_strings = pass_many_strings((StringSlice){c_strings, n});
    free(c_strings);  // Free the allocated memory from malloc

    // Create an R character vector, length from the StringSlice `rust_strings`
    SEXP return_vec = PROTECT(Rf_allocVector(STRSXP, rust_strings.len));

    // Fill the R vector with strings. Rf_mkChar either finds the given string in
    // memory or creates a new one if it doesn't exist. For any null pointers in
    // the returned StringSlice, insert an R NA_character_ (R_NaString).
    for (i = 0; i < rust_strings.len; i++) {
      if (rust_strings.data[i]) {
        SET_STRING_ELT(return_vec, i, Rf_mkChar(rust_strings.data[i]));
      } else {
        SET_STRING_ELT(return_vec, i, R_NaString);
      }
    }
    UNPROTECT(1);

    free_string_slice(rust_strings); // Free the StringSlice memory in Rust

    return return_vec; // Return the vector to R
  } else {
    printf("String allocation failed! Insufficient memory available.");
  }

  return R_NilValue; // If all else fails
}

// Bubble Sort -----------------------------------------------------------------

SEXP bubble_sort_wrapper(SEXP integers){
  // Build an IntegerSlice from the `integers` vector
  IntegerSlice c_integers = {INTEGER(integers), Rf_length(integers)};
  IntegerSlice rust_integers = bubble_sort(c_integers); // Call the Rust fn

  // Create a protected, empty vector of length `rust_integers.len`
  SEXP out = PROTECT(Rf_allocVector(INTSXP, rust_integers.len));

  // Copy the ints from `rust_integers` to the outgoing vector
  memcpy(INTEGER(out), rust_integers.data, rust_integers.len * sizeof(int));
  free_integer_slice(rust_integers); // Free the IntegerSlice memory in Rust
  UNPROTECT(1); // Drop the GC protection

  return out;
}

// Export the wrapper functions, need to add new functions here
static const R_CallMethodDef CallEntries[] = {
  {"pass_one_integer_wrapper", (DL_FUNC) &pass_one_integer_wrapper, 1},
  {"pass_many_integers_wrapper", (DL_FUNC) &pass_many_integers_wrapper, 1},
  {"pass_one_logical_wrapper", (DL_FUNC) &pass_one_logical_wrapper, 1},
  {"pass_many_logicals_wrapper", (DL_FUNC) &pass_many_logicals_wrapper, 1},
  {"pass_one_double_wrapper", (DL_FUNC) &pass_one_double_wrapper, 1},
  {"pass_many_doubles_wrapper", (DL_FUNC) &pass_many_doubles_wrapper, 1},
  {"pass_one_string_wrapper", (DL_FUNC) &pass_one_string_wrapper, 1},
  {"pass_many_strings_wrapper", (DL_FUNC) &pass_many_strings_wrapper, 1},
  {"bubble_sort_wrapper", (DL_FUNC) &bubble_sort_wrapper, 1},
  {NULL, NULL, 0}
};

// This can be ignored, doesn't need to be changed.
void R_init_bubblesortpkg(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
