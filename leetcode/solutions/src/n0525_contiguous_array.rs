/**
 * [525] Contiguous Array
 *
 * Given a binary array, find the maximum length of a contiguous subarray with equal number of 0 and 1.
 *
 *
 * Example 1:<br />
 *
 * Input: [0,1]
 * Output: 2
 * Explanation: [0, 1] is the longest contiguous subarray with equal number of 0 and 1.
 *
 *
 *
 * Example 2:<br />
 *
 * Input: [0,1,0]
 * Output: 2
 * Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
 *
 *
 *
 * Note:
 * The length of the given binary array will not exceed 50,000.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_max_length(nums: Vec<i32>) -> i32 {
		use std::collections::HashMap;
		let mut m: HashMap<_, _> = HashMap::new();
		m.insert(0, 0);
		let mut ans = 0;
		let mut acc = 0;
		for (i, n) in (1..).zip(nums) {
			acc += if n > 0 { 1 } else { -1 };
			let e = m.entry(acc).or_insert(i);
			ans = ans.max(i - *e);
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_525() {
		assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
		assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
	}
}
