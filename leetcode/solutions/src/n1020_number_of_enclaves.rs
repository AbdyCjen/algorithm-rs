/**
 * [1020] Number of Enclaves
 *
 * You are given an m x n binary matrix grid, where 0 represents a sea cell and 1 represents a land cell.
 * A move consists of walking from one land cell to another adjacent (4-directionally) land cell or walking off the boundary of the grid.
 * Return the number of land cells in grid for which we cannot walk off the boundary of the grid in any number of moves.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/enclaves1.jpg" style="width: 333px; height: 333px;" />
 * Input: grid = [[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
 * Output: 3
 * Explanation: There are three 1s that are enclosed by 0s, and one 1 that is not enclosed because its on the boundary.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/enclaves2.jpg" style="width: 333px; height: 333px;" />
 * Input: grid = [[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
 * Output: 0
 * Explanation: All 1s are either on the boundary or can reach the boundary.
 *
 *  
 * Constraints:
 *
 *     m == grid.length
 *     n == grid[i].length
 *     1 <= m, n <= 500
 *     grid[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

impl Solution {
	pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
		fn dfs(grid: &mut [Vec<i32>], i: usize, j: usize) -> i32 {
			let mut ans = 1;
			grid[i][j] = 0;

			if i > 0 && grid[i - 1][j] == 1 {
				ans += dfs(grid, i - 1, j);
			}
			if i + 1 < grid.len() && grid[i + 1][j] == 1 {
				ans += dfs(grid, i + 1, j);
			}
			if j > 0 && grid[i][j - 1] == 1 {
				ans += dfs(grid, i, j - 1);
			}
			if j + 1 < grid[0].len() && grid[i][j + 1] == 1 {
				ans += dfs(grid, i, j + 1);
			}
			ans
		}
		let (m, n) = (grid.len(), grid[0].len());

		for i in 0..m {
			for (i, j) in [(i, 0), (i, n - 1)] {
				if grid[i][j] == 1 {
					dfs(&mut grid, i, j);
				}
			}
		}
		for j in 0..n {
			for (i, j) in [(0, j), (m - 1, j)] {
				if grid[i][j] == 1 {
					dfs(&mut grid, i, j);
				}
			}
		}

		let mut ans = 0;
		for i in 1..m - 1 {
			for j in 1..n - 1 {
				if grid[i][j] == 1 {
					ans += dfs(&mut grid, i, j);
				}
			}
		}
		ans
	}
}
