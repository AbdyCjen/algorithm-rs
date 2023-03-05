/**
 * [1162] As Far from Land as Possible
 *
 * Given an n x n grid containing only values 0 and 1, where 0 represents water and 1 represents land, find a water cell such that its distance to the nearest land cell is maximized, and return the distance. If no land or water exists in the grid, return -1.
 * The distance used in this problem is the Manhattan distance: the distance between two cells (x0, y0) and (x1, y1) is |x0 - x1| + |y0 - y1|.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/05/03/1336_ex1.JPG" style="width: 185px; height: 87px;" />
 * Input: grid = [[1,0,1],[0,0,0],[1,0,1]]
 * Output: 2
 * Explanation: The cell (1, 1) is as far as possible from all the land with distance 2.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/05/03/1336_ex2.JPG" style="width: 184px; height: 87px;" />
 * Input: grid = [[1,0,0],[0,0,0],[0,0,0]]
 * Output: 4
 * Explanation: The cell (2, 2) is as far as possible from all the land with distance 4.
 *
 *  
 * Constraints:
 *
 *     n == grid.length
 *     n == grid[i].length
 *     1 <= n <= 100
 *     grid[i][j] is 0 or 1
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
		let n = grid.len() as i32;
		let mut st = vec![];
		for (r, i) in grid.iter().zip(0..) {
			for (&c, j) in r.iter().zip(0..) {
				if c == 1 {
					st.push((i, j));
				}
			}
		}
		if st.is_empty() || st.len() as i32 == n * n {
			return -1;
		}

		let mut dist = -1;
		while !st.is_empty() {
			dist += 1;
			for (i, j) in std::mem::take(&mut st) {
				for (ni, nj) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
					if ni >= 0 && ni < n && nj >= 0 && nj < n && grid[ni as usize][nj as usize] == 0
					{
						grid[ni as usize][nj as usize] = 1;
						st.push((ni, nj));
					}
				}
			}
		}

		dist
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1162() {
		assert_eq!(
			Solution::max_distance(matrix![[1, 0, 1], [0, 0, 0], [1, 0, 1]]),
			2
		);
	}
}
