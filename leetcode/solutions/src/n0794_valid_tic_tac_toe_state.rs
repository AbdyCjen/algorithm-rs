/**
 * [794] Valid Tic-Tac-Toe State
 *
 * Given a Tic-Tac-Toe board as a string array board, return true if and only if it is possible to reach this board position during the course of a valid tic-tac-toe game.
 * The board is a 3 x 3 array that consists of characters ' ', 'X', and 'O'. The ' ' character represents an empty square.
 * Here are the rules of Tic-Tac-Toe:
 *
 *     Players take turns placing characters into empty squares ' '.
 *     The first player always places 'X' characters, while the second player always places 'O' characters.
 *     'X' and 'O' characters are always placed into empty squares, never filled ones.
 *     The game ends when there are three of the same (non-empty) character filling any row, column, or diagonal.
 *     The game also ends if all squares are non-empty.
 *     No more moves can be played if the game is over.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/15/tictactoe1-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: board = ["O  ","   ","   "]
 * Output: false
 * Explanation: The first player always plays "X".
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/15/tictactoe2-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: board = ["XOX"," X ","   "]
 * Output: false
 * Explanation: Players take turns making moves.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/15/tictactoe3-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: board = ["XXX","   ","OOO"]
 * Output: false
 *
 * Example 4:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/15/tictactoe4-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: board = ["XOX","O O","XOX"]
 * Output: true
 *
 *  
 * Constraints:
 *
 *     board.length == 3
 *     board[i].length == 3
 *     board[i][j] is either 'X', 'O', or ' '.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
		let board = board
			.into_iter()
			.map(|s| s.into_bytes())
			.collect::<Vec<_>>();

		let (co, cx) = board.iter().flatten().fold((0, 0), |(co, cx), &c| {
			if c == b'O' {
				(co + 1, cx)
			} else if c == b'X' {
				(co, cx + 1)
			} else {
				(co, cx)
			}
		});

		let wx = winner(&board, b'X');
		let wo = winner(&board, b'O');
		return !(wo && wx)
			&& if wx { cx - co == 1 } else { true }
			&& if wo { co == cx } else { true }
			&& (0..=1).contains(&(cx - co));

		fn winner(board: &[Vec<u8>], c: u8) -> bool {
			board.iter().any(|row| row.iter().all(|x| *x == c))
				|| (board[0][0] == c && board[1][1] == c && board[2][2] == c)
				|| (board[0][2] == c && board[1][1] == c && board[2][0] == c)
				|| (0..3).any(|i| board[0][i] == c && board[1][i] == c && board[2][i] == c)
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_794() {
		assert!(!Solution::valid_tic_tac_toe(vec_string![
			"O  ", "   ", "   "
		]));
		assert!(!Solution::valid_tic_tac_toe(vec_string![
			"XOX", " X ", "   "
		]));
		assert!(!Solution::valid_tic_tac_toe(vec_string![
			"XXX", "   ", "OOO"
		]));
		assert!(Solution::valid_tic_tac_toe(vec_string![
			"XOX", "O O", "XOX"
		]));
	}
}
