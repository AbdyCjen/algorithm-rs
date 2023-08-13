/**
 * [712] Minimum ASCII Delete Sum for Two Strings
 *
 * Given two strings s1 and s2, return the lowest ASCII sum of deleted characters to make two strings equal.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s1 = "sea", s2 = "eat"
 * Output: 231
 * Explanation: Deleting "s" from "sea" adds the ASCII value of "s" (115) to the sum.
 * Deleting "t" from "eat" adds 116 to the sum.
 * At the end, both strings are equal, and 115 + 116 = 231 is the minimum sum possible to achieve this.
 *
 * <strong class="example">Example 2:
 *
 * Input: s1 = "delete", s2 = "leet"
 * Output: 403
 * Explanation: Deleting "dee" from "delete" to turn the string into "let",
 * adds 100[d] + 101[e] + 101[e] to the sum.
 * Deleting "e" from "leet" adds 101[e] to the sum.
 * At the end, both strings are equal to "let", and the answer is 100+101+101+101 = 403.
 * If instead we turned both strings into "lee" or "eet", we would get answers of 433 or 417, which are higher.
 *
 *  
 * Constraints:
 *
 *     1 <= s1.length, s2.length <= 1000
 *     s1 and s2 consist of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
		let (m, n) = (s1.len(), s2.len());
		let mut dp = vec![vec![0; n + 1]; m + 1];
		for (i, c) in (1..).zip(s1.bytes()) {
			dp[i][0] = dp[i - 1][0] + c as i32;
		}
		for (j, c) in (1..).zip(s2.bytes()) {
			dp[0][j] = dp[0][j - 1] + c as i32;
		}
		for (i, c1) in (1..).zip(s1.bytes()) {
			for (j, c2) in (1..).zip(s2.bytes()) {
				dp[i][j] = (dp[i - 1][j] + c1 as i32).min(dp[i][j - 1] + c2 as i32);
				if c1 == c2 {
					dp[i][j] = dp[i][j].min(dp[i - 1][j - 1])
				}
			}
		}
		dp[m][n]
	}
}

// submission codes end
