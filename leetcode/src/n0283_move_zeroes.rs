/**
 * [283] Move Zeroes
 *
 * Given an array nums, write a function to move all 0's to the end of it while maintaining the relative order of the non-zero elements.
 *
 * Example:
 *
 *
 * Input: [0,1,0,3,12]
 * Output: [1,3,12,0,0]
 *
 * Note:
 *
 * <ol>
 * You must do this in-place without making a copy of the array.
 * Minimize the total number of operations.
 * </ol>
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn move_zeroes(nums: &mut Vec<i32>) {
		let mut i = 0;
		for j in 0..nums.len() {
			match nums[j] {
				0 => {}
				_ => {
					nums.swap(i, j);
					i += 1;
				}
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_283() {
		let mut v1 = vec![0, 1, 0, 3, 12];
		Solution::move_zeroes(&mut v1);
		assert_eq!(v1, vec![1, 3, 12, 0, 0]);
	}
}
