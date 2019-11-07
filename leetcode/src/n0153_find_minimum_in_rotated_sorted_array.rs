/**
 * [153] Find Minimum in Rotated Sorted Array
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 *
 * (i.e.,  [0,1,2,4,5,6,7] might become  [4,5,6,7,0,1,2]).
 *
 * Find the minimum element.
 *
 * You may assume no duplicate exists in the array.
 *
 * Example 1:
 *
 *
 * Input: [3,4,5,1,2]
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input: [4,5,6,7,0,1,2]
 * Output: 0
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_min(nums: Vec<i32>) -> i32 {
		let (mut i, mut j): (isize, isize) = (0, nums.len() as isize - 1);
		while i < j {
			if nums[i as usize] < nums[j as usize] {
				return nums[i as usize];
			}
			let mid: usize = (i + j) as usize / 2;
			if nums[mid] >= nums[i as usize] {
				i = mid as isize + 1;
			} else {
				j = mid as isize;
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
	fn test_153() {
		assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
		assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
		assert_eq!(Solution::find_min(vec![3, 1, 2]), 1);
	}
}
