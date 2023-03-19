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

impl Solution {
	pub fn nth_ugly_number(n: i32) -> i32 {
		let mut nums = vec![1];
		let mut idx = [0; 3];
		for _ in 1..n {
			let next = [nums[idx[0]] * 2, nums[idx[1]] * 3, nums[idx[2]] * 5];
			let ugly = next[0].min(next[1]).min(next[2]);
			for (&n, i) in next.iter().zip(&mut idx) {
				if n == ugly {
					*i += 1;
				}
			}
		}
		nums.pop().unwrap()
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
