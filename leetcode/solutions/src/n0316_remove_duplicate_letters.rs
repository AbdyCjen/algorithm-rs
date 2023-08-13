/**
 * [316] Remove Duplicate Letters
 *
 * Given a string s, remove duplicate letters so that every letter appears once and only once. You must make sure your result is <span data-keyword="lexicographically-smaller-string">the smallest in lexicographical order</span> among all possible results.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "bcabc"
 * Output: "abc"
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "cbacdcbc"
 * Output: "acdb"
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 10^4
 *     s consists of lowercase English letters.
 *
 *  
 * Note: This question is the same as 1081: <a href="https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/" target="_blank">https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/</a>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	//XXX: more expressive
	pub fn remove_duplicate_letters(s: String) -> String {
		let mut cnt = 0_i32;
		let mut occur = vec![vec![]; 26];
		for (i, c) in s.bytes().enumerate().rev() {
			occur[(c - b'a') as usize].push((i, cnt));
			cnt |= 1 << (c - b'a');
		}
		let mut ans = String::new();
		let mut bound = 0_usize;
		for _ in 0..cnt.count_ones() {
			for c in b'a'..=b'z' {
				if cnt & (1 << (c - b'a')) > 0 {
					let aft = cnt - (1 << (c - b'a'));
					let idx = occur[(c - b'a') as usize].partition_point(|v| v.0 >= bound) - 1;
					let blk = occur[(c - b'a') as usize][idx];
					if blk.1 & aft == aft {
						bound = blk.0 + 1;
						ans.push(c as char);
						cnt = aft;
						break;
					}
				}
			}
		}
		ans
	}
}

// submission codes end
#[cfg(test)]
mod tests {
	#[test]
	fn test_316() {
		use super::*;
		assert_eq!(
			Solution::remove_duplicate_letters("bcabc".to_owned()),
			"abc"
		);
		assert_eq!(
			Solution::remove_duplicate_letters("cbacdcbc".to_owned()),
			"acdb"
		);
		assert_eq!(
			Solution::remove_duplicate_letters("abacb".to_owned()),
			"abc"
		);
	}
}
