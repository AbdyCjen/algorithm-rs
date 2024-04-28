/**
 * [446] Arithmetic Slices II - Subsequence
 *
 * Given an integer array nums, return the number of all the arithmetic subsequences of nums.
 * A sequence of numbers is called arithmetic if it consists of at least three elements and if the difference between any two consecutive elements is the same.
 *
 *     For example, [1, 3, 5, 7, 9], [7, 7, 7, 7], and [3, -1, -5, -9] are arithmetic sequences.
 *     For example, [1, 1, 2, 5, 7] is not an arithmetic sequence.
 *
 * A subsequence of an array is a sequence that can be formed by removing some elements (possibly none) of the array.
 *
 *     For example, [2,5,10] is a subsequence of [1,2,1,<u>2</u>,4,1,<u>5</u>,<u>10</u>].
 *
 * The test cases are generated so that the answer fits in 32-bit integer.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,4,6,8,10]
 * Output: 7
 * Explanation: All arithmetic subsequence slices are:
 * [2,4,6]
 * [4,6,8]
 * [6,8,10]
 * [2,4,6,8]
 * [4,6,8,10]
 * [2,4,6,8,10]
 * [2,6,10]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [7,7,7,7,7]
 * Output: 16
 * Explanation: Any subsequence of this array is arithmetic.
 *
 *  
 * Constraints:
 *
 *     1  <= nums.length <= 1000
 *     -2^31 <= nums[i] <= 2^31 - 1
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
		use std::collections::HashMap;
		let mut dp: Vec<HashMap<i32, i32>> = Vec::with_capacity(nums.len());
		let mut ans = 0;
		for (i, &n) in (0..).zip(&nums) {
			let mut cnt = HashMap::new();
			for (j, &prv) in (0..).zip(&nums[..i]) {
				if let Some(dif) = n.checked_sub(prv) {
					let dp_j = *dp[j].get(&dif).unwrap_or(&0);
					ans += dp_j;
					*cnt.entry(dif).or_insert(0) += dp_j + 1;
				}
			}
			dp.push(cnt);
		}
		ans
	}
	pub fn number_of_arithmetic_slices1(nums: Vec<i32>) -> i32 {
		use std::collections::*;
		let mut occurs: HashMap<i32, Vec<i32>> = HashMap::new();
		let mut cnts: Vec<HashMap<i32, i32>> = vec![];
		let mut ans = 0;
		for (i, &n) in (0..).zip(&nums) {
			let mut cnt = HashMap::new();
			for (pi, &prv) in (0..).zip(&nums[..i as usize]) {
				if let Some(dif) = n.checked_sub(prv) {
					let mut dif_cnt = *cnts[pi].get(&dif).unwrap_or(&0);
					if let Some(pprv) = prv.checked_sub(dif) {
						dif_cnt += occurs
							.get(&pprv)
							.map(|pp| pp.partition_point(|j| *j < pi as i32) as i32)
							.unwrap_or(0);
					}
					ans += dif_cnt;
					*cnt.entry(dif).or_insert(0) += dif_cnt;
				}
			}
			cnts.push(cnt);
			occurs.entry(n).or_default().push(i);
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_446() {
		assert_eq!(
			Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]),
			0
		);
	}
}
