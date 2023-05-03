/**
 * [516] Longest Palindromic Subsequence
 *
 * Given a string s, find the longest palindromic subsequence's length in s.
 * A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "bbbab"
 * Output: 4
 * Explanation: One possible longest palindromic subsequence is "bbbb".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "cbbd"
 * Output: 2
 * Explanation: One possible longest palindromic subsequence is "bb".
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 1000
 *     s consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
	pub fn longest_palindrome_subseq(s: String) -> i32 {
		Self::solve(s.as_bytes(), (0, s.len() as i32 - 1), &mut HashMap::new())
	}

	fn solve(s: &[u8], rng: (i32, i32), cache: &mut HashMap<(i32, i32), i32>) -> i32 {
		let (l, r) = rng;
		if let Some(&ans) = cache.get(&rng) {
			return ans;
		} else if r - l < 1 {
			return r - l + 1;
		}
		let ans = if s[l as usize] == s[r as usize] {
			2 + Self::solve(s, (l + 1, r - 1), cache)
		} else {
			Self::solve(s, (l + 1, r), cache).max(Self::solve(s, (l, r - 1), cache))
		};
		cache.insert(rng, ans);
		ans
	}
}

// submission codes end
