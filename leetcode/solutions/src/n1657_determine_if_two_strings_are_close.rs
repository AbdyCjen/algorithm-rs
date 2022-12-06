/**
 * [1657] Determine if Two Strings Are Close
 *
 * Two strings are considered close if you can attain one from the other using the following operations:
 *
 *     Operation 1: Swap any two existing characters.
 *     
 *         For example, a<u>b</u>cd<u>e</u> -> a<u>e</u>cd<u>b</u>
 *     
 *     
 *     Operation 2: Transform every occurrence of one existing character into another existing character, and do the same with the other character.
 *     
 *         For example, <u>aa</u>c<u>abb</u> -> <u>bb</u>c<u>baa</u> (all a's turn into b's, and all b's turn into a's)
 *     
 *     
 *
 * You can use the operations on either string as many times as necessary.
 * Given two strings, word1 and word2, return true if word1 and word2 are close, and false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: word1 = "abc", word2 = "bca"
 * Output: true
 * Explanation: You can attain word2 from word1 in 2 operations.
 * Apply Operation 1: "a<u>bc</u>" -> "a<u>cb</u>"
 * Apply Operation 1: "<u>a</u>c<u>b</u>" -> "<u>b</u>c<u>a</u>"
 *
 * <strong class="example">Example 2:
 *
 * Input: word1 = "a", word2 = "aa"
 * Output: false
 * Explanation: It is impossible to attain word2 from word1, or vice versa, in any number of operations.
 *
 * <strong class="example">Example 3:
 *
 * Input: word1 = "cabbba", word2 = "abbccc"
 * Output: true
 * Explanation: You can attain word2 from word1 in 3 operations.
 * Apply Operation 1: "ca<u>b</u>bb<u>a</u>" -> "ca<u>a</u>bb<u>b</u>"
 * Apply Operation 2: "<u>c</u>aa<u>bbb</u>" -> "<u>b</u>aa<u>ccc</u>"
 * Apply Operation 2: "<u>baa</u>ccc" -> "<u>abb</u>ccc"
 *
 *  
 * Constraints:
 *
 *     1 <= word1.length, word2.length <= 10^5
 *     word1 and word2 contain only lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn close_strings(word1: String, word2: String) -> bool {
		if word1.len() != word2.len() {
			return false;
		}

		let mut cnt1 = [0; 26];
		let mut cnt2 = [0; 26];
		for c in word1.into_bytes() {
			cnt1[(c - b'a') as usize] += 1;
		}
		for c in word2.into_bytes() {
			cnt2[(c - b'a') as usize] += 1;
		}
		for (&n1, n2) in cnt2.iter().zip(cnt1) {
			if (n1 > 0) ^ (n2 > 0) {
				return false;
			}
		}
		cnt1.sort_unstable();
		cnt2.sort_unstable();

		cnt1 == cnt2
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1657() {
		assert!(Solution::close_strings("abc".to_owned(), "bca".to_owned()));
		assert!(!Solution::close_strings("a".to_owned(), "aa".to_owned()));
		assert!(!Solution::close_strings("uau".to_owned(), "ssx".to_owned()));
		assert!(Solution::close_strings(
			"cabbba".to_owned(),
			"abbccc".to_owned()
		));
	}
}
