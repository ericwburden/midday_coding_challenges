package merge_two_arrays

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.api.Nested
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

@DisplayName("Solution")
class SolutionTest {
  @Nested
  @DisplayName("Example 1")
  inner class Example1 {
    @Test
    fun `Matches example`() {
      var nums1 = intArrayOf(1, 2, 3, 0, 0, 0)
      var nums2 = intArrayOf(2, 5, 6)
      Solution().merge(nums1, 3, nums2, 3)
      assertThat(nums1).isEqualTo(intArrayOf(1, 2, 2, 3, 5, 6))
    }
  }

  @Nested
  @DisplayName("Example 2")
  inner class Example2 {
    @Test
    fun `Matches example`() {
      var nums1 = intArrayOf(1)
      var nums2 = intArrayOf()
      Solution().merge(nums1, 1, nums2, 0)
      assertThat(nums1).isEqualTo(intArrayOf(1))
    }
  }
  @Nested
  @DisplayName("Example 3")
  inner class Example3 {
    @Test
    fun `Matches example`() {
      var nums1 = intArrayOf(0)
      var nums2 = intArrayOf(1)
      Solution().merge(nums1, 0, nums2, 1)
      assertThat(nums1).isEqualTo(intArrayOf(1))
    }
  }
}
