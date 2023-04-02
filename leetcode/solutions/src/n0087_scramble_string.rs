/**
 * [87] Scramble String
 *
 * Given a string s1, we may represent it as a binary tree by partitioning it to two non-empty substrings recursively.
 *
 * Below is one possible representation of s1 = "great":
 *
 *
 *     great
 *    /    \
 *   gr    eat
 *  / \    /  \
 * g   r  e   at
 *            / \
 *           a   t
 *
 *
 * To scramble the string, we may choose any non-leaf node and swap its two children.
 *
 * For example, if we choose the node "gr" and swap its two children, it produces a scrambled string "rgeat".
 *
 *
 *     rgeat
 *    /    \
 *   rg    eat
 *  / \    /  \
 * r   g  e   at
 *            / \
 *           a   t
 *
 *
 * We say that "rgeat" is a scrambled string of "great".
 *
 * Similarly, if we continue to swap the children of nodes "eat" and "at", it produces a scrambled string "rgtae".
 *
 *
 *     rgtae
 *    /    \
 *   rg    tae
 *  / \    /  \
 * r   g  ta  e
 *        / \
 *       t   a
 *
 *
 * We say that "rgtae" is a scrambled string of "great".
 *
 * Given two strings s1 and s2 of the same length, determine if s2 is a scrambled string of s1.
 *
 * Example 1:
 *
 *
 * Input: s1 = "great", s2 = "rgeat"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s1 = "abcde", s2 = "caebd"
 * Output: false
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
	pub fn is_scramble(s1: String, s2: String) -> bool {
		Self::solve(s1.as_bytes(), s2.as_bytes(), &mut Default::default())
	}
	fn solve<'a>(
		s1: &'a [u8],
		s2: &'a [u8],
		memo: &mut HashMap<(&'a [u8], &'a [u8]), bool>,
	) -> bool {
		if s1 == s2 {
			return true;
		} else if let Some(&val) = memo.get(&(s1, s2)) {
			return val;
		}

		//滚动判断子串是否有相同的字母集合, 必要不充分, 仅剪枝
		let mut acc_test1 = {
			let mut s = (0, 0);
			move |a: u8, b: u8| -> bool {
				let (a, b) = (a as i32, b as i32);
				s.0 += a - b;
				s.1 += a * a - b * b;
				s == (0, 0)
			}
		};
		// copy counter
		let mut acc_test2 = acc_test1;

		let n = s1.len();
		for i in 1..n {
			if acc_test1(s1[i - 1], s2[n - i])
				&& Self::solve(&s1[..i], &s2[n - i..], memo)
				&& Self::solve(&s1[i..], &s2[..n - i], memo)
				|| acc_test2(s1[i - 1], s2[i - 1])
					&& Self::solve(&s1[..i], &s2[..i], memo)
					&& Self::solve(&s1[i..], &s2[i..], memo)
			{
				memo.insert((s1, s2), true);
				return true;
			}
		}

		memo.insert((s1, s2), false);
		false
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_87() {
		assert!(Solution::is_scramble(
			"great".to_owned(),
			"rgeat".to_owned()
		));
		assert!(!Solution::is_scramble(
			"abcde".to_owned(),
			"caebd".to_owned()
		));
		assert!(Solution::is_scramble(
			"great".to_owned(),
			"eatgr".to_owned()
		));
		assert!(Solution::is_scramble(
			"great".to_owned(),
			"rgtae".to_owned()
		));
	}
}
