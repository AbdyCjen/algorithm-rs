/**
 * [368] Largest Divisible Subset
 *
 * Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:
 *
 *     answer[i] % answer[j] == 0, or
 *     answer[j] % answer[i] == 0
 *
 * If there are multiple solutions, return any of them.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,3]
 * Output: [1,2]
 * Explanation: [1,3] is also accepted.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,4,8]
 * Output: [1,2,4,8]
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 1000
 *     1 <= nums[i] <= 2 * 10^9
 *     All the integers in nums are unique.
 *
 */
pub struct Solution {}

// submission codes start here

//use std::collections::*;
impl Solution {
	pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
		nums.sort_unstable();
		let mut dp = (0..).map(|i| (1, i)).take(nums.len()).collect::<Vec<_>>();
		for (i, &n) in (0..).zip(&nums) {
			for (j, &m) in (0..).zip(&nums[..i]) {
				if n % m == 0 {
					dp[i] = dp[i].max((dp[j].0 + 1, j));
				}
			}
		}
		let mut head = dp.iter().map(|v| v.0).zip(0..).max().unwrap();
		let mut ans = vec![];
		for _ in 0..head.0 {
			ans.push(nums[head.1]);
			head.1 = dp[head.1].1;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_368() {
		assert_eq!(Solution::largest_divisible_subset(vec![2, 4, 6]), [6, 2]);
		assert_eq!(
			Solution::largest_divisible_subset(vec![1, 2, 4, 8]),
			[8, 4, 2, 1]
		);
	}
}
