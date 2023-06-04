/**
 * [2707] Extra Characters in a String
 *
 * You are given a 0-indexed string s and a dictionary of words dictionary. You have to break s into one or more non-overlapping substrings such that each substring is present in dictionary. There may be some extra characters in s which are not present in any of the substrings.
 * Return the minimum number of extra characters left over if you break up s optimally.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "leetscode", dictionary = ["leet","code","leetcode"]
 * Output: 1
 * Explanation: We can break s in two substrings: "leet" from index 0 to 3 and "code" from index 5 to 8. There is only 1 unused character (at index 4), so we return 1.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "sayhelloworld", dictionary = ["hello","world"]
 * Output: 3
 * Explanation: We can break s in two substrings: "hello" from index 3 to 7 and "world" from index 8 to 12. The characters at indices 0, 1, 2 are not used in any substring and thus are considered as extra characters. Hence, we return 3.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 50
 *     1 <= dictionary.length <= 50
 *     1 <= dictionary[i].length <= 50
 *     dictionary[i] and s consists of only lowercase English letters
 *     dictionary contains distinct words
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashSet;
impl Solution {
	pub fn min_extra_char(s: String, dict: Vec<String>) -> i32 {
		Self::solve(
			s.as_bytes(),
			&dict.into_iter().map(|s| s.into_bytes()).collect(),
			&mut vec![-1; s.len() + 1],
		)
	}
	fn solve(s: &[u8], dict: &HashSet<Vec<u8>>, cache: &mut [i32]) -> i32 {
		if cache[s.len()] >= 0 {
			return cache[s.len()];
		} else if s.is_empty() {
			return 0;
		}
		let mut ans = i32::MAX;
		for i in 0..s.len() {
			if dict.contains(&s[..=i]) {
				ans = ans.min(Self::solve(&s[i + 1..], dict, cache));
			} else {
				ans = ans.min(Self::solve(&s[i + 1..], dict, cache) + 1 + i as i32);
			}
		}
		cache[s.len()] = ans;
		ans
	}
}

// submission codes end
