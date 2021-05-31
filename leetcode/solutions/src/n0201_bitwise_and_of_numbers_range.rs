/**
 * [201] Bitwise AND of Numbers Range
 *
 * Given a range [m, n] where 0 <= m <= n <= 2147483647, return the bitwise AND of all numbers in this range, inclusive.
 *
 * Example 1:
 *
 *
 * Input: [5,7]
 * Output: 4
 *
 *
 * Example 2:
 *
 *
 * Input: [0,1]
 * Output: 0
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
		let mut res = 0;
		for i in (0..31).rev() {
			if (1 << i) & n > 0 {
				if (1 << i) & m > 0 {
					res |= 1 << i;
				} else {
					break;
				}
			}
		}
		res
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_201() {
		assert_eq!(Solution::range_bitwise_and(5, 7), 4);
		assert_eq!(Solution::range_bitwise_and(0, 1), 0);
		assert_eq!(Solution::range_bitwise_and(0, 0), 0);
		assert_eq!(Solution::range_bitwise_and(1, 1), 1);
		assert_eq!(Solution::range_bitwise_and(6, 7), 6);
		assert_eq!(Solution::range_bitwise_and(1, 3), 0);
	}
}
