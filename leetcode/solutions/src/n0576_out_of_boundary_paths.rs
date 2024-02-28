/**
 * [576] Out of Boundary Paths
 *
 * There is an m x n grid with a ball. The ball is initially at the position [startRow, startColumn]. You are allowed to move the ball to one of the four adjacent cells in the grid (possibly out of the grid crossing the grid boundary). You can apply at most maxMove moves to the ball.
 * Given the five integers m, n, maxMove, startRow, startColumn, return the number of paths to move the ball out of the grid boundary. Since the answer can be very large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/out_of_boundary_paths_1.png" style="width: 500px; height: 296px;" />
 * Input: m = 2, n = 2, maxMove = 2, startRow = 0, startColumn = 0
 * Output: 6
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/out_of_boundary_paths_2.png" style="width: 500px; height: 293px;" />
 * Input: m = 1, n = 3, maxMove = 3, startRow = 0, startColumn = 1
 * Output: 12
 *
 *  
 * Constraints:
 *
 *     1 <= m, n <= 50
 *     0 <= maxMove <= 50
 *     0 <= startRow < m
 *     0 <= startColumn < n
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
		let mut dp = vec![vec![0; n as usize]; m as usize];
		dp[start_row as usize][start_column as usize] += 1;
		const MO: i32 = 1e9 as i32 + 7;
		let mut ans = 0;
		for _ in 0..max_move {
			for a in dp[0]
				.iter()
				.chain(&dp[dp.len() - 1])
				.chain(dp.iter().map(|row| &row[0]))
				.chain(dp.iter().map(|row| &row[row.len() - 1]))
			{
				ans = (ans + a) % MO;
			}
			for (i, row) in (0..).zip(std::mem::replace(
				&mut dp,
				vec![vec![0; n as usize]; m as usize],
			)) {
				for (j, cnt) in (0..).zip(row) {
					if i > 0 {
						dp[i - 1][j] = (dp[i - 1][j] + cnt) % MO;
					}
					if i < dp.len() - 1 {
						dp[i + 1][j] = (dp[i + 1][j] + cnt) % MO;
					}
					if j > 0 {
						dp[i][j - 1] = (dp[i][j - 1] + cnt) % MO;
					}
					if j < dp[0].len() - 1 {
						dp[i][j + 1] = (dp[i][j + 1] + cnt) % MO;
					}
				}
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
	fn test_576() {
		assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
		assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
		assert_eq!(Solution::find_paths(36, 5, 50, 15, 3), 390153306);
	}
}
