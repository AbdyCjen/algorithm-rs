/**
 * [6] Zigzag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 * string convert(string s, int numRows);
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "A", numRows = 1
 * Output: "A"
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 1000
 *     s consists of English letters (lower-case and upper-case), ',' and '.'.
 *     1 <= numRows <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn convert(s: String, num_rows: i32) -> String {
		if num_rows == 1 {
			return s;
		}
		let mut rows = vec!["".to_owned(); num_rows as usize];
		let it = (0..num_rows - 1).chain((1..num_rows).rev()).cycle();
		for (c, i) in s.chars().zip(it) {
			rows[i as usize].push(c);
		}
		rows.into_iter().collect()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_6() {
		assert_eq!(
			Solution::convert("PAYPALISHIRING".to_owned(), 3),
			"PAHNAPLSIIGYIR".to_owned()
		);
		assert_eq!(
			Solution::convert("PAYPALISHIRING".to_owned(), 1),
			"PAYPALISHIRING".to_owned()
		);
	}
}
