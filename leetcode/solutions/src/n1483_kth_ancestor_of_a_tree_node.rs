/**
 * [1483] Kth Ancestor of a Tree Node
 *
 * You are given a tree with n nodes numbered from 0 to n - 1 in the form of a parent array parent where parent[i] is the parent of i^th node. The root of the tree is node 0. Find the k^th ancestor of a given node.
 * The k^th ancestor of a tree node is the k^th node in the path from that node to the root node.
 * Implement the TreeAncestor class:
 *
 *     TreeAncestor(int n, int[] parent) Initializes the object with the number of nodes in the tree and the parent array.
 *     int getKthAncestor(int node, int k) return the k^th ancestor of the given node node. If there is no such ancestor, return -1.
 *
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/08/28/1528_ex1.png" style="width: 396px; height: 262px;" />
 * Input
 * ["TreeAncestor", "getKthAncestor", "getKthAncestor", "getKthAncestor"]
 * [[7, [-1, 0, 0, 1, 1, 2, 2]], [3, 1], [5, 2], [6, 3]]
 * Output
 * [null, 1, 0, -1]
 * Explanation
 * TreeAncestor treeAncestor = new TreeAncestor(7, [-1, 0, 0, 1, 1, 2, 2]);
 * treeAncestor.getKthAncestor(3, 1); // returns 1 which is the parent of 3
 * treeAncestor.getKthAncestor(5, 2); // returns 0 which is the grandparent of 5
 * treeAncestor.getKthAncestor(6, 3); // returns -1 because there is no such ancestor
 *  
 * Constraints:
 *
 *     1 <= k <= n <= 5 * 10^4
 *     parent.length == n
 *     parent[0] == -1
 *     0 <= parent[i] < n for all 0 < i < n
 *     0 <= node < n
 *     There will be at most 5 * 10^4 queries.
 *
 */
pub struct Solution {}

// submission codes start here

struct TreeAncestor {
	ancestor: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
	fn new(n: i32, parent: Vec<i32>) -> Self {
		let mut ancestor = vec![parent];
		for i in (0..).take_while(|i| 1 << i <= n) {
			let mut parent = vec![-1; n as usize];
			for (&p, c) in ancestor[i].iter().zip(&mut parent) {
				if p >= 0 {
					*c = ancestor[i][p as usize];
				}
			}
			ancestor.push(parent);
		}
		Self { ancestor }
	}

	fn get_kth_ancestor(&self, mut cur: i32, k: i32) -> i32 {
		for i in 0..self.ancestor.len() as i32 {
			if cur >= 0 && k & (1 << i) > 0 {
				cur = self.ancestor[i as usize][cur as usize];
			}
		}
		cur
	}
}

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */

// submission codes end

#[cfg(test)]
mod tests {
	#[test]
	fn test_1483() {}
}
