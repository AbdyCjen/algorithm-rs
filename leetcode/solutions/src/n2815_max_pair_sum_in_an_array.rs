/**
 * [2815] Max Pair Sum in an Array
 *
 * You are given a 0-indexed integer array nums. You have to find the maximum sum of a pair of numbers from nums such that the maximum digit in both numbers are equal.
 * Return the maximum sum or -1 if no such pair exists.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [51,71,17,24,42]
 * Output: 88
 * Explanation:
 * For i = 1 and j = 2, nums[i] and nums[j] have equal maximum digits with a pair sum of 71 + 17 = 88.
 * For i = 3 and j = 4, nums[i] and nums[j] have equal maximum digits with a pair sum of 24 + 42 = 66.
 * It can be shown that there are no other pairs with equal maximum digits, so the answer is 88.
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,3,4]
 * Output: -1
 * Explanation: No pair exists in nums with equal maximum digits.
 *
 *  
 * Constraints:
 *
 *     2 <= nums.length <= 100
 *     1 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_sum(nums: Vec<i32>) -> i32 {
		let mut cnts = vec![vec![]; 10];
		for n in nums {
			cnts[Self::digit(n) as usize].push(n);
		}
		let mut ans = -1;
		for cnt in cnts {
			let mut hp = std::collections::BinaryHeap::from(cnt);
			if let (Some(m), Some(n)) = (hp.pop(), hp.pop()) {
				ans = ans.max(m + n);
			}
		}
		ans
	}

	fn digit(mut n: i32) -> i32 {
		let mut ans = 0;
		while n > 0 {
			ans = ans.max(n % 10);
			n /= 10;
		}
		ans
	}
}

// submission codes end
