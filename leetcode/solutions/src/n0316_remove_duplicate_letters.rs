/**
 * [316] Remove Duplicate Letters
 *
 * Given a string s, remove duplicate letters so that every letter appears once and only once. You must make sure your result is <span data-keyword="lexicographically-smaller-string">the smallest in lexicographical order</span> among all possible results.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "bcabc"
 * Output: "abc"
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "cbacdcbc"
 * Output: "acdb"
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^4
 *     s consists of lowercase English letters.
 *
 *  
 * Note: This question is the same as 1081: <a href="https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/" target="_blank">https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/</a>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn remove_duplicate_letters(s: String) -> String {
		let mut cnts = [0; 26];
		s.bytes().for_each(|c| cnts[(c - b'a') as usize] += 1);
		let mut ans = vec![];
		for c in s.bytes() {
			cnts[(c - b'a') as usize] -= 1;
			while let Some(&tc) = ans.last() {
				if tc >= c && cnts[(tc - b'a') as usize] > 0 {
					ans.pop();
				} else {
					break;
				}
			}
			ans.push(c);
		}
		String::from_utf8(ans).unwrap()
	}
}

// submission codes end
