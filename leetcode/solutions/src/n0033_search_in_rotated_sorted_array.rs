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

impl Solution {
	pub fn search(nums: Vec<i32>, x: i32) -> i32 {
		let (mut l, mut r) = (0, nums.len() - 1);
		while l + 1 < r {
			let mid = (l + r) / 2;
			if nums[mid] >= nums[l] {
				l = mid;
			} else {
				r = mid;
			}
		}
		match nums[..r].binary_search(&x) {
			Ok(i) => i as i32,
			_ => match nums[r..].binary_search(&x) {
				Ok(i) => (i + r) as i32,
				_ => -1,
			},
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_33() {
		assert_eq!(Solution::search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8), 4);
		assert_eq!(Solution::search(vec![5, 1, 3], 5), 0);
		assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
		assert_eq!(Solution::search(vec![1], 2), -1);
		assert_eq!(Solution::search(vec![1], 1), 0);
		assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
		assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 4), 0);
	}
}
