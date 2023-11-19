/**
 * [1930] Unique Length-3 Palindromic Subsequences
 *
 * Given a string s, return the number of unique palindromes of length three that are a subsequence of s.
 * Note that even if there are multiple ways to obtain the same subsequence, it is still only counted once.
 * A palindrome is a string that reads the same forwards and backwards.
 * A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
 *
 *     For example, "ace" is a subsequence of "<u>a</u>b<u>c</u>d<u>e</u>".
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "aabca"
 * Output: 3
 * Explanation: The 3 palindromic subsequences of length 3 are:
 * - "aba" (subsequence of "<u>a</u>a<u>b</u>c<u>a</u>")
 * - "aaa" (subsequence of "<u>aa</u>bc<u>a</u>")
 * - "aca" (subsequence of "<u>a</u>ab<u>ca</u>")
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "adc"
 * Output: 0
 * Explanation: There are no palindromic subsequences of length 3 in "adc".
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "bbcbaba"
 * Output: 4
 * Explanation: The 4 palindromic subsequences of length 3 are:
 * - "bbb" (subsequence of "<u>bb</u>c<u>b</u>aba")
 * - "bcb" (subsequence of "<u>b</u>b<u>cb</u>aba")
 * - "bab" (subsequence of "<u>b</u>bcb<u>ab</u>a")
 * - "aba" (subsequence of "bbcb<u>aba</u>")
 *
 *  
 * Constraints:
 *
 *     3 <= s.length <= 10^5
 *     s consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_palindromic_subsequence(s: String) -> i32 {
		let mut first = [[0; 26]; 26];
		let mut last = [[0; 26]; 26];
		let mut cnt = [0; 26];
		let mut vis = 0;
		for c in s.bytes() {
			let idx = (c - b'a') as usize;
			last[idx] = cnt;
			if (vis & (1 << idx)) == 0 {
				first[idx] = cnt;
			}
			vis |= (1 << (idx));
			cnt[idx] += 1;
		}
		let mut a = 0;
		for (c, (prv, cur)) in (0..).zip(first.iter().zip(last)) {
			for (cc, (&prv, cur)) in (0..).zip(prv.iter().zip(cur)) {
				if prv < cur - 1 || (cc != c && prv < cur) {
					a += 1;
				}
			}
		}
		a
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1930() {
		assert_eq!(
			Solution::count_palindromic_subsequence("aabca".to_owned()),
			3
		);
		assert_eq!(Solution::count_palindromic_subsequence("adc".to_owned()), 0);
		assert_eq!(
			Solution::count_palindromic_subsequence("bbcbaba".to_owned()),
			4
		);
	}
}
