/**
 * [2535] Difference Between Element Sum and Digit Sum of an Array
 *
 * You are given a positive integer array nums.
 *
 *     The element sum is the sum of all the elements in nums.
 *     The digit sum is the sum of all the digits (not necessarily distinct) that appear in nums.
 *
 * Return the absolute difference between the element sum and digit sum of nums.
 * Note that the absolute difference between two integers x and y is defined as |x - y|.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,15,6,3]
 * Output: 9
 * Explanation:
 * The element sum of nums is 1 + 15 + 6 + 3 = 25.
 * The digit sum of nums is 1 + 1 + 5 + 6 + 3 = 16.
 * The absolute difference between the element sum and digit sum is |25 - 16| = 9.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,3,4]
 * Output: 0
 * Explanation:
 * The element sum of nums is 1 + 2 + 3 + 4 = 10.
 * The digit sum of nums is 1 + 2 + 3 + 4 = 10.
 * The absolute difference between the element sum and digit sum is |10 - 10| = 0.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 2000
 *     1 <= nums[i] <= 2000
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
		nums.iter().sum::<i32>() - nums.into_iter().map(Self::digit_cnt).sum::<i32>()
	}

	fn digit_cnt(mut n: i32) -> i32 {
		let mut ans = 0;
		while n > 0 {
			ans += n % 10;
			n /= 10;
		}
		ans
	}
}

// submission codes end
