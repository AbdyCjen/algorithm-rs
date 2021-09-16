/**
 * [205] Isomorphic Strings
 *
 * Given two strings s and t, determine if they are isomorphic.
 * Two strings s and t are isomorphic if the characters in s can be replaced to get t.
 * All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.
 *  
 * Example 1:
 * Input: s = "egg", t = "add"
 * Output: true
 * Example 2:
 * Input: s = "foo", t = "bar"
 * Output: false
 * Example 3:
 * Input: s = "paper", t = "title"
 * Output: true
 *  
 * Constraints:
 *
 *     1 <= s.length <= 5 * 10^4
 *     t.length == s.length
 *     s and t consist of any valid ascii character.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn is_isomorphic(s: String, t: String) -> bool {
		fn gen_mapper() -> impl FnMut(u8) -> i8 {
			let mut map = [-1_i8; 128];
			let mut cur_max = 0_i8;
			move |c: u8| {
				let n = map.get_mut(c as usize).unwrap();
				if *n < 0 {
					*n = cur_max;
					cur_max += 1;
				}
				*n
			}
		}

		s.bytes().map(gen_mapper()).eq(t.bytes().map(gen_mapper()))
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_205() {
		assert!(Solution::is_isomorphic("egg".to_owned(), "add".to_owned()));
		assert!(Solution::is_isomorphic(
			"paper".to_owned(),
			"title".to_owned()
		));
		assert!(!Solution::is_isomorphic("foo".to_owned(), "bar".to_owned()));
	}
}
