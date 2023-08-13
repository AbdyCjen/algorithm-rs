/**
 * [2009] Minimum Number of Operations to Make Array Continuous
 *
 * You are given an integer array nums. In one operation, you can replace any element in nums with any integer.
 * nums is considered continuous if both of the following conditions are fulfilled:
 *
 *     All elements in nums are unique.
 *     The difference between the maximum element and the minimum element in nums equals nums.length - 1.
 *
 * For example, nums = [4, 2, 5, 3] is continuous, but nums = [1, 2, 3, 5, 6] is not continuous.
 * Return the minimum number of operations to make nums continuous.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [4,2,5,3]
 * Output: 0
 * Explanation: nums is already continuous.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,3,5,6]
 * Output: 1
 * Explanation: One possible solution is to change the last element to 4.
 * The resulting array is [1,2,3,5,4], which is continuous.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,10,100,1000]
 * Output: 3
 * Explanation: One possible solution is to:
 * - Change the second element to 2.
 * - Change the third element to 3.
 * - Change the fourth element to 4.
 * The resulting array is [1,2,3,4], which is continuous.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_operations(mut nums: Vec<i32>) -> i32 {
		nums.sort_unstable();
		let mut dq = std::collections::VecDeque::new();
		let len = nums.len() as i32;
		let mut ans = len;
		let mut h = 0;
		for &n in &nums {
			while n - nums[h] + 1 > len {
				h += 1;
				if dq.front() != Some(&nums[h]) {
					dq.pop_front();
				}
			}
			if dq.back() != Some(&n) {
				dq.push_back(n);
			}
			ans = ans.min(len - dq.len() as i32);
		}
		ans
	}
}

// submission codes end
