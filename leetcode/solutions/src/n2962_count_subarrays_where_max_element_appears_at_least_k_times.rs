/**
 * [2962] Count Subarrays Where Max Element Appears at Least K Times
 *
 * You are given an integer array nums and a positive integer k.
 * Return the number of subarrays where the maximum element of nums appears at least k times in that subarray.
 * A subarray is a contiguous sequence of elements within an array.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [1,3,2,3,3], k = 2
 * Output: 6
 * Explanation: The subarrays that contain the element 3 at least 2 times are: [1,3,2,3], [1,3,2,3,3], [3,2,3], [3,2,3,3], [2,3,3] and [3,3].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,4,2,1], k = 3
 * Output: 0
 * Explanation: No subarray contains the element 4 at least 3 times.
 * 
 *  
 * Constraints:
 * 
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^6
 *     1 <= k <= 10^5
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
		let max = nums.iter().copied().max().unwrap();
		let mut ans = 0;
		let mut cnts = vec![];
		for (i, n) in (1..).zip(nums) {
			if n == max {
				cnts.push(i);
			}
			if cnts.len() as i32 >= k {
				ans += cnts[cnts.len() - k as usize];
			}
		}
		ans
	}
}

// submission codes end
