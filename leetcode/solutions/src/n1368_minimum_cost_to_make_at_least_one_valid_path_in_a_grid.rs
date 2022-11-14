/**
 * [1368] Minimum Cost to Make at Least One Valid Path in a Grid
 *
 * Given an m x n grid. Each cell of the grid has a sign pointing to the next cell you should visit if you are currently in this cell. The sign of grid[i][j] can be:
 *
 *     1 which means go to the cell to the right. (i.e go from grid[i][j] to grid[i][j + 1])
 *     2 which means go to the cell to the left. (i.e go from grid[i][j] to grid[i][j - 1])
 *     3 which means go to the lower cell. (i.e go from grid[i][j] to grid[i + 1][j])
 *     4 which means go to the upper cell. (i.e go from grid[i][j] to grid[i - 1][j])
 *
 * Notice that there could be some signs on the cells of the grid that point outside the grid.
 * You will initially start at the upper left cell (0, 0). A valid path in the grid is a path that starts from the upper left cell (0, 0) and ends at the bottom-right cell (m - 1, n - 1) following the signs on the grid. The valid path does not have to be the shortest.
 * You can modify the sign on a cell with cost = 1. You can modify the sign on a cell one time only.
 * Return the minimum cost to make the grid have at least one valid path.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/13/grid1.png" style="width: 400px; height: 390px;" />
 * Input: grid = [[1,1,1,1],[2,2,2,2],[1,1,1,1],[2,2,2,2]]
 * Output: 3
 * Explanation: You will start at point (0, 0).
 * The path to (3, 3) is as follows. (0, 0) --> (0, 1) --> (0, 2) --> (0, 3) change the arrow to down with cost = 1 --> (1, 3) --> (1, 2) --> (1, 1) --> (1, 0) change the arrow to down with cost = 1 --> (2, 0) --> (2, 1) --> (2, 2) --> (2, 3) change the arrow to down with cost = 1 --> (3, 3)
 * The total cost = 3.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/13/grid2.png" style="width: 350px; height: 341px;" />
 * Input: grid = [[1,1,3],[3,2,2],[1,1,4]]
 * Output: 0
 * Explanation: You can follow the path from (0, 0) to (2, 2).
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/13/grid3.png" style="width: 200px; height: 192px;" />
 * Input: grid = [[1,2],[4,3]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     m == grid.length
 *     n == grid[i].length
 *     1 <= m, n <= 100
 *     1 <= grid[i][j] <= 4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
		let (m, n) = (grid.len(), grid[0].len());
		let mut mat = vec![vec![i32::MAX; n]; m];
		let dirs = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
		let valid = |i: i32, j: i32| i >= 0 && i < m as _ && j >= 0 && j < n as _;
		let next = |i: i32, j: i32| -> Option<(i32, i32)> {
			let dir = dirs[grid[i as usize][j as usize] as usize - 1];
			let (k, l) = (i + dir.0, j + dir.1);
			if valid(k, l) {
				Some((k, l))
			} else {
				None
			}
		};

		let mut dq = vec![(0, 0)];
		let mut cost = 0;
		while !dq.is_empty() && mat[m - 1][n - 1] > cost {
			for (mut i, mut j) in dq.split_off(0) {
				loop {
					if mat[i as usize][j as usize] <= cost {
						break;
					}
					mat[i as usize][j as usize] = cost;
					for dir in dirs {
						let (k, l) = (i + dir.0, j + dir.1);
						if valid(k, l) && mat[k as usize][l as usize] > cost + 2 {
							mat[k as usize][l as usize] = cost + 2;
							dq.push((k, l));
						}
					}

					if let Some((k, l)) = next(i, j) {
						i = k;
						j = l;
					} else {
						break;
					}
				}
			}
			cost += 1
		}

		mat[m - 1][n - 1]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1368() {
		assert_eq!(
			Solution::min_cost(matrix![
				[1, 1, 1, 1],
				[2, 2, 2, 2],
				[1, 1, 1, 1],
				[2, 2, 2, 2]
			]),
			3
		);
		assert_eq!(
			Solution::min_cost(matrix![[1, 1, 3], [3, 2, 2], [1, 1, 4]]),
			0
		);
		assert_eq!(Solution::min_cost(matrix![[1, 2], [4, 3]]), 1);
		assert_eq!(Solution::min_cost(matrix![[1, 3], [4, 2]]), 0);
	}
}
