mod iter;

use std::{cell::RefCell, rc::Rc};

const MAX_HEIGHT: usize = 32;

struct Node<T> {
    val: T,
    nexts: RefCell<Vec<Option<Rc<Self>>>>,
}

pub struct Skiplist<T> {
    nexts: [Option<Rc<Node<T>>>; MAX_HEIGHT],
    start: usize,
}

impl<T> Skiplist<T> {
    fn height() -> usize {
        (1 + rand::random::<u32>().trailing_ones() as usize).min(MAX_HEIGHT)
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Ord> Skiplist<T> {
    fn _insert(nexts: &mut [Option<Rc<Node<T>>>], cur: usize, new: &Rc<Node<T>>) {
        for cur in (0..=cur).rev() {
            match nexts[cur].as_deref() {
                Some(inner) if inner.val < new.val => {
                    return Self::_insert(&mut inner.nexts.borrow_mut(), cur, new)
                }
                _ if new.nexts.borrow().len() > cur => {
                    new.nexts.borrow_mut()[cur] = nexts[cur].take();
                    nexts[cur] = Some(new.clone());
                }
                _ => {}
            }
        }
    }
    pub fn insert(&mut self, val: T) {
        let hei = Self::height();
        let new = Rc::new(Node {
            val,
            nexts: RefCell::new(vec![None; hei]),
        });
        self.start = self.start.max(hei - 1);
        Self::_insert(&mut self.nexts, self.start, &new);
    }

    pub fn find(&self, k: &T) -> bool {
        Self::_find(&self.nexts, self.start, k)
    }

    fn _find(nexts: &[Option<Rc<Node<T>>>], cur: usize, k: &T) -> bool {
        for cur in (0..=cur).rev() {
            match nexts[cur].as_deref() {
                Some(inner) if inner.val < *k => return Self::_find(&inner.nexts.borrow(), cur, k),
                Some(inner) if inner.val == *k => return true,
                _ => {}
            }
        }
        false
    }
    pub fn remove(&mut self, k: &T) -> bool {
        Self::_remove(&mut self.nexts, self.start, k)
    }
    fn _remove(nexts: &mut [Option<Rc<Node<T>>>], cur: usize, k: &T) -> bool {
        let mut ret = false;
        for cur in (0..=cur).rev() {
            match nexts[cur].as_deref() {
                Some(inner) if inner.val < *k => {
                    return Self::_remove(&mut inner.nexts.borrow_mut(), cur, k)
                }
                Some(inner) if inner.val == *k => {
                    let next = inner.nexts.borrow_mut()[cur].take();
                    nexts[cur] = next;
                    ret = true;
                }
                _ => {}
            }
        }
        ret
    }
}

impl<T> Drop for Skiplist<T> {
    fn drop(&mut self) {
        let mut cur_node = self.nexts[0].take();
        while let Some(co) = cur_node.take() {
            for cell in co.nexts.borrow_mut().iter_mut().rev() {
                cur_node = cell.take();
            }
        }
    }
}
impl<T> Default for Skiplist<T> {
    fn default() -> Self {
        Self {
            start: 0,
            nexts: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        for _ in 0..1 {
            let mut list = Skiplist::default();
            for i in -100..100 {
                list.insert(i);
            }
            //println!("list: {list}");
            check_ok(&list);
            for i in -50..50 {
                assert!(list.find(&i));
                assert!(list.remove(&i));
                assert!(!list.find(&i));
            }
            check_ok(&list);
            let v: Vec<_> = (0..100).collect();
            let mut list = Skiplist::new();
            for i in v.iter() {
                list.insert(i);
            }

            for i in v.iter().skip(20) {
                assert!(list.find(&i));
            }
        }
    }

    fn check_ok<T: std::cmp::Ord>(list: &Skiplist<T>) {
        // check key sorted and continuous in every level
        let mut prevs: Vec<_> = list.nexts.iter().flatten().cloned().collect();
        let mut link_cnt = 0;
        let mut link_dist = [0; MAX_HEIGHT];
        let mut no = list.nexts[0].clone().unwrap();
        loop {
            let next = no.nexts.borrow()[0].clone();
            if let Some(next) = next {
                for (i, prev) in prevs.iter_mut().enumerate().take(next.nexts.borrow().len()) {
                    assert!(prev.val <= next.val);
                    // continuous
                    if prev.val < next.val {
                        let prv_i_to_cur =
                            std::cell::Ref::map(prev.nexts.borrow(), |ch| ch[i].as_ref().unwrap());
                        assert!(std::rc::Rc::ptr_eq(&prv_i_to_cur, &next));
                    }
                    *prev = next.clone();
                }
                link_cnt += no.nexts.borrow().len();
                link_dist[no.nexts.borrow().len()] += 1;
                no = next;
            } else {
                break;
            }
        }
        let cnt: i32 = link_dist.iter().copied().sum();
        println!("Checks Ok! (total link: {link_cnt} , cnt: {cnt}",);
        println!("link distribution: {:?}", &link_dist[1..=list.start + 1]);
    }

    /*impl<T: std::fmt::Display> std::fmt::Display for Skiplist<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut no = clone_cell(&self.head.nexts[0]);
            while let Some(o) = no {
                write!(f, "{:4}:", o.key)?;
                for oo in o.nexts.iter().map(clone_cell) {
                    match oo {
                        Some(o) => write!(f, "{:4},", o.as_ref().key)?,
                        None => write!(f, "____,")?,
                    };
                }
                writeln!(f)?;
                no = clone_cell(&o.nexts[0]);
            }
            Ok(())
        }
    }*/
}
