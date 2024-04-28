/**
 * [238] Product of Array Except Self
 *
 * Given an array nums of n integers where n > 1,  return an array output such that output[i] is equal to the product of all the elements of nums except nums[i].
 * Example:
 *
 * Input:  [1,2,3,4]
 * Output: [24,12,8,6]
 *
 * Constraint: It's guaranteed that the product of the elements of any prefix or suffix of the array (including the whole array) fits in a 32 bit integer.
 * Note: Please solve it without division and in O(n).
 * Follow up:<br />
 * Could you solve it with constant space complexity? (The output array does not count as extra space for the purpose of space complexity analysis.)
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
		let mut sums = vec![1];
		let mut cur = 1;
		for &n in nums.iter().rev() {
			cur *= n;
			sums.push(cur);
		}
		let mut cur = 1;
		for (n, m) in nums.iter_mut().zip(sums.iter().rev().skip(1)) {
			let a = cur * m;
			cur *= *n;
			*n = a;
		}
		nums
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_238() {
		assert_eq!(
			Solution::product_except_self(vec![1, 2, 3, 4]),
			vec![24, 12, 8, 6]
		);
		assert_eq!(Solution::product_except_self(vec![0, 0]), vec![0, 0]);
	}
}
