/**
 * [931] Minimum Falling Path Sum
 *
 * Given an n x n array of integers matrix, return the minimum sum of any falling path through matrix.
 * A falling path starts at any element in the first row and chooses the element in the next row that is either directly below or diagonally left/right. Specifically, the next element from position (row, col) will be (row + 1, col - 1), (row + 1, col), or (row + 1, col + 1).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/03/failing1-grid.jpg" style="width: 499px; height: 500px;" />
 * Input: matrix = [[2,1,3],[6,5,4],[7,8,9]]
 * Output: 13
 * Explanation: There are two falling paths with a minimum sum as shown.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/03/failing2-grid.jpg" style="width: 164px; height: 365px;" />
 * Input: matrix = [[-19,57],[-40,-5]]
 * Output: -59
 * Explanation: The falling path with a minimum sum is shown.
 *
 *  
 * Constraints:
 *
 *     n == matrix.length == matrix[i].length
 *     1 <= n <= 100
 *     -100 <= matrix[i][j] <= 100
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_falling_path_sum(mut mat: Vec<Vec<i32>>) -> i32 {
		let mut last = mat.pop().unwrap();
		for mut row in mat.into_iter().rev() {
			for (i, (c, min)) in row.iter_mut().zip(&last).enumerate() {
				let mut min = *min;
				if i > 0 {
					min = min.min(last[i - 1]);
				}
				if let Some(&n) = last.get(i + 1) {
					min = min.min(n);
				}
				*c += min;
			}
			last = row;
		}
		last.into_iter().min().unwrap()
	}
	pub fn min_falling_path_sum_1(matrix: Vec<Vec<i32>>) -> i32 {
		let n = matrix.len();
		let mut dp = (vec![0; n], vec![0; n]);
		for (i, row) in matrix.into_iter().enumerate().rev() {
			let (cur, prv) = if i % 2 == 0 {
				(&mut dp.0, &dp.1)
			} else {
				(&mut dp.1, &dp.0)
			};
			for (j, ((num, c), min)) in row.into_iter().zip(cur).zip(prv).enumerate() {
				let mut min = *min;
				if j > 0 {
					min = min.min(prv[j - 1]);
				}
				if let Some(&n) = prv.get(j + 1) {
					min = min.min(n);
				}

				*c = num + min;
			}
		}
		dp.0.into_iter().min().unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_931() {
		assert_eq!(
			Solution::min_falling_path_sum(matrix![[-19, 57], [-40, -5]]),
			-59
		);
		assert_eq!(
			Solution::min_falling_path_sum(matrix![[2, 1, 3], [6, 5, 4], [7, 8, 9]]),
			13
		);
	}
}
