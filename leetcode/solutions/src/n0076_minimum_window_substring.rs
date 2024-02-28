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
 * If there is no such window in S that covers all characters in T, return the empty string "".
 * If there is such window, you are guaranteed that there will always be only one unique minimum window in S.
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_window(s: String, t: String) -> String {
		let mut cnts = [0; 128];
		for c in t.into_bytes() {
			cnts[c as usize] += 1;
		}
		let mut cnt = cnts.iter().filter(|c| **c > 0).count();
		let mut ans = None::<&'_ str>;
		let mut i = 0;
		for (j, c) in (0..).zip(s.bytes()) {
			cnts[c as usize] -= 1;
			cnt -= (cnts[c as usize] == 0) as usize;
			while cnt == 0 {
				if ans.map(str::len).unwrap_or(usize::MAX) > (j - i + 1) {
					ans = Some(&s[i..=j]);
				}
				cnts[s.as_bytes()[i] as usize] += 1;
				cnt += (cnts[s.as_bytes()[i] as usize] == 1) as usize;
				i += 1;
			}
		}
		ans.map(ToOwned::to_owned).unwrap_or_default()
	}
	pub fn min_window2(s: String, t: String) -> String {
		let mut cnts = [0; 52];
		let idx = |c: u8| -> usize {
			match c {
				b'a'..=b'z' => (c - b'a') as usize,
				_ => (c - b'A') as usize + 26,
			}
		};
		for c in t.into_bytes() {
			cnts[idx(c)] += 1;
		}
		let mut ans = None::<&'_ str>;
		let mut i = 0;
		for (j, c) in (0..).zip(s.bytes()) {
			cnts[idx(c)] -= 1;
			while cnts.iter().all(|c| *c <= 0) {
				if ans.map(str::len).unwrap_or(usize::MAX) > (j - i + 1) {
					ans = Some(&s[i..=j]);
				}
				cnts[idx(s.as_bytes()[i])] += 1;
				i += 1;
			}
		}
		ans.map(ToOwned::to_owned).unwrap_or_default()
	}
	pub fn min_window1(s: String, t: String) -> String {
		if s.is_empty() || s.len() < t.len() {
			return Default::default();
		}
		let (s, t) = (s.into_bytes(), t.into_bytes());
		let idx = |i| s[i] as usize;
		let mut cmap = [0; 128];
		let mut imap = [false; 128];
		for c in t.iter().map(|&c| c as usize) {
			cmap[c] += 1;
			imap[c] = true;
		}

		let (mut i, mut start, mut minl) = (0, 0, std::usize::MAX);
		for (j, &c) in s.iter().enumerate() {
			let c = c as usize;
			if imap[c] {
				cmap[c] -= 1;
			}
			while (!imap[idx(i)] || cmap[idx(i)] < 0) && i < j {
				if imap[idx(i)] {
					cmap[idx(i)] += 1;
				}
				i += 1;
			}
			if cmap.iter().all(|x| *x <= 0) && j - i < minl {
				start = i;
				minl = j - i + 1;
			}
		}
		if minl > s.len() {
			Default::default()
		} else {
			String::from_utf8(s[start..start + minl].to_owned()).unwrap()
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
