/**
 * [930] Binary Subarrays With Sum
 *
 * Given a binary array nums and an integer goal, return the number of non-empty subarrays with a sum goal.
 *
 * A subarray is a contiguous part of the array.
 *
 *  
 * <strong class="example">Example 1:
 *
 *
 * Input: nums = [1,0,1,0,1], goal = 2
 * Output: 4
 * Explanation: The 4 subarrays are bolded and underlined below:
 * [<u>1,0,1</u>,0,1]
 * [<u>1,0,1,0</u>,1]
 * [1,<u>0,1,0,1</u>]
 * [1,0,<u>1,0,1</u>]
 *
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: nums = [0,0,0,0,0], goal = 0
 * Output: 15
 *
 *
 *  
 * Constraints:
 *
 *
 *     1 <= nums.length <= 3 * 10^4
 *     nums[i] is either 0 or 1.
 *     0 <= goal <= nums.length
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
		let mut sums = std::collections::HashMap::new();
		sums.insert(0, 1);
		let mut s = 0;
		let mut ans = 0;
		for n in nums {
			s += n;
			ans += sums.get(&(s - goal)).unwrap_or(&0);
			*sums.entry(s).or_insert(0) += 1;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_930() {
		assert_eq!(Solution::num_subarrays_with_sum(vec![0; 5], 0), 15);
		assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
	}
}
