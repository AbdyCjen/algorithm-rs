/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
 * If target is not found in the array, return [-1, -1].
 * Follow up: Could you write an algorithm with O(log n) runtime complexity?
 *  
 * Example 1:
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 * Example 2:
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 * Example 3:
 * Input: nums = [], target = 0
 * Output: [-1,-1]
 *  
 * Constraints:
 *
 *     0 <= nums.length <= 10^5
 *     -10^9 <= nums[i] <= 10^9
 *     nums is a non-decreasing array.
 *     -10^9 <= target <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
		// find the first element >= target
		fn binary_search(nums: &[i32], target: i32) -> usize {
			if nums.is_empty() {
				return 0;
			} else if *nums.last().unwrap() < target {
				return nums.len();
			}

			let (mut lb, mut ub) = (0, nums.len() - 1);

			while lb < ub {
				let mid = (lb + ub) / 2;

				use std::cmp::Ordering::*;
				match nums[mid].cmp(&target) {
					Equal => ub = mid,
					Less => lb = mid + 1,
					Greater => ub = mid,
				}
			}
			lb
		}

		let lb = binary_search(&nums, target);
		if nums.get(lb) != Some(&target) {
			return vec![-1, -1];
		}
		let ub = lb + binary_search(&nums[lb + 1..], target + 1);
		vec![lb as i32, ub as i32]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_34() {
		assert_eq!(
			Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
			vec![3, 4]
		);
		assert_eq!(
			Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
			vec![-1, -1]
		);
		assert_eq!(Solution::search_range(vec![], 6), vec![-1, -1]);
		assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0]);
		assert_eq!(Solution::search_range(vec![2, 2], 1), vec![-1, -1]);
		assert_eq!(Solution::search_range(vec![2, 2], 2), vec![0, 1]);
	}
}
