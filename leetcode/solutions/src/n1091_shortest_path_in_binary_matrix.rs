/**
 * [1091] Shortest Path in Binary Matrix
 *
 * Given an n x n binary matrix grid, return the length of the shortest clear path in the matrix. If there is no clear path, return -1.
 * A clear path in a binary matrix is a path from the top-left cell (i.e., (0, 0)) to the bottom-right cell (i.e., (n - 1, n - 1)) such that:
 *
 *     All the visited cells of the path are 0.
 *     All the adjacent cells of the path are 8-directionally connected (i.e., they are different and they share an edge or a corner).
 *
 * The length of a clear path is the number of visited cells of this path.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/example1_1.png" style="width: 500px; height: 234px;" />
 * Input: grid = [[0,1],[1,0]]
 * Output: 2
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/example2_1.png" style="height: 216px; width: 500px;" />
 * Input: grid = [[0,0,0],[1,1,0],[1,1,0]]
 * Output: 4
 *
 * <strong class="example">Example 3:
 *
 * Input: grid = [[1,0,0],[1,1,0],[1,1,0]]
 * Output: -1
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
	pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
		if grid[0][0] == 1 {
			return -1;
		}
		let mut dq = vec![(0, 0)];
		let (m, n) = (grid.len() as i32, grid[0].len() as i32);
		grid[0][0] = 1;
		let end = (m - 1, n - 1);
		let mut ans = 0;
		while !dq.is_empty() {
			ans += 1;
			for (i, j) in std::mem::take(&mut dq) {
				if (i, j) == end {
					return ans;
				}
				for i in 0.max(i - 1)..m.min(i + 2) {
					for j in 0.max(j - 1)..n.min(j + 2) {
						if grid[i as usize][j as usize] == 0 {
							grid[i as usize][j as usize] = 1;
							dq.push((i, j));
						}
					}
				}
			}
		}
		-1
	}
}

// submission codes end
