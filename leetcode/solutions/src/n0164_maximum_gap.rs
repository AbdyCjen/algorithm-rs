/**
 * [164] Maximum Gap
 *
 * Given an integer array nums, return the maximum difference between two successive elements in its sorted form. If the array contains less than two elements, return 0.
 * You must write an algorithm that runs in linear time and uses linear extra space.
 *  
 * Example 1:
 *
 * Input: nums = [3,6,9,1]
 * Output: 3
 * Explanation: The sorted form of the array is [1,3,6,9], either (3,6) or (6,9) has the maximum difference 3.
 *
 * Example 2:
 *
 * Input: nums = [10]
 * Output: 0
 * Explanation: The array contains less than 2 elements, therefore return 0.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^4
 *     0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn maximum_gap(nums: Vec<i32>) -> i32 {
		if nums.len() <= 1 {
			return 0;
		} else if nums.len() == 2 {
			return (nums[1] - nums[0]).abs();
		}
		let n = nums.len();
		use std::i32::{MAX, MIN};
		let mut maxs = vec![MIN; n];
		let mut mins = vec![MAX; n];
		let max = *nums.iter().max().unwrap();
		let min = *nums.iter().min().unwrap();
		if max == min {
			return 0;
		}

		let avggap = (max - min) as f64 / (n as f64 - 1.0);
		for &v in nums.iter() {
			let idx = ((v - min) as f64 / avggap) as usize;
			mins[idx] = mins[idx].min(v);
			maxs[idx] = maxs[idx].max(v);
		}

		let premax = maxs[0];
		mins.into_iter()
			.zip(maxs)
			.skip(1)
			.scan(premax, |premax, (cur_min, cur_max)| {
				if cur_min <= cur_max {
					let premax_ = *premax;
					*premax = cur_max;
					Some(cur_min - premax_)
				} else {
					Some(MIN)
				}
			})
			.max()
			.unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_164() {
		assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
		assert_eq!(Solution::maximum_gap(vec![10]), 0);
	}
}
