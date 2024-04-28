/**
 * [152] Maximum Product Subarray
 *
 * Given an integer array nums, find a <span data-keyword="subarray-nonempty">subarray</span> that has the largest product, and return the product.
 * The test cases are generated so that the answer will fit in a 32-bit integer.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,3,-2,4]
 * Output: 6
 * Explanation: [2,3] has the largest product 6.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [-2,0,-1]
 * Output: 0
 * Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 2 * 10^4
 *     -10 <= nums[i] <= 10
 *     The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_product(nums: Vec<i32>) -> i32 {
		nums.into_iter()
			.fold((i32::MIN, 1, 1), |(ans, min, max), n| {
				let (a, b) = (n * min, n * max);
				let m = a.max(b).max(n);
				(ans.max(m), a.min(b).min(n), m)
			})
			.0
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_152() {
		assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
		assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
	}
}
