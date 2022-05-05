/**
 * [1745] Palindrome Partitioning IV
 *
 * Given a string s, return true if it is possible to split the string s into three non-empty palindromic substrings. Otherwise, return false.​​​​​
 * A string is said to be palindrome if it the same string when reversed.
 *  
 * Example 1:
 *
 * Input: s = "abcbdd"
 * Output: true
 * Explanation: "abcbdd" = "a" + "bcb" + "dd", and all three substrings are palindromes.
 *
 * Example 2:
 *
 * Input: s = "bcbddxy"
 * Output: false
 * Explanation: s cannot be split into 3 palindromes.
 *
 *  
 * Constraints:
 *
 *     3 <= s.length <= 2000
 *     s​​​​​​ consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn check_partitioning(s: String) -> bool {
		fn is_cut(s: &[u8], dp: &mut [Vec<bool>], cut: usize) -> bool {
			for i in 0..s.len() {
				for (l, r) in (0..=(i.min(s.len() - i - 1)))
					.map(|j| (i - j, i + j))
					.take_while(|&(l, r)| s[l] == s[r])
				{
					if dp[cut - 1][l] {
						dp[cut][r + 1] = true;
					}
				}

				for (l, r) in (0..(i.min(s.len() - i)))
					.map(|j| (i - 1 - j, i + j))
					.take_while(|&(l, r)| s[l] == s[r])
				{
					if dp[cut - 1][l] {
						dp[cut][r + 1] = true;
					}
				}
			}
			*dp[cut].last().unwrap()
		}
		let v = vec![false; s.len() + 1];
		let mut dp = [v.clone(), v.clone(), v.clone(), v];
		dp[0][0] = true;
		is_cut(s.as_bytes(), &mut dp, 1);
		is_cut(s.as_bytes(), &mut dp, 2);
		is_cut(s.as_bytes(), &mut dp, 3)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1745() {
		assert!(Solution::check_partitioning("abcbdd".to_owned()));
		assert!(!Solution::check_partitioning("bcbddxy".to_owned()));
	}
}
