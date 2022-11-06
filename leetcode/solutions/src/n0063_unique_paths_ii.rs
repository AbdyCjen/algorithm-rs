/**
 * [63] Unique Paths II
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 * Now consider if some obstacles are added to the grids. How many unique paths would there be?
 * An obstacle and space is marked as 1 and 0 respectively in the grid.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/robot1.jpg" style="width: 242px; height: 242px;" />
 * Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
 * Output: 2
 * Explanation: There is one obstacle in the middle of the 3x3 grid above.
 * There are two ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down -> Down
 * 2. Down -> Down -> Right -> Right
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/robot2.jpg" style="width: 162px; height: 162px;" />
 * Input: obstacleGrid = [[0,1],[0,0]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     m == obstacleGrid.length
 *     n == obstacleGrid[i].length
 *     1 <= m, n <= 100
 *     obstacleGrid[i][j] is 0 or 1.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
		fn dfs(obstacle_grid: &[Vec<i32>], i: usize, j: usize, cache: &mut [Vec<i32>]) -> i32 {
			if i + 1 == obstacle_grid.len() && j + 1 == obstacle_grid[0].len() {
				return 1;
			} else if i == obstacle_grid.len() || j == obstacle_grid[0].len() {
				return 0;
			} else if cache[i][j] >= 0 {
				return cache[i][j];
			} else if obstacle_grid[i][j] == 1 {
				return 0;
			}

			cache[i][j] = dfs(obstacle_grid, i + 1, j, cache) + dfs(obstacle_grid, i, j + 1, cache);
			cache[i][j]
		}
		if obstacle_grid.is_empty() || 1.eq(obstacle_grid.last().unwrap().last().unwrap()) {
			return 0;
		}

		let mut cache = vec![vec![-1; obstacle_grid[0].len()]; obstacle_grid.len()];

		dfs(&obstacle_grid, 0, 0, &mut cache)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_63() {
		assert_eq!(
			Solution::unique_paths_with_obstacles(matrix![[0, 0, 0], [0, 1, 0], [0, 0, 0]]),
			2
		);
		assert_eq!(
			Solution::unique_paths_with_obstacles(matrix![[0, 1], [0, 0]]),
			1
		);
		assert_eq!(
			Solution::unique_paths_with_obstacles(matrix![[0, 0], [0, 1]]),
			0
		);
	}
}
