/**
 * [51] N-Queens
 *
 * The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
 * Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.
 * Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;" />
 * Input: n = 4
 * Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
 * Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 1
 * Output: [["Q"]]
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
		let mut ans = Vec::new();
		Self::nq_print(&mut vec![((1 << n) - 1, 0, 0, 0)], &mut ans);
		ans
	}

	fn nq_print(st: &mut Vec<(i32, i32, i32, i32)>, ans: &mut Vec<Vec<String>>) {
		if let Some((uplim, row, lr, rr)) = st.last().copied() {
			if row == uplim {
				let n = uplim.count_ones();
				let mat = st
					.windows(2)
					.map(|win| win[1].1 - win[0].1)
					.map(|row| {
						let mut s = vec![b'.'; n as usize];
						s[row.trailing_zeros() as usize] = b'Q';
						String::from_utf8(s).unwrap()
					})
					.collect::<Vec<_>>();
				ans.push(mat);
				return;
			}
			let mut avails = uplim - ((row | lr | rr) & uplim);
			let mut pos = avails & -avails;
			while pos != 0 {
				avails -= pos;
				st.push((uplim, row | pos, (lr | pos) << 1, (rr | pos) >> 1));
				Self::nq_print(st, ans);
				st.pop();
				pos = avails & -avails;
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_51() {
		assert_eq!(Solution::solve_n_queens(1), vec![vec_string!["Q"]]);
		assert_eq!(
			Solution::solve_n_queens(4),
			vec![
				vec_string![".Q..", "...Q", "Q...", "..Q."],
				vec_string!["..Q.", "Q...", "...Q", ".Q.."]
			]
		);
	}
}
