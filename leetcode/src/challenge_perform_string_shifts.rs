#![allow(dead_code)]
pub struct Solution {}

// submission codes start here
use std::iter::FromIterator;
impl Solution {
	pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
		let sum = shift.into_iter().fold(0_i32, |d, ds| match ds[0] {
			0 => (d - ds[1]) % (s.len() as i32),
			1 => (d + ds[1]) % (s.len() as i32),
			_ => unreachable!(),
		});
		// UGLY
		let shift = if sum > 0 {
			sum as usize % s.len()
		} else {
			(s.len() as i32 + sum) as usize % s.len()
		};
		let (l, r) = s.split_at(s.len() - shift);
		String::from_iter(vec![r, l])
	}
}
// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_perform_string_shifts() {
		assert_eq!(
			Solution::string_shift("abc".to_owned(), vec![vec![0, 1], vec![1, 2]]),
			"cab".to_owned()
		);
		assert_eq!(
			Solution::string_shift(
				"abcdefg".to_owned(),
				vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]]
			),
			"efgabcd".to_owned()
		);
	}
}
