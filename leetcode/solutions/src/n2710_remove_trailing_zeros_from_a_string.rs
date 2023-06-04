/**
 * [2710] Remove Trailing Zeros From a String
 *
 * Given a positive integer num represented as a string, return the integer num without trailing zeros as a string.
 *  
 * <strong class="example">Example 1:
 *
 * Input: num = "51230100"
 * Output: "512301"
 * Explanation: Integer "51230100" has 2 trailing zeros, we remove them and return integer "512301".
 *
 * <strong class="example">Example 2:
 *
 * Input: num = "123"
 * Output: "123"
 * Explanation: Integer "123" has no trailing zeros, we return integer "123".
 *
 *  
 * Constraints:
 *
 *     1 <= num.length <= 1000
 *     num consists of only digits.
 *     num doesn't have any leading zeros.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn remove_trailing_zeros(mut num: String) -> String {
		let zl = num.bytes().rev().take_while(|c| *c == b'0').count();
		num.truncate(num.len() - zl);
		num
	}
}

// submission codes end
