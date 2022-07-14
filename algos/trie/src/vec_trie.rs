struct TrieNode {
	c: u8,
	is_end: bool,
	children: Vec<TrieNode>,
}

#[derive(Default)]
pub struct Trie {
	roots: Vec<TrieNode>,
	has_empty: bool,
}

impl TrieNode {
	fn insert(&mut self, s: &[u8]) {
		let mut cur = self;
		for &c in s {
			cur = match cur.children.binary_search_by(|ch| ch.c.cmp(&c)) {
				Err(i) => {
					cur.children.insert(i, Self::new(c));
					&mut cur.children[i]
				}
				Ok(i) => &mut cur.children[i],
			}
		}
		cur.is_end = true;
	}

	#[inline]
	fn new(c: u8) -> Self {
		Self {
			is_end: false,
			c,
			children: Vec::new(),
		}
	}

	fn find(&self, s: &[u8]) -> bool {
		let mut cur = self;
		for &c in s {
			match cur.children.binary_search_by(|ch| ch.c.cmp(&c)) {
				Err(_) => return false,
				Ok(i) => cur = &cur.children[i],
			}
		}
		cur.is_end
	}
}

impl Trie {
	pub fn insert(&mut self, s: &str) {
		if let [c, ..] = s.as_bytes() {
			let cur = match self.roots.binary_search_by(|ch| ch.c.cmp(c)) {
				Err(i) => {
					self.roots.insert(i, TrieNode::new(*c));
					&mut self.roots[i]
				}
				Ok(i) => &mut self.roots[i],
			};
			cur.insert(&s.as_bytes()[1..])
		} else {
			self.has_empty = true;
		}
	}

	pub fn find(&self, s: &str) -> bool {
		if let [c, ..] = s.as_bytes() {
			match self.roots.binary_search_by(|ch| ch.c.cmp(c)) {
				Ok(i) => self.roots[i].find(&s.as_bytes()[1..]),
				Err(_) => false,
			}
		} else {
			self.has_empty
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use rand::Rng;
	#[test]
	fn it_works() {
		let c_rng = b'a'..b'm';
		let test_string = "pxedkummnxux";
		let mut trie = Trie::default();
		for c in c_rng.clone() {
			trie.insert(std::str::from_utf8(&[c]).unwrap());
		}

		for c in c_rng.clone() {
			assert!(trie.find(std::str::from_utf8(&[c]).unwrap()));
		}
		trie.insert(test_string);
		assert!(trie.find(test_string));
		assert!(!trie.find(&test_string[..test_string.len() - 1]));

		let mut rng = rand::thread_rng();

		let test_cases: Vec<_> = (0..rng.gen_range(100..200))
			.map(|_| rand_ascii_str(&mut rng))
			.collect();

		for s in &test_cases {
			dbg!(&s);
			trie.insert(s);
		}
		dbg!(test_cases.len());

		for s in &test_cases {
			assert!(trie.find(s));
		}

		for c in c_rng {
			assert!(trie.find(std::str::from_utf8(&[c]).unwrap()));
		}
	}

	fn rand_ascii_str<R: rand::Rng>(rng: &mut R) -> String {
		let mut s = Vec::new();
		for _ in 0..rng.gen_range(4..16) {
			s.push(rng.gen_range(b'a'..=b'z'));
		}
		String::from_utf8(s).unwrap()
	}
}
