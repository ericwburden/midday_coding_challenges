package merge_two_arrays

class Solution {
  fun merge(nums1: IntArray, m: Int, nums2: IntArray, n: Int): Unit {
    var insertIdx = nums1.size - 1 // Index where values are inserted in nums1
    var nums1Idx = m - 1 // Index for the value in nums1 to check
    var nums2Idx = n - 1 // Index for the value in nums2 to check

    // Working backwards from the largest value in both arrays, insert
    // the largest value into the insertIdx of nums1 and decrement the
    // index for the array the number was inserted _from_. Do thus until
    // one of the arrays is exhausted.
    while (nums1Idx >= 0 && nums2Idx >= 0) {
      if (nums1[nums1Idx] > nums2[nums2Idx]) {
        nums1[insertIdx] = nums1[nums1Idx]
        nums1Idx -= 1
      } else {
        nums1[insertIdx] = nums2[nums2Idx]
        nums2Idx -= 1
      }
      insertIdx -= 1
    }

    // If nums1 is exhausted before nums2, finish inserting the
    // values from nums2 into nums1.
    while (nums2Idx >= 0) {
      nums1[insertIdx] = nums2[nums2Idx]
      nums2Idx -= 1
      insertIdx -= 1
    }
  }
}
