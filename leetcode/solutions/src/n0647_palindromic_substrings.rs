/**
 * [647] Palindromic Substrings
 *
 * Given a string s, return the number of palindromic substrings in it.
 * A string is a palindrome when it reads the same backward as forward.
 * A substring is a contiguous sequence of characters within the string.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "abc"
 * Output: 3
 * Explanation: Three palindromic strings: "a", "b", "c".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "aaa"
 * Output: 6
 * Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 1000
 *     s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_substrings(s: String) -> i32 {
		let mut ans = 0;
		let s = s.into_bytes();
		for i in 0..(s.len() as i32) {
			for (mut l, mut r) in [(i, i), (i - 1, i)] {
				while l >= 0 && r < s.len() as i32 && s[l as usize] == s[r as usize] {
					ans += 1;
					l -= 1;
					r += 1;
				}
			}
		}
		ans
	}
}

// submission codes end
