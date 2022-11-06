/**
 * [863] Sum of Distances in Tree
 *
 * An undirected, connected tree with N nodes labelled 0...N-1 and N-1 edges are given.
 *
 * The ith edge connects nodes edges[i][0] and edges[i][1] together.
 *
 * Return a list ans, where ans[i] is the sum of the distances between node i and all other nodes.
 *
 * Example 1:
 *
 *
 * Input: N = 6, edges = [[0,1],[0,2],[2,3],[2,4],[2,5]]
 * Output: [8,12,6,10,10,10]
 * Explanation:
 * Here is a diagram of the given tree:
 *   0
 *  / \
 * 1   2
 *    /|\
 *   3 4 5
 * We can see that dist(0,1) + dist(0,2) + dist(0,3) + dist(0,4) + dist(0,5)
 * equals 1 + 1 + 2 + 2 + 2 = 8.  Hence, answer[0] = 8, and so on.
 *
 *
 * Note:<font face="monospace"> 1 <= N <= 10000</font>
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
		let n = n as usize;
		let mut adj: Vec<Vec<_>> = vec![Vec::new(); n];
		for edge in edges {
			adj[edge[0] as usize].push(edge[1] as usize);
			adj[edge[1] as usize].push(edge[0] as usize);
		}

		let mut child_cnts = vec![0; n];
		let (_, dist) = count_child(&adj, &mut child_cnts, 0, std::usize::MAX);
		let mut res = vec![0; n];
		res[0] = dist;
		dfs(&mut res, &child_cnts, &adj, 0, std::usize::MAX);
		return res;

		fn count_child(
			adj: &[Vec<usize>],
			child_cnts: &mut [i32],
			cur: usize,
			parent: usize,
		) -> (i32, i32) {
			let (child_cnt, child_dist) = adj[cur]
				.iter()
				.copied()
				.filter(|i| *i != parent)
				.map(|nei| count_child(adj, child_cnts, nei, cur))
				.fold((0, 0), |(a, b), (i, j)| (a + i, b + j));
			child_cnts[cur] = child_cnt;
			(child_cnt + 1, child_dist + child_cnt)
		}

		fn dfs(res: &mut [i32], child_cnts: &[i32], adj: &[Vec<usize>], cur: usize, parent: usize) {
			if let Some(p) = res.get(parent) {
				res[cur] = p - child_cnts[cur] + (child_cnts.len() as i32 - 2 - child_cnts[cur]);
			}

			for nei in adj[cur].iter().copied().filter(|i| *i != parent) {
				dfs(res, child_cnts, adj, nei, cur);
			}
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_863() {
		assert_eq!(
			vec![8, 12, 6, 10, 10, 10],
			Solution::sum_of_distances_in_tree(6, matrix![[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]])
		)
	}
}
