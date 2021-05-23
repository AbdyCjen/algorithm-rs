/**
 * [462] Minimum Moves to Equal Array Elements II
 *
 * Given an integer array nums of size n, return the minimum number of moves required to make all array elements equal.
 * In one move, you can increment or decrement an element of the array by 1.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3]
 * Output: 2
 * Explanation:
 * Only two moves are needed (remember each move increments or decrements one element):
 * [<u>1</u>,2,3]  =>  [2,2,<u>3</u>]  =>  [2,2,2]
 *
 * Example 2:
 *
 * Input: nums = [1,10,2,9]
 * Output: 16
 *
 *  
 * Constraints:
 *
 *     n == nums.length
 *     1 <= nums.length <= 10^5
 *     -10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
		nums.sort_unstable();
		let mid = nums[nums.len() / 2];
		nums.into_iter().map(|i| (i - mid).abs()).sum()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_462() {
		assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
		assert_eq!(Solution::min_moves2(vec![1, 10, 2, 9]), 16);
		assert_eq!(Solution::min_moves2(vec![1, 0, 0, 8, 6]), 14);
	}
}
