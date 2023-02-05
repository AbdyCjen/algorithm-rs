/**
 * [472] Concatenated Words
 *
 * Given an array of strings words (without duplicates), return all the concatenated words in the given list of words.
 * A concatenated word is defined as a string that is comprised entirely of at least two shorter words in the given array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: words = ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]
 * Output: ["catsdogcats","dogcatsdog","ratcatdogcat"]
 * Explanation: "catsdogcats" can be concatenated by "cats", "dog" and "cats";
 * "dogcatsdog" can be concatenated by "dog", "cats" and "dog";
 * "ratcatdogcat" can be concatenated by "rat", "cat", "dog" and "cat".
 * <strong class="example">Example 2:
 *
 * Input: words = ["cat","dog","catdog"]
 * Output: ["catdog"]
 *
 *  
 * Constraints:
 *
 *     1 <= words.length <= 10^4
 *     1 <= words[i].length <= 30
 *     words[i] consists of only lowercase English letters.
 *     All the strings of words are unique.
 *     1 <= sum(words[i].length) <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

#[derive(Default)]
struct Trie<'a> {
	children: std::collections::BTreeMap<u8, Self>,
	end: Option<&'a str>,
}

impl<'a> Trie<'a> {
	fn new(words: &'a [String]) -> Self {
		let mut trie = Self::default();
		for w in words {
			let mut cur = &mut trie;
			for c in w.bytes() {
				cur = cur.children.entry(c).or_insert_with(Default::default);
			}
			cur.end = Some(w);
		}
		trie
	}

	fn words(&self) -> Vec<&Self> {
		let mut st = vec![self];
		let mut ans = vec![];
		while !st.is_empty() {
			for tr in std::mem::take(&mut st) {
				if tr.end.is_some() && !tr.children.is_empty() {
					ans.push(tr);
				}
				st.extend(tr.children.values());
			}
		}
		ans
	}
}

impl Solution {
	pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
		let trie = Trie::new(&words);
		let mut st = trie
			.words()
			.into_iter()
			.map(|w| (w, &trie))
			.collect::<Vec<_>>();

		let mut ans = std::collections::HashSet::new();
		while let Some((ws, p)) = st.pop() {
			if let (Some(w), Some(_)) = (ws.end, p.end) {
				ans.insert(w);
			}
			if p.end.is_some() {
				st.push((ws, &trie));
			}

			for (c, wss) in &ws.children {
				if let Some(pp) = p.children.get(c) {
					st.push((wss, pp));
				}
			}
		}

		ans.into_iter().map(ToOwned::to_owned).collect()
	}
}

// submission codes end

/*#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_472() {
		assert_eq!(
			Solution::find_all_concatenated_words_in_a_dict(vec_string![
				"cat",
				"cats",
				"catsdogcats",
				"dog",
				"dogcatsdog",
				"hippopotamuses",
				"rat",
				"ratcatdogcat"
			]),
			vec_string!["catsdogcats", "ratcatdogcat", "dogcatsdog"]
		);
	}
}*/
