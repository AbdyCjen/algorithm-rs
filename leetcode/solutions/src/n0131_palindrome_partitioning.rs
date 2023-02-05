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

impl Solution {
	// dp
	pub fn partition(s: String) -> Vec<Vec<String>> {
		fn is_palindrom(s: &[u8]) -> bool {
			s[..s.len() / 2]
				.iter()
				.eq(s[(s.len() + 1) / 2..].iter().rev())
		}
		fn dfs<'a>(s: &'a [u8], cache: &mut [Option<Vec<Vec<&'a [u8]>>>]) -> Vec<Vec<&'a [u8]>> {
			if let Some(ans) = &cache[s.len()] {
				return ans.clone();
			}
			let mut ans = vec![];
			for i in 1..s.len() {
				if is_palindrom(&s[..i]) {
					let mut sub_ans = dfs(&s[i..], cache);
					for v in &mut sub_ans {
						v.push(&s[..i]);
					}
					ans.extend(sub_ans);
				}
			}
			if is_palindrom(s) {
				ans.push(vec![s]);
			}
			cache[s.len()] = Some(ans.clone());
			ans
		}
		dfs(s.as_bytes(), &mut vec![None; s.len() + 1])
			.iter()
			.map(|v| {
				v.iter()
					.map(|s| std::str::from_utf8(s).unwrap().to_owned())
					.rev()
					.collect()
			})
			.collect()
	}
	// simple dfs
	pub fn partition_01(s: String) -> Vec<Vec<String>> {
		fn is_palindrom(s: &[u8]) -> bool {
			s[..s.len() / 2]
				.iter()
				.eq(s[(s.len() + 1) / 2..].iter().rev())
		}
		fn dfs<'a>(s: &'a [u8], ans: &mut Vec<Vec<String>>, st: &mut Vec<&'a [u8]>) {
			if s.is_empty() {
				ans.push(
					st.iter()
						.map(|s| std::str::from_utf8(s).unwrap().to_owned())
						.collect(),
				);
				return;
			}
			for i in 1..=s.len() {
				if is_palindrom(&s[..i]) {
					st.push(&s[..i]);
					dfs(&s[i..], ans, st);
					st.pop();
				}
			}
		}
		let mut ans = Vec::new();
		dfs(s.as_bytes(), &mut ans, &mut Vec::new());
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
