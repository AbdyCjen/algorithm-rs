/**
 * [795] Number of Subarrays with Bounded Maximum
 *
 * Given an integer array nums and two integers left and right, return the number of contiguous non-empty subarrays such that the value of the maximum array element in that subarray is in the range [left, right].
 * The test cases are generated so that the answer will fit in a 32-bit integer.
 *  
 * Example 1:
 *
 * Input: nums = [2,1,4,3], left = 2, right = 3
 * Output: 3
 * Explanation: There are three subarrays that meet the requirements: [2], [2, 1], [3].
 *
 * Example 2:
 *
 * Input: nums = [2,9,2,5,6], left = 2, right = 8
 * Output: 7
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     0 <= nums[i] <= 10^9
 *     0 <= left <= right <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
		let n = nums.len();
		let mut mono_stk = vec![0_usize];
		let mut to_r = vec![0; n];
		for (i, &v) in nums.iter().enumerate().skip(1) {
			while let Some(&j) = mono_stk.last() {
				if nums[j] < v {
					mono_stk.pop();
					to_r[j] = (i - j - 1) as i32;
				} else {
					break;
				}
			}
			mono_stk.push(i);
		}
		while let Some(i) = mono_stk.pop() {
			to_r[i] = (n - i - 1) as i32;
		}

		let mut to_l = vec![0; n];
		mono_stk.push(n - 1);
		for (i, &v) in nums.iter().enumerate().rev().skip(1) {
			while let Some(&j) = mono_stk.last() {
				if nums[j] <= v {
					mono_stk.pop();
					to_l[j] = (j - i - 1) as i32;
				} else {
					break;
				}
			}
			mono_stk.push(i);
		}
		while let Some(i) = mono_stk.pop() {
			to_l[i] = i as i32;
		}

		let rng = left..=right;
		nums.into_iter()
			.zip(to_r)
			.zip(to_l)
			.filter(|((v, _), _)| rng.contains(v))
			.map(|((_, i), j)| (i + 1) * (j + 1))
			.sum()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_795() {
		assert_eq!(
			Solution::num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3),
			3
		);
		assert_eq!(
			Solution::num_subarray_bounded_max(vec![2, 9, 2, 5, 6], 2, 8),
			7
		);
	}
}
