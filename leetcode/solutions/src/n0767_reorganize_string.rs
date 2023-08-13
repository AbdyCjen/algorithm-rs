/**
 * [767] Reorganize String
 *
 * Given a string s, rearrange the characters of s so that any two adjacent characters are not the same.
 * Return any possible rearrangement of s or return "" if not possible.
 *  
 * <strong class="example">Example 1:
 * Input: s = "aab"
 * Output: "aba"
 * <strong class="example">Example 2:
 * Input: s = "aaab"
 * Output: ""
 *  
 * Constraints:
 *
 *     1 <= s.length <= 500
 *     s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn reorganize_string(s: String) -> String {
		let mut cnt = [0; 26];
		for c in s.bytes() {
			cnt[(c - b'a') as usize] += 1;
		}
		let mut bh: std::collections::BinaryHeap<_> = (0..)
			.zip(cnt)
			.map(|(a, b)| (b, a))
			.filter(|v| v.0 > 0)
			.collect();
		let mut prv = (0, 0);
		let mut ans = String::new();
		while let Some((cnt, i)) = bh.pop() {
			ans.push((b'a' + i) as char);
			if prv.0 > 0 {
				bh.push(prv);
			}
			prv = (cnt - 1, i);
		}
		if ans.len() < s.len() {
			"".to_owned()
		} else {
			ans
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_767() {
		assert_eq!(Solution::reorganize_string("aab".to_owned()), "aba");
	}
}
