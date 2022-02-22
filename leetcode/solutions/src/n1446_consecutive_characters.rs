/**
 * [1446] Consecutive Characters
 *
 * The power of the string is the maximum length of a non-empty substring that contains only one unique character.
 * Given a string s, return the power of s.
 *  
 * Example 1:
 *
 * Input: s = "leetcode"
 * Output: 2
 * Explanation: The substring "ee" is of length 2 with the character 'e' only.
 *
 * Example 2:
 *
 * Input: s = "abbcccddddeeeeedcba"
 * Output: 5
 * Explanation: The substring "eeeee" is of length 5 with the character 'e' only.
 *
 * Example 3:
 *
 * Input: s = "triplepillooooow"
 * Output: 5
 *
 * Example 4:
 *
 * Input: s = "hooraaaaaaaaaaay"
 * Output: 11
 *
 * Example 5:
 *
 * Input: s = "tourist"
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 500
 *     s consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn max_power(s: String) -> i32 {
		s.bytes()
			.fold((1, 0, 0), |(ans, acc, pre), c| {
				if pre == c {
					(ans.max(acc + 1), acc + 1, c)
				} else {
					(ans.max(acc), 1, c)
				}
			})
			.0
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1446() {
		assert_eq!(Solution::max_power("leetcode".to_owned()), 2);
		assert_eq!(Solution::max_power("abbcccddddeeeeedcba".to_owned()), 5);
		assert_eq!(Solution::max_power("triplepillooooow".to_owned()), 5);
		assert_eq!(Solution::max_power("hooraaaaaaaaaaay".to_owned()), 11);
		assert_eq!(Solution::max_power("tourist".to_owned()), 1);
	}
}
