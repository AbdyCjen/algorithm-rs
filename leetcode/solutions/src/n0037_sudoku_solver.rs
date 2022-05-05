/**
 * [37] Sudoku Solver
 *
 * Write a program to solve a Sudoku puzzle by filling the empty cells.
 * A sudoku solution must satisfy all of the following rules:
 * <ol>
 *     Each of the digits 1-9 must occur exactly once in each row.
 *     Each of the digits 1-9 must occur exactly once in each column.
 *     Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
 * </ol>
 * The '.' character indicates empty cells.
 *  
 * Example 1:
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height:250px; width:250px" />
 * Input: board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
 * Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
 * Explanation: The input board is shown above and the only valid solution is shown below:
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png" style="height:250px; width:250px" />
 *
 *  
 * Constraints:
 *
 *     board.length == 9
 *     board[i].length == 9
 *     board[i][j] is a digit or '.'.
 *     It is guaranteed that the input board has only one solution.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	#[allow(clippy::ptr_arg)]
	pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
		fn solve_inner(mat: &mut [Vec<char>], cur: usize) -> bool {
			if cur >= 9 * 9 {
				return true;
			}
			let (i, j) = (cur / 9, cur % 9);
			match mat[i][j] {
				'.' => {
					for c in '1'..='9' {
						let (ib, jb) = (i - i % 3, j - j % 3);
						if mat[i].iter().any(|cc| cc == &c)
							|| (0..9).map(|ii| mat[ii][j]).any(|cc| cc == c)
							|| mat[ib..ib + 3]
								.iter()
								.any(|row| row[jb..jb + 3].iter().any(|cc| cc == &c))
						{
							continue;
						}

						mat[i][j] = c;
						if solve_inner(mat, cur + 1) {
							return true;
						}
					}
					mat[i][j] = '.';
					false
				}
				_ => solve_inner(mat, cur + 1),
			}
		}

		solve_inner(board, 0);
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_37() {
		let mut mat = vec![
			vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
			vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
			vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
			vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
			vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
			vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
			vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
			vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
			vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
		];
		Solution::solve_sudoku(&mut mat);
		assert_eq!(
			mat,
			vec![
				vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
				vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
				vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
				vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
				vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
				vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
				vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
				vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
				vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
			]
		);
	}
}
