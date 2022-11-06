/**
 * [976] Largest Perimeter Triangle
 *
 * Given an integer array nums, return the largest perimeter of a triangle with a non-zero area, formed from three of these lengths. If it is impossible to form any triangle of a non-zero area, return 0.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,1,2]
 * Output: 5
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     3 <= nums.length <= 10^4
 *     1 <= nums[i] <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
		nums.sort_unstable();
		for blk in nums.windows(3).rev() {
			if blk[1] + blk[0] > blk[2] {
				return blk.iter().sum();
			}
		}
		0
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_976() {
		assert_eq!(Solution::largest_perimeter(vec![1, 2, 1]), 0);
		assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
	}
}
