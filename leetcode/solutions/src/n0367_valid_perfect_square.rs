/**
 * [367] Valid Perfect Square
 *
 * Given a positive integer num, write a function which returns True if num is a perfect square else False.
 *
 * Note: Do not use any built-in library function such as sqrt.
 *
 * Example 1:
 *
 * <div>
 *
 * Input: <span id="example-input-1-1">16</span>
 * Output: <span id="example-output-1">true</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">14</span>
 * Output: <span id="example-output-2">false</span>
 *
 * </div>
 * </div>
 */
pub struct Solution {}

// submission codes start here

use std::cmp::Ordering;
#[allow(dead_code)]
impl Solution {
	pub fn is_perfect_square(num: i32) -> bool {
		match num.cmp(&0) {
			Ordering::Less => return false,
			Ordering::Equal => return true,
			_ => {
				for i in 1..std::i32::MAX {
					if num / i < i {
						return false;
					}
					if i * i == num {
						return true;
					}
				}
			}
		};
		false
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_367() {
		assert_eq!(Solution::is_perfect_square(16), true);
		assert_eq!(Solution::is_perfect_square(1), true);
		assert_eq!(Solution::is_perfect_square(0), true);
		assert_eq!(Solution::is_perfect_square(-1), false);
		assert_eq!(Solution::is_perfect_square(std::i32::MAX), false);
		assert_eq!(Solution::is_perfect_square(std::i32::MAX - 1), false);
	}
}
