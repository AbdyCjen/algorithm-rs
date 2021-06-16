/**
 * [474] Ones and Zeroes
 *
 * You are given an array of binary strings strs and two integers m and n.
 * Return the size of the largest subset of strs such that there are at most m 0's and n 1's in the subset.
 * A set x is a subset of a set y if all elements of x are also elements of y.
 *  
 * Example 1:
 *
 * Input: strs = ["10","0001","111001","1","0"], m = 5, n = 3
 * Output: 4
 * Explanation: The largest subset with at most 5 0's and 3 1's is {"10", "0001", "1", "0"}, so the answer is 4.
 * Other valid but smaller subsets include {"0001", "1"} and {"10", "1", "0"}.
 * {"111001"} is an invalid subset because it contains 4 1's, greater than the maximum of 3.
 *
 * Example 2:
 *
 * Input: strs = ["10","0","1"], m = 1, n = 1
 * Output: 2
 * Explanation: The largest subset is {"0", "1"}, so the answer is 2.
 *
 *  
 * Constraints:
 *
 *     1 <= strs.length <= 600
 *     1 <= strs[i].length <= 100
 *     strs[i] consists only of digits '0' and '1'.
 *     1 <= m, n <= 100
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
		let mut dp = vec![0_i32; ((m + 1) * (n + 1)) as usize];
		let idx = |i: i32, j: i32| ((i * (n + 1)) + j) as usize;

		for s in strs {
			let (cnt0, cnt1) = s.bytes().fold((0, 0), |(cnt0, cnt1), c| {
				let c = (c - b'0') as i32;
				(cnt0 + (c ^ 1), cnt1 + (c & 1))
			});
			for j in (cnt0..=m).rev() {
				for k in (cnt1..=n).rev() {
					dp[idx(j, k)] = dp[idx(j, k)].max(dp[idx(j - cnt0, k - cnt1)] + 1);
				}
			}
		}

		dp[idx(m, n)]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_474() {
		assert_eq!(
			Solution::find_max_form(vec_string!["10", "0", "1"], 1, 1),
			2
		);
		assert_eq!(
			Solution::find_max_form(vec_string!["10", "0001", "111001", "1", "0"], 5, 3),
			4
		);
		assert_eq!(
			Solution::find_max_form(vec_string!["11111", "100", "1101", "1101", "11000"], 5, 7),
			3
		);
	}
}
