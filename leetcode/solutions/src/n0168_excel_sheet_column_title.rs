/**
 * [168] Excel Sheet Column Title
 *
 * Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.
 * For example:
 *
 * A -> 1
 * B -> 2
 * C -> 3
 * ...
 * Z -> 26
 * AA -> 27
 * AB -> 28
 * ...
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: columnNumber = 1
 * Output: "A"
 *
 * <strong class="example">Example 2:
 *
 * Input: columnNumber = 28
 * Output: "AB"
 *
 * <strong class="example">Example 3:
 *
 * Input: columnNumber = 701
 * Output: "ZY"
 *
 *  
 * Constraints:
 *
 *     1 <= columnNumber <= 2^31 - 1
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn convert_to_title(mut n: i32) -> String {
		let mut ans = vec![];
		while n > 0 {
			n -= 1;
			let c = (n % 26) as u8;
			ans.push(b'A' + c);
			n /= 26;
		}
		ans.reverse();
		String::from_utf8(ans).unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_168() {
		assert_eq!(Solution::convert_to_title(1), "A");
		assert_eq!(Solution::convert_to_title(28), "AB");
		assert_eq!(Solution::convert_to_title(701), "ZY");
		assert_eq!(Solution::convert_to_title(26), "Z");
	}
}
