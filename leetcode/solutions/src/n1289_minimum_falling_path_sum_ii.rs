/**
 * [1289] Minimum Falling Path Sum II
 *
 * Given an n x n integer matrix grid, return the minimum sum of a falling path with non-zero shifts.
 * A falling path with non-zero shifts is a choice of exactly one element from each row of grid such that no two elements chosen in adjacent rows are in the same column.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/falling-grid.jpg" style="width: 244px; height: 245px;" />
 * Input: grid = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: 13
 * Explanation:
 * The possible falling paths are:
 * [1,5,9], [1,5,7], [1,6,7], [1,6,8],
 * [2,4,8], [2,4,9], [2,6,7], [2,6,8],
 * [3,4,8], [3,4,9], [3,5,7], [3,5,9]
 * The falling path with the smallest sum is [1,5,7], so the answer is 13.
 *
 * <strong class="example">Example 2:
 *
 * Input: grid = [[7]]
 * Output: 7
 *
 *  
 * Constraints:
 *
 *     n == grid.length == grid[i].length
 *     1 <= n <= 200
 *     -99 <= grid[i][j] <= 99
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::*;
impl Solution {
	pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
		// XXX: actually only top2 is required, may optimise
		let mut dp: BinaryHeap<_> = grid[0].iter().map(|v| -v).zip(0..).collect();
		for mut row in grid.into_iter().skip(1) {
			for (i, n) in (0..).zip(&mut row) {
				let (m, j) = *dp.peek().unwrap();
				if j == i {
					let tmp = dp.pop().unwrap();
					*n -= dp.peek().unwrap().0;
					dp.push(tmp);
				} else {
					*n -= m;
				}
			}
			dp = row.into_iter().map(|v| -v).zip(0..).collect();
		}
		-dp.pop().unwrap().0
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1289() {
		assert_eq!(
			Solution::min_falling_path_sum(matrix![
				[-73, 61, 43, -48, -36],
				[3, 30, 27, 57, 10],
				[96, -76, 84, 59, -15],
				[5, -49, 76, 31, -7],
				[97, 91, 61, -46, 67]
			]),
			-192
		);
		assert_eq!(
			Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
			13
		);
		assert_eq!(
			Solution::min_falling_path_sum(vec![vec![2, 2, 1, 2, 2]; 5]),
			7
		);
	}
}
