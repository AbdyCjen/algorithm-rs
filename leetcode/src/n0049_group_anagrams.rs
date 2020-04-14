/**
 * [49] Group Anagrams
 *
 * Given an array of strings, group anagrams together.
 *
 * Example:
 *
 *
 * Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
 * Output:
 * [
 *   ["ate","eat","tea"],
 *   ["nat","tan"],
 *   ["bat"]
 * ]
 *
 * Note:
 *
 *
 * 	All inputs will be in lowercase.
 * 	The order of your output does not matter.
 *
 *
 */
#[allow(dead_code)]
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
	pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
		let mut m: HashMap<_, Vec<String>> = HashMap::new();
		for s in strs.into_iter() {
			let mut k = s.clone().into_bytes();
			k.sort();
			m.entry(k).or_insert(Vec::new()).push(s);
		}
		m.into_iter().map(|(k, v)| v).collect()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_49() {
		return;
		assert_eq!(
			Solution::group_anagrams(vec_string!["eat", "tea", "tan", "ate", "nat", "bat"]),
			vec![
				vec_string!["ate", "eat", "tea"],
				vec_string!["nat", "tan"],
				vec_string!["bat"]
			]
		);
	}
}
