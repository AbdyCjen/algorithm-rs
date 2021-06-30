/**
 * [1363] Largest Multiple of Three
 *
 * Given an integer array of digits, return the largest multiple of three that can be formed by concatenating some of the given digits in any order.
 * Since the answer may not fit in an integer data type, return the answer as a string.
 * If there is no answer return an empty string.
 *  
 * Example 1:
 *
 * Input: digits = [8,1,9]
 * Output: "981"
 *
 * Example 2:
 *
 * Input: digits = [8,6,7,1,0]
 * Output: "8760"
 *
 * Example 3:
 *
 * Input: digits = [1]
 * Output: ""
 *
 * Example 4:
 *
 * Input: digits = [0,0,0,0,0,0]
 * Output: "0"
 *
 *  
 * Constraints:
 *
 *     1 <= digits.length <= 10^4
 *     0 <= digits[i] <= 9
 *     The returning answer must not contain unnecessary leading zeros.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
		let mut cnts = [0; 10];
		for i in digits {
			cnts[i as usize] += 1;
		}
		let s = cnts
			.iter()
			.enumerate()
			.map(|(i, c)| i as i32 * c)
			.sum::<i32>();
		if let n @ 1..=2 = s % 3 {
			let (skip1, skip2) = (n as usize, n as usize * 2 % 3);
			if let Some(c) = cnts.iter_mut().skip(skip1).step_by(3).find(|c| **c > 0) {
				*c -= 1
			} else {
				for _ in 0..2 {
					if let Some(c) = cnts.iter_mut().skip(skip2).step_by(3).find(|c| **c > 0) {
						*c -= 1;
					} else {
						return "".to_owned();
					}
				}
			}
		}
		let mut ans = Vec::new();
		for (d, &cnt) in cnts.iter().enumerate().rev() {
			if d == 0 && ans.is_empty() && cnt > 0 {
				ans.push(b'0');
			} else {
				ans.resize(ans.len() + cnt as usize, b'0' + d as u8);
			}
		}

		unsafe { String::from_utf8_unchecked(ans) }
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1363() {
		assert_eq!(
			Solution::largest_multiple_of_three(vec![8, 1, 9]),
			"981".to_owned()
		);
		assert_eq!(
			Solution::largest_multiple_of_three(vec![8, 6, 7, 1, 0]),
			"8760".to_owned()
		);
		assert_eq!(
			Solution::largest_multiple_of_three(vec![1, 0]),
			"0".to_owned()
		);
		assert_eq!(Solution::largest_multiple_of_three(vec![1]), "".to_owned());
		assert_eq!(
			Solution::largest_multiple_of_three(vec![0, 0, 0]),
			"0".to_owned()
		);
	}
}
