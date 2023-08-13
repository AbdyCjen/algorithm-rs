/**
 * [2328] Number of Increasing Paths in a Grid
 *
 * You are given an m x n integer matrix grid, where you can move from a cell to any adjacent cell in all 4 directions.
 * Return the number of strictly increasing paths in the grid such that you can start from any cell and end at any cell. Since the answer may be very large, return it modulo 10^9 + 7.
 * Two paths are considered different if they do not have exactly the same sequence of visited cells.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/10/griddrawio-4.png" style="width: 181px; height: 121px;" />
 * Input: grid = [[1,1],[3,4]]
 * Output: 8
 * Explanation: The strictly increasing paths are:
 * - Paths with length 1: [1], [1], [3], [4].
 * - Paths with length 2: [1 -> 3], [1 -> 4], [3 -> 4].
 * - Paths with length 3: [1 -> 3 -> 4].
 * The total number of paths is 4 + 3 + 1 = 8.
 *
 * <strong class="example">Example 2:
 *
 * Input: grid = [[1],[2]]
 * Output: 3
 * Explanation: The strictly increasing paths are:
 * - Paths with length 1: [1], [2].
 * - Paths with length 2: [1 -> 2].
 * The total number of paths is 2 + 1 = 3.
 *
 *  
 * Constraints:
 *
 *     m == grid.length
 *     n == grid[i].length
 *     1 <= m, n <= 1000
 *     1 <= m * n <= 10^5
 *     1 <= grid[i][j] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
		let (m, n) = (grid.len(), grid[0].len());
		let mut ans = 0;
		let mut cache = vec![vec![0; n]; m];
		const MO: i32 = 1e9 as i32 + 7;
		for i in 0..m as i32 {
			for j in 0..n as i32 {
				ans = (ans + Self::dfs(&grid, i, j, &mut cache)) % MO;
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
		const MO: i32 = 1e9 as i32 + 7;
		for (ni, nj) in [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)] {
			if ni >= 0 && ni < m && nj >= 0 && nj < n && grid[ni as usize][nj as usize] > cur {
				ans = (ans + Self::dfs(grid, ni, nj, cache)) % MO;
			}
		}
		cache[i as usize][j as usize] = ans;
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2328() {
		assert_eq!(Solution::count_paths(matrix![[1, 1], [3, 4]]), 8);
	}
}
