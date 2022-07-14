struct TrieNode {
	// XXX: smallvec maybe?
	s: Vec<u8>,
	is_end: bool,
	children: Vec<Self>,
}

#[derive(Default)]
pub struct Trie {
	roots: Vec<TrieNode>,
	has_empty: bool,
}

impl TrieNode {
	fn new(s: &[u8]) -> Self {
		Self {
			s: s.to_owned(),
			is_end: true,
			children: Vec::new(),
		}
	}

	fn find(&self, mut s: &[u8]) -> bool {
		// XXX: better way?
		let comm_pref = self.s.iter().zip(s).take_while(|(c1, c2)| c1 == c2).count();
		if comm_pref < self.s.len() {
			return false;
		}
		s = &s[comm_pref..];
		let c = match s.first() {
			Some(c) => c,
			None => return self.is_end,
		};

		match self.children.binary_search_by(|ch| ch.s[0].cmp(c)) {
			Ok(i) => self.children[i].find(s),
			Err(_) => false,
		}
	}

	fn insert(&mut self, mut s: &[u8]) {
		let comm_pref = self.s.iter().zip(s).take_while(|(c1, c2)| c1 == c2).count();
		if comm_pref < self.s.len() {
			self.split(comm_pref);
		}
		s = &s[comm_pref..];
		let c = match s.first() {
			Some(c) => c,
			None => {
				self.is_end = true;
				return;
			}
		};
		match self.children.binary_search_by(|ch| ch.s[0].cmp(c)) {
			Ok(i) => self.children[i].insert(s),
			Err(i) => self.children.insert(i, Self::new(s)),
		}
	}

	fn split(&mut self, i: usize) {
		let split_node = Self {
			is_end: self.is_end,
			s: self.s.split_off(i),
			children: std::mem::take(&mut self.children),
		};
		self.is_end = false;
		self.children.push(split_node);
	}
}

impl Trie {
	pub fn find(&self, s: &str) -> bool {
		if let [c, ..] = s.as_bytes() {
			match self.roots.binary_search_by(|ch| ch.s[0].cmp(c)) {
				Ok(i) => self.roots[i].find(s.as_bytes()),
				_ => false,
			}
		} else {
			self.has_empty
		}
	}

	pub fn insert(&mut self, s: &str) {
		if let [c, ..] = s.as_bytes() {
			match self.roots.binary_search_by(|ch| ch.s[0].cmp(c)) {
				Ok(i) => self.roots[i].insert(s.as_bytes()),
				Err(i) => self.roots.insert(i, TrieNode::new(s.as_bytes())),
			}
		} else {
			self.has_empty = true;
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use rand::Rng;
	#[test]
	fn test_compact_trie() {
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

		let test_cases: Vec<_> = (0..rng.gen_range(10_000..20_000))
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
