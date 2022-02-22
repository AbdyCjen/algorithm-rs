/**
 * [400] Nth Digit
 *
 * Given an integer n, return the n^th digit of the infinite integer sequence [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...].
 *  
 * Example 1:
 *
 * Input: n = 3
 * Output: 3
 *
 * Example 2:
 *
 * Input: n = 11
 * Output: 0
 * Explanation: The 11^th digit of the sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... is a 0, which is part of the number 10.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 2^31 - 1
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn find_nth_digit(mut n: i32) -> i32 {
		for i in 1..100 {
			let (b, cnt) = (10_i32.pow(i - 1), 10_i64.pow(i - 1) * 9 * i as i64);

			if (1..=cnt).contains(&(n as i64)) {
				let i = i as i32;
				let mut num = b + (n - 1) / i;
				for _ in 0..(i - (n % i)) % i {
					num /= 10;
				}
				return num % 10;
			}
			n -= cnt as i32;
		}

		unreachable!()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_400() {
		assert_eq!(Solution::find_nth_digit(3), 3);
		assert_eq!(Solution::find_nth_digit(9), 9);
		assert_eq!(Solution::find_nth_digit(10), 1);
		assert_eq!(Solution::find_nth_digit(11), 0);
		assert_eq!(Solution::find_nth_digit(189), 9);
		assert_eq!(Solution::find_nth_digit(190), 1);
		assert_eq!(Solution::find_nth_digit(191), 0);
		assert_eq!(Solution::find_nth_digit(191), 0);
		assert_eq!(Solution::find_nth_digit(1000000000), 1);
	}
}
