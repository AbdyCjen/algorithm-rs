/**
 * [1047] Remove All Adjacent Duplicates In String
 *
 * You are given a string s consisting of lowercase English letters. A duplicate removal consists of choosing two adjacent and equal letters and removing them.
 * We repeatedly make duplicate removals on s until we no longer can.
 * Return the final string after all such duplicate removals have been made. It can be proven that the answer is unique.
 *  
 * Example 1:
 *
 * Input: s = "abbaca"
 * Output: "ca"
 * Explanation:
 * For example, in "abbaca" we could remove "bb" since the letters are adjacent and equal, and this is the only possible move.  The result of this move is that the string is "aaca", of which only "aa" is possible, so the final string is "ca".
 *
 * Example 2:
 *
 * Input: s = "azxxzy"
 * Output: "ay"
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	// local
	pub fn remove_duplicates(s: String) -> String {
		let mut i = 0;
		let mut s = s.into_bytes();
		for j in 0..s.len() {
			if i > 0 && s[i - 1] == s[j] {
				i -= 1;
			} else {
				s[i] = s[j];
				i += 1;
			}
		}
		s.truncate(i);
		String::from_utf8(s).unwrap()
	}

	pub fn remove_duplicates_01(s: String) -> String {
		let s = s.into_bytes();
		let mut ans = Vec::new();

		for c in s {
			if ans.last() == Some(&c) {
				ans.pop();
			} else {
				ans.push(c);
			}
		}

		unsafe { String::from_utf8_unchecked(ans) }
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1047() {
		assert_eq!(
			Solution::remove_duplicates("abbaca".to_owned()),
			"ca".to_owned()
		);
		assert_eq!(
			Solution::remove_duplicates("azxxzy".to_owned()),
			"ay".to_owned()
		);
		assert_eq!(
			Solution::remove_duplicates("aaaaa".to_owned()),
			"a".to_owned()
		);
	}
}
