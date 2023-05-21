/**
 * [1799] Maximize Score After N Operations
 *
 * You are given nums, an array of positive integers of size 2 * n. You must perform n operations on this array.
 * In the i^th operation (1-indexed), you will:
 *
 *     Choose two elements, x and y.
 *     Receive a score of i * gcd(x, y).
 *     Remove x and y from nums.
 *
 * Return the maximum score you can receive after performing n operations.
 * The function gcd(x, y) is the greatest common divisor of x and y.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2]
 * Output: 1
 * Explanation: The optimal choice of operations is:
 * (1 * gcd(1, 2)) = 1
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,4,6,8]
 * Output: 11
 * Explanation: The optimal choice of operations is:
 * (1 * gcd(3, 6)) + (2 * gcd(4, 8)) = 3 + 8 = 11
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,2,3,4,5,6]
 * Output: 14
 * Explanation: The optimal choice of operations is:
 * (1 * gcd(1, 5)) + (2 * gcd(2, 4)) + (3 * gcd(3, 6)) = 1 + 4 + 9 = 14
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 7
 *     nums.length == 2 * n
 *     1 <= nums[i] <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	fn gcd(a: i32, b: i32) -> i32 {
		match b {
			0 => a,
			_ => Self::gcd(b, a % b),
		}
	}
	pub fn max_score(nums: Vec<i32>) -> i32 {
		let n = nums.len() as i32 / 2;
		Self::solve(&nums, n, 0, &mut vec![-1; (1 << (n * 2)) as usize])
	}
	fn solve(nums: &[i32], n: i32, mask: i32, cache: &mut [i32]) -> i32 {
		if cache[mask as usize] >= 0 {
			return cache[mask as usize];
		}
		let mut ans = 0;
		for i in (0..nums.len() - 1).filter(|i| (1 << i) & mask == 0) {
			for j in (i + 1..nums.len()).filter(|j| (1 << j) & mask == 0) {
				ans = ans.max(
					Self::gcd(nums[i], nums[j]) * n
						+ Self::solve(nums, n - 1, mask | (1 << i) | (1 << j), cache),
				);
			}
		}
		cache[mask as usize] = ans;
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1799() {
		assert_eq!(
			Solution::max_score(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]),
			138
		);
	}
}
