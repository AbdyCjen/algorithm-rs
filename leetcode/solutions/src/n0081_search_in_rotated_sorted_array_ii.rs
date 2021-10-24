/**
 * [81] Search in Rotated Sorted Array II
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 *
 * (i.e., [0,0,1,2,2,5,6] might become [2,5,6,0,0,1,2]).
 *
 * You are given a target value to search. If found in the array return true, otherwise return false.
 *
 * Example 1:
 *
 *
 * Input: nums = [2,5,6,0,0,1,2], target = 0
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [2,5,6,0,0,1,2], target = 3
 * Output: false
 *
 * Follow up:
 *
 *
 * This is a follow up problem to <a href="/problems/search-in-rotated-sorted-array/description/">Search in Rotated Sorted Array</a>, where nums may contain duplicates.
 * Would this affect the run-time complexity? How and why?
 *
 *
 */
pub struct Solution {}

// submission codes start here

// 没有log(n)的算法, 因为最坏的情况下, 必须遍历全部数组才能判断不存在(全是dup)
#[allow(dead_code)]
impl Solution {
	pub fn search(nums: Vec<i32>, target: i32) -> bool {
		let (mut i, mut j): (isize, isize) = (0, nums.len() as isize - 1);

		while i <= j {
			let mid: usize = (i + j) as usize / 2;
			if nums[mid] == target {
				return true;
			} else if nums[mid] == nums[i as usize] && nums[mid] == nums[j as usize] {
				j -= 1;
				i += 1;
			} else if nums[mid] >= nums[i as usize] {
				if target < nums[mid] && target >= nums[i as usize] {
					j = mid as isize - 1;
				} else {
					i = mid as isize + 1;
				}
			} else if target > nums[mid] && target <= nums[j as usize] {
				i = mid as isize + 1;
			} else {
				j = mid as isize - 1;
			}
		}
		false
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_81() {
		assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
		assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
		assert!(!Solution::search(vec![], 3));
		assert!(Solution::search(vec![1], 1));
		assert!(!Solution::search(vec![1], 0));
		assert!(Solution::search(vec![3, 1], 1));
		assert!(Solution::search(vec![1, 3], 3));
		assert!(Solution::search(vec![5, 1, 3], 3));
		assert!(Solution::search(vec![1, 3, 1, 1, 1], 3));
		assert!(Solution::search(vec![1, 1, 1, 3, 1], 3));
		assert!(Solution::search(vec![1, 0, 0, 0, 1], 1));
	}
}
