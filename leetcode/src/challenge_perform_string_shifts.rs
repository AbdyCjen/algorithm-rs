#![allow(dead_code)]
pub struct Solution {}

// submission codes start here
use std::iter::FromIterator;
impl Solution {
	pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
		let shift = shift.into_iter().fold(0_i32, |d, ds| match ds.as_slice() {
			&[0, ds] => (d - ds).rem_euclid(s.len() as i32),
			&[1, ds] => (d + ds).rem_euclid(s.len() as i32),
			_ => unreachable!(),
		});
		let (l, r) = s.split_at(s.len() - shift as usize);
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
