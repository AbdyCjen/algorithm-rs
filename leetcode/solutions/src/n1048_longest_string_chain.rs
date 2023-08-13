/**
 * [1048] Longest String Chain
 *
 * You are given an array of words where each word consists of lowercase English letters.
 * wordA is a predecessor of wordB if and only if we can insert exactly one letter anywhere in wordA without changing the order of the other characters to make it equal to wordB.
 *
 *     For example, "abc" is a predecessor of "ab<u>a</u>c", while "cba" is not a predecessor of "bcad".
 *
 * A word chain is a sequence of words [word1, word2, ..., wordk] with k >= 1, where word1 is a predecessor of word2, word2 is a predecessor of word3, and so on. A single word is trivially a word chain with k == 1.
 * Return the length of the longest possible word chain with words chosen from the given list of words.
 *  
 * <strong class="example">Example 1:
 *
 * Input: words = ["a","b","ba","bca","bda","bdca"]
 * Output: 4
 * Explanation: One of the longest word chains is ["a","<u>b</u>a","b<u>d</u>a","bd<u>c</u>a"].
 *
 * <strong class="example">Example 2:
 *
 * Input: words = ["xbc","pcxbcf","xb","cxbc","pcxbc"]
 * Output: 5
 * Explanation: All the words can be put in a word chain ["xb", "xb<u>c</u>", "<u>c</u>xbc", "<u>p</u>cxbc", "pcxbc<u>f</u>"].
 *
 * <strong class="example">Example 3:
 *
 * Input: words = ["abcd","dbqca"]
 * Output: 1
 * Explanation: The trivial word chain ["abcd"] is one of the longest word chains.
 * ["abcd","dbqca"] is not a valid word chain because the ordering of the letters is changed.
 *
 *  
 * Constraints:
 *
 *     1 <= words.length <= 1000
 *     1 <= words[i].length <= 16
 *     words[i] only consists of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn longest_str_chain(words: Vec<String>) -> i32 {
		let mut words_by_len = vec![vec![]; 16];
		let mut dp = vec![1; words.len()];
		for (i, w) in (0..).zip(words) {
			words_by_len[w.len() - 1].push((i, w));
		}
		for win in words_by_len.windows(2).rev() {
			for (i, w) in &win[1] {
				for (j, pr) in &win[0] {
					if Self::is_predecessor(w.as_bytes(), pr.as_bytes()) {
						dp[*j] = dp[*j].max(dp[*i] + 1);
					}
				}
			}
		}
		dp.into_iter().max().unwrap()
	}

	fn is_predecessor(s: &[u8], pr: &[u8]) -> bool {
		let i = s.iter().zip(pr).take_while(|(c1, c2)| c1 == c2).count();
		s[i + 1..] == pr[i..]
	}
}

// submission codes end
