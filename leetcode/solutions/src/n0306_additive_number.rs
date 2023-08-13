/**
 * [306] Additive Number
 *
 * An additive number is a string whose digits can form an additive sequence.
 * A valid additive sequence should contain at least three numbers. Except for the first two numbers, each subsequent number in the sequence must be the sum of the preceding two.
 * Given a string containing only digits, return true if it is an additive number or false otherwise.
 * Note: Numbers in the additive sequence cannot have leading zeros, so sequence 1, 2, 03 or 1, 02, 3 is invalid.
 *  
 * <strong class="example">Example 1:
 *
 * Input: "112358"
 * Output: true
 * Explanation:
 * The digits can form an additive sequence: 1, 1, 2, 3, 5, 8.
 * 1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
 *
 * <strong class="example">Example 2:
 *
 * Input: "199100199"
 * Output: true
 * Explanation:
 * The additive sequence is: 1, 99, 100, 199.
 * 1 + 99 = 100, 99 + 100 = 199
 *
 *  
 * Constraints:
 *
 *     1 <= num.length <= 35
 *     num consists only of digits.
 *
 *  
 * Follow up: How would you handle overflow for very large input integers?
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn is_additive_number(num: String) -> bool {
		let mut m = 0;
		match num.as_bytes() {
			[b'0', s @ ..] => Self::find1(s, 0),
			s => (1..s.len() - 1).zip(s).take(s.len() / 2).any(|(i, &c)| {
				m = (m * 10) + (c - b'0') as i64;
				Self::find1(&s[i..], m)
			}),
		}
	}
	fn find1(buf: &[u8], prv: i64) -> bool {
		let mut n = 0;
		match buf {
			[b'0', s @ ..] => Self::find2(s, (prv, n)),
			s => (1..s.len()).zip(s).take(s.len() / 2).any(|(i, c)| {
				n = (n * 10) + (c - b'0') as i64;
				Self::find2(&s[i..], (prv, n))
			}),
		}
	}
	fn find2(mut buf: &[u8], mut cur: (i64, i64)) -> bool {
		let mut s = (cur.0 + cur.1).to_string();
		while buf.get(..s.len()) == Some(s.as_bytes()) {
			cur = (cur.1, cur.0 + cur.1);
			buf = &buf[s.len()..];
			s = (cur.0 + cur.1).to_string();
		}
		buf.is_empty()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_306() {
		assert!(Solution::find1(b"00", 0));
		assert!(Solution::is_additive_number(
			"121474836472147483648".to_owned()
		));
		assert!(!Solution::is_additive_number(
			"121474836472147483649".to_owned()
		));
		assert!(Solution::is_additive_number(
			"1999999999999999910000000000000000".to_owned()
		));
		assert!(Solution::is_additive_number("000".to_owned()));
		assert!(Solution::is_additive_number("112358".to_owned()));
		assert!(Solution::is_additive_number("101".to_owned()));
		assert!(!Solution::is_additive_number("1203".to_owned()));
		assert!(!Solution::is_additive_number("110".to_owned()));
		assert!(!Solution::is_additive_number("10".to_owned()));
		assert!(Solution::is_additive_number("11011".to_owned()));
		assert!(Solution::is_additive_number("199100199".to_owned()));
	}
}
