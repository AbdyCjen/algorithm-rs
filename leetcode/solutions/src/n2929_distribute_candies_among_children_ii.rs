/**
 * [2929] Distribute Candies Among Children II
 *
 * You are given two positive integers n and limit.
 * Return the total number of ways to distribute n candies among 3 children such that no child gets more than limit candies.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 5, limit = 2
 * Output: 3
 * Explanation: There are 3 ways to distribute 5 candies such that no child gets more than 2 candies: (1, 2, 2), (2, 1, 2) and (2, 2, 1).
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 3, limit = 3
 * Output: 10
 * Explanation: There are 10 ways to distribute 3 candies such that no child gets more than 3 candies: (0, 0, 3), (0, 1, 2), (0, 2, 1), (0, 3, 0), (1, 0, 2), (1, 1, 1), (1, 2, 0), (2, 0, 1), (2, 1, 0) and (3, 0, 0).
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^6
 *     1 <= limit <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn distribute_candies(n: i32, lim: i32) -> i64 {
		let max = n.min(lim);
		let mut ans = 0;
		for i in 0..=max {
			ans += Self::dist2(n - i, lim);
		}
		ans
	}

	fn dist2(n: i32, lim: i32) -> i64 {
		if n > 2 * lim {
			0
		} else {
			let min = (n - lim).max(0);
			let max = n.min(lim);
			(max - min + 1) as i64
		}
	}
}

// submission codes end
