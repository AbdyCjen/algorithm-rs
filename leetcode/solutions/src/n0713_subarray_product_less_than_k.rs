/**
 * [713] Subarray Product Less Than K
 *
 * Given an array of integers nums and an integer k, return the number of contiguous subarrays where the product of all the elements in the subarray is strictly less than k.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [10,5,2,6], k = 100
 * Output: 8
 * Explanation: The 8 subarrays that have product less than 100 are:
 * [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
 * Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,3], k = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 3 * 10^4
 *     1 <= nums[i] <= 1000
 *     0 <= k <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
		let mut i = 0;
		let mut cur = 1;
		let mut ans = 0;
		for j in 0..nums.len() {
			cur *= nums[j];
			while cur >= k && i <= j {
				cur /= nums[i];
				i += 1;
			}
			ans += (j + 1 - i) as i32;
		}
		ans
	}
}

// submission codes end
