/**
 * [79] Word Search
 *
 * Given an m x n grid of characters board and a string word, return true if word exists in the grid.
 * The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.
 *  
 * <strong class='example'>Example 1:
 * <img alt='' src='https://assets.leetcode.com/uploads/2020/11/04/word2.jpg' style='width: 322px; height: 242px;' />
 * Input: board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], word = 'ABCCED'
 * Output: true
 *
 * <strong class='example'>Example 2:
 * <img alt='' src='https://assets.leetcode.com/uploads/2020/11/04/word-1.jpg' style='width: 322px; height: 242px;' />
 * Input: board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], word = 'SEE'
 * Output: true
 *
 * <strong class='example'>Example 3:
 * <img alt='' src='https://assets.leetcode.com/uploads/2020/10/15/word3.jpg' style='width: 322px; height: 242px;' />
 * Input: board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], word = 'ABCB'
 * Output: false
 *
 *  
 * Constraints:
 *
 *     m == board.length
 *     n = board[i].length
 *     1 <= m, n <= 6
 *     1 <= word.length <= 15
 *     board and word consists of only lowercase and uppercase English letters.
 *
 *  
 * Follow up: Could you use search pruning to make your solution faster with a larger board?
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
		for i in 0..board.len() {
			for j in 0..board[0].len() {
				if Self::solve(&mut board, word.as_bytes(), i as i32, j as i32) {
					return true;
				}
			}
		}
		false
	}

	fn solve(mat: &mut [Vec<char>], word: &[u8], i: i32, j: i32) -> bool {
		match word {
			[c, rest @ ..] => {
				if i >= 0
					&& j >= 0 && i < mat.len() as _
					&& j < mat[0].len() as _
					&& mat[i as usize][j as usize] == *c as _
				{
					mat[i as usize][j as usize] = ' ';
					for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
						let next = (i + dir.0, j + dir.1);
						if Self::solve(mat, rest, next.0, next.1) {
							return true;
						}
					}
					mat[i as usize][j as usize] = *c as _;
				}
				false
			}
			_ => true,
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_79() {
		assert!(!Solution::exist(
			matrix![
				['A', 'B', 'C', 'E'],
				['S', 'F', 'C', 'S'],
				['A', 'D', 'E', 'E']
			],
			"ABCB".to_owned()
		));
		assert!(Solution::exist(
			matrix![
				['A', 'B', 'C', 'E'],
				['S', 'F', 'C', 'S'],
				['A', 'D', 'E', 'E']
			],
			"SEE".to_owned()
		));
		assert!(Solution::exist(matrix![['a']], "a".to_owned()));
		assert!(Solution::exist(
			matrix![
				['A', 'B', 'C', 'E'],
				['S', 'F', 'C', 'S'],
				['A', 'D', 'E', 'E']
			],
			"ABCCED".to_owned()
		));
	}
}
