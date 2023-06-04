/**
 * [35] Search Insert Position
 *
 * Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
 * You must write an algorithm with O(log n) runtime complexity.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,3,5,6], target = 5
 * Output: 2
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,3,5,6], target = 2
 * Output: 1
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,3,5,6], target = 7
 * Output: 4
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^4
 *     -10^4 <= nums[i] <= 10^4
 *     nums contains distinct values sorted in ascending order.
 *     -10^4 <= target <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn search_insert_01(nums: Vec<i32>, target: i32) -> i32 {
		let (mut l, mut r) = (0, nums.len() as i32);
		while l < r {
			let m = (l + r) / 2;
			if nums[m as usize] < target {
				l = m + 1;
			} else {
				r = m;
			}
		}
		l
	}
	pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
		match nums.binary_search(&target) {
			Ok(i) => i as i32,
			Err(i) => i as i32,
		}
	}
}

// submission codes end
