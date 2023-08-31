package dev.ericburden.dsa

import org.assertj.core.api.Assertions.assertThat
import org.assertj.core.api.Condition
import java.util.function.Predicate
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.api.Nested
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

@DisplayName("Solution")
class SolutionTest {
  fun validateResult(nums: IntArray, `val`: Int, k: Int) {
    val isVal = Condition<Int>(Predicate<Int> { it == `val` }, "equals val")
    assertThat(nums.take(k)).areNot(isVal)
  }

  @Nested
  @DisplayName("Example 1")
  inner class Example1 {
    @Test
    fun `Matches example`() {
      val nums = intArrayOf(3, 2, 2, 3)
      val `val` = 3
      val k = Solution().removeElement(nums, `val`)
      assertThat(k).isEqualTo(2)
      validateResult(nums, `val`, k)
    }
  }

  @Nested
  @DisplayName("Example 2")
  inner class Example2 {
    @Test
    fun `Matches example`() {
      val nums = intArrayOf(0, 1, 2, 2, 3, 0, 4, 2)
      val `val` = 2
      val k = Solution().removeElement(nums, `val`)
      assertThat(k).isEqualTo(5)
      validateResult(nums, `val`, k)
    }
  }
  
  @Nested
  @DisplayName("Edge Case 1")
  inner class EdgeCase1 {
    @Test
    fun `Resolves edge case`() {
      val nums = intArrayOf(1)
      val `val` = 1
      val k = Solution().removeElement(nums, `val`)
      assertThat(k).isEqualTo(0)
      validateResult(nums, `val`, k)
    }
  }
    
  @Nested
  @DisplayName("Edge Case 2")
  inner class EdgeCase2 {
    @Test
    fun `Resolves edge case`() {
      val nums = intArrayOf(1)
      val `val` = 2
      val k = Solution().removeElement(nums, `val`)
      assertThat(k).isEqualTo(1)
      validateResult(nums, `val`, k)
    }
  }

  @Nested
  @DisplayName("Edge Case 3")
  inner class EdgeCase3 {
    @Test
    fun `Resolves edge case`() {
      val nums = intArrayOf(4, 5)
      val `val` = 5
      val k = Solution().removeElement(nums, `val`)
      assertThat(k).isEqualTo(1)
      validateResult(nums, `val`, k)
    }
  }

    
  @Nested
  @DisplayName("Edge Case 4")
  inner class EdgeCase4 {
    @Test
    fun `Resolves edge case`() {
      val nums = intArrayOf()
      val `val` = 1
      val k = Solution().removeElement(nums, `val`)
      assertThat(k).isEqualTo(0)
      validateResult(nums, `val`, k)
    }
  }
}
