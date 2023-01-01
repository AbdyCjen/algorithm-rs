/**
 * [290] Word Pattern
 *
 * Given a pattern and a string s, find if s follows the same pattern.
 * Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.
 *  
 * <strong class="example">Example 1:
 *
 * Input: pattern = "abba", s = "dog cat cat dog"
 * Output: true
 *
 * <strong class="example">Example 2:
 *
 * Input: pattern = "abba", s = "dog cat cat fish"
 * Output: false
 *
 * <strong class="example">Example 3:
 *
 * Input: pattern = "aaaa", s = "dog cat cat dog"
 * Output: false
 *
 *  
 * Constraints:
 *
 *     1 <= pattern.length <= 300
 *     pattern contains only lower-case English letters.
 *     1 <= s.length <= 3000
 *     s contains only lowercase English letters and spaces ' '.
 *     s does not contain any leading or trailing spaces.
 *     All the words in s are separated by a single space.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn word_pattern(p: String, s: String) -> bool {
		let mut cset = [None; 26];
		let mut set = std::collections::HashMap::new();
		let mut it = s.split(' ');
		let mut pit = p.bytes();
		loop {
			match (it.next(), pit.next()) {
				(Some(w), Some(c)) => {
					match &mut cset[(c - b'a') as usize] {
						Some(s) => {
							if *s != w {
								break false;
							}
						}
						s => *s = Some(w),
					};
					let e = set.entry(w).or_insert(c);
					if *e != c {
						break false;
					}
				}

				(None, None) => break true,
				_ => break false,
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {

	#[test]
	fn test_290() {}
}
