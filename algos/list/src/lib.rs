#[derive(Debug, Default)]
pub struct LinkList<T> {
	head: Option<Box<ListNode<T>>>,
}

#[derive(Debug)]
pub struct ListNode<T> {
	value: T,
	next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
	fn new(v: T) -> ListNode<T> {
		ListNode {
			value: v,
			next: None,
		}
	}
}

impl<T> LinkList<T> {
	// TODO: 有safe的办法直接在头保存&mut 尾节点吗? ==> 原则上不行, 下回写个用RefCell的版本吧
	pub fn iter(&self) -> Iter<T> { self.into_iter() }
	pub fn iter_mut(&mut self) -> IterMut<T> { self.into_iter() }

	pub fn add(&mut self, v: T) {
		let mut new_node = Box::new(ListNode::new(v));
		new_node.next = self.head.take();
		self.head = Some(new_node);
	}
}

impl<T: Eq> LinkList<T> {
	pub fn remove(&mut self, v: &T) -> Option<T> {
		let mut node = &mut self.head;
		while let Some(n) = node {
			if n.value.eq(v) {
				let nn = n.next.take();
				return std::mem::replace(node, nn).map(|no| no.value);
			}
			// 如果写成node = &mut n.next会导致n的生命周期延长, 所以borrow check失败
			node = &mut node.as_mut().unwrap().next;
		}
		None
	}
}

pub struct Iter<'a, T: 'a> {
	node: Option<&'a ListNode<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.node.take().map(|no| {
			self.node = no.next.as_ref().map(AsRef::as_ref);
			&no.value
		})
	}
}

pub struct IterMut<'a, T> {
	node: Option<&'a mut ListNode<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.node.take().map(|no| {
			self.node = no.next.as_mut().map(AsMut::as_mut);
			&mut no.value
		})
	}
}

pub struct IntoIter<T> {
	node: Option<ListNode<T>>,
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		self.node.take().map(|mut no| {
			self.node = no.next.take().map(|a| *a);
			no.value
		})
	}
}

impl<'a, T> IntoIterator for &'a LinkList<T> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;
	fn into_iter(self) -> Iter<'a, T> {
		Iter {
			node: self.head.as_ref().map(AsRef::as_ref),
		}
	}
}

impl<'a, T> IntoIterator for &'a mut LinkList<T> {
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;
	fn into_iter(self) -> IterMut<'a, T> {
		IterMut {
			node: self.head.as_mut().map(AsMut::as_mut),
		}
	}
}

impl<T> IntoIterator for LinkList<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;
	fn into_iter(self) -> IntoIter<T> {
		IntoIter {
			node: self.head.map(|o| *o),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		let mut list = LinkList::default();
		const TEST_RANGE: std::ops::Range<i32> = 0..21;

		for i in TEST_RANGE.rev() {
			list.add(i);
		}

		assert!(TEST_RANGE.eq(list.iter().copied()));

		list.iter_mut().for_each(|o| {
			*o += 1;
		});
		let new_range = (TEST_RANGE.start + 1)..(TEST_RANGE.end + 1);
		assert!(new_range.eq(list.iter().copied()));

		for i in 0..20 {
			list.remove(&i);
		}

		assert!((20..(TEST_RANGE.end + 1)).eq(list.iter().copied()));
	}
}
