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
	#[inline]
    fn new(s: &[u8]) -> Self {
        Self {
            s: s.to_owned(),
            is_end: true,
            children: Vec::new(),
        }
    }

    fn find(&self, s: &[u8]) -> bool {
        if s.is_empty() {
            return self.is_end;
        }
        match self.children.binary_search_by(|ch| child_cmp(ch, s)) {
            Ok(i) => self.children[i].find(&s[self.children[i].s.len()..]),
            Err(_) => false,
        }
    }

    fn insert(&mut self, s: &[u8]) {
        match s {
            [] => self.is_end = true,
            [c, ..] => match self.children.binary_search_by(|ch| ch.s[0].cmp(c)) {
                Ok(i) => {
                    let ch = &mut self.children[i];
                    let pref = common_pref(&ch.s, s);
                    ch.split(pref);
                    ch.insert(&s[pref..]);
                }
                Err(i) => self.children.insert(i, TrieNode::new(s)),
            },
        }
    }

	#[inline]
    fn split(&mut self, i: usize) {
		if i == self.s.len() {
			return ;
		}
        let split_node = Self {
            is_end: self.is_end,
            s: self.s.split_off(i),
            children: std::mem::take(&mut self.children),
        };
        self.is_end = false;
        self.children.push(split_node);
    }
}

#[inline]
fn common_pref(s: &[u8], t: &[u8]) -> usize {
    s.iter().zip(t).take_while(|(c1, c2)| c1 == c2).count()
}

use std::cmp::Ordering;
#[inline]
fn child_cmp(ch: &TrieNode, s: &[u8]) -> Ordering {
    ch.s.as_slice().cmp(s.get(..ch.s.len()).unwrap_or(s))
}

impl Trie {
    pub fn find(&self, s: &[u8]) -> bool {
        match self.roots.binary_search_by(|ch| child_cmp(ch, s)) {
            Ok(i) => self.roots[i].find(&s[self.roots[i].s.len()..]),
            Err(_) if s.is_empty() => self.has_empty,
            _ => false,
        }
    }

    pub fn insert(&mut self, s: &[u8]) {
        if s.is_empty() {
            return self.has_empty = true;
        }
        match s {
            [c, ..] => match self.roots.binary_search_by(|ch| ch.s[0].cmp(c)) {
                Ok(i) => {
                    let ch = &mut self.roots[i];
                    let pref = common_pref(&ch.s, s);
                    ch.split(pref);
                    ch.insert(&s[pref..]);
                }
                Err(i) => self.roots.insert(i, TrieNode::new(s)),
            },
            [] => self.has_empty = true,
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
        let test_string = b"pxedkummnxux";
        let mut trie = Trie::default();
        for c in c_rng.clone() {
            trie.insert(&[c]);
        }

        for c in c_rng.clone() {
            assert!(trie.find(&[c]));
        }
        trie.insert(test_string);
        assert!(trie.find(test_string));
        assert!(!trie.find(&test_string[..test_string.len() - 1]));

        let mut rng = rand::thread_rng();

        let test_cases: Vec<_> = (0..2).map(|_| rand_ascii_str(&mut rng)).collect();

        for s in &test_cases {
            trie.insert(s);
        }

        for s in &test_cases {
            assert!(trie.find(s));
        }

        for c in c_rng {
            assert!(trie.find(&[c]));
        }
    }

    fn rand_ascii_str<R: Rng>(rng: &mut R) -> Vec<u8> {
        (0..rng.gen_range(4..16))
            .map(|_| rng.gen_range(b'a'..=b'z'))
            .collect()
    }
}
