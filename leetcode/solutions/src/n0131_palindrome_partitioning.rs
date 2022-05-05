/**
 * [131] Palindrome Partitioning
 *
 * Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.
 * A palindrome string is a string that reads the same backward as forward.
 *  
 * Example 1:
 * Input: s = "aab"
 * Output: [["a","a","b"],["aa","b"]]
 * Example 2:
 * Input: s = "a"
 * Output: [["a"]]
 *  
 * Constraints:
 *
 *     1 <= s.length <= 16
 *     s contains only lowercase English letters.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn partition(s: String) -> Vec<Vec<String>> {
		fn is_palindrom(s: &[u8]) -> bool {
			s.iter()
				.rev()
				.zip(s.iter())
				.take(s.len() / 2)
				.all(|(c1, c2)| c1 == c2)
		}
		// dp?
		fn dfs<'a>(
			s: &'a [u8],
			start: usize,
			ans: &mut Vec<Vec<String>>,
			cur_stk: &mut Vec<&'a [u8]>,
		) {
			if start == s.len() {
				ans.push(cur_stk.iter().copied().map(to_string).collect());
				return;
			}
			for i in (start + 1)..=s.len() {
				if is_palindrom(&s[start..i]) {
					cur_stk.push(&s[start..i]);
					dfs(s, i, ans, cur_stk);
					cur_stk.pop();
				}
			}
		}

		fn to_string(s: &[u8]) -> String { unsafe { std::str::from_utf8_unchecked(s).to_owned() } }
		let mut ans = Vec::new();
		dfs(s.as_bytes(), 0, &mut ans, &mut Vec::new());
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_131() {
		assert_eq!(
			Solution::partition("aab".to_owned()),
			vec![vec_string!["a", "a", "b"], vec_string!["aa", "b"]]
		);
		assert_eq!(
			Solution::partition("a".to_owned()),
			vec![vec!["a".to_owned()]]
		);
	}
}
