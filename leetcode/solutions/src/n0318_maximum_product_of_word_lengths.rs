/**
 * [318] Maximum Product of Word Lengths
 *
 * Given a string array words, return the maximum value of length(word[i]) * length(word[j]) where the two words do not share common letters. If no such two words exist, return 0.
 *  
 * Example 1:
 *
 * Input: words = ["abcw","baz","foo","bar","xtfn","abcdef"]
 * Output: 16
 * Explanation: The two words can be "abcw", "xtfn".
 *
 * Example 2:
 *
 * Input: words = ["a","ab","abc","d","cd","bcd","abcd"]
 * Output: 4
 * Explanation: The two words can be "ab", "cd".
 *
 * Example 3:
 *
 * Input: words = ["a","aa","aaa","aaaa"]
 * Output: 0
 * Explanation: No such pair of words.
 *
 *  
 * Constraints:
 *
 *     2 <= words.length <= 1000
 *     1 <= words[i].length <= 1000
 *     words[i] consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn max_product(words: Vec<String>) -> i32 {
		let words: Vec<_> = words
			.into_iter()
			.map(|w| {
				let w = w.into_bytes();
				let mut set = 0_i32;
				for c in &w {
					set |= 1 << (c - b'a');
				}
				(set, w.len() as i32)
			})
			.collect();

		let mut ans = 0;
		for (i, &(w1, l1)) in words.iter().enumerate() {
			for (w2, l2) in &words[i + 1..] {
				if (w1 & w2) == 0 {
					ans = ans.max(l1 * l2);
				}
			}
		}

		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_318() {
		assert_eq!(
			Solution::max_product(vec_string!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]),
			16
		);
		assert_eq!(
			Solution::max_product(vec_string!["a", "ab", "abc", "d", "cd", "bcd", "abcd"]),
			4
		);
		assert_eq!(
			Solution::max_product(vec_string!["a", "aa", "aaa", "aaaa"]),
			0
		);
	}
}
