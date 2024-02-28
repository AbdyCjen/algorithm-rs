/**
 * [2982] Find Longest Special Substring That Occurs Thrice II
 *
 * You are given a string s that consists of lowercase English letters.
 * A string is called special if it is made up of only a single character. For example, the string "abc" is not special, whereas the strings "ddd", "zz", and "f" are special.
 * Return the length of the longest special substring of s which occurs at least thrice, or -1 if no special substring occurs at least thrice.
 * A substring is a contiguous non-empty sequence of characters within a string.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "aaaa"
 * Output: 2
 * Explanation: The longest special substring which occurs thrice is "aa": substrings "<u>aa</u>aa", "a<u>aa</u>a", and "aa<u>aa</u>".
 * It can be shown that the maximum length achievable is 2.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "abcdef"
 * Output: -1
 * Explanation: There exists no special substring which occurs at least thrice. Hence return -1.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "abcaba"
 * Output: 1
 * Explanation: The longest special substring which occurs thrice is "a": substrings "<u>a</u>bcaba", "abc<u>a</u>ba", and "abcab<u>a</u>".
 * It can be shown that the maximum length achievable is 1.
 *
 *  
 * Constraints:
 *
 *     3 <= s.length <= 5 * 10^5
 *     s consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn maximum_length(s: String) -> i32 {
		use std::collections::*;
		let mut prv = (s.as_bytes()[0], 0);
		let mut cnts = vec![BTreeMap::new(); 26];
		for c in s.bytes() {
			if c == prv.0 {
				prv.1 += 1;
			} else {
				*cnts[(prv.0 - b'a') as usize].entry(prv.1).or_insert(0) += 1;
				prv = (c, 1);
			}
		}
		*cnts[(prv.0 - b'a') as usize].entry(prv.1).or_insert(0) += 1;
		let mut ans = -1;
		for cnt in cnts {
			for (&c, &times) in cnt.iter() {
				let mut s = times;
				if cnt.range((c + 1)..).next().is_some() {
					s += 2;
				}
				let aa = match s {
					3.. => c,
					2 => c - 1,
					_ => c - 2,
				};
				if aa > 0 {
					ans = ans.max(aa);
				}
			}
		}
		ans
	}
}

// submission codes end
