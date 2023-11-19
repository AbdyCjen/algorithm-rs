/**
 * [719] Find K-th Smallest Pair Distance
 *
 * The distance of a pair of integers a and b is defined as the absolute difference between a and b.
 * Given an integer array nums and an integer k, return the k^th smallest distance among all the pairs nums[i] and nums[j] where 0 <= i < j < nums.length.
 *  
 * Example 1:
 *
 * Input: nums = [1,3,1], k = 1
 * Output: 0
 * Explanation: Here are all the pairs:
 * (1,3) -> 2
 * (1,1) -> 0
 * (3,1) -> 2
 * Then the 1^st smallest distance pair is (1,1), and its distance is 0.
 *
 * Example 2:
 *
 * Input: nums = [1,1,1], k = 2
 * Output: 0
 *
 * Example 3:
 *
 * Input: nums = [1,6,1], k = 3
 * Output: 5
 *
 *  
 * Constraints:
 *
 *     n == nums.length
 *     2 <= n <= 10^4
 *     0 <= nums[i] <= 10^6
 *     1 <= k <= n * (n - 1) / 2
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
		nums.sort_unstable();
		let n = nums.len();
		let mut l = nums.windows(2).map(|s| s[1] - s[0]).min().unwrap();
		let mut r = nums[n - 1] - nums[0];
		while l < r {
			let m = (l + r) / 2;
			let mut cnt = 0;
			let mut j = 0;
			for i in 1..n {
				while j < i && nums[i] - nums[j] > m {
					j += 1;
				}

				cnt += (i - j) as i32;
			}

			if cnt >= k {
				r = m;
			} else {
				l = m + 1;
			}
		}
		r
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	fn foo(nums: &[i32], m: i32) -> i32 {
		let n = nums.len();
		let mut cnt = 0;
		let mut j = 0;
		for i in 1..n {
			while j < i && nums[i] - nums[j] > m {
				j += 1;
			}

			cnt += (i - j) as i32;
		}
		cnt
	}
	#[test]
	fn test_719() {
		assert_eq!(Solution::smallest_distance_pair(vec![1, 3, 1], 1), 0);
		assert_eq!(Solution::smallest_distance_pair(vec![1, 1, 1], 2), 0);
		assert_eq!(Solution::smallest_distance_pair(vec![1, 6, 1], 3), 5);
		assert_eq!(
			Solution::smallest_distance_pair(vec![2, 2, 0, 1, 1, 0, 0, 1, 2, 0], 2),
			0
		);
	}
}
