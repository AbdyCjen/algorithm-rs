/**
 * [33] Search in Rotated Sorted Array
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 *
 * (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
 *
 * You are given a target value to search. If found in the array return its index, otherwise return -1.
 *
 * You may assume no duplicate exists in the array.
 *
 * Your algorithm's runtime complexity must be in the order of O(log n).
 *
 * Example 1:
 *
 *
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 *
 */
pub struct Solution {}

// submission codes start here

// 狗屎
use std::cmp::Ordering;
#[allow(dead_code)]
impl Solution {
	pub fn search(nums: Vec<i32>, target: i32) -> i32 {
		if nums.is_empty() {
			return -1;
		}
		let mut lb = 0;
		let mut ub = nums.len() - 1;
		while lb < ub {
			let mid = (ub + lb) / 2;
			match (
				nums[mid].cmp(&target),
				nums[ub] >= target,
				nums[lb] <= target,
			) {
				(Ordering::Equal, ..) => return mid as i32,
				(Ordering::Less, true, _) => lb = mid + 1,
				(Ordering::Less, false, _) => ub = mid,
				(Ordering::Greater, _, true) => ub = mid,
				(Ordering::Greater, _, false) => lb = mid + 1,
			};
		}
		-1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_33() {
		assert_eq!(Solution::search(vec![], 0), -1);
		assert_eq!(Solution::search(vec![1], 2), -1);
		assert_eq!(Solution::search(vec![1], 1), 0);
		assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
		assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
		assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 4), 0);
	}
}
