/**
 * [792] Number of Matching Subsequences
 *
 * Given a string s and an array of strings words, return the number of words[i] that is a subsequence of s.
 * A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
 *
 *     For example, "ace" is a subsequence of "abcde".
 *
 *  
 * Example 1:
 *
 * Input: s = "abcde", words = ["a","bb","acd","ace"]
 * Output: 3
 * Explanation: There are three strings in words that are a subsequence of s: "a", "acd", "ace".
 *
 * Example 2:
 *
 * Input: s = "dsahjpjauf", words = ["ahjpjau","ja","ahbwzgqnuk","tnmlanowax"]
 * Output: 2
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 5 * 10^4
 *     1 <= words.length <= 5000
 *     1 <= words[i].length <= 50
 *     s and words[i] consist of only lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
		let mut c2idx = vec![vec![]; 26];
		for (i, c) in s.bytes().enumerate() {
			c2idx[(c - b'a') as usize].push(i as isize);
		}
		let sub_seq_of = move |s: &str| -> bool {
			let mut cur = -1;
			for c in s.bytes() {
				if let Some(&i) = c2idx[(c - b'a') as usize].iter().find(|i| **i > cur) {
					cur = i;
				} else {
					return false;
				}
			}
			true
		};
		words.into_iter().filter(|s| sub_seq_of(s)).count() as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_792() {
		assert_eq!(
			Solution::num_matching_subseq("abcde".to_owned(), vec_string!["a", "bb", "acd", "ace"]),
			3
		);
		assert_eq!(
			Solution::num_matching_subseq(
				"dsahjpjauf".to_owned(),
				vec_string!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]
			),
			2
		);
	}
}
