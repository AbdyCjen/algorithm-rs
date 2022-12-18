/**
 * [2503] Maximum Number of Points From Grid Queries
 *
 * You are given an m x n integer matrix grid and an array queries of size k.
 * Find an array answer of size k such that for each integer queres[i] you start in the top left cell of the matrix and repeat the following process:
 *
 *     If queries[i] is strictly greater than the value of the current cell that you are in, then you get one point if it is your first time visiting this cell, and you can move to any adjacent cell in all 4 directions: up, down, left, and right.
 *     Otherwise, you do not get any points, and you end this process.
 *
 * After the process, answer[i] is the maximum number of points you can get. Note that for each query you are allowed to visit the same cell multiple times.
 * Return the resulting array answer.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/10/19/yetgriddrawio.png" style="width: 571px; height: 151px;" />
 * Input: grid = [[1,2,3],[2,5,7],[3,5,1]], queries = [5,6,2]
 * Output: [5,8,1]
 * Explanation: The diagrams above show which cells we visit to get points for each query.
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/10/20/yetgriddrawio-2.png" />
 * Input: grid = [[5,2,1],[1,1,2]], queries = [3]
 * Output: [0]
 * Explanation: We can not get any points because the value of the top left cell is already greater than or equal to 3.
 *
 *  
 * Constraints:
 *
 *     m == grid.length
 *     n == grid[i].length
 *     2 <= m, n <= 1000
 *     4 <= m * n <= 10^5
 *     k == queries.length
 *     1 <= k <= 10^4
 *     1 <= grid[i][j], queries[i] <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_points(mut grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
		let (m, n) = (grid.len() as i32, grid[0].len() as i32);
		let mut bh = std::collections::BinaryHeap::new();
		let mut cnt = vec![];
		let mut cur = (grid[0][0], 0);
		bh.push((-grid[0][0], 0, 0));
		grid[0][0] = -1;
		while let Some((nn, i, j)) = bh.pop() {
			if -nn > cur.0 {
				cnt.push(cur);
				cur.0 = -nn;
			}
			cur.1 += 1;
			for (i, j) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
				if i >= 0 && j >= 0 && i < m && j < n && grid[i as usize][j as usize] > 0 {
					bh.push((-grid[i as usize][j as usize], i, j));
					grid[i as usize][j as usize] = -1;
				}
			}
		}
		cnt.push(cur);
		queries
			.into_iter()
			.map(|q| match cnt.binary_search_by_key(&(q - 1), |&(a, _)| a) {
				Ok(i) => cnt[i].1,
				Err(i) if i > 0 => cnt[i - 1].1,
				_ => 0,
			})
			.collect()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2503() {
		assert_eq!(
			Solution::max_points(matrix![[1, 2, 3], [2, 5, 7], [3, 5, 1]], vec![5, 6, 2]),
			[5, 8, 1]
		);
	}
}
