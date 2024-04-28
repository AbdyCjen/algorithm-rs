/**
 * [992] Subarrays with K Different Integers
 *
 * Given an integer array nums and an integer k, return the number of good subarrays of nums.
 * A good array is an array where the number of different integers in that array is exactly k.
 *
 *     For example, [1,2,3,1,2] has 3 different integers: 1, 2, and 3.
 *
 * A subarray is a contiguous part of an array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,1,2,3], k = 2
 * Output: 7
 * Explanation: Subarrays formed with exactly 2 different integers: [1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,1,3,4], k = 3
 * Output: 3
 * Explanation: Subarrays formed with exactly 3 different integers: [1,2,1,3], [2,1,3], [1,3,4].
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 2 * 10^4
 *     1 <= nums[i], k <= nums.length
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
		Solution::at_most_k_distinct(&nums, k) - Solution::at_most_k_distinct(&nums, k - 1)
	}
	fn at_most_k_distinct(nums: &[i32], k: i32) -> i32 {
		let mut freq = vec![0; nums.len() + 1];
		let mut cnt = 0;
		let mut i = 0;
		let mut ans = 0;
		for (j, &n) in (0..).zip(nums) {
			freq[n as usize] += 1;
			cnt += (freq[n as usize] == 1) as i32;
			while cnt > k {
				let num = nums[i];
				freq[num as usize] -= 1;
				cnt -= (freq[num as usize] == 0) as i32;
				i += 1;
			}
			ans += j - i + 1;
		}
		ans as i32
	}
}

// submission codes end
