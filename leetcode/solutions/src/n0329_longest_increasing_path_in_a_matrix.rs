/**
 * [329] Longest Increasing Path in a Matrix
 *
 * Given an m x n integers matrix, return the length of the longest increasing path in matrix.
 * From each cell, you can either move in four directions: left, right, up, or down. You may not move diagonally or move outside the boundary (i.e., wrap-around is not allowed).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/05/grid1.jpg" style="width: 242px; height: 242px;" />
 * Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
 * Output: 4
 * Explanation: The longest increasing path is [1, 2, 6, 9].
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/27/tmp-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: matrix = [[3,4,5],[3,2,6],[2,2,1]]
 * Output: 4
 * Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
 *
 * <strong class="example">Example 3:
 *
 * Input: matrix = [[1]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     m == matrix.length
 *     n == matrix[i].length
 *     1 <= m, n <= 200
 *     0 <= matrix[i][j] <= 2^31 - 1
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn longest_increasing_path(grid: Vec<Vec<i32>>) -> i32 {
		let (m, n) = (grid.len(), grid[0].len());
		let mut ans = 0;
		let mut cache = vec![vec![0; n]; m];
		for i in 0..m as i32 {
			for j in 0..n as i32 {
				ans = ans.max(Self::dfs(&grid, i, j, &mut cache));
			}
		}
		ans
	}
	fn dfs(grid: &[Vec<i32>], i: i32, j: i32, cache: &mut [Vec<i32>]) -> i32 {
		if cache[i as usize][j as usize] > 0 {
			return cache[i as usize][j as usize];
		}
		let (m, n) = (grid.len() as i32, grid[0].len() as i32);
		let cur = grid[i as usize][j as usize];
		let mut ans = 1;
		for (ni, nj) in [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)] {
			if ni >= 0 && ni < m && nj >= 0 && nj < n && grid[ni as usize][nj as usize] > cur {
				ans = ans.max(1 + Self::dfs(grid, ni, nj, cache));
			}
		}
		cache[i as usize][j as usize] = ans;
		ans
	}
}

// submission codes end
