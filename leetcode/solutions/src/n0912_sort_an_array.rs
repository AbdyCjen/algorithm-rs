/**
 * [912] Sort an Array
 *
 * Given an array of integers nums, sort the array in ascending order and return it.
 * You must solve the problem without using any built-in functions in O(nlog(n)) time complexity and with the smallest space complexity possible.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [5,2,3,1]
 * Output: [1,2,3,5]
 * Explanation: After sorting the array, the positions of some numbers are not changed (for example, 2 and 3), while the positions of other numbers are changed (for example, 1 and 5).
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [5,1,1,2,0,0]
 * Output: [0,0,1,1,2,5]
 * Explanation: Note that the values of nums are not necessairly unique.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 5 * 10^4
 *     -5 * 10^4 <= nums[i] <= 5 * 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
		let mut buck = vec![0; 1e5 as usize + 1];
		for i in nums.drain(0..) {
			buck[(i + 5e4 as i32) as usize] += 1;
		}

		for (t, i) in buck.into_iter().zip((-5e4 as i32)..) {
			for _ in 0..t {
				nums.push(i);
			}
		}

		nums
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_912() {
		assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
	}
}
