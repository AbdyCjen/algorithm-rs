/**
 * [689] Maximum Sum of 3 Non-Overlapping Subarrays
 *
 * Given an integer array nums and an integer k, find three non-overlapping subarrays of length k with maximum sum and return them.
 * Return the result as a list of indices representing the starting position of each interval (0-indexed). If there are multiple answers, return the lexicographically smallest one.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,1,2,6,7,5,1], k = 2
 * Output: [0,3,5]
 * Explanation: Subarrays [1, 2], [2, 6], [7, 5] correspond to the starting indices [0, 3, 5].
 * We could have also taken [2, 1], but an answer of [1, 3, 5] would be lexicographically larger.
 *
 * Example 2:
 *
 * Input: nums = [1,2,1,2,1,2,1,2,1], k = 2
 * Output: [0,2,4]
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 2 * 10^4
 *     1 <= nums[i] < 2^16
 *     1 <= k <= floor(nums.length / 3)
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
		let n = nums.len();
		let k = k as usize;

		let mut pre_sum = vec![0];
		pre_sum.extend(nums);
		for i in 1..=n {
			pre_sum[i] += pre_sum[i - 1];
		}
		let sum = move |i| pre_sum[i + k] - pre_sum[i];

		let mut left_most = vec![0; n + 1];
		left_most[k - 1] = sum(0);
		for i in 0..(n - 3 * k) {
			left_most[i + k] = left_most[i + k - 1].max(sum(i + 1))
		}

		let mut right_most = vec![0; n + 1];
		for i in (2 * k..=n - k).rev() {
			right_most[i] = right_most[i + 1].max(sum(i));
		}

		let mut sum_trip = (0, 0, 0);
		let mut max_sum = std::i32::MIN;
		for i in k..n - k {
			let cur_sum = sum(i) + left_most[i - 1] + right_most[i + k];
			if cur_sum > max_sum {
				max_sum = cur_sum;
				sum_trip = (left_most[i - 1], i, right_most[i + k]);
			}
		}

		let find_sum = |rang: std::ops::Range<usize>, val| -> i32 {
			rang.into_iter()
				.map(|i| (i, sum(i)))
				.find(|&(_, s)| s == val)
				.unwrap()
				.0 as i32
		};
		vec![
			find_sum(0..sum_trip.1, sum_trip.0),
			sum_trip.1 as i32,
			find_sum(sum_trip.1 + k..n, sum_trip.2),
		]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_689() {
		assert_eq!(
			Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2),
			[0, 3, 5]
		);
		assert_eq!(
			Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 1, 2, 1, 2, 1], 2),
			[0, 2, 4]
		);
		assert_eq!(
			Solution::max_sum_of_three_subarrays(vec![4, 3, 2, 1], 1),
			[0, 1, 2]
		);
		assert_eq!(
			Solution::max_sum_of_three_subarrays(vec![7, 13, 20, 19, 19, 2, 10, 1, 1, 19], 3),
			[1, 4, 7]
		);
	}
}
