/**
 * [171] Excel Sheet Column Number
 *
 * Given a string columnTitle that represents the column title as appears in an Excel sheet, return its corresponding column number.
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
 * Input: columnTitle = "A"
 * Output: 1
 *
 * <strong class="example">Example 2:
 *
 * Input: columnTitle = "AB"
 * Output: 28
 *
 * <strong class="example">Example 3:
 *
 * Input: columnTitle = "ZY"
 * Output: 701
 *
 *  
 * Constraints:
 *
 *     1 <= columnTitle.length <= 7
 *     columnTitle consists only of uppercase English letters.
 *     columnTitle is in the range ["A", "FXSHRXW"].
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn title_to_number(title: String) -> i32 {
		let mut ans = 0;
		for c in title.bytes() {
			ans *= 26;
			ans += (c - b'A') as i32 + 1;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_171() {
		assert_eq!(Solution::title_to_number("A".to_owned()), 1);
		assert_eq!(Solution::title_to_number("Z".to_owned()), 26);
		assert_eq!(Solution::title_to_number("AA".to_owned()), 27);
		assert_eq!(Solution::title_to_number("ZY".to_owned()), 701);
	}
}
