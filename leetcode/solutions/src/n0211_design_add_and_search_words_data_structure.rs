/**
 * [211] Design Add and Search Words Data Structure
 *
 * Design a data structure that supports adding new words and finding if a string matches any previously added string.
 * Implement the WordDictionary class:
 *
 *     WordDictionary() Initializes the object.
 *     void addWord(word) Adds word to the data structure, it can be matched later.
 *     bool search(word) Returns true if there is any string in the data structure that matches word or false otherwise. word may contain dots '.' where dots can be matched with any letter.
 *
 *  
 * <strong class="example">Example:
 *
 * Input
 * ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
 * [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
 * Output
 * [null,null,null,null,false,true,true,true]
 * Explanation
 * WordDictionary wordDictionary = new WordDictionary();
 * wordDictionary.addWord("bad");
 * wordDictionary.addWord("dad");
 * wordDictionary.addWord("mad");
 * wordDictionary.search("pad"); // return False
 * wordDictionary.search("bad"); // return True
 * wordDictionary.search(".ad"); // return True
 * wordDictionary.search("b.."); // return True
 *
 *  
 * Constraints:
 *
 *     1 <= word.length <= 25
 *     word in addWord consists of lowercase English letters.
 *     word in search consist of '.' or lowercase English letters.
 *     There will be at most 3 dots in word for search queries.
 *     At most 10^4 calls will be made to addWord and search.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
struct WordDictionary {
	end: bool,
	next: Vec<Self>,
	c: u8,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
	fn new() -> Self {
		Self {
			end: false,
			next: Vec::new(),
			c: b'a',
		}
	}

	fn new_with(c: u8) -> Self {
		Self {
			end: false,
			next: Vec::new(),
			c,
		}
	}

	fn add_word(mut self: &mut Self, word: String) {
		for c in word.into_bytes() {
			self = match self.next.binary_search_by(|nc| nc.c.cmp(&c)) {
				Ok(i) => &mut self.next[i],
				Err(i) => {
					self.next.insert(i, Self::new_with(c));
					&mut self.next[i]
				}
			};
		}
		self.end = true;
	}

	fn search(&self, word: String) -> bool { self.search_inner(word.as_bytes()) }
	fn search_inner(mut self: &Self, word: &[u8]) -> bool {
		for (i, c) in word.iter().enumerate() {
			match c {
				b'.' => {
					let word = &word[i + 1..];
					return self.next.iter().any(|n| n.search_inner(word));
				}
				c => match self.next.binary_search_by(|nc| nc.c.cmp(c)) {
					Ok(j) => self = &self.next[j],
					_ => return false,
				},
			}
		}

		self.end
	}
}
