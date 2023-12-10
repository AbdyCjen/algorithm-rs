/**
 * [880] Decoded String at Index
 *
 * You are given an encoded string s. To decode the string to a tape, the encoded string is read one character at a time and the following steps are taken:
 *
 *     If the character read is a letter, that letter is written onto the tape.
 *     If the character read is a digit d, the entire current tape is repeatedly written d - 1 more times in total.
 *
 * Given an integer k, return the k^th letter (1-indexed) in the decoded string.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "leet2code3", k = 10
 * Output: "o"
 * Explanation: The decoded string is "leetleetcode leetleetcode leetleetcode".
 * The 10^th letter in the string is "o".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "ha22", k = 5
 * Output: "h"
 * Explanation: The decoded string is "hahahaha".
 * The 5^th letter is "h".
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "a2345678999999999999999", k = 1
 * Output: "a"
 * Explanation: The decoded string is "a" repeated 8301530446056247680 times.
 * The 1^st letter is "a".
 *
 *  
 * Constraints:
 *
 *     2 <= s.length <= 100
 *     s consists of lowercase English letters and digits 2 through 9.
 *     s starts with a letter.
 *     1 <= k <= 10^9
 *     It is guaranteed that k is less than or equal to the length of the decoded string.
 *     The decoded string is guaranteed to have less than 2^63 letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn decode_at_index(s: String, k: i32) -> String {
		let s = s.into_bytes();
		let mut k = k as usize - 1;
		let mut st = vec![];
		let (mut prv, mut prv_len, mut slc) = (0, 0, &s[0..0]);
		for (i, w) in s.windows(2).enumerate() {
			if matches!(w[0], b'2'..=b'9') && w[1].is_ascii_lowercase() {
				let mut n = 1;
				for c in &s[prv..=i] {
					n *= (c - b'0') as usize;
				}
				st.push((slc, prv_len + slc.len()));
				prv_len = (prv_len + slc.len()) * n;
				prv = i + 1;
			} else if w[1].is_ascii_digit() && w[0].is_ascii_lowercase() {
				slc = &s[prv..=i];
				prv = i + 1;
			}
		}
		match s[prv] {
			b'0'..=b'9' => st.push((slc, prv_len + slc.len())),
			_ => st.push((&s[prv..], prv_len + s[prv..].len())),
		}
		for (slc, repeat_len) in st.into_iter().rev() {
			k %= repeat_len;
			if let Some(idx) = k.checked_sub(repeat_len - slc.len()) {
				return (slc[idx] as char).to_string();
			}
		}
		"".to_owned()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_880() {
		assert_eq!(
			Solution::decode_at_index("vzpp636m8y".to_owned(), 2920),
			"z"
		);
		assert_eq!(Solution::decode_at_index("leet2code3".to_owned(), 10), "o");
	}
}
