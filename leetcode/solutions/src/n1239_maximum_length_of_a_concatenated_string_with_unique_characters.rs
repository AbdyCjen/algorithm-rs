/**
 * [1239] Maximum Length of a Concatenated String with Unique Characters
 *
 * Given an array of strings arr. String s is a concatenation of a sub-sequence of arr which have unique characters.
 * Return the maximum possible length of s.
 *  
 * Example 1:
 *
 * Input: arr = ["un","iq","ue"]
 * Output: 4
 * Explanation: All possible concatenations are "","un","iq","ue","uniq" and "ique".
 * Maximum length is 4.
 *
 * Example 2:
 *
 * Input: arr = ["cha","r","act","ers"]
 * Output: 6
 * Explanation: Possible solutions are "chaers" and "acters".
 *
 * Example 3:
 *
 * Input: arr = ["abcdefghijklmnopqrstuvwxyz"]
 * Output: 26
 *
 *  
 * Constraints:
 *
 *     1 <= arr.length <= 16
 *     1 <= arr[i].length <= 26
 *     arr[i] contains only lower case English letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn max_length(arr: Vec<String>) -> i32 {
		use std::collections::BTreeSet;
		let mut dp = vec![BTreeSet::new()];
		for s in arr {
			let set = s.bytes().collect::<BTreeSet<_>>();
			if set.len() < s.as_bytes().len() {
				continue;
			}
			for mut s in dp.clone() {
				if s.is_disjoint(&set) {
					s.extend(set.iter().copied());
					dp.push(s);
				}
			}
		}
		dp.into_iter().map(|s| s.len()).max().unwrap() as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1239() {
		assert_eq!(Solution::max_length(vec_string!["un", "iq", "ue"]), 4);
		assert_eq!(
			Solution::max_length(vec_string![
				"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"
			]),
			16
		);
		assert_eq!(
			Solution::max_length(vec_string!["cha", "r", "act", "ers"]),
			6
		);
		assert_eq!(
			Solution::max_length(vec_string!["abcdefghijklmnopqrstuvwxyz"]),
			26
		);
	}
}
