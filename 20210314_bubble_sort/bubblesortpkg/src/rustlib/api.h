#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Structs used to pass vectors to and from Rust

typedef struct {
  const int *data;
  uint32_t len;
} IntegerSlice;

typedef struct {
  const int *data;
  uint32_t len;
} LogicalSlice;

typedef struct {
  const double *data;
  uint32_t len;
} DoubleSlice;

typedef struct {
  const char **data;
  uint32_t len;
} StringSlice;

// Add functions from Rust here...

// Passing integer values/vectors
int32_t pass_one_integer(int32_t);
IntegerSlice pass_many_integers(IntegerSlice);
void free_integer_slice(IntegerSlice);

// Passing logical values/vectors
int32_t pass_one_logical(int32_t);
LogicalSlice pass_many_logicals(LogicalSlice);
void free_logical_slice(LogicalSlice);

// Passing double values/vectors
double pass_one_double(double);
DoubleSlice pass_many_doubles(DoubleSlice);
void free_double_slice(DoubleSlice);

// Passing string values/vectors
char * pass_one_string(const char *);
StringSlice pass_many_strings(StringSlice);
void free_string_slice(StringSlice);

// Passing integer values/vectors
IntegerSlice bubble_sort(IntegerSlice);

#ifdef __cplusplus
}
#endif
