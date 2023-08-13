/**
 * [139] Word Break
 *
 * Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.
 * Note that the same word in the dictionary may be reused multiple times in the segmentation.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "leetcode", wordDict = ["leet","code"]
 * Output: true
 * Explanation: Return true because "leetcode" can be segmented as "leet code".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "applepenapple", wordDict = ["apple","pen"]
 * Output: true
 * Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
 * Note that you are allowed to reuse a dictionary word.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
 * Output: false
 *
 *  
 * Constraints:
 *
 *     1 <= s.length <= 300
 *     1 <= wordDict.length <= 1000
 *     1 <= wordDict[i].length <= 20
 *     s and wordDict[i] consist of only lowercase English letters.
 *     All the strings of wordDict are unique.
 *
 */
pub struct Solution {}

struct Trie {
	end: bool,
	child: Vec<Self>,
	c: u8,
}
impl Trie {
	fn new(c: u8) -> Self {
		Self {
			end: false,
			child: Vec::new(),
			c,
		}
	}
	fn insert(&mut self, s: &str) {
		let mut cur = self;
		for c in s.bytes() {
			cur = match cur.child.binary_search_by_key(&c, |t| t.c) {
				Ok(i) => &mut cur.child[i],
				Err(i) => {
					cur.child.insert(i, Self::new(c));
					&mut cur.child[i]
				}
			};
		}
		cur.end = true;
	}
	fn trav(&self, mut s: &[u8], cache: &mut [Option<bool>]) -> bool {
		let len = s.len();
		if let Some(ans) = cache[len] {
			return ans;
		}
		let mut cur = self;
		while let [c, rest @ ..] = s {
			if cur.end && self.trav(s, cache) {
				return true;
			}
			cur = match cur.child.binary_search_by_key(c, |t| t.c) {
				Ok(i) => &cur.child[i],
				_ => {
					cache[len] = Some(false);
					return false;
				}
			};
			s = rest;
		}
		cache[len] = Some(cur.end);
		cur.end
	}
}
// submission codes start here

impl Solution {
	pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
		let mut cache = vec![None; s.as_bytes().len()];
		cache.push(Some(true));
		Self::word_break_inner(s.as_bytes(), &word_dict, 0, &mut cache)
	}

	fn word_break_inner(s: &[u8], dict: &[String], pos: usize, cache: &mut [Option<bool>]) -> bool {
		if let Some(ans) = cache[pos] {
			return ans;
		}
		for word in dict {
			let word = word.as_bytes();
			if s.len() >= word.len() && &s[..word.len()] == word {
				let tmp = Self::word_break_inner(&s[word.len()..], dict, pos + word.len(), cache);
				if *cache[pos + word.len()].insert(tmp) {
					return true;
				}
			}
		}
		false
	}
	pub fn word_break1(s: String, word_dict: Vec<String>) -> bool {
		let mut trie = Trie::new(0);
		for w in word_dict {
			trie.insert(&w);
		}
		trie.trav(s.as_bytes(), &mut vec![None; s.len() + 1])
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_139() {
		assert!(Solution::word_break(
			"leetcode".to_owned(),
			vec_string!["leet", "code"]
		));
		assert!(Solution::word_break(
			"applepenapple".to_owned(),
			vec_string!["apple", "pen"]
		));
		assert!(!Solution::word_break(
			"catsandog".to_owned(),
			vec_string!["cats", "dog", "sand", "and", "cat"]
		));
		assert!(Solution::word_break(
			"catsanddog".to_owned(),
			vec_string!["cats", "dog", "sand", "and", "cat"]
		));
		assert!(Solution::word_break(
			"catanddog".to_owned(),
			vec_string!["cats", "dog", "sand", "and", "cat"]
		));
		assert!(!Solution::word_break(
			"a".repeat(150) + "b",
			(1..=10).map(|i| "a".repeat(i)).collect::<Vec<_>>()
		));
	}
}
