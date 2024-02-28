/**
 * [1239] Maximum Length of a Concatenated String with Unique Characters
 *
 * Given an array of strings arr. String s is a concatenation of a sub-sequence of arr which have unique characters.
 * Return the maximum possible length of s.
 *  
 * Example 1:
 *
 * Input: arr = ["un","iq","ue"]
 * Output: 4
 * Explanation: All possible concatenations are "","un","iq","ue","uniq" and "ique".
 * Maximum length is 4.
 *
 * Example 2:
 *
 * Input: arr = ["cha","r","act","ers"]
 * Output: 6
 * Explanation: Possible solutions are "chaers" and "acters".
 *
 * Example 3:
 *
 * Input: arr = ["abcdefghijklmnopqrstuvwxyz"]
 * Output: 26
 *
 *  
 * Constraints:
 *
 *     1 <= arr.length <= 16
 *     1 <= arr[i].length <= 26
 *     arr[i] contains only lower case English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_length(arr: Vec<String>) -> i32 {
		fn set(s: &[u8]) -> i32 {
			let mut ans = 0;
			for &c in s {
				ans |= 1 << (c - b'a');
			}
			ans
		}
		let mut dp = vec![0];
		for s in arr {
			let set = set(s.as_bytes());
			if set.count_ones() < s.len() as _ {
				continue;
			}
			for i in 0..dp.len() {
				let s = dp[i];
				if s & set == 0 {
					dp.push(s | set);
				}
			}
		}
		dp.iter().map(|s| s.count_ones()).max().unwrap() as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1239() {
		assert_eq!(Solution::max_length(vec_string!["un", "iq", "ue"]), 4);
		assert_eq!(
			Solution::max_length(('a'..='p').map(|c| c.to_string()).collect()),
			16
		);
		assert_eq!(
			Solution::max_length(vec_string!["cha", "r", "act", "ers"]),
			6
		);
		assert_eq!(
			Solution::max_length(vec_string!["abcdefghijklmnopqrstuvwxyz"]),
			26
		);
	}
}
