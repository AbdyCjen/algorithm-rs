/**
 * [2609] Find the Longest Balanced Substring of a Binary String
 *
 * You are given a binary string s consisting only of zeroes and ones.
 * A substring of s is considered balanced if all zeroes are before ones and the number of zeroes is equal to the number of ones inside the substring. Notice that the empty substring is considered a balanced substring.
 * Return the length of the longest balanced substring of s.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "01000111"
 * Output: 6
 * Explanation: The longest balanced substring is "000111", which has length 6.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "00111"
 * Output: 4
 * Explanation: The longest balanced substring is "0011", which has length 4.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "111"
 * Output: 0
 * Explanation: There is no balanced substring except the empty substring, so the answer is 0.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 50
 *     '0' <= s[i] <= '1'
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_the_longest_balanced_substring(s: String) -> i32 {
		let mut s = s.into_bytes();
		s.push(0);
		s.into_iter()
			.fold((0, 0, 0, 0), |(ans, cur, cnt, pcnt), c| {
				if c == cur {
					(ans, cur, cnt + 1, pcnt)
				} else if cur == 1 {
					(ans.max(cnt.min(pcnt) * 2), c, 1, cnt)
				} else {
					(ans, c, 1, cnt)
				}
			})
			.0
	}
}

// submission codes end
