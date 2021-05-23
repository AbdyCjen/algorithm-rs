/**
 * [926] Find and Replace Pattern
 *
 * Given a list of strings words and a string pattern, return a list of words[i] that match pattern. You may return the answer in any order.
 * A word matches the pattern if there exists a permutation of letters p so that after replacing every letter x in the pattern with p(x), we get the desired word.
 * Recall that a permutation of letters is a bijection from letters to letters: every letter maps to another letter, and no two letters map to the same letter.
 *  
 * Example 1:
 *
 * Input: words = ["abc","deq","mee","aqq","dkd","ccc"], pattern = "abb"
 * Output: ["mee","aqq"]
 * Explanation: "mee" matches the pattern because there is a permutation {a -> m, b -> e, ...}.
 * "ccc" does not match the pattern because {a -> c, b -> c, ...} is not a permutation, since a and b map to the same letter.
 *
 * Example 2:
 *
 * Input: words = ["a","b","c"], pattern = "a"
 * Output: ["a","b","c"]
 *
 *  
 * Constraints:
 *
 *     1 <= pattern.length <= 20
 *     1 <= words.length <= 50
 *     words[i].length == pattern.length
 *     pattern and words[i] are lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
		use std::iter::*;
		fn is_match(word1: &[u8], word2: &[u8]) -> bool {
			fn matcher_builder() -> impl FnMut(&u8) -> i8 {
				let ascii2idx = |i: u8| (i - b'a') as usize;
				let (mut mp, mut max_idx) = ([-1_i8; 26], 0);
				move |&i: &u8| -> i8 {
					let idx = ascii2idx(i);
					if mp[idx] < 0 {
						mp[idx] = max_idx;
						max_idx += 1;
					}
					mp[idx]
				}
			}

			let it1 = word1.iter().map(matcher_builder());
			let it2 = word2.iter().map(matcher_builder());
			it1.eq(it2)
		}

		words
			.into_iter()
			.filter(move |s| is_match(s.as_bytes(), pattern.as_bytes()))
			.collect()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_926() {
		assert_eq!(
			Solution::find_and_replace_pattern(
				vec_string!["abc", "deq", "mee", "aqq", "dkd", "ccc"],
				"abb".to_owned()
			),
			vec_string!["mee", "aqq"]
		);
		assert_eq!(
			Solution::find_and_replace_pattern(vec_string!["a", "b", "c"], "a".to_owned()),
			vec_string!["a", "b", "c"]
		);
	}
}
