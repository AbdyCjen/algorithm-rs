/**
 * [1218] Longest Arithmetic Subsequence of Given Difference
 *
 * Given an integer array arr and an integer difference, return the length of the longest subsequence in arr which is an arithmetic sequence such that the difference between adjacent elements in the subsequence equals difference.
 * A subsequence is a sequence that can be derived from arr by deleting some or no elements without changing the order of the remaining elements.
 *  
 * Example 1:
 *
 * Input: arr = [1,2,3,4], difference = 1
 * Output: 4
 * Explanation: The longest arithmetic subsequence is [1,2,3,4].
 * Example 2:
 *
 * Input: arr = [1,3,5,7], difference = 1
 * Output: 1
 * Explanation: The longest arithmetic subsequence is any single element.
 *
 * Example 3:
 *
 * Input: arr = [1,5,7,8,5,3,4,2,1], difference = -2
 * Output: 4
 * Explanation: The longest arithmetic subsequence is [7,5,3,1].
 *
 *  
 * Constraints:
 *
 *     1 <= arr.length <= 10^5
 *     -10^4 <= arr[i], difference <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn longest_subsequence(arr: Vec<i32>, diff: i32) -> i32 {
		use std::collections::HashMap;
		let mut val2cnt: HashMap<i32, i32> = HashMap::with_capacity(arr.len());
		arr.into_iter()
			.map(move |v| {
				let cnt = val2cnt.get(&(v - diff)).unwrap_or(&0) + 1;
				val2cnt.insert(v, cnt);
				cnt
			})
			.max()
			.unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1218() {
		assert_eq!(Solution::longest_subsequence(vec![1, 3, 5, 7], 1), 1);
		assert_eq!(Solution::longest_subsequence(vec![1, 2, 3, 4], 1), 4);
		assert_eq!(
			Solution::longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2),
			4
		);
	}
}
