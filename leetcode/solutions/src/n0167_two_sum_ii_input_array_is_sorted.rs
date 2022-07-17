/**
 * [167] Two Sum II - Input Array Is Sorted
 *
 * Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
 * Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
 * The tests are generated such that there is exactly one solution. You may not use the same element twice.
 * Your solution must use only constant extra space.
 *  
 * Example 1:
 *
 * Input: numbers = [<u>2</u>,<u>7</u>,11,15], target = 9
 * Output: [1,2]
 * Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
 *
 * Example 2:
 *
 * Input: numbers = [<u>2</u>,3,<u>4</u>], target = 6
 * Output: [1,3]
 * Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].
 *
 * Example 3:
 *
 * Input: numbers = [<u>-1</u>,<u>0</u>], target = -1
 * Output: [1,2]
 * Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
 *
 *  
 * Constraints:
 *
 *     2 <= numbers.length <= 3 * 10^4
 *     -1000 <= numbers[i] <= 1000
 *     numbers is sorted in non-decreasing order.
 *     -1000 <= target <= 1000
 *     The tests are generated such that there is exactly one solution.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
		for (i, n) in nums.iter().enumerate() {
			if let Ok(j) = &nums[(i + 1)..].binary_search(&(target - n)) {
				return vec![i as i32 + 1, (i + j) as i32 + 2];
			}
		}
		unreachable!()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_167() {
		assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
		assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
		assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
		assert_eq!(Solution::two_sum(vec![1, 3, 4, 4], 8), vec![3, 4]);
	}
}
