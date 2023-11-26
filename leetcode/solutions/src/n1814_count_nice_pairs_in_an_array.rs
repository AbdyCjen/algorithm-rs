/**
 * [1814] Count Nice Pairs in an Array
 *
 * You are given an array nums that consists of non-negative integers. Let us define rev(x) as the reverse of the non-negative integer x. For example, rev(123) = 321, and rev(120) = 21. A pair of indices (i, j) is nice if it satisfies all of the following conditions:
 *
 *     0 <= i < j < nums.length
 *     nums[i] + rev(nums[j]) == nums[j] + rev(nums[i])
 *
 * Return the number of nice pairs of indices. Since that number can be too large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [42,11,1,97]
 * Output: 2
 * Explanation: The two pairs are:
 *  - (0,3) : 42 + rev(97) = 42 + 79 = 121, 97 + rev(42) = 97 + 24 = 121.
 *  - (1,2) : 11 + rev(1) = 11 + 1 = 12, 1 + rev(11) = 1 + 11 = 12.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [13,10,35,24,76]
 * Output: 4
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
		let mut cnts = std::collections::HashMap::new();
		for n in nums {
			*cnts.entry(n - Self::rev(n)).or_insert(0_i64) += 1;
		}
		cnts.into_values()
			.fold(0, |a, c| (a + c * (c - 1) / 2) % (1e9 as i64 + 7)) as i32
	}

	fn rev(mut n: i32) -> i32 {
		let mut ans = 0;
		while n > 0 {
			ans = ans * 10 + n % 10;
			n /= 10;
		}
		ans
	}
}

// submission codes end
