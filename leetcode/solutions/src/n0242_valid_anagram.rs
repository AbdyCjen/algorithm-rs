/**
 * [242] Valid Anagram
 *
 * Given two strings s and t, return true if t is an anagram of s, and false otherwise.
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 *  
 * <strong class="example">Example 1:
 * Input: s = "anagram", t = "nagaram"
 * Output: true
 * <strong class="example">Example 2:
 * Input: s = "rat", t = "car"
 * Output: false
 *  
 * Constraints:
 *
 *     1 <= s.length, t.length <= 5 * 10^4
 *     s and t consist of lowercase English letters.
 *
 *  
 * Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn is_anagram(s: String, t: String) -> bool {
		let count = |s: &str| {
			let mut cnt = [0; 26];
			for c in s.bytes() {
				cnt[(c - b'a') as usize] += 1;
			}
			cnt
		};
		count(s.as_str()) == count(t.as_str())
	}
}

// submission codes end
