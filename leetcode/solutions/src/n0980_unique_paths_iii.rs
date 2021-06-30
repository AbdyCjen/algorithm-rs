/**
 * [980] Unique Paths III
 *
 * On a 2-dimensional grid, there are 4 types of squares:
 *
 *
 *     1 represents the starting square.  There is exactly one starting square.
 *     2 represents the ending square.  There is exactly one ending square.
 *     0 represents empty squares we can walk over.
 *     -1 represents obstacles that we cannot walk over.
 *
 *
 * Return the number of 4-directional walks from the starting square to the ending square, that walk over every non-obstacle square exactly once.
 *
 *  
 *
 * <div>
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">[[1,0,0,0],[0,0,0,0],[0,0,2,-1]]</span>
 * Output: <span id="example-output-1">2</span>
 * Explanation: We have the following two paths:
 * 1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2)
 * 2. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2)
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">[[1,0,0,0],[0,0,0,0],[0,0,0,2]]</span>
 * Output: <span id="example-output-2">4</span>
 * Explanation: We have the following four paths:
 * 1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2),(2,3)
 * 2. (0,0),(0,1),(1,1),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,3),(1,3),(2,3)
 * 3. (0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(1,1),(0,1),(0,2),(0,3),(1,3),(2,3)
 * 4. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2),(2,3)
 *
 * <div>
 * Example 3:
 *
 *
 * Input: <span id="example-input-3-1">[[0,1],[2,0]]</span>
 * Output: <span id="example-output-3">0</span>
 * Explanation:
 * There is no path that walks over every empty square exactly once.
 * Note that the starting and ending square can be anywhere in the grid.
 *
 * </div>
 * </div>
 * </div>
 *
 *  
 *
 * Note:
 *
 * <ol>
 *     1 <= grid.length * grid[0].length <= 20
 * </ol>
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
		fn dfs(grid: &mut [Vec<i32>], i: usize, j: usize, cnt: i32) -> i32 {
			if grid[i][j] == 2 {
				if cnt == 0 {
					return 1;
				} else {
					return 0;
				}
			} else if grid[i][j] < 0 {
				return 0;
			}
			let (m, n) = (grid.len() as isize, grid[0].len() as isize);

			grid[i][j] = -2;
			let walked = |grid: &[Vec<i32>], i: isize, j: isize| {
				(0..m).contains(&i) && (0..n).contains(&j) && -2 == grid[i as usize][j as usize]
			};
			let valid_all = |grid: &[Vec<i32>], i: isize, j: isize| {
				(0..m).contains(&i)
					&& (0..n).contains(&j)
					&& [0, 2].contains(&grid[i as usize][j as usize])
			};
			let ans = [(0, 1), (0, -1), (-1, 0), (1, 0)]
				.iter()
				.filter_map(|(ii, jj)| {
					let (i, j) = (i as isize + ii, j as isize + jj);
					if valid_all(grid, i, j)
						&& (!walked(grid, i + ii, j + jj) // 额, 这个剪枝不用其实也能过...
							|| !valid_all(grid, i + jj, j + ii)
							|| !valid_all(grid, i - jj, j - ii))
					{
						Some(dfs(grid, i as usize, j as usize, cnt - 1))
					} else {
						None
					}
				})
				.sum();
			grid[i][j] = 0;

			ans
		}
		let (i, j) = grid
			.iter()
			.map(|row| row.iter().position(|&i| i == 1))
			.enumerate()
			.find_map(|(i, o)| Some((i, o?)))
			.unwrap_or((0, 0));
		let cnt = grid
			.iter()
			.map(|r| r.iter().filter(|i| **i == 0).count())
			.sum::<usize>() as i32;
		dfs(&mut grid, i, j, cnt + 1)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_980() {
		assert_eq!(
			Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
			2
		);
		assert_eq!(
			Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
			4
		);
		assert_eq!(
			Solution::unique_paths_iii(vec![
				vec![0, 0, 0, 0, 0],
				vec![0, 2, 1, 0, 0],
				vec![0, 0, 0, 0, 0],
				vec![0, 0, 0, 0, 0]
			]),
			8
		);
	}
}
