/**
 * [2468] Split Message Based on Limit
 *
 * You are given a string, message, and a positive integer, limit.
 * You must split message into one or more parts based on limit. Each resulting part should have the suffix "<a/b>", where "b" is to be replaced with the total number of parts and "a" is to be replaced with the index of the part, starting from 1 and going up to b. Additionally, the length of each resulting part (including its suffix) should be equal to limit, except for the last part whose length can be at most limit.
 * The resulting parts should be formed such that when their suffixes are removed and they are all concatenated in order, they should be equal to message. Also, the result should contain as few parts as possible.
 * Return the parts message would be split into as an array of strings. If it is impossible to split message as required, return an empty array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: message = "this is really a very awesome message", limit = 9
 * Output: ["thi<1/14>","s i<2/14>","s r<3/14>","eal<4/14>","ly <5/14>","a v<6/14>","ery<7/14>"," aw<8/14>","eso<9/14>","me<10/14>"," m<11/14>","es<12/14>","sa<13/14>","ge<14/14>"]
 * Explanation:
 * The first 9 parts take 3 characters each from the beginning of message.
 * The next 5 parts take 2 characters each to finish splitting message.
 * In this example, each part, including the last, has length 9.
 * It can be shown it is not possible to split message into less than 14 parts.
 *
 * <strong class="example">Example 2:
 *
 * Input: message = "short message", limit = 15
 * Output: ["short mess<1/2>","age<2/2>"]
 * Explanation:
 * Under the given constraints, the string can be split into two parts:
 * - The first part comprises of the first 10 characters, and has a length 15.
 * - The next part comprises of the last 3 characters, and has a length 8.
 *
 *  
 * Constraints:
 *
 *     1 <= message.length <= 10^4
 *     message consists only of lowercase English letters and ' '.
 *     1 <= limit <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn split_message(s: String, lim: i32) -> Vec<String> {
		let mut s = &s[..];
		let (mut l, mut r) = (1, s.len() as i32);
		while l < r {
			let m = (l + r) / 2;
			let m_s = m.to_string();
			let total = s.len() as i32 + Self::digit(m) + (3 + m_s.len() as i32) * m;

			let should_be = (total / lim) + (total % lim > 0) as i32;
			if should_be > m {
				l = m + 1;
			} else {
				r = m;
			}
		}
		let (pl, ps) = (l, l.to_string());
		let mut ans = Vec::new();
		if ps.len() * 2 + 3 >= lim as usize {
			return ans;
		}
		for i in 1..pl {
			let i_s = i.to_string();
			let remain = lim as usize - (3 + ps.len() + i_s.len());
			let (cur, rest) = s.split_at(remain);
			ans.push(format!("{}<{}/{}>", cur, i_s, ps));
			s = rest;
		}
		ans.push(format!("{}<{}/{}>", s, ps, ps));
		ans
	}

	fn digit(n: i32) -> i32 {
		let mut b = 1;
		let mut ans = 0;
		while n >= b {
			ans += n - b + 1;
			b *= 10;
		}
		ans
	}
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2468() {
		//"this is really a very awesome message", limit = 9
		assert_eq!(
			Solution::split_message("this is really a very awesome message".to_owned(), 9),
			vec_string![
				"thi<1/14>",
				"s i<2/14>",
				"s r<3/14>",
				"eal<4/14>",
				"ly <5/14>",
				"a v<6/14>",
				"ery<7/14>",
				" aw<8/14>",
				"eso<9/14>",
				"me<10/14>",
				" m<11/14>",
				"es<12/14>",
				"sa<13/14>",
				"ge<14/14>"
			]
		);

		assert!(Solution::split_message("boxan".to_owned(), 5).is_empty());
		assert_eq!(
			Solution::split_message("short message".to_owned(), 15),
			vec_string!["short mess<1/2>", "age<2/2>"]
		);
	}
}
