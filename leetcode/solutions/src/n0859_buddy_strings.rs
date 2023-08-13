/**
 * [859] Buddy Strings
 *
 * Given two strings s and goal, return true if you can swap two letters in s so the result is equal to goal, otherwise, return false.
 * Swapping letters is defined as taking two indices i and j (0-indexed) such that i != j and swapping the characters at s[i] and s[j].
 *
 *     For example, swapping at indices 0 and 2 in "abcd" results in "cbad".
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "ab", goal = "ba"
 * Output: true
 * Explanation: You can swap s[0] = 'a' and s[1] = 'b' to get "ba", which is equal to goal.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "ab", goal = "ab"
 * Output: false
 * Explanation: The only letters you can swap are s[0] = 'a' and s[1] = 'b', which results in "ba" != goal.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "aa", goal = "aa"
 * Output: true
 * Explanation: You can swap s[0] = 'a' and s[1] = 'a' to get "aa", which is equal to goal.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length, goal.length <= 2 * 10^4
 *     s and goal consist of lowercase letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn buddy_strings(s: String, goal: String) -> bool {
		if s.len() != goal.len() {
			return false;
		}
		let mut prv = None;
		let mut diff = 0;
		let mut cnt = [0; 26];
		for (c1, c2) in s.bytes().zip(goal.bytes()) {
			cnt[(c1 - b'a') as usize] += 1;
			if c1 != c2 {
				match prv {
					Some(prv) => {
						if prv != (c2, c1) {
							return false;
						}
					}
					_ => prv = Some((c1, c2)),
				}
				diff += 1;
			}
		}
		diff == 2 || (diff == 0 && cnt.iter().any(|i| *i > 1))
	}
}

// submission codes end
