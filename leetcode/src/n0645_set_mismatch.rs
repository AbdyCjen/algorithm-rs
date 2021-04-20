/**
 * [645] Set Mismatch
 *
 * You have a set of integers s, which originally contains all the numbers from 1 to n. Unfortunately, due to some error, one of the numbers in s got duplicated to another number in the set, which results in repetition of one number and loss of another number.
 * You are given an integer array nums representing the data status of this set after the error.
 * Find the number that occurs twice and the number that is missing and return them in the form of an array.
 *  
 * Example 1:
 * Input: nums = [1,2,2,4]
 * Output: [2,3]
 * Example 2:
 * Input: nums = [1,1]
 * Output: [1,2]
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^4
 * 	1 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
		for i in 0..nums.len() {
			while nums[i] - 1 != i as i32 {
				let j = nums[i];

				if nums[j as usize - 1] != j {
					nums.swap(i, j as usize - 1);
				} else {
					break;
				}
			}
		}

		for (i, j) in nums.into_iter().enumerate() {
			let i = i as i32;
			if i + 1 != j {
				return vec![j, i + 1];
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
	fn test_645() {
		assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
		assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
		assert_eq!(Solution::find_error_nums(vec![2, 3, 2]), vec![2, 1]);
	}
}
