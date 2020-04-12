/**
 * [76] Minimum Window Substring
 *
 * Given a string S and a string T, find the minimum window in S which will contain all the characters in T in complexity O(n).
 *
 * Example:
 *
 *
 * Input: S = "ADOBECODEBANC", T = "ABC"
 * Output: "BANC"
 *
 *
 * Note:
 *
 *
 * 	If there is no such window in S that covers all characters in T, return the empty string "".
 * 	If there is such window, you are guaranteed that there will always be only one unique minimum window in S.
 *
 *
 */
pub struct Solution {}

// submission codes start here

// 太丑了, 我看见一个题解是维护了一个counter变量保存t里的待匹配字符数量, 等于零就可以匹配了;
impl Solution {
	pub fn min_window(s: String, t: String) -> String {
		if s.len() == 0 || s.len() < t.len() {
			return Default::default();
		}
		let (mut res, mut cmap, mut counter) = (String::new(), vec![0; 128], t.len() as i32);
		for c in t.bytes().map(|c| c as usize) {
			cmap[c] += 1;
		}

		let (mut i, mut start, mut minl) = (0, 0, std::usize::MAX);
		for (j, c) in s.bytes().enumerate() {
			let c = c as usize;
			if cmap[c] > 0 {
				counter -= 1;
			}
			cmap[c] = cmap[c] - 1;
			while counter == 0 {
				if j - i + 1 < minl {
					start = i;
					minl = j - i + 1;
				}
				if cmap[s.as_bytes()[i] as usize] == 0 {
					counter += 1;
				}
				cmap[s.as_bytes()[i] as usize] += 1;
				i += 1;
			}
		}
		if minl > s.len() {
			Default::default()
		} else {
			s[start..start + minl].to_owned()
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_76() {
		assert_eq!(
			Solution::min_window("ADOBECODEBANCQGHJMNL".to_owned(), "ABC".to_owned()),
			"BANC".to_owned()
		);
	}
}
