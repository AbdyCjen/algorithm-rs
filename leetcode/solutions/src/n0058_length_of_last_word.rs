/**
 * [58] Length of Last Word
 *
 * Given a string s consisting of words and spaces, return the length of the last word in the string.
 * A word is a maximal <span data-keyword="substring-nonempty">substring</span> consisting of non-space characters only.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "Hello World"
 * Output: 5
 * Explanation: The last word is "World" with length 5.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "   fly me   to   the moon  "
 * Output: 4
 * Explanation: The last word is "moon" with length 4.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "luffy is still joyboy"
 * Output: 6
 * Explanation: The last word is "joyboy" with length 6.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^4
 *     s consists of only English letters and spaces ' '.
 *     There will be at least one word in s.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn length_of_last_word(s: String) -> i32 {
		let mut i = 0;
		let mut ans = 0;
		for (j, c) in (0..).zip(s.bytes()) {
			match c {
				b' ' => i = j + 1,
				_ => ans = j - i + 1,
			}
		}
		ans
	}
	pub fn length_of_last_word1(s: String) -> i32 {
		s.split(' ')
			.filter(|s| !s.is_empty())
			.last()
			.map(|s| s.len() as i32)
			.unwrap()
	}
}

// submission codes end
