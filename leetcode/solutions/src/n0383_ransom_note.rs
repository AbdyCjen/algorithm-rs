/**
 * [383] Ransom Note
 *
 * Given an arbitrary ransom note string and another string containing letters from all the magazines, write a function that will return true if the ransom note can be constructed from the magazines ; otherwise, it will return false.
 * Each letter in the magazine string can only be used once in your ransom note.
 *  
 * Example 1:
 * Input: ransomNote = "a", magazine = "b"
 * Output: false
 * Example 2:
 * Input: ransomNote = "aa", magazine = "ab"
 * Output: false
 * Example 3:
 * Input: ransomNote = "aa", magazine = "aab"
 * Output: true
 *  
 * Constraints:
 *
 * You may assume that both strings contain only lowercase letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn can_construct(ransom_note: String, magazine: String) -> bool {
		let (ransom_note, magazine) = (ransom_note.into_bytes(), magazine.into_bytes());
		let mut cnt = [0; 26];
		ransom_note
			.into_iter()
			.for_each(|c| cnt[(c - b'a') as usize] += 1);
		magazine.into_iter().for_each(|c| {
			let c = cnt.get_mut((c - b'a') as usize).unwrap();
			if *c > 0 {
				*c -= 1
			}
		});
		!cnt.iter().any(|&c| c > 0)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_383() {
		assert!(!Solution::can_construct("a".to_owned(), "b".to_owned()));
		assert!(!Solution::can_construct("aa".to_owned(), "ab".to_owned()));
		assert!(Solution::can_construct("aa".to_owned(), "aab".to_owned()));
	}
}
