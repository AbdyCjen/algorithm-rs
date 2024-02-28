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

impl Solution {
	pub fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
		let mut deg = vec![0; n as usize];
		let mut nei = vec![vec![]; n as usize];
		for e in edges {
			deg[e[0] as usize] += 1;
			deg[e[1] as usize] += 1;
			nei[e[0] as usize].push(e[1]);
			nei[e[1] as usize].push(e[0]);
		}
		let mut que: Vec<_> = (0..).zip(&deg).filter(|v| *v.1 <= 1).map(|v| v.0).collect();
		while n > 2 {
			n -= que.len() as i32;
			for i in std::mem::take(&mut que) {
				for &ne in &nei[i as usize] {
					deg[ne as usize] -= 1;
					if deg[ne as usize] == 1 {
						que.push(ne);
					}
				}
			}
		}
		que
	}
	pub fn find_min_height_trees1(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
		let mut neis = vec![vec![]; n as usize];
		for edge in edges {
			let (p1, p2) = (edge[0], edge[1]);
			neis[p1 as usize].push((p2, -1));
			neis[p2 as usize].push((p1, -1));
		}
		neis.push((0..n).map(|i| (i, -1)).collect());

		let mut ans = (n, vec![]);
		for i in 0..n {
			let h = Self::dfs(i, n, &mut neis);
			match h - ans.0 {
				..=-1 => ans = (h, vec![i]),
				0 => ans.1.push(i),
				_ => {}
			}
		}
		ans.1
	}

	fn dfs(i: i32, prv: i32, neis: &mut [Vec<(i32, i32)>]) -> i32 {
		let mut d = 1;
		for j in 0..neis[i as usize].len() {
			let jj = neis[i as usize][j].0;
			if jj != prv {
				if neis[i as usize][j].1 < 0 {
					neis[i as usize][j].1 = Self::dfs(jj, i, neis);
				}
				d = d.max(neis[i as usize][j].1 + 1);
			}
		}
		d
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_310() {
		assert_eq!(
			Solution::find_min_height_trees(4, matrix![[1, 0], [1, 2], [1, 3]]),
			vec![1]
		);
		assert_eq!(
			Solution::find_min_height_trees(6, matrix![[0, 3], [1, 3], [2, 3], [4, 3], [5, 4]]),
			vec![3, 4]
		);
		assert_eq!(
			Solution::find_min_height_trees(2, matrix![[0, 1]]),
			vec![0, 1]
		);
		assert_eq!(
			Solution::find_min_height_trees(
				7,
				matrix![[0, 1], [1, 2], [1, 3], [2, 4], [3, 5], [4, 6]]
			),
			vec![1, 2]
		);
		assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);
	}
}
