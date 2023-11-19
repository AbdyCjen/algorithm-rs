/**
 * [1759] Count Number of Homogenous Substrings
 *
 * Given a string s, return the number of homogenous substrings of s. Since the answer may be too large, return it modulo 10^9 + 7.
 *
 * A string is homogenous if all the characters of the string are the same.
 *
 * A substring is a contiguous sequence of characters within a string.
 *
 *  
 * <strong class="example">Example 1:
 *
 *
 * Input: s = "abbcccaa"
 * Output: 13
 * Explanation: The homogenous substrings are listed as below:
 * "a"   appears 3 times.
 * "aa"  appears 1 time.
 * "b"   appears 2 times.
 * "bb"  appears 1 time.
 * "c"   appears 3 times.
 * "cc"  appears 2 times.
 * "ccc" appears 1 time.
 * 3 + 1 + 2 + 1 + 3 + 2 + 1 = 13.
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: s = "xy"
 * Output: 2
 * Explanation: The homogenous substrings are "x" and "y".
 *
 * <strong class="example">Example 3:
 *
 *
 * Input: s = "zzzzz"
 * Output: 15
 *
 *
 *  
 * Constraints:
 *
 *
 *     1 <= s.length <= 10^5
 *     s consists of lowercase letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_homogenous(s: String) -> i32 {
		const MO: i32 = 1e9 as i32 + 7;
		let mut prv = (0, 0);
		s.bytes()
			.fold((0, 0, 1), |(a, prv, cnt), c| {
				if c == prv {
					((a + cnt) % MO, prv, cnt + 1)
				} else {
					((a + 1) % MO, c, 2)
				}
			})
			.0
	}
}

// submission codes end
