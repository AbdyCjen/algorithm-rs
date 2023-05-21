/**
 * [934] Shortest Bridge
 *
 * You are given an n x n binary matrix grid where 1 represents land and 0 represents water.
 * An island is a 4-directionally connected group of 1's not connected to any other 1's. There are exactly two islands in grid.
 * You may change 0's to 1's to connect the two islands to form one island.
 * Return the smallest number of 0's you must flip to connect the two islands.
 *  
 * <strong class="example">Example 1:
 *
 * Input: grid = [[0,1],[1,0]]
 * Output: 1
 *
 * <strong class="example">Example 2:
 *
 * Input: grid = [[0,1,0],[0,0,0],[0,0,1]]
 * Output: 2
 *
 * <strong class="example">Example 3:
 *
 * Input: grid = [[1,1,1,1,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,0,0,1],[1,1,1,1,1]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     n == grid.length == grid[i].length
 *     2 <= n <= 100
 *     grid[i][j] is either 0 or 1.
 *     There are exactly two islands in grid.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
		let mut dq = vec![];
		let (m, n) = (grid.len() as i32, grid[0].len() as i32);
		for i in 0..m {
			if let Some(j) = (0..n).find(|&j| grid[i as usize][j as usize] == 1) {
				Self::dfs(&mut grid, (i, j), &mut dq);
				break;
			}
		}
		let mut ans = 0;
		while !dq.is_empty() {
			ans += 1;
			for (i, j) in std::mem::take(&mut dq) {
				for (ni, nj) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
					if (0..m).contains(&ni) && (0..n).contains(&nj) {
						match grid[ni as usize][nj as usize] {
							0 => {
								grid[ni as usize][nj as usize] = 2;
								dq.push((ni, nj));
							}
							1 => return ans,
							_ => {}
						}
					}
				}
			}
		}
		ans
	}

	fn dfs(grid: &mut [Vec<i32>], (i, j): (i32, i32), dq: &mut Vec<(i32, i32)>) {
		grid[i as usize][j as usize] = 2;
		let (m, n) = (grid.len() as i32, grid[0].len() as i32);
		for (ni, nj) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
			if (0..m).contains(&ni) && (0..n).contains(&nj) {
				match &mut grid[ni as usize][nj as usize] {
					c @ 0 => {
						dq.push((ni, nj));
						*c = 2;
					}
					1 => Self::dfs(grid, (ni, nj), dq),
					_ => {}
				}
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_934() {
		assert_eq!(
			Solution::shortest_bridge(matrix![
				[0, 1, 0, 0, 0],
				[0, 1, 0, 1, 1],
				[0, 0, 0, 0, 1],
				[0, 0, 0, 0, 0],
				[0, 0, 0, 0, 0]
			]),
			1
		);
	}
}
