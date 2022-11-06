/**
 * [766] Toeplitz Matrix
 *
 * Given an m x n matrix, return true if the matrix is Toeplitz. Otherwise, return false.
 * A matrix is Toeplitz if every diagonal from top-left to bottom-right has the same elements.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/ex1.jpg" style="width: 322px; height: 242px;" />
 * Input: matrix = [[1,2,3,4],[5,1,2,3],[9,5,1,2]]
 * Output: true
 * Explanation:
 * In the above grid, the diagonals are:
 * "[9]", "[5, 5]", "[1, 1, 1]", "[2, 2, 2]", "[3, 3]", "[4]".
 * In each diagonal all elements are the same, so the answer is True.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/04/ex2.jpg" style="width: 162px; height: 162px;" />
 * Input: matrix = [[1,2],[2,2]]
 *
 * Output: false
 * Explanation:
 * The diagonal "[1, 2]" has different elements.
 *
 *  
 * Constraints:
 *
 *     m == matrix.length
 *     n == matrix[i].length
 *     1 <= m, n <= 20
 *     0 <= matrix[i][j] <= 99
 *
 *  
 * Follow up:
 *
 *     What if the matrix is stored on disk, and the memory is limited such that you can only load at most one row of the matrix into the memory at once?
 *     What if the matrix is so large that you can only load up a partial row into the memory at once?
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn is_toeplitz_matrix(mat: Vec<Vec<i32>>) -> bool {
		let (m, n) = (mat.len(), mat[0].len());
		let wid = m.min(n);
		for i in 0..m {
			let prv = mat[i][0];
			for j in 0..wid {
				if i + j < m && j < n && mat[i + j][j] != prv {
					return false;
				}
			}
		}

		for i in 0..n {
			let prv = mat[0][i];
			for (j, row) in mat.iter().enumerate() {
				if i + j < n && row[i + j] != prv {
					return false;
				}
			}
		}

		true
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_766() {
		let mat = matrix![[1, 2, 3, 4], [5, 1, 2, 3], [9, 5, 1, 2]];
		assert!(Solution::is_toeplitz_matrix(mat));
		let mat = matrix![[1, 2], [2, 2]];
		assert!(!Solution::is_toeplitz_matrix(mat));
		let mat = matrix![[22, 0, 94, 45, 46, 96], [10, 22, 80, 94, 45, 46]];
		assert!(!Solution::is_toeplitz_matrix(mat));
	}
}
