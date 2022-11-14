/**
 * [1323] Maximum 69 Number
 *
 * You are given a positive integer num consisting only of digits 6 and 9.
 * Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).
 *  
 * <strong class="example">Example 1:
 *
 * Input: num = 9669
 * Output: 9969
 * Explanation:
 * Changing the first digit results in 6669.
 * Changing the second digit results in 9969.
 * Changing the third digit results in 9699.
 * Changing the fourth digit results in 9666.
 * The maximum number is 9969.
 *
 * <strong class="example">Example 2:
 *
 * Input: num = 9996
 * Output: 9999
 * Explanation: Changing the last digit 6 to 9 results in the maximum number.
 *
 * <strong class="example">Example 3:
 *
 * Input: num = 9999
 * Output: 9999
 * Explanation: It is better not to apply any change.
 *
 *  
 * Constraints:
 *
 *     1 <= num <= 10^4
 *     num consists of only 6 and 9 digits.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn maximum69_number(mut num: i32) -> i32 {
		let mut base = 6000;
		while base > 0 {
			if num >= base && num % base < base / 2 {
				num += base / 2;
				break;
			}
			base /= 10;
		}
		num
	}

	pub fn maximum69_number_02(mut num: i32) -> i32 {
		let mut arr = [0; 4];
		for f in &mut arr {
			*f = num % 10;
			num /= 10;
		}
		if let Some(f) = arr.iter_mut().rev().find(|i| **i == 6) {
			*f = 9;
		}
		let mut ans = 0;
		for &f in arr.iter().rev() {
			ans *= 10;
			ans += f;
		}
		ans
	}
	pub fn maximum69_number_01(mut num: i32) -> i32 {
		let mut ans = 0;
		let mut base = 1e4 as i32;
		let mut set = false;
		while base > 0 {
			if num / base > 0 {
				if num / base == 6 && !set {
					set = true;
					ans = ans * 10 + 9;
				} else {
					ans = ans * 10 + num / base;
				}
			}
			num %= base;
			base /= 10;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1323() {
		assert_eq!(Solution::maximum69_number(9999), 9999);
		assert_eq!(Solution::maximum69_number(9996), 9999);
		assert_eq!(Solution::maximum69_number(9669), 9969);
		assert_eq!(Solution::maximum69_number(69), 99);
		assert_eq!(Solution::maximum69_number(6), 9);
	}
}
