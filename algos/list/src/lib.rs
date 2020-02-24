#[derive(Debug)]
pub struct LinkList<T: Eq> {
	head: Option<Box<ListNode<T>>>,
}

#[derive(Debug)]
pub struct ListNode<T: Eq> {
	value: T,
	next: Option<Box<ListNode<T>>>,
}

impl<T: Eq> ListNode<T> {
	fn new(v: T) -> ListNode<T> { ListNode { value: v, next: None } }
}

impl<T: Eq> LinkList<T> {
	// TODO: 有safe的办法直接在头保存&mut 尾节点吗? ==> 原则上不行, 下回写个用RefCell的版本吧
	pub fn new() -> LinkList<T> { LinkList { head: None } }
	pub fn iter<'a>(&'a self) -> Iter<'a, T> { Iter { node: self.head.as_ref() } }

	pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> { IterMut { node: self.head.as_mut() } }
	pub fn add(&mut self, v: T) {
		let mut new_node = Box::new(ListNode::new(v));
		new_node.next = self.head.take();
		self.head = Some(new_node);
	}

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

pub struct Iter<'a, T>
	where T: 'a + Eq
{
	node: Option<&'a Box<ListNode<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> where T: Eq
{
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.node.take().map(|no| {
			                self.node = no.next.as_ref();
			                &no.value
		                })
	}
}

pub struct IterMut<'a, T>
	where T: Eq
{
	node: Option<&'a mut Box<ListNode<T>>>,
}

impl<'a, T> Iterator for IterMut<'a, T> where T: Eq + 'static
{
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.node.take().map(|no| {
			                self.node = no.next.as_mut();
			                &mut no.value
		                })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		let mut list = LinkList::new();
		let plist = |list: &LinkList<i32>| {
			for i in list.iter() {
				print!("{},", i);
			}
			println!("");
		};
		let pmut = |list: &mut LinkList<i32>| {
			for i in list.iter_mut() {
				*i += 1;
				print!("{},", i);
			}
			println!("");
		};

		for i in (0..30_i32).rev() {
			list.add(i);
		}
		plist(&mut list);
		pmut(&mut list);
		plist(&mut list);

		for i in 0..20 {
			list.remove(&i);
		}
	}
}
