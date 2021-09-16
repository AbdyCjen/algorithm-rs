/**
 * [310] Minimum Height Trees
 *
 * For an undirected graph with tree characteristics, we can choose any node as the root. The result graph is then a rooted tree. Among all possible rooted trees, those with minimum height are called minimum height trees (MHTs). Given such a graph, write a function to find all the MHTs and return a list of their root labels.
 *
 * Format<br />
 * The graph contains n nodes which are labeled from 0 to n - 1. You will be given the number n and a list of undirected edges (each edge is a pair of labels).
 *
 * You can assume that no duplicate edges will appear in edges. Since all edges are undirected, [0, 1] is the same as [1, 0] and thus will not appear together in edges.
 *
 * Example 1 :
 *
 *
 * Input: n = 4, edges = [[1, 0], [1, 2], [1, 3]]
 *
 *         0
 *         |
 *         1
 *        / \
 *       2   3
 *
 * Output: [1]
 *
 *
 * Example 2 :
 *
 *
 * Input: n = 6, edges = [[0, 3], [1, 3], [2, 3], [4, 3], [5, 4]]
 *
 *      0  1  2
 *       \ | /
 *         3
 *         |
 *         4
 *         |
 *         5
 *
 * Output: [3, 4]
 *
 * Note:
 *
 *
 * According to the <a href="https://en.wikipedia.org/wiki/Tree_(graph_theory)" target="_blank">definition of tree on Wikipedia</a>: &ldquo;a tree is an undirected graph in which any two vertices are connected by exactly one path. In other words, any connected graph without simple cycles is a tree.&rdquo;
 * The height of a rooted tree is the number of edges on the longest downward path between the root and a leaf.
 *
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
#[allow(dead_code)]
impl Solution {
	pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
		let mut gr = vec![vec![]; n as usize];
		let mut dg: Vec<_> = vec![0; n as usize];
		for edge in edges {
			let (p1, p2) = (edge[0] as usize, edge[1] as usize);
			gr[p1].push(p2);
			gr[p2].push(p2);
			dg[p1] += 1;
			dg[p2] += 1;
		}
		while n > 2 {
			let mut st: Vec<_> = dg
				.iter_mut()
				.enumerate()
				.filter_map(|(i, d)| if *d == 1 { Some(i) } else { None })
				.collect();
			for o in st {
				dg.remove(o);
				for i in gr[o].iter() {}
			}
		}
		vec![]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_310() {
		assert_eq!(
			Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
			vec![1]
		);
		assert_eq!(
			Solution::find_min_height_trees(
				6,
				vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 4]]
			),
			vec![3, 4]
		);
		assert_eq!(
			Solution::find_min_height_trees(2, vec![vec![0, 1]]),
			vec![0, 1]
		);
		assert_eq!(
			Solution::find_min_height_trees(
				7,
				vec![
					vec![0, 1],
					vec![1, 2],
					vec![1, 3],
					vec![2, 4],
					vec![3, 5],
					vec![4, 6]
				]
			),
			vec![1, 2]
		);
		assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);
	}
}
