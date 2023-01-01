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

impl Solution {
	pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
		fn dfs(grid: &mut [Vec<i32>], i: i32, j: i32, cnt: i32) -> i32 {
			match (grid[i as usize][j as usize], cnt) {
				(2, 0) => return 1,
				(2 | -1, _) => return 0,
				_ => {}
			}
			let (m, n) = (grid.len() as i32, grid[0].len() as i32);
			grid[i as usize][j as usize] = -1;
			let mut s = 0;
			for (i, j) in [(i, j + 1), (i, j - 1), (i - 1, j), (i + 1, j)] {
				if i >= 0 && i < m && j >= 0 && j < n && grid[i as usize][j as usize] >= 0 {
					s += dfs(grid, i, j, cnt - 1);
				}
			}
			grid[i as usize][j as usize] = 0;
			s
		}
		let (mut pos, mut cnt) = ((0, 0), 0);
		for (row, i) in grid.iter().zip(0..) {
			for (&v, j) in row.iter().zip(0..) {
				match v {
					0 => cnt += 1,
					1 => pos = (i, j),
					_ => {}
				}
			}
		}
		dfs(&mut grid, pos.0, pos.1, cnt + 1)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_980() {
		assert_eq!(
			Solution::unique_paths_iii(matrix![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]]),
			2
		);
		assert_eq!(
			Solution::unique_paths_iii(matrix![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]]),
			4
		);
		assert_eq!(
			Solution::unique_paths_iii(matrix![
				[0, 0, 0, 0, 0],
				[0, 2, 1, 0, 0],
				[0, 0, 0, 0, 0],
				[0, 0, 0, 0, 0]
			]),
			8
		);
	}
}
