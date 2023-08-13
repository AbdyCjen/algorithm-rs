/**
 * [1631] Path With Minimum Effort
 *
 * You are a hiker preparing for an upcoming hike. You are given heights, a 2D array of size rows x columns, where heights[row][col] represents the height of cell (row, col). You are situated in the top-left cell, (0, 0), and you hope to travel to the bottom-right cell, (rows-1, columns-1) (i.e., 0-indexed). You can move up, down, left, or right, and you wish to find a route that requires the minimum effort.
 *
 * A route's effort is the maximum absolute difference in heights between two consecutive cells of the route.
 *
 * Return the minimum effort required to travel from the top-left cell to the bottom-right cell.
 *
 *  
 * <strong class="example">Example 1:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/04/ex1.png" style="width: 300px; height: 300px;" />
 *
 *
 * Input: heights = [[1,2,2],[3,8,2],[5,3,5]]
 * Output: 2
 * Explanation: The route of [1,3,5,3,5] has a maximum absolute difference of 2 in consecutive cells.
 * This is better than the route of [1,2,2,2,5], where the maximum absolute difference is 3.
 *
 *
 * <strong class="example">Example 2:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/04/ex2.png" style="width: 300px; height: 300px;" />
 *
 *
 * Input: heights = [[1,2,3],[3,8,4],[5,3,5]]
 * Output: 1
 * Explanation: The route of [1,2,3,4,5] has a maximum absolute difference of 1 in consecutive cells, which is better than route [1,3,5,3,5].
 *
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/04/ex3.png" style="width: 300px; height: 300px;" />
 *
 * Input: heights = [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]
 * Output: 0
 * Explanation: This route does not require any effort.
 *
 *
 *  
 * Constraints:
 *
 *
 *     rows == heights.length
 *     columns == heights[i].length
 *     1 <= rows, columns <= 100
 *     1 <= heights[i][j] <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn minimum_effort_path(mat: Vec<Vec<i32>>) -> i32 {
		let (m, n) = (mat.len(), mat[0].len());
		let mut edges = std::collections::BinaryHeap::new();
		edges.push((-(mat[0][1] - mat[0][0]).abs(), (0, 0), (0, 1)));
		edges.push((-(mat[1][0] - mat[0][0]).abs(), (0, 0), (1, 0)));
		let mut dis = vec![vec![i32::MAX; n]; m];
		dis[0][0] = 0;
		while let Some((nw, f, t)) = edges.pop() {
			if dis[f.0][f.1].max(-nw) < dis[t.0][t.1] {
				dis[t.0][t.1] = dis[f.0][f.1].max(-nw);
				if t.0 > 0 {
					let w = (mat[t.0 - 1][t.1] - mat[t.0][t.1]).abs();
					edges.push((-w, t, (t.0 - 1, t.1)));
				}
				if t.0 + 1 < m {
					let w = (mat[t.0 + 1][t.1] - mat[t.0][t.1]).abs();
					edges.push((-w, t, (t.0 + 1, t.1)));
				}
				if t.1 > 0 {
					let w = (mat[t.0][t.1 - 1] - mat[t.0][t.1]).abs();
					edges.push((-w, t, (t.0, t.1 - 1)));
				}
				if t.1 + 1 < n {
					let w = (mat[t.0][t.1 + 1] - mat[t.0][t.1]).abs();
					edges.push((-w, t, (t.0, t.1 + 1)));
				}
			}
		}

		dis[m - 1][n - 1]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1631() {
		assert_eq!(
			Solution::minimum_effort_path(matrix![[1, 2, 2], [3, 8, 2], [5, 3, 5]]),
			2
		);
	}
}
