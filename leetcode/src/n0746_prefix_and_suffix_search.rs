/**
 * [746] Prefix and Suffix Search
 *
 * Design a special dictionary with some words that searchs the words in it by a prefix and a suffix.
 * Implement the WordFilter class:
 *
 *     WordFilter(string[] words) Initializes the object with the words in the dictionary.
 *     f(string prefix, string suffix) Returns the index of the word in the dictionary, which has the prefix prefix and the suffix suffix. If there is more than one valid index, return the largest of them. If there is no such word in the dictionary, return -1.
 *
 *  
 * Example 1:
 *
 * Input
 * ["WordFilter", "f"]
 * [[["apple"]], ["a", "e"]]
 * Output
 * [null, 0]
 * Explanation
 * WordFilter wordFilter = new WordFilter(["apple"]);
 * wordFilter.f("a", "e"); // return 0, because the word at index 0 has prefix = "a" and suffix = 'e".
 *
 *  
 * Constraints:
 *
 *     1 <= words.length <= 15000
 *     1 <= words[i].length <= 10
 *     1 <= prefix.length, suffix.length <= 10
 *     words[i], prefix and suffix consist of lower-case English letters only.
 *     At most 15000 calls will be made to the function f.
 *
 */
// submission codes start here
use std::collections::*;
struct WordFilter {
	prefixs: BTreeMap<Box<[u8]>, i32>,
	suffixs: BTreeMap<Box<[u8]>, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl WordFilter {
	fn new(words: Vec<String>) -> Self {
		Self {
			prefixs: words
				.iter()
				.enumerate()
				.map(|(i, k)| (k.clone().into_bytes().into_boxed_slice(), i as i32))
				.collect(),
			suffixs: words
				.into_iter()
				.enumerate()
				.map(|(i, k)| {
					let mut k = k.into_bytes().into_boxed_slice();
					k.reverse();
					(k, i as i32)
				})
				.collect(),
		}
	}

	fn f(&self, prefix: String, suffix: String) -> i32 {
		fn find_pref(prefs: &BTreeMap<Box<[u8]>, i32>, pref: Vec<u8>) -> BTreeSet<i32> {
			if pref.is_empty() {
				let mut bset = BTreeSet::new();
				bset.insert(prefs.len() as i32);
				return bset;
			}

			let pref = pref.into_boxed_slice();
			let mut pref_plusone = pref.clone();
			*pref_plusone.last_mut().unwrap() += 1;

			prefs.range(pref..pref_plusone).map(|(_, &j)| j).collect()
		}

		let (prefix, mut suffix) = (prefix.into_bytes(), suffix.into_bytes());
		suffix.reverse();

		if prefix.is_empty() {
			find_pref(&self.suffixs, suffix).into_iter().last()
		} else if suffix.is_empty() {
			find_pref(&self.prefixs, prefix).into_iter().last()
		} else {
			let suffs = find_pref(&self.suffixs, suffix);
			find_pref(&self.prefixs, prefix)
				.intersection(&suffs)
				.last()
				.copied()
		}
		.unwrap_or(-1)
	}
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_746() {
		assert_eq!(
			WordFilter::new(vec_string!["apple"]).f("a".to_owned(), "e".to_owned()),
			0
		);
		assert_eq!(
			WordFilter::new(vec_string!["apple", "applete"]).f("a".to_owned(), "e".to_owned()),
			1
		);
		assert_eq!(
			WordFilter::new(vec_string!["apple", "applete"]).f("a".to_owned(), "te".to_owned()),
			1
		);
	}
}
