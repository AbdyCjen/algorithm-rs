use crate::*;
use std::iter::FromIterator;
pub struct Iter<'a, T> {
    cur_node: Option<Rc<Node<T>>>,
    marker: std::marker::PhantomData<&'a crate::Skiplist<T>>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur_node = self.cur_node.take()?;
        self.cur_node = cur_node.nexts.borrow()[0].clone();
        // FIXME: can we eliminate unsafe
        Some(unsafe { &(*Rc::as_ptr(&cur_node)).val })
    }
}

impl<'a, T: 'a> IntoIterator for &'a Skiplist<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            cur_node: self.nexts[0].clone(),
            marker: Default::default(),
        }
    }
}

impl<T> Skiplist<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        self.into_iter()
    }
}

pub struct IntoIter<T> {
    cur_node: Option<Rc<Node<T>>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur_node = self.cur_node.take()?;
        self.cur_node = cur_node.nexts.borrow_mut()[0].take();
        Some(Rc::try_unwrap(cur_node).ok().unwrap().val)
    }
}

impl<T> IntoIterator for Skiplist<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        Self::IntoIter {
            cur_node: self.nexts[0].take(),
        }
    }
}

impl<T: std::cmp::Ord + Default> FromIterator<T> for Skiplist<T> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Self {
        let mut list = Skiplist::default();
        list.extend(it);
        list
    }
}

impl<T: std::cmp::Ord> std::iter::Extend<T> for Skiplist<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, it: I) {
        for i in it {
            self.insert(i);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_iter() {
        const TEST_RANGE: std::ops::Range<i32> = -100..100;
        let list: Skiplist<i32> = TEST_RANGE.collect();
        assert!((TEST_RANGE).eq(list.iter().copied()));
        assert!((TEST_RANGE).eq(list.into_iter()));
    }
}
