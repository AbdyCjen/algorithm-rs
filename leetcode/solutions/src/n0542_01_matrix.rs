/**
 * [542] 01 Matrix
 *
 * Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.
 * The distance between two adjacent cells is 1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/01-1-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
 * Output: [[0,0,0],[0,1,0],[0,0,0]]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/01-2-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
 * Output: [[0,0,0],[0,1,0],[1,2,1]]
 *
 *  
 * Constraints:
 *
 *     m == mat.length
 *     n == mat[i].length
 *     1 <= m, n <= 10^4
 *     1 <= m * n <= 10^4
 *     mat[i][j] is either 0 or 1.
 *     There is at least one 0 in mat.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let (m, n) = (mat.len() as i32, mat[0].len() as i32);
		let mut st = vec![];
		for (i, row) in (0..).zip(&mut mat) {
			for (j, blk) in (0..).zip(row) {
				if *blk == 0 {
					st.push((i, j));
				} else {
					*blk = -1;
				}
			}
		}
		let mut dis = 0;
		while !st.is_empty() {
			dis += 1;
			for (i, j) in std::mem::take(&mut st) {
				for (ii, jj) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
					if ii >= 0 && ii < m && jj >= 0 && jj < n && mat[ii as usize][jj as usize] < 0 {
						mat[ii as usize][jj as usize] = dis;
						st.push((ii, jj));
					}
				}
			}
		}
		mat
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_542() {
		assert_eq!(
			Solution::update_matrix(matrix![[0, 0, 0], [0, 1, 0], [0, 0, 0]]),
			matrix![[0, 0, 0], [0, 1, 0], [0, 0, 0]]
		);
	}
}
