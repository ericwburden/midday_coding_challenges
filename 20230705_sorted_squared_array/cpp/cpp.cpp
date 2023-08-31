#include <iostream>
#include <algorithm>
#include <vector>

using namespace std;

namespace sorted_squared_array {

vector<int> square_and_sort(vector<int> numbers) {
  // Prepare a vector for the squares of the negative numbers and the
  // squares of the positive numbers
  vector<int> negative_squares {};
  vector<int> positive_squares {};

  // Iterate over the input, squaring each number and placing it into
  // it's respective squares vector.
  for (auto & number : numbers) {
    if (number < 0) {
      negative_squares.push_back(number * number);
    } else {
      positive_squares.push_back(number * number);
    }
  }

  // Reverse the negative squares vector so that it is ordered from
  // smallest to largest, just like the squares of the positive numbers.
  reverse(negative_squares.begin(), negative_squares.end());

  // Create iterators over the positive and negative squares, and a
  // vector to hold the final output. Could probably do this more efficiently
  // by modifying the squared positives vector in place, but that gets a
  // bit hairy and confusing to read.
  vector<int>::iterator p1 = positive_squares.begin();
  vector<int>::iterator p2 = negative_squares.begin();
  vector<int> out {};

  // Check along both the positive and negative squares, appending whichever
  // value is smaller to the output vector.
  while (p1 != positive_squares.end() && p2 != negative_squares.end()) {
    if (*p2 < *p1) {
      out.push_back(*p2);
      p2++;
    } else {
      out.push_back(*p1);
      p1++;
    }
  }

  // If we run out of positive squares above, then we can just append the rest
  // of the positive squares to the output.
  if (p1 != positive_squares.end()) {
    out.insert(out.end(), p1, positive_squares.end());
  }

  // Same thing for the negative squares.
  if (p2 != negative_squares.end()) {
    out.insert(out.end(), p2, negative_squares.end());
  }

  return out;
}

}
