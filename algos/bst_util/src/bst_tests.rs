use bst::*;
pub fn bst_valid<Tree: BsTree>(tr: &Tree)
where Tree::Item: Copy + Ord + Default {
	tr.root_ref().map(bst_valid_inner);

	fn bst_valid_inner<TreeNode, Item>(tr: &TreeNode) -> Result<(&Item, &Item), ()>
	where
		TreeNode: BstNode<Item = Item>,
		Item: Ord,
	{
		let (mut min_k, mut max_k) = (tr.key_ref(), tr.key_ref());
		if let Some(ltr) = tr.left_ref() {
			let (lmin, lmax) = bst_valid_inner(ltr)?;
			if lmax >= tr.key_ref() {
				return Err(());
			}
			min_k = lmin.min(min_k);
			max_k = lmax.max(max_k);
		}

		if let Some(rtr) = tr.right_ref() {
			let (rmin, rmax) = bst_valid_inner(rtr)?;
			if rmin <= tr.key_ref() {
				return Err(());
			}
			min_k = rmin.min(min_k);
			max_k = rmax.max(max_k);
		}
		Ok((min_k, max_k))
	}
}
