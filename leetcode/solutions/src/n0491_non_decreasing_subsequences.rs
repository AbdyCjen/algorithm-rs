/**
 * [491] Non-decreasing Subsequences
 *
 * Given an integer array nums, return all the different possible non-decreasing subsequences of the given array with at least two elements. You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [4,6,7,7]
 * Output: [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [4,4,3,2,1]
 * Output: [[4,4]]
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 15
 *     -100 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashSet;
impl Solution {
	pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
		let mut ans = HashSet::new();
		Self::solve(&nums, &mut vec![], &mut ans);
		ans.into_iter().collect()
	}

	fn solve(nums: &[i32], cur: &mut Vec<i32>, ans: &mut HashSet<Vec<i32>>) {
		if nums.is_empty() {
			return;
		}
		for (i, n) in nums.iter().enumerate() {
			if cur.last() <= Some(n) {
				cur.push(*n);
				Self::solve(&nums[(i + 1)..], cur, ans);
				if cur.len() > 1 {
					ans.insert(cur.clone());
				}
				cur.pop();
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_491() {}
}
