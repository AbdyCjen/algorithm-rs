/**
 * [2531] Make Number of Distinct Characters Equal
 *
 * You are given two 0-indexed strings word1 and word2.
 * A move consists of choosing two indices i and j such that 0 <= i < word1.length and 0 <= j < word2.length and swapping word1[i] with word2[j].
 * Return true if it is possible to get the number of distinct characters in word1 and word2 to be equal with exactly one move. Return false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: word1 = "ac", word2 = "b"
 * Output: false
 * Explanation: Any pair of swaps would yield two distinct characters in the first string, and one in the second string.
 *
 * <strong class="example">Example 2:
 *
 * Input: word1 = "abcc", word2 = "aab"
 * Output: true
 * Explanation: We swap index 2 of the first string with index 0 of the second string. The resulting strings are word1 = "abac" and word2 = "cab", which both have 3 distinct characters.
 *
 * <strong class="example">Example 3:
 *
 * Input: word1 = "abcde", word2 = "fghij"
 * Output: true
 * Explanation: Both resulting strings will have 5 distinct characters, regardless of which indices we swap.
 *
 *  
 * Constraints:
 *
 *     1 <= word1.length, word2.length <= 10^5
 *     word1 and word2 consist of only lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn is_it_possible(w1: String, w2: String) -> bool {
		let (mut cnt1, mut cnt2) = ([0; 26], [0; 26]);
		for c in w1.bytes() {
			cnt1[(c - b'a') as usize] += 1;
		}
		for c in w2.bytes() {
			cnt2[(c - b'a') as usize] += 1;
		}
		let (s1, s2) = (Self::count(&cnt1), Self::count(&cnt2));
		for i in (0..26).filter(|i| cnt1[*i] > 0) {
			for j in (0..26).filter(|j| cnt2[*j] > 0) {
				if i != j {
					let s1 = s1 - (cnt1[i] == 1) as i32 + (cnt1[j] == 0) as i32;
					let s2 = s2 - (cnt2[j] == 1) as i32 + (cnt2[i] == 0) as i32;
					if s1 == s2 {
						return true;
					}
				} else if s1 == s2 {
					return true;
				}
			}
		}
		false
	}

	fn count(c: &[i32; 26]) -> i32 { c.iter().filter(|i| **i > 0).count() as i32 }
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2531() {
		assert!(Solution::is_it_possible("abc".to_owned(), "abc".to_owned()));
		assert!(!Solution::is_it_possible("ac".to_owned(), "b".to_owned()));
		assert!(Solution::is_it_possible(
			"abcc".to_owned(),
			"aab".to_owned()
		));
		assert!(Solution::is_it_possible("abc".to_owned(), "acd".to_owned()));
		assert!(!Solution::is_it_possible(
			"aabb".to_owned(),
			"cd".to_owned()
		));
		assert!(!Solution::is_it_possible(
			"ab".to_owned(),
			"abcc".to_owned()
		));
	}
}
