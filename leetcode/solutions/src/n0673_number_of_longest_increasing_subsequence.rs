/**
 * [673] Number of Longest Increasing Subsequence
 *
 * Given an integer array nums, return the number of longest increasing subsequences.
 * Notice that the sequence has to be strictly increasing.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,3,5,4,7]
 * Output: 2
 * Explanation: The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [2,2,2,2,2]
 * Output: 5
 * Explanation: The length of the longest increasing subsequence is 1, and there are 5 increasing subsequences of length 1, so output 5.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 2000
 *     -10^6 <= nums[i] <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
		let mut dp = vec![(0, 1); nums.len()]; // max_len, sum
		let (mut max_len, mut ans) = (0, 0);
		for (i, &n) in (0..).zip(&nums) {
			if let [dp @ .., stat] = &mut dp[..=i] {
				for (&prv, pprv) in nums.iter().zip(dp) {
					if prv < n {
						if pprv.0 > stat.0 {
							*stat = *pprv;
						} else if pprv.0 == stat.0 {
							stat.1 += pprv.1;
						}
					}
				}
				stat.0 += 1;
				if max_len < stat.0 {
					max_len = stat.0;
					ans = stat.1;
				} else if max_len == stat.0 {
					ans += stat.1;
				}
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_673() {
		assert_eq!(Solution::find_number_of_lis(vec![2; 5]), 5);
		assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
	}
}
