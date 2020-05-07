/**
 * [387] First Unique Character in a String
 *
 *
 * Given a string, find the first non-repeating character in it and return it's index. If it doesn't exist, return -1.
 *
 * Examples:
 *
 * s = "leetcode"
 * return 0.
 *
 * s = "loveleetcode",
 * return 2.
 *
 *
 *
 *
 * Note: You may assume the string contain only lowercase letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn first_uniq_char(s: String) -> i32 {
		let s = s.into_bytes();
		let mut cnt = [0; 26];
		for c in &s {
			cnt[(c - b'a') as usize] += 1;
		}
		s.iter()
			.enumerate()
			.find(|(_, &c)| cnt[(c - b'a') as usize] == 1)
			.map(|(i, _)| i as i32)
			.unwrap_or(-1)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_387() {
		assert_eq!(Solution::first_uniq_char("leetcode".to_owned()), 0);
		assert_eq!(Solution::first_uniq_char("loveleetcode".to_owned()), 2);
	}
}
