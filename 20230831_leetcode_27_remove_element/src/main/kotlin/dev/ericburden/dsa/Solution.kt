package dev.ericburden.dsa

class Solution {
  fun removeElement(nums: IntArray, `val`: Int): Int {
    if (nums.isEmpty()) return 0

    var insertionIndex = nums.lastIndex
    var checkIndex = 0

    while (checkIndex < insertionIndex) {
      if (nums[checkIndex] == `val`) {
        nums[checkIndex] = nums[insertionIndex].also { nums[insertionIndex] = nums[checkIndex] }
        insertionIndex--
      } else {
        checkIndex++
      }
    }

    return if (nums[checkIndex] == `val`) checkIndex else checkIndex + 1
  }
}
