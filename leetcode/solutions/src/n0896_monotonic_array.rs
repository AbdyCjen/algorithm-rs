/**
 * [896] Monotonic Array
 *
 * An array is monotonic if it is either monotone increasing or monotone decreasing.
 * An array nums is monotone increasing if for all i <= j, nums[i] <= nums[j]. An array nums is monotone decreasing if for all i <= j, nums[i] >= nums[j].
 * Given an integer array nums, return true if the given array is monotonic, or false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,2,3]
 * Output: true
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [6,5,4,4]
 * Output: true
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,3,2]
 * Output: false
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     -10^5 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn is_monotonic(nums: Vec<i32>) -> bool {
		nums.windows(2).all(|w| w[0] <= w[1]) || nums.windows(2).all(|w| w[0] >= w[1])
	}
}

// submission codes end
