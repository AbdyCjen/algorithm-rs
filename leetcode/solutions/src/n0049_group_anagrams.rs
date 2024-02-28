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
 * All inputs will be in lowercase.
 * The order of your output does not matter.
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
		let mut m = std::collections::HashMap::new();
		for s in strs {
			let mut cnt: [u16; 26] = [0; 26];
			for c in s.bytes() {
				cnt[(c - b'a') as usize] += 1;
			}
			m.entry(cnt).or_insert_with(Vec::new).push(s);
		}
		m.into_values().collect()
	}
	pub fn group_anagrams1(strs: Vec<String>) -> Vec<Vec<String>> {
		let mut m = std::collections::HashMap::new();
		for s in strs.into_iter() {
			let mut k = s.clone().into_bytes();
			k.sort_unstable();
			m.entry(k).or_insert_with(Vec::new).push(s);
		}
		m.into_values().collect()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_49() {
		// lc输出不要求有序, 本地要修饰一下测试用例
		let mut res =
			Solution::group_anagrams(vec_string!["eat", "tea", "tan", "ate", "nat", "bat"]);
		res.sort_by_key(|a| a.len());
		assert_eq!(
			res,
			vec![
				vec_string!["bat"],
				vec_string!["tan", "nat"],
				vec_string!["eat", "tea", "ate"]
			]
		);
	}
}
