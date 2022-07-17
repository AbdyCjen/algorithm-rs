/**
 * [165] Compare Version Numbers
 *
 * Given two version numbers, version1 and version2, compare them.
 *
 *
 * Version numbers consist of one or more revisions joined by a dot '.'. Each revision consists of digits and may contain leading zeros. Every revision contains at least one character. Revisions are 0-indexed from left to right, with the leftmost revision being revision 0, the next revision being revision 1, and so on. For example 2.5.33 and 0.1 are valid version numbers.
 * To compare version numbers, compare their revisions in left-to-right order. Revisions are compared using their integer value ignoring any leading zeros. This means that revisions 1 and 001 are considered equal. If a version number does not specify a revision at an index, then treat the revision as 0. For example, version 1.0 is less than version 1.1 because their revision 0s are the same, but their revision 1s are 0 and 1 respectively, and 0 < 1.
 * Return the following:
 *
 *     If version1 < version2, return -1.
 *     If version1 > version2, return 1.
 *     Otherwise, return 0.
 *
 *  
 * Example 1:
 *
 * Input: version1 = "1.01", version2 = "1.001"
 * Output: 0
 * Explanation: Ignoring leading zeroes, both "01" and "001" represent the same integer "1".
 *
 * Example 2:
 *
 * Input: version1 = "1.0", version2 = "1.0.0"
 * Output: 0
 * Explanation: version1 does not specify revision 2, which means it is treated as "0".
 *
 * Example 3:
 *
 * Input: version1 = "0.1", version2 = "1.1"
 * Output: -1
 * Explanation: version1's revision 0 is "0", while version2's revision 0 is "1". 0 < 1, so version1 < version2.
 *
 *  
 * Constraints:
 *
 *     1 <= version1.length, version2.length <= 500
 *     version1 and version2 only contain digits and '.'.
 *     version1 and version2 are valid version numbers.
 *     All the given revisions in version1 and version2 can be stored in a 32-bit integer.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn compare_version(v1: String, v2: String) -> i32 {
		let mut sp1 = v1.split('.');
		let mut sp2 = v2.split('.');
		loop {
			match (sp1.next(), sp2.next()) {
				(None, None) => break 0,
				(Some(s1), Some(s2)) => {
					let i1 = s1.parse::<i32>().unwrap();
					let i2 = s2.parse::<i32>().unwrap();
					use std::cmp::Ordering;
					match i1.cmp(&i2) {
						Ordering::Less => break -1,
						Ordering::Greater => break 1,
						_ => {}
					}
				}
				(Some(s1), _) => {
					if s1.parse::<i32>().unwrap() != 0 {
						break 1;
					}
				}
				(_, Some(s2)) => {
					if s2.parse::<i32>().unwrap() != 0 {
						break -1;
					}
				}
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_165() {
		assert_eq!(
			Solution::compare_version("1.011".to_owned(), "1.0011".to_owned()),
			0
		);
		assert_eq!(
			Solution::compare_version("1.0".to_owned(), "1.0.0".to_owned()),
			0
		);
		assert_eq!(
			Solution::compare_version("0.1".to_owned(), "1.1".to_owned()),
			-1
		);
		assert_eq!(
			Solution::compare_version("1.0".to_owned(), "1.0.1".to_owned()),
			-1
		);
		assert_eq!(
			Solution::compare_version("1.0.1".to_owned(), "1".to_owned()),
			1
		);
		assert_eq!(
			Solution::compare_version("1.01".to_owned(), "1.10".to_owned()),
			-1
		);
	}
}
