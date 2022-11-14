/**
 * [2472] Maximum Number of Non-overlapping Palindrome Substrings
 *
 * You are given a string s and a positive integer k.
 * Select a set of non-overlapping substrings from the string s that satisfy the following conditions:
 *
 *     The length of each substring is at least k.
 *     Each substring is a palindrome.
 *
 * Return the maximum number of substrings in an optimal selection.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "abaccdbbd", k = 3
 * Output: 2
 * Explanation: We can select the substrings underlined in s = "<u>aba</u>cc<u>dbbd</u>". Both "aba" and "dbbd" are palindromes and have a length of at least k = 3.
 * It can be shown that we cannot find a selection with more than two valid substrings.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "adbcda", k = 2
 * Output: 0
 * Explanation: There is no palindrome substring of length at least 2 in the string.
 *
 *  
 * Constraints:
 *
 *     1 <= k <= s.length <= 2000
 *     s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_palindromes(s: String, k: i32) -> i32 {
		let k = k as usize;
		let s = s.as_bytes();
		let mut prv = 0;
		let mut ans = 0;
		let mut i = k;
		while i <= s.len() {
			if Self::is_pal(&s[i - k..i]) || (i > prv + k && Self::is_pal(&s[i - k - 1..i])) {
				prv = i;
				i += k;
				ans += 1;
			} else {
				i += 1;
			}
		}
		ans
	}

	fn is_pal(s: &[u8]) -> bool {
		let mut it = s.iter();
		while let (Some(l), Some(r)) = (it.next(), it.next_back()) {
			if l != r {
				return false;
			}
		}
		true
	}
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2472() {
		assert_eq!(Solution::max_palindromes("abaccdbbd".to_owned(), 3), 2);
		assert_eq!(Solution::max_palindromes("adbcda".to_owned(), 2), 0);
	}
}
