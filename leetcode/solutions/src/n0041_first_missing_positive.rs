/**
 * [41] First Missing Positive
 *
 * Given an unsorted integer array nums. Return the smallest positive integer that is not present in nums.
 * You must implement an algorithm that runs in O(n) time and uses O(1) auxiliary space.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,0]
 * Output: 3
 * Explanation: The numbers in the range [1,2] are all in the array.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,4,-1,1]
 * Output: 2
 * Explanation: 1 is in the array but 2 is missing.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [7,8,9,11,12]
 * Output: 1
 * Explanation: The smallest positive integer 1 is missing.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     -2^31 <= nums[i] <= 2^31 - 1
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
		for i in 0..nums.len() {
			while nums[i] >= 1
				&& nums[i] <= nums.len() as i32
				&& nums[i] != nums[nums[i] as usize - 1]
			{
				let ni = nums[i] as usize - 1;
				nums.swap(i, ni);
			}
		}
		(1..)
			.zip(&nums)
			.find(|(i, n)| i != *n)
			.map(|v| v.0)
			.unwrap_or(nums.len() as i32 + 1)
	}
}

// submission codes end
