/**
 * [28] Implement strStr()
 *
 * Implement <a href="http://www.cplusplus.com/reference/cstring/strstr/" target="_blank">strStr()</a>.
 *
 * Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
 *
 * Example 1:
 *
 *
 * Input: haystack = "hello", needle = "ll"
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: haystack = "aaaaa", needle = "bba"
 * Output: -1
 *
 *
 * Clarification:
 *
 * What should we return when needle is an empty string? This is a great question to ask during an interview.
 *
 * For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's <a href="http://www.cplusplus.com/reference/cstring/strstr/" target="_blank">strstr()</a> and Java's <a href="https://docs.oracle.com/javase/7/docs/api/java/lang/String.html#indexOf(java.lang.String)" target="_blank">indexOf()</a>.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn str_str(haystack: String, needle: String) -> i32 {
		let kmp = Solution::gen_kmp(needle.as_bytes());

		match kmp(haystack.as_bytes()) {
			Some(i) => i as i32,
			None => -1,
		}
	}
	fn gen_kmp<T>(p: &[T]) -> impl Fn(&[T]) -> Option<usize>
	where
		T: std::clone::Clone + PartialEq,
	{
		let p: Vec<T> = Vec::from(p);
		let mut v: Vec<usize> = vec![0; p.len() + 1];
		let mut j: usize = 0;
		for i in 2..p.len() {
			while j > 0 && p[j] != p[i - 1] {
				j = v[j]
			}
			if p[j] == p[i - 1] {
				j += 1
			}
			v[i] = if p[j] == p[i] { v[j] } else { j }
		}

		move |s: &[T]| {
			if p.len() == 0 {
				return Some(0);
			}
			let mut j: usize = 0;
			for i in 0..s.len() {
				while j > 0 && p[j] != s[i] {
					j = v[j]
				}
				if p[j] == s[i] {
					j += 1
				}
				if j == p.len() {
					return Some(i - j + 1);
				}
			}
			None
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_28() {
		assert_eq!(Solution::str_str("aaaaa".to_owned(), "bba".to_owned()), -1);
		assert_eq!(Solution::str_str("hello".to_owned(), "ll".to_owned()), 2);
	}
}
