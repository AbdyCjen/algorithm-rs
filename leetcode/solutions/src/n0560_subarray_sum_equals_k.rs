/**
 * [560] Subarray Sum Equals K
 *
 * Given an array of integers and an integer k, you need to find the total number of continuous subarrays whose sum equals to k.
 * Example 1:
 *
 * Input:nums = [1,1,1], k = 2
 * Output: 2
 *
 *  
 * Constraints:
 *
 * The length of the array is in range [1, 20,000].
 * The range of numbers in the array is [-1000, 1000] and the range of the integer k is [-1e7, 1e7].
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
		let mut hm = std::collections::HashMap::<i32, i32>::new();
		hm.insert(0, 1);

		let mut sum_ = 0;
		let mut cnt = 0;

		for num in nums {
			sum_ += num;
			cnt += *hm.get(&(sum_ - k)).unwrap_or(&0);
			*hm.entry(sum_).or_insert(0) += 1;
		}
		cnt
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_560() {
		assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
		assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
		assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 0), 0);
	}
}
