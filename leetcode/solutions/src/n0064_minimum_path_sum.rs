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

impl Solution {
	pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
		let (m, n) = (grid.len(), grid[0].len());
		for j in 1..n {
			grid[0][j] += grid[0][j - 1];
		}
		for i in 1..m {
			grid[i][0] += grid[i - 1][0];
		}
		for i in 1..m {
			for j in 1..n {
				grid[i][j] += grid[i - 1][j].min(grid[i][j - 1]);
			}
		}
		grid[m - 1][n - 1]
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
