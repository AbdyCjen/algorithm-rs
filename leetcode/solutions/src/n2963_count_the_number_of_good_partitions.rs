/**
 * [2963] Count the Number of Good Partitions
 *
 * You are given a 0-indexed array nums consisting of positive integers.
 * A partition of an array into one or more contiguous subarrays is called good if no two subarrays contain the same number.
 * Return the total number of good partitions of nums.
 * Since the answer may be large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [1,2,3,4]
 * Output: 8
 * Explanation: The 8 possible good partitions are: ([1], [2], [3], [4]), ([1], [2], [3,4]), ([1], [2,3], [4]), ([1], [2,3,4]), ([1,2], [3], [4]), ([1,2], [3,4]), ([1,2,3], [4]), and ([1,2,3,4]).
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,1,1,1]
 * Output: 1
 * Explanation: The only possible good partition is: ([1,1,1,1]).
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = [1,2,1,3]
 * Output: 2
 * Explanation: The 2 possible good partitions are: ([1,2,1], [3]) and ([1,2,1,3]).
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
	pub fn number_of_good_partitions(nums: Vec<i32>) -> i32 {
		use std::collections::*;
		let mut prv = HashMap::new();
		let mut rng = vec![(0, 0); 0];
		for (i, n) in (0..).zip(nums) {
			let mut pr = *prv.entry(n).or_insert(i);
			while let Some(&(ppr, ppr_til)) = rng.last() {
				if ppr_til >= pr {
					pr = ppr.min(pr);
					rng.pop();
				} else {
					break;
				}
			}
			rng.push((pr, i));
		}
		Self::solve(rng.len() as i32 - 1)
	}

	fn solve(mut exp: i32) -> i32 {
		let mut ans = 1;
		let mut po = 2_i64;
		const MO: i64 = 1e9 as i64 + 7;
		while exp > 0 {
			if exp % 2 == 1 {
				ans = (ans * po) % MO;
			}
			po = (po * po) % MO;
			exp >>= 1;
		}
		ans as i32
	}
}

// submission codes end
