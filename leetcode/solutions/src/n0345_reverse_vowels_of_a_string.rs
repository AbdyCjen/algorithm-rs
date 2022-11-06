/**
 * [345] Reverse Vowels of a String
 *
 * Given a string s, reverse only all the vowels in the string and return it.
 * The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.
 *  
 * <strong class="example">Example 1:
 * Input: s = "hello"
 * Output: "holle"
 * <strong class="example">Example 2:
 * Input: s = "leetcode"
 * Output: "leotcede"
 *  
 * Constraints:
 *
 *     1 <= s.length <= 3 * 10^5
 *     s consist of printable ASCII characters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn reverse_vowels(s: String) -> String {
		let mut s = s.into_bytes();

		let mut it = s
			.iter()
			.enumerate()
			.filter_map(|(i, c)| {
				if [b'a', b'e', b'i', b'o', b'u'].contains(&c.to_ascii_lowercase()) {
					Some(i)
				} else {
					None
				}
			})
			.collect::<Vec<_>>()
			.into_iter();
		while let (Some(i), Some(j)) = (it.next(), it.next_back()) {
			s.swap(i, j);
		}

		String::from_utf8(s).unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_345() {
		assert_eq!(
			Solution::reverse_vowels("leetcode".to_owned()),
			"leotcede".to_owned()
		);
		assert_eq!(
			Solution::reverse_vowels("hello".to_owned()),
			"holle".to_owned()
		);
		assert_eq!(
			Solution::reverse_vowels("hEllo".to_owned()),
			"hollE".to_owned()
		);
	}
}
