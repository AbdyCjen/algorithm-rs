/**
 * [898] Bitwise ORs of Subarrays
 *
 * We have an array arr of non-negative integers.
 * For every (contiguous) subarray sub = [arr[i], arr[i + 1], ..., arr[j]] (with i <= j), we take the bitwise OR of all the elements in sub, obtaining a result arr[i] | arr[i + 1] | ... | arr[j].
 * Return the number of possible results. Results that occur more than once are only counted once in the final answer
 *  
 * <strong class="example">Example 1:
 *
 * Input: arr = [0]
 * Output: 1
 * Explanation: There is only one possible result: 0.
 *
 * <strong class="example">Example 2:
 *
 * Input: arr = [1,1,2]
 * Output: 3
 * Explanation: The possible subarrays are [1], [1], [2], [1, 1], [1, 2], [1, 1, 2].
 * These yield the results 1, 1, 2, 1, 3, 3.
 * There are 3 unique values, so the answer is 3.
 *
 * <strong class="example">Example 3:
 *
 * Input: arr = [1,2,4]
 * Output: 6
 * Explanation: The possible results are 1, 2, 3, 4, 6, and 7.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 5 * 10^4
 *     0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
		let mut ans = vec![];
		let mut tmp = vec![];
		let mut prv = 0;
		for i in arr {
			tmp.extend(ans[prv..].iter().map(|&n| n | i));
			tmp.push(i);
			tmp.dedup();
			prv = ans.len();
			ans.extend(tmp.drain(0..));
		}
		ans.into_iter()
			.collect::<std::collections::HashSet<_>>()
			.len() as i32
	}
	pub fn subarray_bitwise_o_rs_01(arr: Vec<i32>) -> i32 {
		use std::collections::HashSet;
		let mut ans = HashSet::new();
		// incr at i_th round contains all subarray bit-or sum which is ended with arr[i]
		// at any round, incr.len() <= 30;
		let mut incr = HashSet::new();
		for i in arr {
			incr = incr.drain().map(|n: i32| (n | i)).collect();
			incr.insert(i);
			ans.extend(incr.iter().copied());
		}
		ans.len() as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_898() {
		assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 1, 2]), 3);
		assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 2, 4]), 6);
	}
}
