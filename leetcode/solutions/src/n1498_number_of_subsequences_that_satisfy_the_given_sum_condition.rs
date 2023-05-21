/**
 * [1498] Number of Subsequences That Satisfy the Given Sum Condition
 *
 * You are given an array of integers nums and an integer target.
 * Return the number of non-empty subsequences of nums such that the sum of the minimum and maximum element on it is less or equal to target. Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [3,5,6,7], target = 9
 * Output: 4
 * Explanation: There are 4 subsequences that satisfy the condition.
 * [3] -> Min value + max value <= target (3 + 3 <= 9)
 * [3,5] -> (3 + 5 <= 9)
 * [3,5,6] -> (3 + 6 <= 9)
 * [3,6] -> (3 + 6 <= 9)
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,3,6,8], target = 10
 * Output: 6
 * Explanation: There are 6 subsequences that satisfy the condition. (nums can have repeated numbers).
 * [3] , [3] , [3,3], [3,6] , [3,6] , [3,3,6]
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [2,3,3,4,6,7], target = 12
 * Output: 61
 * Explanation: There are 63 non-empty subsequences, two of them do not satisfy the condition ([6,7], [7]).
 * Number of valid subsequences (63 - 2 = 61).
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^6
 *     1 <= target <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	const MO: i32 = 1e9 as i32 + 7;
	pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
		nums.sort_unstable();
		let pow: Vec<i32> = std::iter::successors(Some(1), |n| Some(n * 2 % Self::MO))
			.take(nums.len())
			.collect();
		let (mut ans, mut i, mut j) = (0, 0, nums.len() as i32 - 1);
		while i <= j {
			if nums[i as usize] + nums[j as usize] > target {
				j -= 1;
			} else {
				ans = (ans + pow[(j - i) as usize]) % Self::MO;
				i += 1;
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1498() {
		assert_eq!(Solution::num_subseq(vec![1], 1), 0);
		assert_eq!(Solution::num_subseq(vec![2, 3, 3, 4, 6, 7], 12), 61);
		assert_eq!(Solution::num_subseq(vec![3, 5, 6, 7], 9), 4);
		assert_eq!(Solution::num_subseq(vec![3, 3, 6, 8], 10), 6);
	}
}
