#![allow(dead_code)]
extern crate rand;
use std::cell::{Cell, UnsafeCell};

const MAX_HEIGHT: usize = 32;
#[derive(Debug)]
struct Node<T: std::cmp::Ord + 'static> {
	key : T,
	nexts: Vec<Cell<Option<&'static Node<T>>>>,
	next: UnsafeCell<Option<Box<Node<T>>>>,
}

// 1. 结构体引用里的生命周期是啥?
// 2. 函数参数里的生命周期是啥?
// 3. 函数返回值里的生命周期是啥?

// 4. 结构成员在调用成员类型的fun(&self) 的时候,
// self的生命周期为啥是结构在代码里出现的位置而不是'static => 因为这个引用是要回收的

impl<T: std::cmp::Ord> Node<T> {
	fn new(key: T) -> Node<T> {
		let mut next_len = 1;
		while rand::random() && next_len < MAX_HEIGHT {
			next_len += 1;
		}
		Node {
			key,
			nexts: vec![Cell::new(None);next_len],
			next: UnsafeCell::new(None),
		}
	}

	fn find_prev_lev(&self, key: &'_ T, lev: usize) -> &Node<T> {
		let mut o = self;
		while let Some(no) = o.nexts[lev].get() {
			if &no.key < key {
				o = no;
			} else {
				break;
			}
		}
		o 
	}

	fn find_prev(&self, key: &'_ T, lev: usize) -> &Node<T> {
		let mut o = self;
		for lev in (0..=lev).rev() {
			o = o.find_prev_lev(key, lev);
		}
		o
	}

	fn remove(&'static self, key: &'_ T) -> Option<T> {
		let mut no = self;
		for lev in (0..self.nexts.len()).rev() {
			no = no.find_prev_lev(key, lev);
			if let Some(target_node) = no.nexts[lev].get()
				.filter(|o| o.key.eq(key)) {
				no.nexts[lev].set(target_node.nexts[lev].take());
				if lev == 0 {
					unsafe {
						let target_node = (*no.next.get()).take()?;
						(*no.next.get()) = (*target_node.next.get()).take();
						return Some(target_node.key);
					}
				}
			}
		}
		None
	}

	// 因为ins虽然会破坏结构, 但是不会导致悬挂的引用等状态,
	// 所以函数不用unsafe
	// new_node.nexts.len() <= lev 的话返回None
	fn ins(&'static self, new_node: Box<Node<T>>, lev: usize) -> Option<&Node<T>> {
		if lev > 0 {
			let new_node = self.find_prev_lev(&new_node.key, lev - 1)
				.ins(new_node, lev - 1)?;
			new_node.nexts.get(lev)?.set(self.nexts[lev].take());
			self.nexts[lev].set(Some(&new_node));
			Some(new_node)
		} else {
			new_node.nexts[lev].set(self.nexts[lev].take());
			unsafe {
				*new_node.next.get() = (*self.next.get()).take();
				*self.next.get() = Some(new_node);
				self.nexts[lev].set((*self.next.get())
								   .as_ref().map(|o| o.as_ref()));
			}
			self.nexts[lev].get()
		}
	}
}

#[derive(Debug)]
pub struct SkipList<T: std::cmp::Ord + 'static> {
	head_: UnsafeCell<Box<Node<T>>>,
	// point to self.head_
	head: Option<&'static Node<T>>,
	max_height: usize,
	len: usize,
}

impl<T: std::cmp::Ord + std::default::Default + 'static> SkipList<T> {
	fn new() -> SkipList<T> {
		let mut list = SkipList {
			head_ : UnsafeCell::new(Box::new(Node {
				key: Default::default(),
				nexts: vec![Cell::new(None);MAX_HEIGHT],
				next: UnsafeCell::new(None),
			})),
			head: None,
			max_height: 1,
			len: 0,
		};
		unsafe {
			list.head = Some(&*list.head_.get());
		}
		list
	}
}

impl<T: std::cmp::Ord + 'static> SkipList<T> {
	fn find(&self, key: &T) -> Option<()> {
		if self.head.unwrap()
			.find_prev(key, self.max_height - 1).nexts[0].get()?
			.key.eq(key) {
			Some(())
		} else {
			None
		}
	}
	fn ins(&mut self, key: T) {
		if self.find(&key).is_none() {
			let _new_node = Box::new(Node::new(key));
			if _new_node.nexts.len() > self.max_height {
				self.max_height = _new_node.nexts.len();
			}
			self.head.unwrap()
				.find_prev_lev(&_new_node.key, self.max_height - 1)
				.ins(_new_node, self.max_height - 1);
		}
	}
	
	fn remove(&mut self, key: &T) -> Option<T> {
		self.head.unwrap().remove(key)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		// TODO: 添加一些非'static 的类型T测试, 说不定不小心破坏了生命周期的检查;
		for _ in 0..100 {
			let mut list = SkipList::<i32>::new();
			for i in 0..10000{
				list.ins(i);
			}
			for i in 0..5000 {
				list.remove(&i);
			}
			check_ok(&list);
		}
	}

	fn check_ok(list: &SkipList<i32>){
		for (lev, o) in list.head.unwrap().nexts.iter().enumerate().rev() {
			let mut no = o.get();
			let mut prev:i32 = 0;
			if let Some(Some(oo)) = no.map(|no| no.nexts[lev].get()){
					prev = oo.key;
					no = oo.nexts[lev].get();
			}
			while let Some(oo) = no {
				assert!(oo.key > prev);
				prev = oo.key;
				no = oo.nexts[lev].get();
			}
		}
		println!("Checks Ok!");
	}

	impl<T: std::cmp::Ord + std::fmt::Display + 'static> std::fmt::Display for SkipList<T> {
		fn fmt(&self, f :&mut std::fmt::Formatter) -> std::fmt::Result {
			let mut no = self.head;
			while let Some(o) = no {
				write!(f, "{:4}:", o.key)?;
				for oo in o.nexts.iter() {
					match oo.get() {
						Some(o) => {write!(f, "{:4},", o.key)?},
						None => {write!(f, "____,")?},
					};
				}
				writeln!(f, "")?;
				no = o.nexts[0].get();
			}
			Ok(())
		}
	}
}
