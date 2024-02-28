/**
 * [1704] Determine if String Halves Are Alike
 *
 * You are given a string s of even length. Split this string into two halves of equal lengths, and let a be the first half and b be the second half.
 * Two strings are alike if they have the same number of vowels ('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'). Notice that s contains uppercase and lowercase letters.
 * Return true if a and b are alike. Otherwise, return false.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "book"
 * Output: true
 * Explanation: a = "b<u>o</u>" and b = "<u>o</u>k". a has 1 vowel and b has 1 vowel. Therefore, they are alike.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "textbook"
 * Output: false
 * Explanation: a = "t<u>e</u>xt" and b = "b<u>oo</u>k". a has 1 vowel whereas b has 2. Therefore, they are not alike.
 * Notice that the vowel o is counted twice.
 *
 *  
 * Constraints:
 *
 *     2 <= s.length <= 1000
 *     s.length is even.
 *     s consists of uppercase and lowercase letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn halves_are_alike(s: String) -> bool {
		fn count_vowels(s: &[u8]) -> usize {
			const VOWELS: &[u8] = &[b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];
			s.iter().filter(|c| VOWELS.contains(c)).count()
		}
		let s = s.into_bytes();
		count_vowels(&s[..s.len() / 2]) == count_vowels(&s[s.len() / 2..])
	}
}

// submission codes end
