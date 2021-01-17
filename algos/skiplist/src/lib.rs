use arrayvec::ArrayVec;
use std::ptr::NonNull;
mod iter;

const MAX_HEIGHT: usize = 32;
struct Node<T: std::cmp::Ord> {
	key: T,
	nexts: Box<[Option<NonNull<Node<T>>>]>,
}

impl<'a, T: std::cmp::Ord + 'a> Node<T> {
	fn new(key: T) -> Node<T> {
		let height = (rand::random::<i32>().trailing_ones() as usize + 1).min(MAX_HEIGHT);
		Node {
			key,
			nexts: vec![None; height].into_boxed_slice(),
		}
	}

	fn find_greater_or_equal<'b: 'a>(&'b self, key: &'_ T, mut lev: usize) -> Option<&'b Node<T>> {
		unsafe {
			let mut co = self;
			loop {
				while let Some(no) = co.nexts[lev] {
					if no.as_ref().key >= *key {
						break;
					}
					co = &*no.as_ptr();
				}

				if lev == 0 {
					break co.nexts[0].map(|co| &*co.as_ptr());
				} else {
					lev -= 1;
				}
			}
		}
	}

	fn find_greater_or_equal_mut<'b: 'a>(
		&'b mut self,
		key: &'_ T,
		mut lev: usize,
		prev: &mut [NonNull<Node<T>>],
	) -> Option<&'b mut Node<T>> {
		unsafe {
			let mut co = self;
			loop {
				while let Some(no) = co.nexts[lev] {
					if no.as_ref().key >= *key {
						break;
					}
					co = &mut *no.as_ptr();
				}

				prev[lev] = NonNull::new_unchecked(co as *mut Node<T>);

				if lev == 0 {
					break co.nexts[0].map(|co| &mut *co.as_ptr());
				} else {
					lev -= 1;
				}
			}
		}
	}
}

pub struct SkipList<T: std::cmp::Ord> {
	head: Node<T>,
	max_height: usize,
	len: usize,
}

impl<T: std::cmp::Ord> SkipList<T> {
	pub fn new(some_key: T) -> SkipList<T> {
		SkipList {
			head: Node {
				key: some_key,
				nexts: vec![None; MAX_HEIGHT].into_boxed_slice(),
			},
			max_height: 1,
			len: 0,
		}
	}

	pub fn len(&self) -> usize { self.len }

	pub fn is_empty(&self) -> bool { self.len == 0 }

	pub fn find(&self, key: &T) -> Option<()> {
		if self
			.head
			.find_greater_or_equal(key, self.max_height - 1)?
			.key == *key
		{
			Some(())
		} else {
			None
		}
	}

	// FIXME: insertion is terribly slow
	pub fn insert(&mut self, key: T) {
		unsafe {
			//is there a simple way to init prev?
			let mut head_iter =
				std::iter::repeat(NonNull::new_unchecked(&mut self.head as *mut Node<T>));
			let mut prev: ArrayVec<[NonNull<Node<T>>; MAX_HEIGHT]> =
				head_iter.by_ref().take(self.max_height).collect();

			let so = self
				.head
				.find_greater_or_equal_mut(&key, self.max_height - 1, &mut prev);
			if so.is_none() || so.unwrap().key > key {
				let new_node = Box::new(Node::new(key));
				let height = new_node.nexts.len();
				let new_node = NonNull::new_unchecked(Box::into_raw(new_node));

				if height > self.max_height {
					prev.extend(head_iter.take(height - self.max_height));
					self.max_height = height;
				}

				for (lev, (mut prev, co_next)) in prev
					.into_iter()
					.zip(new_node.clone().as_mut().nexts.iter_mut())
					.enumerate()
				{
					*co_next = prev.as_mut().nexts[lev].replace(new_node);
				}

				self.len += 1;
			}
		}
	}

	pub fn remove(&mut self, key: &T) -> Option<T> {
		unsafe {
			let mut prev: ArrayVec<[NonNull<Node<T>>; MAX_HEIGHT]> =
				std::iter::repeat(NonNull::new_unchecked(&mut self.head as *mut Node<T>))
					.take(self.max_height)
					.collect();
			let co = self
				.head
				.find_greater_or_equal_mut(&key, self.max_height - 1, &mut prev);
			if co.as_ref()?.key == *key {
				let co = co.unwrap();
				let height = co.nexts.len();
				for (lev, (mut prev, co_next)) in prev
					.into_iter()
					.zip(co.nexts.iter_mut())
					.take(height)
					.enumerate()
				{
					prev.as_mut().nexts[lev] = co_next.take();
				}

				let to_ret = Some(Box::from_raw(co as *mut Node<T>).key);
				while self.head.nexts[self.max_height - 1].is_none() && self.max_height > 1 {
					self.max_height -= 1;
				}
				self.len -= 1;
				to_ret
			} else {
				None
			}
		}
	}
}

impl<T: std::cmp::Ord> Drop for SkipList<T> {
	fn drop(&mut self) {
		unsafe {
			let mut cur_node = self.head.nexts[0].take();
			while let Some(mut co) = cur_node {
				cur_node = co.as_mut().nexts[0].take();
				Box::from_raw(co.as_ptr());
			}
		}
	}
}

impl<T: Default + std::cmp::Ord> Default for SkipList<T> {
	fn default() -> Self { Self::new(Default::default()) }
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn it_works() {
		for _ in 0..10 {
			let mut list = SkipList::new(std::i32::MIN);
			for i in -1_00_000..1_00_000 {
				list.insert(i);
			}
			check_ok(&list);
			for i in -5000..5000 {
				assert!(list.find(&i).is_some());
				assert_eq!(list.remove(&i), Some(i));
				assert!(list.find(&i).is_none());
			}
			check_ok(&list);
			let v: Vec<_> = (0..100).collect();
			let mut list = SkipList::new(&v[0]);
			for i in v.iter() {
				list.insert(i);
			}

			for i in v.iter().skip(20) {
				assert!(list.find(&i).is_some());
			}
		}
	}

	fn check_ok<T: std::cmp::Ord>(list: &SkipList<T>) {
		unsafe {
			// check key sorted and continuous in every level
			let mut prev: Vec<_> = list.head.nexts.iter().copied().flatten().collect();
			let mut link_cnt = 0;
			let mut link_dist = [0; MAX_HEIGHT];
			if let Some(mut no) = list.head.nexts[0] {
				while let Some(o) = no.as_ref().nexts[0] {
					for i in 0..o.as_ref().nexts.len() {
						assert!(prev[i].as_ref().key <= o.as_ref().key);
						// continuous
						if prev[i].as_ref().key < o.as_ref().key {
							assert!(prev[i].as_ref().nexts[i] == Some(o));
						}
						prev[i] = o;
					}
					link_cnt += no.as_ref().nexts.len();
					link_dist[no.as_ref().nexts.len()] += 1;
					no = o;
				}
			}
			let cnt: i32 = link_dist.iter().copied().sum();
			println!(
				"Checks Ok! (total link: {}, total node: {} , cnt: {}",
				link_cnt,
				list.len(),
				cnt
			);
			println!("link distribution: {:?}", link_dist);
		}
	}

	impl<T: std::cmp::Ord + std::fmt::Display> std::fmt::Display for SkipList<T> {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			let mut no = self.head.nexts[0];
			unsafe {
				while let Some(o) = no {
					write!(f, "{:4}:", o.as_ref().key)?;
					for oo in o.as_ref().nexts.iter() {
						match oo {
							Some(o) => write!(f, "{:4},", o.as_ref().key)?,
							None => write!(f, "____,")?,
						};
					}
					writeln!(f)?;
					no = o.as_ref().nexts[0];
				}
			}
			Ok(())
		}
	}
}
