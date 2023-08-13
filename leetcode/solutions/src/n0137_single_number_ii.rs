/**
 * [137] Single Number II
 *
 * Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.
 * You must implement a solution with a linear runtime complexity and use only constant extra space.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [2,2,3,2]
 * Output: 3
 * <strong class="example">Example 2:
 * Input: nums = [0,1,0,1,0,1,99]
 * Output: 99
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 3 * 10^4
 *     -2^31 <= nums[i] <= 2^31 - 1
 *     Each element in nums appears exactly three times except for one element which appears once.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn single_number(mut nums: Vec<i32>) -> i32 { Self::solve(&mut nums) }
	fn solve(nums: &mut [i32]) -> i32 {
		match nums {
			[n] => *n,
			nums => {
				let ax = nums[0];
				let (mut i, mut j, mut k) = (0, 0, nums.len() - 1);
				while j <= k {
					use std::cmp::Ordering::*;
					match nums[j].cmp(&ax) {
						Less => {
							nums.swap(i, j);
							i += 1;
							j += 1;
						}
						Greater => {
							nums.swap(j, k);
							k -= 1;
						}
						_ => j += 1,
					}
				}
				if i % 3 != 0 {
					Self::solve(&mut nums[..i])
				} else if j % 3 == 0 {
					Self::solve(&mut nums[j..])
				} else {
					ax
				}
			}
		}
	}
}

// submission codes end
