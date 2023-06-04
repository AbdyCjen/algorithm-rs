/**
 * [2696] Minimum String Length After Removing Substrings
 *
 * You are given a string s consisting only of uppercase English letters.
 * You can apply some operations to this string where, in one operation, you can remove any occurrence of one of the substrings "AB" or "CD" from s.
 * Return the minimum possible length of the resulting string that you can obtain.
 * Note that the string concatenates after removing the substring and could produce new "AB" or "CD" substrings.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "ABFCACDB"
 * Output: 2
 * Explanation: We can do the following operations:
 * - Remove the substring "<u>AB</u>FCACDB", so s = "FCACDB".
 * - Remove the substring "FCA<u>CD</u>B", so s = "FCAB".
 * - Remove the substring "FC<u>AB</u>", so s = "FC".
 * So the resulting length of the string is 2.
 * It can be shown that it is the minimum length that we can obtain.
 * <strong class="example">Example 2:
 *
 * Input: s = "ACBBD"
 * Output: 5
 * Explanation: We cannot do any operations on the string so the length remains the same.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 100
 *     s consists only of uppercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_length(s: String) -> i32 {
		let mut ans = vec![];
		for c in s.bytes() {
			match (ans.last(), c) {
				(Some(b'A'), b'B') | (Some(b'C'), b'D') => {
					ans.pop();
				}
				_ => ans.push(c),
			}
		}
		ans.len() as i32
	}
}

// submission codes end
