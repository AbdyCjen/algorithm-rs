/**
 * [1671] Minimum Number of Removals to Make Mountain Array
 *
 * You may recall that an array arr is a mountain array if and only if:
 *
 *     arr.length >= 3
 *     There exists some index i (0-indexed) with 0 < i < arr.length - 1 such that:
 *     
 *         arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
 *         arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
 *     
 *     
 *
 * Given an integer array nums​​​, return the minimum number of elements to remove to make nums​​​ a mountain array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,3,1]
 * Output: 0
 * Explanation: The array itself is a mountain array so we do not need to remove any elements.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [2,1,1,5,6,2,3,1]
 * Output: 3
 * Explanation: One solution is to remove the elements at indices 0, 1, and 5, making the array nums = [1,5,6,3,1].
 *
 *  
 * Constraints:
 *
 *     3 <= nums.length <= 1000
 *     1 <= nums[i] <= 10^9
 *     It is guaranteed that you can make a mountain array out of nums.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
		use std::collections::*;
		let mut leng = BTreeMap::new();
		let mut cnts = vec![];
		for &n in nums.iter().rev() {
			let l = 1 + leng.range(..n).map(|v| *v.1).max().unwrap_or(0);
			cnts.push(l);
			leng.insert(n, l);
		}
		leng.clear();
		nums.len() as i32
			- cnts
				.into_iter()
				.rev()
				.zip(nums)
				.filter_map(|(c, n)| {
					let l = 1 + leng.range(..n).map(|v| *v.1).max().unwrap_or(0);
					leng.insert(n, l);
					(c > 1 && l > 1).then_some(c + l - 1)
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
	fn test_1671() {
		assert_eq!(
			Solution::minimum_mountain_removals(vec![9, 8, 1, 7, 6, 5, 4, 3, 2, 1]),
			2
		);
		assert_eq!(
			Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]),
			3
		);
	}
}
