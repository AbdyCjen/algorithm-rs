/**
 * [166] Fraction to Recurring Decimal
 *
 * Given two integers representing the numerator and denominator of a fraction, return the fraction in string format.
 * If the fractional part is repeating, enclose the repeating part in parentheses.
 * If multiple answers are possible, return any of them.
 * It is guaranteed that the length of the answer string is less than 10^4 for all the given inputs.
 *  
 * Example 1:
 *
 * Input: numerator = 1, denominator = 2
 * Output: "0.5"
 *
 * Example 2:
 *
 * Input: numerator = 2, denominator = 1
 * Output: "2"
 *
 * Example 3:
 *
 * Input: numerator = 4, denominator = 333
 * Output: "0.(012)"
 *
 *  
 * Constraints:
 *
 *     -2^31 <= numerator, denominator <= 2^31 - 1
 *     denominator != 0
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn fraction_to_decimal(num: i32, de: i32) -> String {
		let mut s = String::new();
		if num.signum() * de.signum() < 0 {
			s.push('-');
		}
		let num = (num as i64).abs();
		let de = (de as i64).abs();
		s.push_str(&(num / de).to_string());
		let mut num = (num % de * 10).abs();
		let de = de.abs();
		if num > 0 {
			let mut vis = std::collections::BTreeMap::new();
			let mut dec = Vec::new();
			s.push('.');

			for i in 0.. {
				if num == 0 {
					s += std::str::from_utf8(&dec).unwrap();
					break;
				} else if let Some(&prv) = vis.get(&num) {
					// loop found
					s = format!(
						"{}{}({})",
						s,
						std::str::from_utf8(&dec[0..prv]).unwrap(),
						std::str::from_utf8(&dec[prv..]).unwrap()
					);
					break;
				} else {
					dec.push(b'0' + (num / de) as u8);
					vis.insert(num, i);
					num = (num % de) * 10;
				}
			}
		}
		s
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_166() {
		assert_eq!(&Solution::fraction_to_decimal(1, 2), "0.5");
		assert_eq!(&Solution::fraction_to_decimal(2, 1), "2");
		assert_eq!(&Solution::fraction_to_decimal(4, 333), "0.(012)");
		assert_eq!(&Solution::fraction_to_decimal(37, 66), "0.5(60)");
		assert_eq!(&Solution::fraction_to_decimal(37, -66), "-0.5(60)");
		assert_eq!(&Solution::fraction_to_decimal(-37, -66), "0.5(60)");
		assert_eq!(&Solution::fraction_to_decimal(-37, 66), "-0.5(60)");
		assert_eq!(
			&Solution::fraction_to_decimal(-2147483648, 1),
			"-2147483648"
		);
		assert_eq!(
			&Solution::fraction_to_decimal(-1, -2147483648),
			"0.0000000004656612873077392578125"
		);
	}
}
