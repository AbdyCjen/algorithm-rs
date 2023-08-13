/**
 * [808] Soup Servings
 *
 * There are two types of soup: type A and type B. Initially, we have n ml of each type of soup. There are four kinds of operations:
 * <ol>
 *     Serve 100 ml of soup A and 0 ml of soup B,
 *     Serve 75 ml of soup A and 25 ml of soup B,
 *     Serve 50 ml of soup A and 50 ml of soup B, and
 *     Serve 25 ml of soup A and 75 ml of soup B.
 * </ol>
 * When we serve some soup, we give it to someone, and we no longer have it. Each turn, we will choose from the four operations with an equal probability 0.25. If the remaining volume of soup is not enough to complete the operation, we will serve as much as possible. We stop once we no longer have some quantity of both types of soup.
 * Note that we do not have an operation where all 100 ml's of soup B are used first.
 * Return the probability that soup A will be empty first, plus half the probability that A and B become empty at the same time. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 50
 * Output: 0.62500
 * Explanation: If we choose the first two operations, A will become empty first.
 * For the third operation, A and B will become empty at the same time.
 * For the fourth operation, B will become empty first.
 * So the total probability of A becoming empty first plus half the probability that A and B become empty at the same time, is 0.25 * (1 + 1 + 0.5 + 0) = 0.625.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 100
 * Output: 0.71875
 *
 *  
 * Constraints:
 *
 *     0 <= n <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn soup_servings(n: i32) -> f64 {
		let n = (n + 24) / 25;
		match n {
			192..=i32::MAX => 1.0,
			_ => Self::solve(n, n, &mut vec![vec![-1.0; n as usize + 1]; n as usize + 1]),
		}
	}
	fn solve(m: i32, n: i32, cache: &mut [Vec<f64>]) -> f64 {
		match (m, n) {
			(-3..=0, -3..=0) => 0.5,
			(-3..=0, _) => 1.0,
			(_, -3..=0) => 0.0,
			_ => {
				if cache[m as usize][n as usize] > 0.0 {
					return cache[m as usize][n as usize];
				}
				let mut ans = 0.0;
				for i in 1..=4 {
					ans += Self::solve(m - i, n - 4 + i, cache) / 4.0;
				}
				cache[m as usize][n as usize] = ans;
				ans
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_808() {
		assert_eq!(Solution::soup_servings(50), 0.625);
		assert!((Solution::soup_servings(50) - 0.625).abs() < 1e-5);
		assert!((Solution::soup_servings(4800) - 1.0).abs() < 1e-5);
	}
}
