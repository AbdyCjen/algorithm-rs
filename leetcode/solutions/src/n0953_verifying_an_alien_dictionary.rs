/**
 * [953] Verifying an Alien Dictionary
 *
 * In an alien language, surprisingly, they also use English lowercase letters, but possibly in a different order. The order of the alphabet is some permutation of lowercase letters.
 * Given a sequence of words written in the alien language, and the order of the alphabet, return true if and only if the given words are sorted lexicographically in this alien language.
 *  
 * <strong class="example">Example 1:
 *
 * Input: words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
 * Output: true
 * Explanation: As 'h' comes before 'l' in this language, then the sequence is sorted.
 *
 * <strong class="example">Example 2:
 *
 * Input: words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
 * Output: false
 * Explanation: As 'd' comes after 'l' in this language, then words[0] > words[1], hence the sequence is unsorted.
 *
 * <strong class="example">Example 3:
 *
 * Input: words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
 * Output: false
 * Explanation: The first three characters "app" match, and the second string is shorter (in size.) According to lexicographical rules "apple" > "app", because 'l' > '&empty;', where '&empty;' is defined as the blank character which is less than any other character (<a href="https://en.wikipedia.org/wiki/Lexicographical_order" target="_blank">More info</a>).
 *
 *  
 * Constraints:
 *
 *     1 <= words.length <= 100
 *     1 <= words[i].length <= 20
 *     order.length == 26
 *     All characters in words[i] and order are English lowercase letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
		let mut map = [0; 26];
		for (idx, c) in order.bytes().zip(0..) {
			map[(idx - b'a') as usize] = c;
		}
		let mapped = words
			.into_iter()
			.map(|s| {
				s.bytes()
					.map(|c| map[(c - b'a') as usize])
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		mapped.windows(2).all(|win| win[0] <= win[1])
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_953() {
		assert!(!Solution::is_alien_sorted(
			vec_string!["apple", "app"],
			"abcdefghijklmnopqrstuvwxyz".to_owned()
		));
		assert!(!Solution::is_alien_sorted(
			vec_string!["word", "world", "row"],
			"worldabcefghijkmnpqstuvxyz".to_owned()
		));
		assert!(Solution::is_alien_sorted(
			vec_string!["hello", "leetcode"],
			"hlabcdefgijkmnopqrstuvwxyz".to_owned()
		));
		assert!(Solution::is_alien_sorted(
			vec_string!["kuvp", "q"],
			"ngxlkthsjuoqcpavbfdermiywz".to_owned()
		));
	}
}
