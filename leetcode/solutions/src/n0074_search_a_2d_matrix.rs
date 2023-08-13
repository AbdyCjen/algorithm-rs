/**
 * [74] Search a 2D Matrix
 *
 * You are given an m x n integer matrix matrix with the following two properties:
 *
 *     Each row is sorted in non-decreasing order.
 *     The first integer of each row is greater than the last integer of the previous row.
 *
 * Given an integer target, return true if target is in matrix or false otherwise.
 * You must write a solution in O(log(m * n)) time complexity.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/05/mat.jpg" style="width: 322px; height: 242px;" />
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
 * Output: true
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/05/mat2.jpg" style="width: 322px; height: 242px;" />
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
 * Output: false
 *
 *  
 * Constraints:
 *
 *     m == matrix.length
 *     n == matrix[i].length
 *     1 <= m, n <= 100
 *     -10^4 <= matrix[i][j], target <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn search_matrix1(mat: Vec<Vec<i32>>, x: i32) -> bool {
		let i = mat.partition_point(|v| v[0] <= x).saturating_sub(1);
		mat[i].binary_search(&x).is_ok()
	}
	pub fn search_matrix(mat: Vec<Vec<i32>>, x: i32) -> bool {
		let (m, n) = (mat.len(), mat[0].len());
		let (mut l, mut r) = (0, m * n - 1);
		while l < r {
			let mid = (l + r) / 2;
			let y = mat[mid / n][mid % n];
			if y >= x {
				r = mid;
			} else {
				l = mid + 1;
			}
		}
		mat[r / n][r % n] == x
	}
}

// submission codes end
