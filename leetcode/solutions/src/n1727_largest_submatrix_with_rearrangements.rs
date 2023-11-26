/**
 * [1727] Largest Submatrix With Rearrangements
 *
 * You are given a binary matrix matrix of size m x n, and you are allowed to rearrange the columns of the matrix in any order.
 * Return the area of the largest submatrix within matrix where every element of the submatrix is 1 after reordering the columns optimally.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/29/screenshot-2020-12-30-at-40536-pm.png" style="width: 500px; height: 240px;" />
 * Input: matrix = [[0,0,1],[1,1,1],[1,0,1]]
 * Output: 4
 * Explanation: You can rearrange the columns as shown above.
 * The largest submatrix of 1s, in bold, has an area of 4.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/29/screenshot-2020-12-30-at-40852-pm.png" style="width: 500px; height: 62px;" />
 * Input: matrix = [[1,0,1,0,1]]
 * Output: 3
 * Explanation: You can rearrange the columns as shown above.
 * The largest submatrix of 1s, in bold, has an area of 3.
 *
 * <strong class="example">Example 3:
 *
 * Input: matrix = [[1,1,0],[1,0,1]]
 * Output: 2
 * Explanation: Notice that you must rearrange entire columns, and there is no way to make a submatrix of 1s larger than an area of 2.
 *
 *  
 * Constraints:
 *
 *     m == matrix.length
 *     n == matrix[i].length
 *     1 <= m * n <= 10^5
 *     matrix[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
		let mut cnts = vec![0; matrix[0].len()];
		let mut ans = 0;
		for row in matrix {
			for (c, n) in cnts.iter_mut().zip(row) {
				*c = match n {
					0 => 0,
					_ => *c + 1,
				};
			}
			let mut ccnt = cnts.clone();
			ccnt.sort_unstable();
			for (c, i) in ccnt.into_iter().rev().zip(1..) {
				ans = ans.max(c * i);
				if c == 0 {
					break;
				}
			}
		}
		ans
	}
}

// submission codes end
