/*
* Created a fn that passes in an array then loops and finds the top three highest values in that array then returns the multiplied value of the three highest values.

* Each time the array loops, its finding the highest value then adds to the topThreeProductValues[] and pop the value from products_values[]. Then uses a reduce method to return a single value

This problem was asked by Facebook.
Given a list of integers, return the largest product that can be made by multiplying any three integers.


*/

const inventory_values = [-12, -12, 11, 3, 11, 11, 3];

const product_values = [...inventory_values]; // a copy of the inventory_values[] to prevent nutation od the original array

function getTheHighestThreeProductsValues(arr) {
  const sortedArr = arr.sort((a, b) => a - b);
  const topThree  = arr.slice(arr.length - 3, arr.length);
  const bottomTwoAndMax = [...arr.slice(0, 2), arr[arr.length - 1]];

  const topThreeProduct = topThree.reduce((prev, current) => prev * current);
  const withNegProduct = bottomTwoAndMax.reduce((prev, current) => prev * current);

  return Math.max(topThreeProduct, withNegProduct);
}


const result = getTheHighestThreeProductsValues(product_values);
console.log(result);
