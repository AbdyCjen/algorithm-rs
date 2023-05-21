/**
 * [1572] Matrix Diagonal Sum
 *
 * Given a square matrix mat, return the sum of the matrix diagonals.
 * Only include the sum of all the elements on the primary diagonal and all the elements on the secondary diagonal that are not part of the primary diagonal.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/14/sample_1911.png" style="width: 336px; height: 174px;" />
 * Input: mat = [[1,2,3],
 *               [4,5,6],
 *               [7,8,9]]
 * Output: 25
 * Explanation: Diagonals sum: 1 + 5 + 9 + 3 + 7 = 25
 * Notice that element mat[1][1] = 5 is counted only once.
 *
 * <strong class="example">Example 2:
 *
 * Input: mat = [[1,1,1,1],
 *               [1,1,1,1],
 *               [1,1,1,1],
 *               [1,1,1,1]]
 * Output: 8
 *
 * <strong class="example">Example 3:
 *
 * Input: mat = [[5]]
 * Output: 5
 *
 *  
 * Constraints:
 *
 *     n == mat.length == mat[i].length
 *     1 <= n <= 100
 *     1 <= mat[i][j] <= 100
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
		let n = mat.len();
		let mut ans = (0..)
			.zip(&mat)
			.map(|(i, r)| r[i] + r[n - i - 1])
			.sum::<i32>();
		if n % 2 == 1 {
			ans -= mat[n / 2][n / 2];
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1572() {
		assert_eq!(
			Solution::diagonal_sum(matrix![
				[1, 1, 1, 1],
				[1, 1, 1, 1],
				[1, 1, 1, 1],
				[1, 1, 1, 1]
			]),
			8
		);
		assert_eq!(
			Solution::diagonal_sum(matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
			25
		);
	}
}
