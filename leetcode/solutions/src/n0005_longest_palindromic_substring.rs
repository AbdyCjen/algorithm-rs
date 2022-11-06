/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, return the longest palindromic substring in s.
 * A string is called a palindrome string if the reverse of that string is the same as the original string.
 *  
 * Example 1:
 *
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 *
 * Example 2:
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 1000
 *     s consist of only digits and English letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn longest_palindrome(s: String) -> String {
		let s = s.into_bytes();
		let mut ans = &s[..1];

		for i in 0..s.len() {
			if i < ans.len() {
				continue;
			}

			let (l, r) = (i - ans.len(), i + 1);
			if Self::palindrome(&s[l..r]) {
				ans = &s[l..r];
			}

			if let Some(l) = l.checked_sub(1) {
				if Self::palindrome(&s[l..r]) {
					ans = &s[l..r];
				}
			}
		}

		String::from_utf8(ans.to_vec()).unwrap()
	}

	fn palindrome(s: &[u8]) -> bool {
		s[..s.len() / 2]
			.iter()
			.eq(s[(s.len() + 1) / 2..].iter().rev())
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_5() {
		assert_eq!(Solution::longest_palindrome("babad".to_owned()), "bab");
		assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
		assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
		assert_eq!(Solution::longest_palindrome("caba".to_owned()), "aba");
		assert_eq!(
			Solution::longest_palindrome("bbbbbbbbb".to_owned()),
			"bbbbbbbbb"
		);
		assert_eq!(
			Solution::longest_palindrome("bbbbbbbbbb".to_owned()),
			"bbbbbbbbbb"
		);
	}
}
