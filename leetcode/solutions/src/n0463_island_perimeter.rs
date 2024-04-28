/**
 * [463] Island Perimeter
 *
 * You are given row x col grid representing a map where grid[i][j] = 1 represents land and grid[i][j] = 0 represents water.
 * Grid cells are connected horizontally/vertically (not diagonally). The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).
 * The island doesn't have "lakes", meaning the water inside isn't connected to the water around the island. One cell is a square with side length 1. The grid is rectangular, width and height don't exceed 100. Determine the perimeter of the island.
 *  
 * <strong class="example">Example 1:
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/island.png" style="width: 221px; height: 213px;" />
 * Input: grid = [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]]
 * Output: 16
 * Explanation: The perimeter is the 16 yellow stripes in the image above.
 *
 * <strong class="example">Example 2:
 *
 * Input: grid = [[1]]
 * Output: 4
 *
 * <strong class="example">Example 3:
 *
 * Input: grid = [[1,0]]
 * Output: 4
 *
 *  
 * Constraints:
 *
 *     row == grid.length
 *     col == grid[i].length
 *     1 <= row, col <= 100
 *     grid[i][j] is 0 or 1.
 *     There is exactly one island in grid.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
		let mut ans = 0;
		let (m, n) = (grid.len(), grid[0].len());
		for (i, row) in (0..).zip(&grid) {
			for (j, c) in (0..).zip(row) {
				if *c == 1 {
					if i == 0 || grid[i - 1][j] == 0 {
						ans += 1;
					}
					if j == 0 || grid[i][j - 1] == 0 {
						ans += 1;
					}
					if i + 1 == m || grid[i + 1][j] == 0 {
						ans += 1;
					}
					if j + 1 == n || grid[i][j + 1] == 0 {
						ans += 1;
					}
				}
			}
		}
		ans
	}
}

// submission codes end
