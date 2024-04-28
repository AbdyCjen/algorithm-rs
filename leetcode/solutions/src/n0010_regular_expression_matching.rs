/**
 * [10] Regular Expression Matching
 *
 * Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
 *
 *     '.' Matches any single character.​​​​
 *     '*' Matches zero or more of the preceding element.
 *
 * The matching should cover the entire input string (not partial).
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "aa", p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "aa", p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "ab", p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 20
 *     1 <= p.length <= 20
 *     s contains only lowercase English letters.
 *     p contains only lowercase English letters, '.', and '*'.
 *     It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn is_match(s: String, p: String) -> bool { Self::solve(s.as_bytes(), p.as_bytes()) }
	// XXX: cache retval with (s.len(), p.len()) as cache key to reduce runtime complexity,
	fn solve(s: &[u8], p: &[u8]) -> bool {
		match s {
			[] => match p {
				[_, b'*', pr @ ..] => Self::solve(s, pr),
				p => p.is_empty(),
			},
			[c1, sr @ ..] => match p {
				[c2, b'*', pr @ ..] => {
					((c1 == c2 || *c2 == b'.') && Self::solve(sr, p)) || Self::solve(s, pr)
				}
				[c2, pr @ ..] if c1 == c2 || *c2 == b'.' => Self::solve(sr, pr),
				_ => false,
			},
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_10() {
		assert!(Solution::is_match(
			"aabcbcbcaccbcaabc".to_owned(),
			".*a*aa*.*b*.c*.*a*".to_owned()
		));
		assert!(Solution::is_match("aab".to_owned(), "c*a*b".to_owned()));
		assert!(!Solution::is_match("aa".to_owned(), "a".to_owned()));
		assert!(Solution::is_match("aa".to_owned(), "a*".to_owned()));
		assert!(Solution::is_match("ab".to_owned(), ".*".to_owned()));
		assert!(!Solution::is_match(
			"mississippi".to_owned(),
			"mis*is*p*.".to_owned()
		));
	}
}
