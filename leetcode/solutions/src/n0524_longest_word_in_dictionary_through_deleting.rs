/**
 * [524] Longest Word in Dictionary through Deleting
 *
 *
 * Given a string and a string dictionary, find the longest string in the dictionary that can be formed by deleting some characters of the given string. If there are more than one possible results, return the longest word with the smallest lexicographical order. If there is no possible result, return the empty string.
 *
 * Example 1:<br>
 *
 * Input:
 * s = "abpcplea", d = ["ale","apple","monkey","plea"]
 *
 * Output:
 * "apple"
 *
 *
 *
 *
 * Example 2:<br>
 *
 * Input:
 * s = "abpcplea", d = ["a","b","c"]
 *
 * Output:
 * "a"
 *
 *
 *
 * Note:<br>
 * <ol>
 * All the strings in the input will only contain lower-case letters.
 * The size of the dictionary won't exceed 1,000.
 * The length of all the strings in the input won't exceed 1,000.
 * </ol>
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn find_longest_word(s: String, mut d: Vec<String>) -> String {
		d.sort_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(&b)));

		for p in d.into_iter() {
			if sub_seq(&s, &p) {
				return p;
			}
		}

		return "".to_owned();

		fn sub_seq(s: &str, p: &str) -> bool {
			if p.is_empty() {
				return true;
			} else if s.is_empty() {
				return false;
			}

			if s.chars().next() == p.chars().next() {
				sub_seq(&s[1..], &p[1..])
			} else {
				sub_seq(&s[1..], p)
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_524() {
		assert_eq!(
			Solution::find_longest_word(
				"abpcplea".to_owned(),
				vec_string!["ale", "apple", "monkey", "plea"]
			),
			"apple"
		);
		assert_eq!(
			Solution::find_longest_word("abpcplea".to_owned(), vec_string!["a", "b", "c"]),
			"a"
		);
		assert_eq!(
			Solution::find_longest_word("d".to_owned(), vec_string!["a", "b", "c"]),
			""
		);
	}
}
