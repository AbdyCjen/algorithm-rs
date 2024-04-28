/**
 * [2370] Longest Ideal Subsequence
 *
 * You are given a string s consisting of lowercase letters and an integer k. We call a string t ideal if the following conditions are satisfied:
 *
 *     t is a subsequence of the string s.
 *     The absolute difference in the alphabet order of every two adjacent letters in t is less than or equal to k.
 *
 * Return the length of the longest ideal string.
 * A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.
 * Note that the alphabet order is not cyclic. For example, the absolute difference in the alphabet order of 'a' and 'z' is 25, not 1.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "acfgbd", k = 2
 * Output: 4
 * Explanation: The longest ideal string is "acbd". The length of this string is 4, so 4 is returned.
 * Note that "acfgbd" is not ideal because 'c' and 'f' have a difference of 3 in alphabet order.
 * <strong class="example">Example 2:
 *
 * Input: s = "abcd", k = 3
 * Output: 4
 * Explanation: The longest ideal string is "abcd". The length of this string is 4, so 4 is returned.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     0 <= k <= 25
 *     s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn longest_ideal_string(s: String, k: i32) -> i32 {
		let mut cnts = [0; 26];
		let mut ans = 0;
		for c in s.bytes().map(|c| c - b'a') {
			let l = c.saturating_sub(k as u8) as usize;
			let r = (c + k as u8).min(25) as usize;
			let m = *cnts[l..=r].iter().max().unwrap();
			cnts[c as usize] = m + 1;
		}
		*cnts.iter().max().unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2370() {}
}
