#include "cpp.cpp"
#include "test/catch.hpp"

using namespace std;

/* TEST_CASE("Family secrets have not been altered") { */
/*     REQUIRE(zhang::bank_number_part(1) == 8541); */
/*     REQUIRE(zhang::bank_number_part(3) == 8541 * 3 % 10'000); */
/*     REQUIRE(khan::bank_number_part(1) == 4142); */
/*     REQUIRE(khan::bank_number_part(3) == 4142 * 3 % 10'000); */
/*     REQUIRE(garcia::bank_number_part(1) == 4023); */
/*     REQUIRE(garcia::bank_number_part(3) == 4023 * 3 % 10'000); */

/*     REQUIRE(zhang::red::code_fragment() == 512); */
/*     REQUIRE(khan::red::code_fragment() == 148); */
/*     REQUIRE(garcia::red::code_fragment() == 118); */

/*     REQUIRE(zhang::blue::code_fragment() == 677); */
/*     REQUIRE(khan::blue::code_fragment() == 875); */
/*     REQUIRE(garcia::blue::code_fragment() == 923); */
/* } */

TEST_CASE("Squared values are sorted correctly") {
  vector<int> input1  {1, 2, 3, 5, 6, 8, 9};
  vector<int> output1 {1, 4, 9, 25, 36, 64, 81};
  REQUIRE(sorted_squared_array::square_and_sort(input1) == output1);

  vector<int> input2  {1};
  vector<int> output2 {1};
  REQUIRE(sorted_squared_array::square_and_sort(input2) == output2);

  vector<int> input3  {0};
  vector<int> output3 {0};
  REQUIRE(sorted_squared_array::square_and_sort(input3) == output3);

  vector<int> input4  {10};
  vector<int> output4 {100};
  REQUIRE(sorted_squared_array::square_and_sort(input4) == output4);

  vector<int> input5  {-1};
  vector<int> output5 {1};
  REQUIRE(sorted_squared_array::square_and_sort(input5) == output5);

  vector<int> input6  {-2, -1};
  vector<int> output6 {1, 4};
  REQUIRE(sorted_squared_array::square_and_sort(input6) == output6);

  vector<int> input7  {-5, -4, -3, -2, -1};
  vector<int> output7 {1, 4, 9, 16, 25};
  REQUIRE(sorted_squared_array::square_and_sort(input7) == output7);

  vector<int> input8  {-10, -5, 0, 5, 10};
  vector<int> output8 {0, 25, 25, 100, 100};
  REQUIRE(sorted_squared_array::square_and_sort(input8) == output8);

  vector<int> input9  {-7, -3, 1, 9, 22, 30};
  vector<int> output9 {1, 9, 49, 81, 484, 900};
  REQUIRE(sorted_squared_array::square_and_sort(input9) == output9);

  vector<int> input10  {-50, -13, -2, -1, 0, 0, 1, 1, 2, 3, 19, 20};
  vector<int> output10 {0, 0, 1, 1, 1, 4, 4, 9, 169, 361, 400, 2500};
  REQUIRE(sorted_squared_array::square_and_sort(input10) == output10);

}




































