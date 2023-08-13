/**
 * [2713] Maximum Strictly Increasing Cells in a Matrix
 *
 * Given a 1-indexed m x n integer matrix mat, you can select any cell in the matrix as your starting cell.
 * From the starting cell, you can move to any other cell in the same row or column, but only if the value of the destination cell is strictly greater than the value of the current cell. You can repeat this process as many times as possible, moving from cell to cell until you can no longer make any moves.
 * Your task is to find the maximum number of cells that you can visit in the matrix by starting from some cell.
 * Return an integer denoting the maximum number of cells that can be visited.
 *  
 * <strong class="example">Example 1:
 * <strong class="example"><img alt="" src="https://assets.leetcode.com/uploads/2023/04/23/diag1drawio.png" style="width: 200px; height: 176px;" />
 *
 * Input: mat = [[3,1],[3,4]]
 * Output: 2
 * Explanation: The image shows how we can visit 2 cells starting from row 1, column 2. It can be shown that we cannot visit more than 2 cells no matter where we start from, so the answer is 2.
 *
 * <strong class="example">Example 2:
 * <strong class="example"><img alt="" src="https://assets.leetcode.com/uploads/2023/04/23/diag3drawio.png" style="width: 200px; height: 176px;" />
 *
 * Input: mat = [[1,1],[1,1]]
 * Output: 1
 * Explanation: Since the cells must be strictly increasing, we can only visit one cell in this example.
 *
 * <strong class="example">Example 3:
 * <strong class="example"><img alt="" src="https://assets.leetcode.com/uploads/2023/04/23/diag4drawio.png" style="width: 350px; height: 250px;" />
 *
 * Input: mat = [[3,1,6],[-9,5,7]]
 * Output: 4
 * Explanation: The image above shows how we can visit 4 cells starting from row 2, column 1. It can be shown that we cannot visit more than 4 cells no matter where we start from, so the answer is 4.
 *
 *  
 * Constraints:
 *
 *     m == mat.length
 *     n == mat[i].length
 *     1 <= m, n <= 10^5
 *     1 <= m * n <= 10^5
 *     -10^5 <= mat[i][j] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_increasing_cells(mut mat: Vec<Vec<i32>>) -> i32 {
		let (m, n) = (mat.len(), mat[0].len());
		let mut dist = std::collections::BTreeMap::new();
		for (i, row) in mat.iter().enumerate() {
			for (j, &v) in row.iter().enumerate() {
				dist.entry(v).or_insert_with(Vec::new).push((i, j));
			}
		}
		let (mut row_max, mut col_max) = (vec![0; m], vec![0; n]);
		let mut ans = 0;
		for posistions in dist.into_values() {
			for &(i, j) in &posistions {
				mat[i][j] = row_max[i].max(col_max[j]) + 1;
			}
			for (i, j) in posistions {
				row_max[i] = row_max[i].max(mat[i][j]);
				col_max[j] = col_max[j].max(mat[i][j]);
				ans = ans.max(mat[i][j]);
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2713() {
		assert_eq!(Solution::max_increasing_cells(matrix![[3, 1], [3, 4]]), 2);
		assert_eq!(
			Solution::max_increasing_cells(matrix![
				[7, 6, 3],
				[-7, -5, 6],
				[-7, 0, -4],
				[6, 6, 0],
				[-8, 6, 0]
			]),
			7
		);
		assert_eq!(
			Solution::max_increasing_cells(matrix![[3, 1, 6], [-9, 5, 7]]),
			4
		);
		assert_eq!(
			Solution::max_increasing_cells(matrix![
				[-37, -50, -3, 44],
				[-37, 46, 13, -32],
				[47, -42, -3, -40],
				[-17, -22, -39, 24]
			]),
			6
		);
	}
}
