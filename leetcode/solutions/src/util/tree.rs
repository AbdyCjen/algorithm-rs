use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq)]
pub struct TreeNode {
	pub val: i32,
	pub left: Option<Rc<RefCell<TreeNode>>>,
	pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
	#[inline]
	pub fn new(val: i32) -> Self {
		TreeNode {
			val,
			left: None,
			right: None,
		}
	}
}

impl std::fmt::Debug for TreeNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "(")?;
		if let Some(lt) = self.left.as_ref() {
			write!(f, "{:?} ", lt.borrow())?;
		}
		write!(f, "{}", self.val)?;
		if let Some(rt) = self.right.as_ref() {
			write!(f, " {:?}", rt.borrow())?;
		}
		write!(f, ")")?;

		Ok(())
	}
}

pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
	use std::collections::VecDeque;
	let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
	let mut queue = VecDeque::new();
	queue.push_back(head.as_ref().unwrap().clone());

	for children in vec[1..].chunks(2) {
		let parent = queue.pop_front().unwrap();
		let mut parent = parent.borrow_mut();
		if let Some(v) = children[0] {
			parent.left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
			queue.push_back(parent.left.clone().unwrap());
		}
		if let Some(&Some(v)) = children.get(1) {
			parent.right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
			queue.push_back(parent.right.clone().unwrap());
		}
	}
	head
}

#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}
