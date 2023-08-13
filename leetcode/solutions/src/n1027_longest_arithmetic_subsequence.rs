/**
 * [1027] Longest Arithmetic Subsequence
 *
 * Given an array nums of integers, return the length of the longest arithmetic subsequence in nums.
 * Note that:
 *
 *     A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
 *     A sequence seq is arithmetic if seq[i + 1] - seq[i] are all the same value (for 0 <= i < seq.length - 1).
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [3,6,9,12]
 * Output: 4
 * Explanation:  The whole array is an arithmetic sequence with steps of length = 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [9,4,7,2,10]
 * Output: 3
 * Explanation:  The longest arithmetic subsequence is [4,7,10].
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [20,1,15,3,10,5,8]
 * Output: 4
 * Explanation:  The longest arithmetic subsequence is [20,15,10,5].
 *
 *  
 * Constraints:
 *
 *     2 <= nums.length <= 1000
 *     0 <= nums[i] <= 500
 *
 */
pub struct Solution {}

// submission codes start here

// TODO: better complextiy
impl Solution {
	pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
		use std::collections::HashMap;
		let mut cache = vec![HashMap::new(); nums.len()];
		let mut ans = 0;
		for i in 0..nums.len() {
			for j in 0..i {
				let dif = nums[i] - nums[j];
				let prv_cnt = *cache[j].get(&dif).unwrap_or(&0);
				let cnt = cache[i].entry(dif).or_insert(2);
				*cnt = (*cnt).max(prv_cnt + 1);
				ans = ans.max(*cnt);
			}
		}
		ans
	}
}

// submission codes end
