/**
 * [2930] Number of Strings Which Can Be Rearranged to Contain Substring
 *
 * You are given an integer n.
 * A string s is called good if it contains only lowercase English characters and it is possible to rearrange the characters of s such that the new string contains "leet" as a substring.
 * For example:
 *
 *     The string "lteer" is good because we can rearrange it to form "leetr" .
 *     "letl" is not good because we cannot rearrange it to contain "leet" as a substring.
 *
 * Return the total number of good strings of length n.
 * Since the answer may be large, return it modulo 10^9 + 7.
 * A substring is a contiguous sequence of characters within a string.
 * <div class="notranslate" style="all: initial;"> </div>
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 4
 * Output: 12
 * Explanation: The 12 strings which can be rearranged to have "leet" as a substring are: "eelt", "eetl", "elet", "elte", "etel", "etle", "leet", "lete", "ltee", "teel", "tele", and "tlee".
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 10
 * Output: 83943898
 * Explanation: The number of strings with length 10 which can be rearranged to have "leet" as a substring is 526083947580. Hence the answer is 526083947580 % (10^9 + 7) = 83943898.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn string_count(n: i32) -> i32 {
		if n < 4 {
			return 0;
		}
		const MO: i64 = 1e9 as i64 + 7;
		let mut ans = [1, 0, 0, 0, 0, 0, 0, 0]; // none, one, two dupe, two, 3 uniq, ee*, ans
		for _ in 0..n {
			let [a, b, c, d, e, f, g, h] = ans;
			ans = [
				a * 23,                 // none
				b * 23 + a,             // l e t
				c * 24 + b,             // ee or eeee
				d * 24 + b * 2 + c,     // {l|t}e
				e * 25 + b * 2 + c * 2, //lt
				f * 25 + d * 2 + e,     // let
				g * 25 + c + d,         // {l|t}ee
				h * 26 + g * 2 + f,     // leet
			];
			ans.iter_mut().for_each(|a| *a %= MO);
		}
		ans[7] as i32
	}
}

// submission codes end
