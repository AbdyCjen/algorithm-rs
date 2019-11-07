/**
 * [154] Find Minimum in Rotated Sorted Array II
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 *
 * (i.e.,  [0,1,2,4,5,6,7] might become  [4,5,6,7,0,1,2]).
 *
 * Find the minimum element.
 *
 * The array may contain duplicates.
 *
 * Example 1:
 *
 *
 * Input: [1,3,5]
 * Output: 1
 *
 * Example 2:
 *
 *
 * Input: [2,2,2,0,1]
 * Output: 0
 *
 * Note:
 *
 *
 * 	This is a follow up problem to <a href="https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/description/">Find Minimum in Rotated Sorted Array</a>.
 * 	Would allow duplicates affect the run-time complexity? How and why?
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_min(nums: Vec<i32>) -> i32 {
		let (mut i, mut j): (isize, isize) = (0, nums.len() as isize - 1);
		while i < j {
			// BUGGY!
			if nums[i as usize] < nums[j as usize] {
				return nums[i as usize];
			}
			let mid = (i + j) as usize / 2;
			if nums[i as usize] == nums[j as usize] {
				i = i + 1;
			} else if nums[mid] <= nums[j as usize] {
				j = mid as isize;
			} else {
				i = mid as isize + 1;
			}
		}
		nums[i as usize]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_154() {
		assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
		assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
		assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1, 2]), 0);
		assert_eq!(Solution::find_min(vec![1, 2, 1]), 1);
		assert_eq!(Solution::find_min(vec![2, 0, 1, 1, 1]), 0);
	}
}
