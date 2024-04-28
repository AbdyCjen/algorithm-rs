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
	pub fn exist(mut mat: Vec<Vec<char>>, word: String) -> bool {
		(0..mat.len())
			.any(|i| (0..mat[0].len()).any(|j| Self::solve(&mut mat, word.as_bytes(), i, j)))
	}

	fn solve(mat: &mut [Vec<char>], word: &[u8], i: usize, j: usize) -> bool {
		match word {
			[c, rest @ ..] => {
				if mat[i][j] == *c as char {
					mat[i][j] = ' ';
					if rest.is_empty()
						|| i > 0 && Self::solve(mat, rest, i - 1, j)
						|| i + 1 < mat.len() && Self::solve(mat, rest, i + 1, j)
						|| j > 0 && Self::solve(mat, rest, i, j - 1)
						|| j + 1 < mat[0].len() && Self::solve(mat, rest, i, j + 1)
					{
						return true;
					}
					mat[i][j] = *c as char;
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
