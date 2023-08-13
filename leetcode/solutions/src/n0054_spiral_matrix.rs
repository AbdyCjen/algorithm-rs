/**
 * [54] Spiral Matrix
 *
 * Given an m x n matrix, return all elements of the matrix in spiral order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral1.jpg" style="width: 242px; height: 242px;" />
 * Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [1,2,3,6,9,8,7,4,5]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral.jpg" style="width: 322px; height: 242px;" />
 * Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
 * Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 *
 *  
 * Constraints:
 *
 *     m == matrix.length
 *     n == matrix[i].length
 *     1 <= m, n <= 10
 *     -100 <= matrix[i][j] <= 100
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	// pure iterator, strictly O(m * n) ==> makes no difference when m, n in [1, 10]
	pub fn spiral_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
		let mut ans = vec![];
		let mat: Vec<_> = mat.into_iter().map(|v| v.into_iter()).collect();
		let mut mat = mat.into_iter();
		while let (Some(head), tail) = (mat.next(), mat.next_back()) {
			ans.extend(head);
			let mat = mat.as_mut_slice();
			ans.extend(mat.iter_mut().flat_map(|row| row.next_back()));
			ans.extend(tail.into_iter().flatten().rev());
			ans.extend(mat.iter_mut().rev().flat_map(|row| row.next()));
		}
		ans
	}

	// partial iterator, O(m^2 * n)
	pub fn spiral_order1(mat: Vec<Vec<i32>>) -> Vec<i32> {
		let mut ans = vec![];
		let mut mat: Vec<_> = mat.into_iter().map(|v| v.into_iter()).collect();
		while !mat.is_empty() {
			ans.extend(mat.remove(0));
			let last = mat.pop();
			ans.extend(mat.iter_mut().flat_map(|row| row.next_back()));
			ans.extend(last.into_iter().flatten().rev());
			ans.extend(mat.iter_mut().rev().flat_map(|row| row.next()));
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_54() {
		assert_eq!(
			Solution::spiral_order(matrix![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12],]),
			vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
		);
		assert_eq!(
			Solution::spiral_order(matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9],]),
			vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
		);
	}
}
