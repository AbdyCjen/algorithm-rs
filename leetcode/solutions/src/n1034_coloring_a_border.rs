/**
 * [1034] Coloring A Border
 *
 * You are given an m x n integer matrix grid, and three integers row, col, and color. Each value in the grid represents the color of the grid square at that location.
 * Two squares belong to the same connected component if they have the same color and are next to each other in any of the 4 directions.
 * The border of a connected component is all the squares in the connected component that are either 4-directionally adjacent to a square not in the component, or on the boundary of the grid (the first or last row or column).
 * You should color the border of the connected component that contains the square grid[row][col] with color.
 * Return the final grid.
 *  
 * Example 1:
 * Input: grid = [[1,1],[1,2]], row = 0, col = 0, color = 3
 * Output: [[3,3],[3,2]]
 * Example 2:
 * Input: grid = [[1,2,2],[2,3,2]], row = 0, col = 1, color = 3
 * Output: [[1,3,3],[2,3,3]]
 * Example 3:
 * Input: grid = [[1,1,1],[1,1,1],[1,1,1]], row = 1, col = 1, color = 2
 * Output: [[2,2,2],[2,1,2],[2,2,2]]
 *  
 * Constraints:
 *
 *     m == grid.length
 *     n == grid[i].length
 *     1 <= m, n <= 50
 *     1 <= grid[i][j], color <= 1000
 *     0 <= row < m
 *     0 <= col < n
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	fn find(set: &mut [usize], i: usize) -> usize {
		if set[i] != i {
			set[i] = Self::find(set, set[i]);
		}
		set[i]
	}

	fn union(set: &mut [usize], mut i: usize, j: usize) {
		i = Self::find(set, i);
		set[i] = Self::find(set, j);
	}
	pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, colo: i32) -> Vec<Vec<i32>> {
		let (m, n) = (grid.len(), grid[0].len());
		let set = &mut (0..m * n).collect::<Vec<_>>()[..];
		let orig_color = grid[row as usize][col as usize];
		let idxmap = |i: usize, j: usize| (i * n + j);
		for (i, row) in grid.iter().enumerate() {
			for (j, color) in row.iter().enumerate() {
				if j != 0 && grid[i][j - 1] == *color {
					Self::union(set, idxmap(i, j), idxmap(i, j - 1));
				}
				if i != 0 && grid[i - 1][j] == *color {
					Self::union(set, idxmap(i, j), idxmap(i - 1, j));
				}
			}
		}

		let component = Self::find(set, idxmap(row as usize, col as usize));
		let mut is_border = |i: usize, j: usize| -> bool {
			Self::find(set, idxmap(i, j)) == component
				&& (i == 0
					|| j == 0 || i == m - 1
					|| j == n - 1 || grid[i - 1][j] != orig_color
					|| grid[i + 1][j] != orig_color
					|| grid[i][j - 1] != orig_color
					|| grid[i][j + 1] != orig_color)
		};
		let mut borders = Vec::new();
		for i in 0..m {
			for j in 0..n {
				if is_border(i, j) {
					borders.push((i, j));
				}
			}
		}
		for (i, j) in borders {
			grid[i][j] = colo;
		}

		grid
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1034() {
		assert_eq!(
			Solution::color_border(matrix![[1, 1], [1, 2]], 0, 0, 3),
			matrix![[3, 3], [3, 2]]
		);
		assert_eq!(
			Solution::color_border(matrix![[1, 2, 2], [2, 3, 2]], 0, 1, 3),
			matrix![[1, 3, 3], [2, 3, 3]]
		);
		assert_eq!(
			Solution::color_border(matrix![[2, 1, 2, 2], [2, 2, 1, 2], [1, 1, 2, 1]], 2, 2, 2),
			[[2, 1, 2, 2], [2, 2, 1, 2], [1, 1, 2, 1]]
		);

		assert_eq!(
			Solution::color_border(
				matrix![[1, 2, 1, 2, 1, 2], [2, 2, 2, 2, 1, 2], [1, 2, 2, 2, 1, 2]],
				1,
				3,
				1
			),
			matrix![[1, 1, 1, 1, 1, 2], [1, 2, 1, 1, 1, 2], [1, 1, 1, 1, 1, 2]]
		);
	}
}
