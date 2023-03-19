/**
 * [208] Implement Trie (Prefix Tree)
 *
 * A <a href="https://en.wikipedia.org/wiki/Trie" target="_blank">trie</a> (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.
 * Implement the Trie class:
 *
 *     Trie() Initializes the trie object.
 *     void insert(String word) Inserts the string word into the trie.
 *     boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
 *     boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input
 * ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
 * [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
 * Output
 * [null, null, true, false, true, null, true]
 * Explanation
 * Trie trie = new Trie();
 * trie.insert("apple");
 * trie.search("apple");   // return True
 * trie.search("app");     // return False
 * trie.startsWith("app"); // return True
 * trie.insert("app");
 * trie.search("app");     // return True
 *
 *  
 * Constraints:
 *
 *     1 <= word.length, prefix.length <= 2000
 *     word and prefix consist only of lowercase English letters.
 *     At most 3 * 10^4 calls in total will be made to insert, search, and startsWith.
 *
 */

struct Trie {
	children: Vec<Self>,
	c: u8,
	end: bool,
}

impl Trie {
	fn new() -> Self {
		Self {
			children: Vec::new(),
			end: false,
			c: 0,
		}
	}

	fn new_c(c: u8) -> Self {
		Self {
			children: Vec::new(),
			end: false,
			c,
		}
	}

	fn insert(&mut self, word: String) {
		let mut cur = self;
		for c in word.bytes() {
			match cur.children.binary_search_by(|child| child.c.cmp(&c)) {
				Ok(i) => cur = &mut cur.children[i],
				Err(i) => {
					cur.children.insert(i, Self::new_c(c));
					cur = &mut cur.children[i];
				}
			}
		}
		cur.end = true;
	}

	fn search(&self, word: String) -> bool {
		let mut cur = self;
		for c in word.bytes() {
			match cur.children.binary_search_by(|child| child.c.cmp(&c)) {
				Err(_) => return false,
				Ok(i) => cur = &cur.children[i],
			}
		}
		cur.end
	}

	fn starts_with(&self, prefix: String) -> bool {
		let mut cur = self;
		for c in prefix.bytes() {
			match cur.children.binary_search_by(|child| child.c.cmp(&c)) {
				Err(_) => return false,
				Ok(i) => cur = &cur.children[i],
			}
		}
		true
	}
}
