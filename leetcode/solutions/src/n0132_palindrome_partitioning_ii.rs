/**
 * [132] Palindrome Partitioning II
 *
 * Given a string s, partition s such that every substring of the partition is a palindrome.
 * Return the minimum cuts needed for a palindrome partitioning of s.
 *  
 * Example 1:
 *
 * Input: s = "aab"
 * Output: 1
 * Explanation: The palindrome partitioning ["aa","b"] could be produced using 1 cut.
 *
 * Example 2:
 *
 * Input: s = "a"
 * Output: 0
 *
 * Example 3:
 *
 * Input: s = "ab"
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 2000
 *     s consists of lowercase English letters only.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	// 遍历获取所有回文串, 然后将任一回文串p[i,j]当成一条(i->j)的边, bfs遍历求最短路径
	pub fn min_cut(s: String) -> i32 {
		let s = s.into_bytes();
		let mut dp: Vec<_> = (0..=s.len() as i32).collect();
		for i in 0..s.len() {
			for (l, r) in (0..=(i.min(s.len() - i - 1)))
				.map(|j| (i - j, i + j))
				.take_while(|&(l, r)| s[l] == s[r])
			{
				dp[r + 1] = dp[r + 1].min(dp[l] + 1);
			}
			for (l, r) in (0..((i + 1).min(s.len() - i - 1)))
				.map(|j| (i - j, i + 1 + j))
				.take_while(|&(l, r)| s[l] == s[r])
			{
				dp[r + 1] = dp[r + 1].min(dp[l] + 1);
			}
		}

		dp[s.len()] - 1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_132() {
		assert_eq!(Solution::min_cut("aab".to_owned()), 1);
		assert_eq!(Solution::min_cut("a".to_owned()), 0);
		assert_eq!(Solution::min_cut("ab".to_owned()), 1);
		assert_eq!(Solution::min_cut("aaaaaaaa131".to_owned()), 1);
		assert_eq!(Solution::min_cut("abcdedcba131".to_owned()), 1);
		assert_eq!(Solution::min_cut("abcdedcbaaba".to_owned()), 1);
		assert_eq!(Solution::min_cut("abcdedcbaaba3776773".to_owned()), 2);
	}
}
