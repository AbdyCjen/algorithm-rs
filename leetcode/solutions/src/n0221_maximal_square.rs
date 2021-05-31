/**
 * [221] Maximal Square
 *
 * Given a 2D binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.
 *
 * Example:
 *
 *
 * Input:
 *
 * 1 0 1 0 0
 * 1 0 1 1 1
 * 1 1 1 1 1
 * 1 0 0 1 0
 *
 * Output: 4
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn maximal_square(mat: Vec<Vec<char>>) -> i32 {
		if mat.is_empty() {
			return 0;
		}
		let mut dp = vec![vec![0; mat[0].len()]; mat.len()];
		let mut res = 0;
		for (c, dp) in mat[0].iter().zip(dp[0].iter_mut()) {
			if *c == '1' {
				*dp = 1;
				res = 1;
			}
		}
		for (i, row) in mat.into_iter().enumerate().skip(1) {
			if row[0] == '1' {
				dp[i][0] = 1;
				res = std::cmp::max(res, 1);
			}
			for (j, c) in row.into_iter().enumerate().skip(1) {
				if c == '1' {
					dp[i][j] = *[dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1]]
						.iter()
						.min()
						.unwrap() + 1;
					res = std::cmp::max(res, dp[i][j]);
				}
			}
		}
		res * res
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_221() {
		assert_eq!(
			Solution::maximal_square(vec![
				vec_char![1, 0, 1, 0, 0],
				vec_char![1, 0, 1, 1, 1],
				vec_char![1, 1, 1, 1, 1],
				vec_char![1, 0, 0, 1, 0]
			]),
			4
		);
		assert_eq!(
			Solution::maximal_square(vec![
				vec_char![1, 1, 1, 0, 0],
				vec_char![1, 1, 1, 0, 1],
				vec_char![1, 1, 1, 1, 1],
				vec_char![0, 1, 1, 0, 1]
			]),
			9
		);
		assert_eq!(Solution::maximal_square(vec![vec_char![0, 1]]), 1);
		assert_eq!(Solution::maximal_square(vec![vec_char![1]]), 1);
	}
}
