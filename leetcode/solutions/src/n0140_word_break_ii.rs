/**
 * [140] Word Break II
 *
 * Given a string s and a dictionary of strings wordDict, add spaces in s to construct a sentence where each word is a valid dictionary word. Return all such possible sentences in any order.
 * Note that the same word in the dictionary may be reused multiple times in the segmentation.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
 * Output: ["cats and dog","cat sand dog"]
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "pineapplepenapple", wordDict = ["apple","pen","applepen","pine","pineapple"]
 * Output: ["pine apple pen apple","pineapple pen apple","pine applepen apple"]
 * Explanation: Note that you are allowed to reuse a dictionary word.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
 * Output: []
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 20
 *     1 <= wordDict.length <= 1000
 *     1 <= wordDict[i].length <= 10
 *     s and wordDict[i] consist of only lowercase English letters.
 *     All the strings of wordDict are unique.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
		let mut cache = vec![None; s.as_bytes().len()];
		Self::word_break_inner(s.as_bytes(), &word_dict, &mut cache);
		cache[0]
			.take()
			.unwrap()
			.into_iter()
			.map(|row| {
				row[1..].iter().fold(row[0].to_owned(), |mut s, w| {
					s.push(' ');
					s.push_str(w);
					s
				})
			})
			.collect()
	}

	pub fn word_break_inner<'a>(
		s: &[u8],
		dict: &'a [String],
		cache: &mut [Option<Vec<Vec<&'a String>>>],
	) {
		match cache {
			[cur @ None, left @ ..] => {
				let cur = cur.insert(vec![]);
				for w_s in dict {
					let w = w_s.as_bytes();
					if s.get(..w.len()) == Some(w) {
						if s.len() == w.len() {
							cur.push(vec![w_s]);
						} else {
							Self::word_break_inner(&s[w.len()..], dict, &mut left[w.len() - 1..]);
							for s in left[w.len() - 1].iter().flatten() {
								let mut ans = vec![w_s];
								ans.extend(s);
								cur.push(ans);
							}
						}
					}
				}
			}
			[Some(_), ..] => {}
			_ => unreachable!(),
		};
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_140() {
		assert_eq!(
			Solution::word_break("leetcode".to_owned(), vec_string!("leet", "code")),
			vec_string!("leet code")
		);
		assert_eq!(
			Solution::word_break(
				"catsanddog".to_owned(),
				vec_string!["cat", "cats", "and", "sand", "dog"]
			),
			vec_string!["cat sand dog", "cats and dog"]
		);
		assert_eq!(
			Solution::word_break(
				"pineapplepenapple".to_owned(),
				vec_string!["apple", "pen", "applepen", "pine", "pineapple"]
			),
			vec_string![
				"pine apple pen apple",
				"pine applepen apple",
				"pineapple pen apple",
			]
		);
		assert!(Solution::word_break(
			"catsandog".to_owned(),
			vec_string!["cats", "dog", "sand", "and", "cat"]
		)
		.is_empty());
	}
}
