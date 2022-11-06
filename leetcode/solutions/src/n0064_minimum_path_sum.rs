/**
 * [64] Minimum Path Sum
 *
 * Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.
 *
 * Note: You can only move either down or right at any point in time.
 *
 * Example:
 *
 *
 * Input:
 * [
 *   [1,3,1],
 *   [1,5,1],
 *   [4,2,1]
 * ]
 * Output: 7
 * Explanation: Because the path 1&rarr;3&rarr;1&rarr;1&rarr;1 minimizes the sum.
 *
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
		if grid.is_empty() {
			return 0;
		}
		let (rl, cl) = (grid.len(), grid[0].len());
		let mut st = vec![vec![0_i32; cl]; rl];
		st[0][0] = grid[0][0];
		for i in 1..cl {
			st[0][i] = st[0][i - 1] + grid[0][i];
		}
		for i in 1..rl {
			st[i][0] = st[i - 1][0] + grid[i][0];
		}
		for i in 1..rl {
			for j in 1..cl {
				st[i][j] = std::cmp::min(st[i - 1][j], st[i][j - 1]) + grid[i][j];
			}
		}
		st[rl - 1][cl - 1]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_64() {
		assert_eq!(Solution::min_path_sum(matrix![[1, 2, 5], [3, 2, 1]]), 6);
		assert_eq!(
			Solution::min_path_sum(matrix![[1, 3, 1], [1, 5, 1], [4, 2, 1]]),
			7
		)
	}
}
