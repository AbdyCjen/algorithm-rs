/**
 * [567] Permutation in String
 *
 * Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
 * In other words, return true if one of s1's permutations is the substring of s2.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s1 = "ab", s2 = "eidbaooo"
 * Output: true
 * Explanation: s2 contains one permutation of s1 ("ba").
 *
 * <strong class="example">Example 2:
 *
 * Input: s1 = "ab", s2 = "eidboaoo"
 * Output: false
 *
 *  
 * Constraints:
 *
 *     1 <= s1.length, s2.length <= 10^4
 *     s1 and s2 consist of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn check_inclusion(s1: String, s2: String) -> bool {
		let mut cnt = [0; 26];
		for c in s1.bytes() {
			cnt[(c - b'a') as usize] += 1;
		}
		let mut i = 0;
		for (j, c) in s2.bytes().enumerate() {
			cnt[(c - b'a') as usize] -= 1;
			while cnt[(c - b'a') as usize] < 0 {
				cnt[(s2.as_bytes()[i] - b'a') as usize] += 1;
				i += 1;
			}
			if j + 1 - i == s1.len() && cnt == [0; 26] {
				return true;
			}
		}
		false
	}
}

// submission codes end
