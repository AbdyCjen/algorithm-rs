/**
 * [2506] Count Pairs Of Similar Strings
 *
 * You are given a 0-indexed string array words.
 * Two strings are similar if they consist of the same characters.
 *
 *     For example, "abca" and "cba" are similar since both consist of characters 'a', 'b', and 'c'.
 *     However, "abacba" and "bcfd" are not similar since they do not consist of the same characters.
 *
 * Return the number of pairs (i, j) such that 0 <= i < j <= word.length - 1 and the two strings words[i] and words[j] are similar.
 *  
 * <strong class="example">Example 1:
 *
 * Input: words = ["aba","aabb","abcd","bac","aabc"]
 * Output: 2
 * Explanation: There are 2 pairs that satisfy the conditions:
 * - i = 0 and j = 1 : both words[0] and words[1] only consist of characters 'a' and 'b'.
 * - i = 3 and j = 4 : both words[3] and words[4] only consist of characters 'a', 'b', and 'c'.
 *
 * <strong class="example">Example 2:
 *
 * Input: words = ["aabb","ab","ba"]
 * Output: 3
 * Explanation: There are 3 pairs that satisfy the conditions:
 * - i = 0 and j = 1 : both words[0] and words[1] only consist of characters 'a' and 'b'.
 * - i = 0 and j = 2 : both words[0] and words[2] only consist of characters 'a' and 'b'.
 * - i = 1 and j = 2 : both words[1] and words[2] only consist of characters 'a' and 'b'.
 *
 * <strong class="example">Example 3:
 *
 * Input: words = ["nba","cba","dba"]
 * Output: 0
 * Explanation: Since there does not exist any pair that satisfies the conditions, we return 0.
 *  
 * Constraints:
 *
 *     1 <= words.length <= 100
 *     1 <= words[i].length <= 100
 *     words[i] consist of only lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn similar_pairs(words: Vec<String>) -> i32 {
		let mut set = std::collections::HashMap::new();
		for w in words {
			*set.entry(Self::to_set(w.as_bytes())).or_insert(0) += 1;
		}
		let mut ans = 0;
		for cnt in set.into_values() {
			ans += (cnt - 1) * cnt / 2;
		}
		ans
	}

	fn to_set(s: &[u8]) -> i32 {
		let mut set = 0;
		for &c in s {
			set |= 1 << (c - b'a');
		}
		set
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2506() {}
}
