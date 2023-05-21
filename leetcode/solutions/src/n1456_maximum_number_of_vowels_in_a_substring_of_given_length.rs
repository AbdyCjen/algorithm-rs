/**
 * [1456] Maximum Number of Vowels in a Substring of Given Length
 *
 * Given a string s and an integer k, return the maximum number of vowel letters in any substring of s with length k.
 * Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "abciiidef", k = 3
 * Output: 3
 * Explanation: The substring "iii" contains 3 vowel letters.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "aeiou", k = 2
 * Output: 2
 * Explanation: Any substring of length 2 contains 2 vowels.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "leetcode", k = 3
 * Output: 2
 * Explanation: "lee", "eet" and "ode" contain 2 vowels.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s consists of lowercase English letters.
 *     1 <= k <= s.length
 *
 */
pub struct Solution {}

impl Solution {
	const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
	pub fn max_vowels(s: String, k: i32) -> i32 {
		let (mut i, mut cnt, s) = (0, 0, s.as_bytes());
		s.iter().zip(0..).fold(0, |ans, (c, j)| {
			if j - i >= k {
				cnt -= Self::VOWELS.contains(&s[i as usize]) as i32;
				i += 1;
			}
			cnt += Self::VOWELS.contains(c) as i32;
			ans.max(cnt)
		})
	}
	pub fn max_vowels1(s: String, k: i32) -> i32 {
		let s = s.into_bytes();
		const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
		let mut cnt = s[..k as usize]
			.iter()
			.filter(|c| VOWELS.contains(*c))
			.count() as i32;
		s[k as usize..]
			.iter()
			.zip(&s)
			.fold(cnt, |ans, (head, tail)| {
				cnt += VOWELS.contains(head) as i32 - VOWELS.contains(tail) as i32;
				ans.max(cnt)
			})
	}
}
