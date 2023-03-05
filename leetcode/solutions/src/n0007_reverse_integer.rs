/**
 * [7] Reverse Integer
 *
 * Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.
 * Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
 *  
 * <strong class="example">Example 1:
 *
 * Input: x = 123
 * Output: 321
 *
 * <strong class="example">Example 2:
 *
 * Input: x = -123
 * Output: -321
 *
 * <strong class="example">Example 3:
 *
 * Input: x = 120
 * Output: 21
 *
 *  
 * Constraints:
 *
 *     -2^31 <= x <= 2^31 - 1
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn reverse(mut x: i32) -> i32 {
		let mut ans = 0_i32;
		while x != 0 {
			match ans.checked_mul(10).and_then(|i| i.checked_add(x % 10)) {
				None => return 0,
				Some(a) => ans = a,
			}
			x /= 10;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_7() {
		assert_eq!(Solution::reverse(120), 21);
		assert_eq!(Solution::reverse(-123), -321);
		assert_eq!(Solution::reverse(-23), -32);
	}
}
