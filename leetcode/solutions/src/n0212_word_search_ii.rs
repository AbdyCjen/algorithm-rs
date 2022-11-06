/**
 * [212] Word Search II
 *
 * Given an m x n board of characters and a list of strings words, return all words on the board.
 * Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/07/search1.jpg" style="width: 322px; height: 322px;" />
 * Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
 * Output: ["eat","oath"]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/07/search2.jpg" style="width: 162px; height: 162px;" />
 * Input: board = [["a","b"],["c","d"]], words = ["abcb"]
 * Output: []
 *
 *  
 * Constraints:
 *
 *     m == board.length
 *     n == board[i].length
 *     1 <= m, n <= 12
 *     board[i][j] is a lowercase English letter.
 *     1 <= words.length <= 3 * 10^4
 *     1 <= words[i].length <= 10
 *     words[i] consists of lowercase English letters.
 *     All the strings of words are unique.
 *
 */
pub struct Solution {}

// submission codes start here

struct Trie {
	next: Vec<Trie>,
	end: Option<usize>,
	c: u8,
}

impl Trie {
	fn new(c: u8) -> Self {
		Self {
			next: Vec::new(),
			end: None,
			c,
		}
	}

	fn from_words(words: &[String]) -> Self {
		let mut root = Self::new(b'a');
		for (idx, word) in words.iter().enumerate() {
			let mut cur = &mut root;
			for &c in word.as_bytes() {
				cur = match cur.next.binary_search_by(|ch| ch.c.cmp(&c)) {
					Err(i) => {
						cur.next.insert(i, Self::new(c));
						&mut cur.next[i]
					}
					Ok(i) => &mut cur.next[i],
				};
			}
			cur.end = Some(idx);
		}
		root
	}
}

#[allow(dead_code)]
impl Solution {
	pub fn find_words(board: Vec<Vec<char>>, mut words: Vec<String>) -> Vec<String> {
		let mut mat = board
			.into_iter()
			.map(|v| v.into_iter().map(|c| c as u8).collect::<Vec<_>>())
			.collect::<Vec<_>>();
		let mut trie = Trie::from_words(&words);
		let mut ans = Vec::new();
		for i in 0..(mat.len() as i32) {
			for j in 0..(mat[0].len() as i32) {
				Self::dfs(&mut mat, i, j, &mut trie, &mut ans);
			}
		}
		ans.sort_unstable();
		ans.into_iter()
			.rev()
			.map(|i| words.swap_remove(i))
			.collect()
	}

	fn dfs(mat: &mut [Vec<u8>], i: i32, j: i32, trie: &mut Trie, ans: &mut Vec<usize>) -> bool {
		if i < 0 || j < 0 || i >= mat.len() as _ || j >= mat[0].len() as _ {
			return false;
		}

		let c = mat[i as usize][j as usize];
		if let Ok(n) = trie.next.binary_search_by(|ch| ch.c.cmp(&c)) {
			let cur = &mut trie.next[n];
			if let Some(e) = cur.end.take() {
				ans.push(e);
				if cur.next.is_empty() {
					trie.next.remove(n);
					return true;
				}
			}

			const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
			mat[i as usize][j as usize] = b'0';
			for dir in DIR {
				let (k, l) = (i + dir.0, j + dir.1);
				if Self::dfs(mat, k, l, cur, ans) && cur.next.is_empty() {
					trie.next.remove(n);
					mat[i as usize][j as usize] = c;
					return true;
				}
			}
			mat[i as usize][j as usize] = c;
		}
		false
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_212() {
		assert_eq!(
			Solution::find_words(
				matrix![
					['o', 'a', 'a', 'n'],
					['e', 't', 'a', 'e'],
					['i', 'h', 'k', 'r'],
					['i', 'f', 'l', 'v']
				],
				vec_string!["oath", "pea", "eat", "rain"]
			),
			vec_string!["eat", "oath"]
		);
		assert!(
			Solution::find_words(matrix![['a', 'b'], ['c', 'd']], vec_string!["abcd"]).is_empty()
		);
		assert!(Solution::find_words(matrix![['a', 'a']], vec_string!["aaa"]).is_empty());
		assert_eq!(
			Solution::find_words(
				matrix![
					['o', 'a', 'a', 'n'],
					['e', 't', 'a', 'e'],
					['i', 'h', 'k', 'r'],
					['i', 'f', 'l', 'v']
				],
				vec_string!["oath", "pea", "eat", "rain", "hklf", "hf"]
			),
			vec_string!["hf", "hklf", "eat", "oath"]
		);
	}
}
