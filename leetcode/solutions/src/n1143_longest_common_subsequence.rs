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

impl Solution {
	pub fn longest_common_subsequence(s1: String, s2: String) -> i32 {
		Self::solve(
			s1.as_bytes(),
			s2.as_bytes(),
			&mut vec![vec![-1; s2.len() + 1]; s1.len() + 1],
		)
	}
	fn solve(s1: &[u8], s2: &[u8], cache: &mut [Vec<i32>]) -> i32 {
		if cache[s1.len()][s2.len()] >= 0 {
			return cache[s1.len()][s2.len()];
		}

		match (s1, s2) {
			([c1, sn1 @ ..], [c2, sn2 @ ..]) => {
				let mut ans = Self::solve(sn1, s2, cache).max(Self::solve(s1, sn2, cache));
				if c1 == c2 {
					ans = ans.max(1 + Self::solve(sn1, sn2, cache));
				}
				cache[s1.len()][s2.len()] = ans;
				ans
			}
			_ => 0,
		}
	}
	pub fn longest_common_subsequence_1(text1: String, text2: String) -> i32 {
		let (s1, s2) = (text1.into_bytes(), text2.into_bytes());
		let mut dp = vec![vec![0; s2.len()]; s1.len()];
		s1.iter().zip(dp.iter_mut()).fold(0, |pr, (c, dp)| {
			dp[0] = if s2[0] == *c { 1 } else { pr };
			dp[0]
		});
		s2.iter().zip(dp[0].iter_mut()).fold(0, |pr, (c, dp)| {
			*dp = if s1[0] == *c { 1 } else { pr };
			*dp
		});

		for (i, c1) in s1.iter().enumerate().skip(1) {
			for (j, c2) in s2.iter().enumerate().skip(1) {
				if c1 == c2 {
					dp[i][j] = dp[i - 1][j - 1] + 1;
				} else {
					dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
				}
			}
		}
		dp[s1.len() - 1][s2.len() - 1]
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
