/**
 * [264] Ugly Number II
 *
 * Write a program to find the n-th ugly number.
 *
 * Ugly numbers are positive numbers whose prime factors only include 2, 3, 5.
 *
 * Example:
 *
 *
 * Input: n = 10
 * Output: 12
 * Explanation: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 is the sequence of the first 10 ugly numbers.
 *
 * Note:  
 *
 * <ol>
 *     1 is typically treated as an ugly number.
 *     n does not exceed 1690.
 * </ol>
 */
pub struct Solution {}

// submission codes start here

use std::cmp::min;
#[allow(dead_code)]
impl Solution {
	pub fn nth_ugly_number(n: i32) -> i32 {
		let mut ugly_nums = vec![1];
		let (mut ind_2, mut ind_3, mut ind_5) = (0, 0, 0);
		for _ in 0..(n - 1) {
			let next_2 = ugly_nums[ind_2] * 2;
			let next_3 = ugly_nums[ind_3] * 3;
			let next_5 = ugly_nums[ind_5] * 5;
			let next_ugly_num = min(next_2, min(next_3, next_5));
			ugly_nums.push(next_ugly_num);

			// can't increase in branch, or some dupe will be add to the answer
			if next_ugly_num == next_2 {
				ind_2 += 1;
			}
			if next_ugly_num == next_3 {
				ind_3 += 1;
			}
			if next_ugly_num == next_5 {
				ind_5 += 1;
			}
		}
		//dbg!(&ugly_nums);
		ugly_nums.pop().unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_264() {
		assert_eq!(Solution::nth_ugly_number(10), 12);
	}
}
