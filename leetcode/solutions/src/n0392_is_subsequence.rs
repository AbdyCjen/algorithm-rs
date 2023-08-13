/**
 * [392] Is Subsequence
 *
 * Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
 * A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "<u>a</u>b<u>c</u>d<u>e</u>" while "aec" is not).
 *  
 * <strong class="example">Example 1:
 * Input: s = "abc", t = "ahbgdc"
 * Output: true
 * <strong class="example">Example 2:
 * Input: s = "axc", t = "ahbgdc"
 * Output: false
 *  
 * Constraints:
 *
 *     0 <= s.length <= 100
 *     0 <= t.length <= 10^4
 *     s and t consist only of lowercase English letters.
 *
 *  
 * Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 10^9, and you want to check one by one to see if t has its subsequence. In this scenario, how would you change your code?
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn is_subsequence(s: String, t: String) -> bool {
		let mut i = 0;
		let s = s.as_bytes();
		for c in t.as_bytes() {
			if Some(c) == s.get(i) {
				i += 1;
			}
		}
		i == s.len()
	}
}

// submission codes end
