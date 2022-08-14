use super::{BTree, BTreeInnerNode, BTreeLeafNode};

pub struct Iter<'a, T> {
	st: Vec<(&'a BTreeInnerNode<T>, usize)>,
	leaves_iter: std::slice::Iter<'a, T>,
}

impl<'a, T> std::iter::Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		if let Some(n) = self.leaves_iter.next() {
			return Some(n);
		}

		loop {
			let (inner_node, it) = self.st.last_mut()?;
			match inner_node.keys.get(*it) {
				to_ret @ Some(_) => {
					*it += 1;
					match &inner_node.children {
						super::Children::Leaf(leaves) => {
							self.leaves_iter = leaves[*it].keys.iter();
						}
						super::Children::Inner(inners) => {
							let mut inner = inners[*it].as_ref();
							loop {
								self.st.push((inner, 0));
								match &inner.children {
									super::Children::Inner(inner_ch) => {
										inner = inner_ch[0].as_ref();
										self.st.push((inner, 0));
									}
									super::Children::Leaf(leaf_ch) => {
										self.leaves_iter = leaf_ch[0].keys.iter();
										break;
									}
								}
							}
						}
					}
					break to_ret;
				}
				None => {
					self.st.pop();
				}
			}
		}
	}
}

impl<'a, T> IntoIterator for &'a BTree<T> {
	type IntoIter = Iter<'a, T>;
	type Item = <Iter<'a, T> as Iterator>::Item;
	fn into_iter(self) -> Self::IntoIter {
		match self {
			BTree::Leaf(leaf) => Iter {
				st: Vec::new(),
				leaves_iter: leaf.keys.iter(),
			},
			BTree::Inner(ref inner) => {
				let mut st = vec![(inner, 0)];
				let mut inner = inner;
				loop {
					match &inner.children {
						super::Children::Inner(inner_ch) => {
							inner = inner_ch[0].as_ref();
							st.push((inner, 0));
						}
						super::Children::Leaf(leaf_ch) => {
							break Iter {
								st,
								leaves_iter: leaf_ch[0].keys.iter(),
							}
						}
					}
				}
			}
		}
	}
}

impl<T: Ord> FromIterator<T> for BTree<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut tr = Self::default();
		for v in iter {
			tr.insert(v)
		}
		tr
	}
}
#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn iter_tests() {
		let v: Vec<_> = (0..100).collect();
		let tr: BTree<_> = v.iter().copied().collect();
		let new_v: Vec<_> = (&tr).into_iter().copied().collect();
		assert_eq!(v, new_v);
	}
}
