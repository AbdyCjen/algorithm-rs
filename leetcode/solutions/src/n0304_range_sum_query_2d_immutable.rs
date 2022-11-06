/**
 * [304] Range Sum Query 2D - Immutable
 *
 * Given a 2D matrix matrix, handle multiple queries of the following type:
 * <ol>
 *     Calculate the sum of the elements of matrix inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
 * </ol>
 * Implement the NumMatrix class:
 *
 *     NumMatrix(int[][] matrix) Initializes the object with the integer matrix matrix.
 *     int sumRegion(int row1, int col1, int row2, int col2) Returns the sum of the elements of matrix inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/sum-grid.jpg" style="width: 415px; height: 415px;" />
 * Input
 * ["NumMatrix", "sumRegion", "sumRegion", "sumRegion"]
 * [[[[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]], [2, 1, 4, 3], [1, 1, 2, 2], [1, 2, 2, 4]]
 * Output
 * [null, 8, 11, 12]
 * Explanation
 * NumMatrix numMatrix = new NumMatrix([[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]);
 * numMatrix.sumRegion(2, 1, 4, 3); // return 8 (i.e sum of the red rectangle)
 * numMatrix.sumRegion(1, 1, 2, 2); // return 11 (i.e sum of the green rectangle)
 * numMatrix.sumRegion(1, 2, 2, 4); // return 12 (i.e sum of the blue rectangle)
 *
 *  
 * Constraints:
 *
 *     m == matrix.length
 *     n == matrix[i].length
 *     1 <= m, n <= 200
 *     -10^5 <= matrix[i][j] <= 10^5
 *     0 <= row1 <= row2 < m
 *     0 <= col1 <= col2 < n
 *     At most 10^4 calls will be made to sumRegion.
 *
 */

// submission codes start here

struct NumMatrix {
	mat_sum: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl NumMatrix {
	fn new(matrix: Vec<Vec<i32>>) -> Self {
		let (rows, cols) = (matrix.len(), matrix[0].len());
		let mut mat = vec![vec![0; cols + 1]; rows + 1];

		for (i, row) in (1..=rows).zip(matrix) {
			for (j, blk) in (1..=cols).zip(row) {
				mat[i][j] = mat[i - 1][j] + mat[i][j - 1] - mat[i - 1][j - 1] + blk;
			}
		}

		Self { mat_sum: mat }
	}

	fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
		let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
		let mat_sum = &self.mat_sum;

		mat_sum[row2 + 1][col2 + 1] + mat_sum[row1][col1]
			- mat_sum[row2 + 1][col1]
			- mat_sum[row1][col2 + 1]
	}
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_304() {
		let obj = NumMatrix::new(matrix![
			[3, 0, 1, 4, 2],
			[5, 6, 3, 2, 1],
			[1, 2, 0, 1, 5],
			[4, 1, 0, 1, 7],
			[1, 0, 3, 0, 5],
		]);
		assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
		assert_eq!(obj.sum_region(1, 1, 2, 2), 11);
		assert_eq!(obj.sum_region(1, 2, 2, 4), 12);
	}
}
