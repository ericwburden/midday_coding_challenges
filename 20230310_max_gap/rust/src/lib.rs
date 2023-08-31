//! Given an integer array nums, return the maximum difference between two successive
//! elements in its sorted form. If the array contains less than two elements, return
//! 0.
//!
//! You must write an algorithm that runs in linear time and uses linear extra space.
//!
//!  
//! ```
//! Example 1:
//!
//! Input: nums = [3,6,9,1]
//! Output: 3
//! Explanation: The sorted form of the array is [1,3,6,9], either (3,6) or (6,9) has
//! the maximum difference 3.
//! ```
//!
//! ```
//! Example 2:
//!
//! Input: nums = [10]
//! Output: 0
//! Explanation: The array contains less than 2 elements, therefore return 0.
//! ```
//!  
//!
//! Constraints:
//!
//! - `1 <= nums.length <= 10^5`
//! - `0 <= nums[i] <= 10^9`

#[derive(Clone, Copy, Debug)]
pub struct Bucket(i32, i32);

impl Default for Bucket {
    fn default() -> Self {
        Bucket(i32::MAX, i32::MIN)
    }
}

impl Bucket {
    pub fn push(&mut self, num: i32) {
        self.0 = self.0.min(num);
        self.1 = self.1.max(num);
    }

    pub fn gap(&self, next: &Bucket) -> i32 {
        next.0 - self.1
    }

    pub fn is_empty(&self) -> bool {
        self.0 == i32::MAX
    }
}

pub fn maximum_gap(nums: Vec<i32>) -> i32 {
    // Start by determining the range of values we need to examine
    let (min_num, max_num) = nums.iter().fold((i32::MAX, i32::MIN), |(lo, hi), num| {
        (lo.min(*num), hi.max(*num))
    });
    let range = (max_num - min_num) as usize;
    if range == 0 {
        // If the min and max are the same, there is no gap
        return 0;
    }

    // We want to distribute the numbers into "buckets", or groups of numbers where
    // the size of the group is smaller than the maximum gap. This means that the
    // actual maximum gap must be between buckets and not inside one bucket. We can
    // calculate the minimum value for max gap as the range divided by the number of
    // values minus 1, rounded up. For example, given the values [1, 2, 100], the
    // bucket size should be (100 - 1) / 2 == 49.5 > 50. While the middle number is
    // not involved in this calculation, you can see that there is no value between
    // 1 and 100 that will give a smaller maximum gap than 50.
    let mut bucket_width = range / (nums.len() - 1);
    if range % (nums.len() - 1) != 0 {
        // Round up for integer division
        bucket_width += 1;
    }

    // We'll have as many buckets as original values
    let mut buckets = vec![Bucket::default(); nums.len()];

    // Distribute the numbers into their buckets. We only need the minimum and maximum
    // value in each bucket, since we know the max gap will be between buckets.
    for num in nums {
        // This formula yields a value between 0 and nums.len()
        let bucket_idx = (num - min_num) as usize / bucket_width;
        buckets[bucket_idx].push(num);
    }

    // Now, we just need to calculate the maximum gap between buckets, ignoring
    // any empty buckets.
    let mut max_gap = bucket_width as i32; // The true max gap is always at least as large as the buckets
    let mut last_bucket = buckets[0]; // Start with the first bucket
    for bucket in buckets {
        if bucket.is_empty() {
            continue;
        }
        let gap = last_bucket.gap(&bucket);
        max_gap = max_gap.max(gap);
        last_bucket = bucket;
    }
    max_gap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![3, 6, 9, 1];
        assert_eq!(maximum_gap(nums), 3);
    }

    #[test]
    fn case2() {
        let nums = vec![10];
        assert_eq!(maximum_gap(nums), 0);
    }

    #[test]
    fn case3() {
        let nums = vec![2, 99999999];
        assert_eq!(maximum_gap(nums), 99999997);
    }
}
