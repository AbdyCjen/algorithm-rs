/**
 * [782] Jewels and Stones
 *
 * You're given strings J representing the types of stones that are jewels, and S representing the stones you have.  Each character in S is a type of stone you have.  You want to know how many of the stones you have are also jewels.
 *
 * The letters in J are guaranteed distinct, and all characters in J and S are letters. Letters are case sensitive, so "a" is considered a different type of stone from "A".
 *
 * Example 1:
 *
 *
 * Input: J = "aA", S = "aAAbbbb"
 * Output: 3
 *
 *
 * Example 2:
 *
 *
 * Input: J = "z", S = "ZZ"
 * Output: 0
 *
 *
 * Note:
 *
 *
 * S and J will consist of letters and have length at most 50.
 * The characters in J are distinct.
 *
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashSet;
#[allow(dead_code)]
impl Solution {
	pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
		let (j, s) = (j.into_bytes(), s.into_bytes());
		let jewel_set: HashSet<_> = j.into_iter().collect();
		s.into_iter().filter(|c| jewel_set.contains(c)).count() as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_782() {
		assert_eq!(
			Solution::num_jewels_in_stones("aA".to_owned(), "aAAbbbb".to_owned()),
			3
		);
		assert_eq!(
			Solution::num_jewels_in_stones("z".to_owned(), "ZZ".to_owned()),
			0
		);
	}
}
