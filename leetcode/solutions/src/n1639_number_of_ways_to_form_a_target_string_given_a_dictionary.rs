/**
 * [1639] Number of Ways to Form a Target String Given a Dictionary
 *
 * You are given a list of strings of the same length words and a string target.
 * Your task is to form target using the given words under the following rules:
 *
 *     target should be formed from left to right.
 *     To form the i^th character (0-indexed) of target, you can choose the k^th character of the j^th string in words if target[i] = words[j][k].
 *     Once you use the k^th character of the j^th string of words, you can no longer use the x^th character of any string in words where x <= k. In other words, all characters to the left of or at index k become unusuable for every string.
 *     Repeat the process until you form the string target.
 *
 * Notice that you can use multiple characters from the same string in words provided the conditions above are met.
 * Return the number of ways to form target from words. Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: words = ["acca","bbbb","caca"], target = "aba"
 * Output: 6
 * Explanation: There are 6 ways to form target.
 * "aba" -> index 0 ("<u>a</u>cca"), index 1 ("b<u>b</u>bb"), index 3 ("cac<u>a</u>")
 * "aba" -> index 0 ("<u>a</u>cca"), index 2 ("bb<u>b</u>b"), index 3 ("cac<u>a</u>")
 * "aba" -> index 0 ("<u>a</u>cca"), index 1 ("b<u>b</u>bb"), index 3 ("acc<u>a</u>")
 * "aba" -> index 0 ("<u>a</u>cca"), index 2 ("bb<u>b</u>b"), index 3 ("acc<u>a</u>")
 * "aba" -> index 1 ("c<u>a</u>ca"), index 2 ("bb<u>b</u>b"), index 3 ("acc<u>a</u>")
 * "aba" -> index 1 ("c<u>a</u>ca"), index 2 ("bb<u>b</u>b"), index 3 ("cac<u>a</u>")
 *
 * <strong class="example">Example 2:
 *
 * Input: words = ["abba","baab"], target = "bab"
 * Output: 4
 * Explanation: There are 4 ways to form target.
 * "bab" -> index 0 ("<u>b</u>aab"), index 1 ("b<u>a</u>ab"), index 2 ("ab<u>b</u>a")
 * "bab" -> index 0 ("<u>b</u>aab"), index 1 ("b<u>a</u>ab"), index 3 ("baa<u>b</u>")
 * "bab" -> index 0 ("<u>b</u>aab"), index 2 ("ba<u>a</u>b"), index 3 ("baa<u>b</u>")
 * "bab" -> index 1 ("a<u>b</u>ba"), index 2 ("ba<u>a</u>b"), index 3 ("baa<u>b</u>")
 *
 *  
 * Constraints:
 *
 *     1 <= words.length <= 1000
 *     1 <= words[i].length <= 1000
 *     All strings in words have the same length.
 *     1 <= target.length <= 1000
 *     words[i] and target contain only lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
	pub fn num_ways(words: Vec<String>, target: String) -> i32 {
		let mut cnts = vec![[0; 26]; words[0].len()];
		for w in words {
			for (c, cnt) in w.bytes().zip(&mut cnts) {
				cnt[(c - b'a') as usize] += 1;
			}
		}
		Self::solve(&cnts, target.as_bytes(), &mut HashMap::new())
	}

	fn solve(cnts: &[[u16; 26]], s: &[u8], cache: &mut HashMap<(u16, u16), i32>) -> i32 {
		const MO: i64 = 1e9 as i64 + 7;
		if s.is_empty() {
			return 1;
		} else if s.len() > cnts.len() {
			return 0;
		} else if let Some(&ans) = cache.get(&(cnts.len() as _, s.len() as _)) {
			return ans;
		}
		let mut ans = 0;
		if cnts[0][(s[0] - b'a') as usize] > 0 {
			ans = (ans
				+ cnts[0][(s[0] - b'a') as usize] as i64
					* Self::solve(&cnts[1..], &s[1..], cache) as i64)
				% MO;
		}
		ans = (ans + Self::solve(&cnts[1..], s, cache) as i64) % MO;
		cache.insert((cnts.len() as _, s.len() as _), ans as i32);
		ans as i32
	}
}

// submission codes end
