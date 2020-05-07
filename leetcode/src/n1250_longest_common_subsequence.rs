/**
 * [1250] Longest Common Subsequence
 *
 * Given two strings text1 and text2, return the length of their longest common subsequence.
 * A subsequence of a string is a new string generated from the original string with some characters(can be none) deleted without changing the relative order of the remaining characters. (eg, "ace" is a subsequence of "abcde" while "aec" is not). A common subsequence of two strings is a subsequence that is common to both strings.
 *  
 * If there is no common subsequence, return 0.
 *  
 * Example 1:
 *
 * Input: text1 = "abcde", text2 = "ace"
 * Output: 3  
 * Explanation: The longest common subsequence is "ace" and its length is 3.
 *
 * Example 2:
 *
 * Input: text1 = "abc", text2 = "abc"
 * Output: 3
 * Explanation: The longest common subsequence is "abc" and its length is 3.
 *
 * Example 3:
 *
 * Input: text1 = "abc", text2 = "def"
 * Output: 0
 * Explanation: There is no such common subsequence, so the result is 0.
 *
 *  
 * Constraints:
 *
 * 1 <= text1.length <= 1000
 * 1 <= text2.length <= 1000
 * The input strings consist of lowercase English characters only.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
		let (text1, text2) = (text1.into_bytes(), text2.into_bytes());
		let mut dp = vec![vec![0; text2.len()]; text1.len()];
		text1.iter().zip(dp.iter_mut()).fold(0, |pr, (c, dp)| {
			dp[0] = if text2[0] == *c { 1 } else { pr };
			dp[0]
		});
		text2.iter().zip(dp[0].iter_mut()).fold(0, |pr, (c, dp)| {
			*dp = if text1[0] == *c { 1 } else { pr };
			*dp
		});

		for (i, c1) in text1.iter().enumerate().skip(1) {
			for (j, c2) in text2.iter().enumerate().skip(1) {
				if c1 == c2 {
					dp[i][j] = dp[i - 1][j - 1] + 1;
				} else {
					dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
				}
			}
		}
		dp[text1.len() - 1][text2.len() - 1]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1250() {
		assert_eq!(
			Solution::longest_common_subsequence("abcde".to_owned(), "ace".to_owned()),
			3
		);
		assert_eq!(
			Solution::longest_common_subsequence("abc".to_owned(), "abc".to_owned()),
			3
		);
		assert_eq!(
			Solution::longest_common_subsequence("abc".to_owned(), "def".to_owned()),
			0
		);
		assert_eq!(
			Solution::longest_common_subsequence("abc".to_owned(), "abcc".to_owned()),
			3
		);
	}
}
