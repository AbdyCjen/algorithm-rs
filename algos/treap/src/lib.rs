#![feature(arbitrary_self_types, test, nll)]
extern crate rand;
extern crate test;
use std::{cmp::Ordering, mem};

#[derive(Debug)]
struct TreapNode<T: std::cmp::Ord> {
	r: u8,
	k: T,
	child: [Option<Box<TreapNode<T>>>;2],
}

#[derive(Debug)]
pub enum TreapError {
    DupEntryError,
}

pub struct Treap<T: std::cmp::Ord> {
	root: Option<Box<TreapNode<T>>>,
}

impl<T: std::cmp::Ord> Treap<T> {
	pub fn new() -> Treap<T> { Treap { root: None } }
	pub fn find(self: &Self, k: &T) -> Option<()> {
        self.root.as_ref()?.find(k)
	}

	pub fn ins(self: &mut Self, k: T)-> Result<(), TreapError> {
		match self.root.as_mut() {
			None => { mem::replace(&mut self.root, Some(Box::new(TreapNode::new(k)))); Ok(())}
			Some(no) =>  no.ins(k),
		}
	}

    pub fn remove(self: &mut Self, k: &T) -> Option<T> {
        TreapNode::remove(&mut self.root, k)
    }
}

impl<T: std::cmp::Ord> TreapNode<T> {
	pub fn new(k: T) -> TreapNode<T> {
        TreapNode { 
            r: rand::random(),
            k,
            child: [None, None],
        }
	}

	fn rotate(self: &mut Box<Self>, d: usize) {
		let mut k = mem::replace(&mut self.child[d ^ 1], None).unwrap();
		self.child[d ^ 1] = mem::replace(&mut k.child[d], None);
		mem::swap(self, &mut k);
		mem::replace(&mut self.child[d], Some(k));
    }

	pub fn find(self: &Self, k: &T) -> Option<()> {
		match k.cmp(&self.k) {
			Ordering::Less => self.child[0].as_ref()?.find(k),
			Ordering::Greater => self.child[1].as_ref()?.find(k),
			Ordering::Equal => Some(()),
		}
	}

	pub fn ins(self: &mut Box<Self>, k: T) -> Result<(), TreapError> {
        let d = match k.cmp(&self.k) {
            Ordering::Less => 0,
            Ordering::Greater => 1,
            Ordering::Equal => Err(TreapError::DupEntryError)?,
        };

        match self.child[d].as_mut() {
            None => {self.child[d] = Some(Box::new(TreapNode::new(k)));},
            Some(no) => {
                no.ins(k)?;
                if let Ordering::Less = self.r.cmp(&no.r) {
                    self.rotate(d^1);
                }
            },
        }
        Ok(())
    }

    fn remove(no : &mut Option<Box<Self>>, k: &T) -> Option<T> {
        let o = no.as_mut()?;
        match k.cmp(&o.k) {
            Ordering::Less => { TreapNode::remove(&mut o.child[0], k) },
            Ordering::Greater => { TreapNode::remove(&mut o.child[1], k) },
            Ordering::Equal => {
                if o.child[0].is_none() {
                    let tmp = mem::replace(&mut o.child[1], None);
                    mem::replace(no, tmp).map(|t| t.k)
                } else if o.child[1].is_none() {
                    let tmp = mem::replace(&mut o.child[1], None);
                    mem::replace(no, tmp).map(|t| t.k)
                } else {
                    if let Ordering::Greater = o.child[1].as_ref()?.r.cmp(&o.child[0].as_ref()?.r) {
                        o.rotate(1);
                        TreapNode::remove(&mut o.child[1], k) 
                    } else {
                        o.rotate(0);
                        TreapNode::remove(&mut o.child[0], k) 
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test_treap {
	use super::*;
	fn check_treap<T: Ord + std::fmt::Display + Copy>(t: &Box<TreapNode<T>>) -> Result<(), T> {
		print!(r#"{{"#);
		if let Some(l) = t.child[0].as_ref() {
			print!(r#""l": "#);
			if l.k > t.k || l.r > t.r {
				eprint!("err {}", l.k);
				//Err(t.k)?
			}
			check_treap(l)?;
			print!(",");
		}
		print!(r#""{}": {}"#, t.k, t.r);
		if let Some(r) = t.child[1].as_ref() {
			print!(r#","r": "#);
			if r.k < t.k || r.r > t.r {
				//Err(t.k)?
				eprint!("err {}", r.k);
			}
			check_treap(r)?;
		}
		print!("}}");
		Ok(())
	}
	#[test]
	fn it_works() {
		let mut tr = Treap::new();
		for i in 0..100 {
			tr.ins(i).unwrap()
		}
		check_treap(&tr.root.as_ref().unwrap())
			.unwrap();
		println!();
		tr.ins(3).unwrap_err();
	}
}
