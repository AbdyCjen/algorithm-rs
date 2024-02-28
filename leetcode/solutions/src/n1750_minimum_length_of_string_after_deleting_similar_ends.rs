/**
 * [1750] Minimum Length of String After Deleting Similar Ends
 *
 * Given a string s consisting only of characters 'a', 'b', and 'c'. You are asked to apply the following algorithm on the string any number of times:
 * <ol>
 *     Pick a non-empty prefix from the string s where all the characters in the prefix are equal.
 *     Pick a non-empty suffix from the string s where all the characters in this suffix are equal.
 *     The prefix and the suffix should not intersect at any index.
 *     The characters from the prefix and suffix must be the same.
 *     Delete both the prefix and the suffix.
 * </ol>
 * Return the minimum length of s after performing the above operation any number of times (possibly zero times).
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "ca"
 * Output: 2
 * Explanation: You can't remove any characters, so the string stays as is.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "cabaabac"
 * Output: 0
 * Explanation: An optimal sequence of operations is:
 * - Take prefix = "c" and suffix = "c" and remove them, s = "abaaba".
 * - Take prefix = "a" and suffix = "a" and remove them, s = "baab".
 * - Take prefix = "b" and suffix = "b" and remove them, s = "aa".
 * - Take prefix = "a" and suffix = "a" and remove them, s = "".
 * <strong class="example">Example 3:
 *
 * Input: s = "aabccabba"
 * Output: 3
 * Explanation: An optimal sequence of operations is:
 * - Take prefix = "aa" and suffix = "a" and remove them, s = "bccabb".
 * - Take prefix = "b" and suffix = "bb" and remove them, s = "cca".
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s only consists of characters 'a', 'b', and 'c'.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn minimum_length(s: String) -> i32 {
		let mut s = s.as_bytes();
		let mut prv = 0;
		while !s.is_empty() {
			match s {
				[a, rest @ .., b] if a == b => {
					prv = *a;
					s = rest;
				}
				[a, rest @ ..] | [rest @ .., a] if *a == prv => s = rest,
				_ => break,
			}
		}
		s.len() as i32
	}
}

// submission codes end
