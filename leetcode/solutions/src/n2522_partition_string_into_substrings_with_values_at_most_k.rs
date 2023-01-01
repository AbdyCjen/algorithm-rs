/**
 * [2522] Partition String Into Substrings With Values at Most K
 *
 * You are given a string s consisting of digits from 1 to 9 and an integer k.
 * A partition of a string s is called good if:
 *
 *     Each digit of s is part of exactly one substring.
 *     The value of each substring is less than or equal to k.
 *
 * Return the minimum number of substrings in a good partition of s. If no good partition of s exists, return -1.
 * Note that:
 *
 *     The value of a string is its result when interpreted as an integer. For example, the value of "123" is 123 and the value of "1" is 1.
 *     A substring is a contiguous sequence of characters within a string.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "165462", k = 60
 * Output: 4
 * Explanation: We can partition the string into substrings "16", "54", "6", and "2". Each substring has a value less than or equal to k = 60.
 * It can be shown that we cannot partition the string into less than 4 substrings.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "238182", k = 5
 * Output: -1
 * Explanation: There is no good partition for this string.
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^5
 *     s[i] is a digit from '1' to '9'.
 *     1 <= k <= 10^9
 *
 *  
 * <style type="text/css">.spoilerbutton {display:block; border:dashed; padding: 0px 0px; margin:10px 0px; font-size:150%; font-weight: bold; color:#000000; background-color:cyan; outline:0;
 * }
 * .spoiler {overflow:hidden;}
 * .spoiler > div {-webkit-transition: all 0s ease;-moz-transition: margin 0s ease;-o-transition: all 0s ease;transition: margin 0s ease;}
 * .spoilerbutton[value="Show Message"] + .spoiler > div {margin-top:-500%;}
 * .spoilerbutton[value="Hide Message"] + .spoiler {padding:5px;}
 * </style>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn minimum_partition(s: String, k: i32) -> i32 {
		let k = k.to_string();
		Self::solve(
			s.as_bytes(),
			k.as_bytes(),
			&mut vec![None; s.as_bytes().len() + 1],
		)
		.unwrap_or(-1)
	}

	fn solve(s: &[u8], k: &[u8], cache: &mut [Option<Option<i32>>]) -> Option<i32> {
		if s.is_empty() {
			return Some(0);
		}
		if let Some(a) = cache[s.len()] {
			return a;
		}

		let mut ans = None;
		for l in 1..=k.len().min(s.len()) {
			if l == k.len() && &s[..l] > k {
				break;
			}

			if let Some(la) = Self::solve(&s[l..], k, cache) {
				ans = Some(ans.unwrap_or(i32::MAX).min(la + 1));
			}
		}
		cache[s.len()] = Some(ans);
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2522() {}
}
