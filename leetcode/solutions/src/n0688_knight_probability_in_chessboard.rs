/**
 * [688] Knight Probability in Chessboard
 *
 * On an n x n chessboard, a knight starts at the cell (row, column) and attempts to make exactly k moves. The rows and columns are 0-indexed, so the top-left cell is (0, 0), and the bottom-right cell is (n - 1, n - 1).
 * A chess knight has eight possible moves it can make, as illustrated below. Each move is two cells in a cardinal direction, then one cell in an orthogonal direction.
 * <img src="https://assets.leetcode.com/uploads/2018/10/12/knight.png" style="width: 300px; height: 300px;" />
 * Each time the knight is to move, it chooses one of eight possible moves uniformly at random (even if the piece would go off the chessboard) and moves there.
 * The knight continues moving until it has made exactly k moves or has moved off the chessboard.
 * Return the probability that the knight remains on the board after it has stopped moving.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 3, k = 2, row = 0, column = 0
 * Output: 0.06250
 * Explanation: There are two moves (to (1,2), (2,1)) that will keep the knight on the board.
 * From each of those positions, there are also two moves that will keep the knight on the board.
 * The total probability the knight stays on the board is 0.0625.
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 1, k = 0, row = 0, column = 0
 * Output: 1.00000
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 25
 *     0 <= k <= 100
 *     0 <= row, column <= n - 1
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn knight_probability(n: i32, k: i32, row: i32, col: i32) -> f64 {
		let mut st = vec![(row, col, 1.0)];
		let mut mat = vec![vec![0.0; n as usize]; n as usize];
		for _ in 0..k {
			for (i, j, prob) in std::mem::take(&mut st) {
				for (ni, nj) in [
					(i - 2, j + 1),
					(i - 2, j - 1),
					(i + 2, j + 1),
					(i + 2, j - 1),
					(i - 1, j - 2),
					(i - 1, j + 2),
					(i + 1, j - 2),
					(i + 1, j + 2),
				] {
					if ni >= 0 && ni < n && nj >= 0 && nj < n {
						mat[ni as usize][nj as usize] += prob / 8.0;
					}
				}
			}
			for (i, row) in (0..).zip(&mut mat) {
				for (j, blk) in (0..).zip(row).filter(|v| *v.1 > 0.0) {
					st.push((i, j, std::mem::take(blk)));
				}
			}
		}
		st.iter().map(|v| v.2).sum()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_688() {
		assert_eq!(Solution::knight_probability(3, 2, 0, 0), 0.0625);
	}
}
