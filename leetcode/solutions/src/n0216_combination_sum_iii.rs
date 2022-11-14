/**
 * [216] Combination Sum III
 *
 * Find all valid combinations of k numbers that sum up to n such that the following conditions are true:
 *
 *     Only numbers 1 through 9 are used.
 *     Each number is used at most once.
 *
 * Return a list of all possible valid combinations. The list must not contain the same combination twice, and the combinations may be returned in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: k = 3, n = 7
 * Output: [[1,2,4]]
 * Explanation:
 * 1 + 2 + 4 = 7
 * There are no other valid combinations.
 * <strong class="example">Example 2:
 *
 * Input: k = 3, n = 9
 * Output: [[1,2,6],[1,3,5],[2,3,4]]
 * Explanation:
 * 1 + 2 + 6 = 9
 * 1 + 3 + 5 = 9
 * 2 + 3 + 4 = 9
 * There are no other valid combinations.
 *
 * <strong class="example">Example 3:
 *
 * Input: k = 4, n = 1
 * Output: []
 * Explanation: There are no valid combinations.
 * Using 4 different numbers in the range [1,9], the smallest sum we can get is 1+2+3+4 = 10 and since 10 > 1, there are no valid combination.
 *
 *  
 * Constraints:
 *
 *     2 <= k <= 9
 *     1 <= n <= 60
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
		let nums = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
		Self::solve(k, n, nums)
	}

	fn solve(k: i32, tar: i32, nums: &[i32]) -> Vec<Vec<i32>> {
		if k == 1 {
			return if nums.contains(&tar) {
				vec![vec![tar]]
			} else {
				Vec::new()
			};
		}

		let mut ans = Vec::new();
		for (i, &n) in nums.iter().take_while(|n| tar >= **n).enumerate() {
			for mut v in Self::solve(k - 1, tar - n, &nums[i + 1..]) {
				v.push(n);
				ans.push(v);
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
	fn test_216() {
		assert_eq!(Solution::combination_sum3(3, 7), matrix![[4, 2, 1]]);
		assert_eq!(
			Solution::combination_sum3(3, 9),
			matrix![[6, 2, 1], [5, 3, 1], [4, 3, 2]]
		);
		assert!(Solution::combination_sum3(4, 1).is_empty());
	}
}
