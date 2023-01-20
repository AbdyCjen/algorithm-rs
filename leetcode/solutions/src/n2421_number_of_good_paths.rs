/**
 * [2421] Number of Good Paths
 *
 * There is a tree (i.e. a connected, undirected graph with no cycles) consisting of n nodes numbered from 0 to n - 1 and exactly n - 1 edges.
 * You are given a 0-indexed integer array vals of length n where vals[i] denotes the value of the i^th node. You are also given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists an undirected edge connecting nodes ai and bi.
 * A good path is a simple path that satisfies the following conditions:
 * <ol>
 *     The starting node and the ending node have the same value.
 *     All nodes between the starting node and the ending node have values less than or equal to the starting node (i.e. the starting node's value should be the maximum value along the path).
 * </ol>
 * Return the number of distinct good paths.
 * Note that a path and its reverse are counted as the same path. For example, 0 -> 1 is considered to be the same as 1 -> 0. A single node is also considered as a valid path.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/08/04/f9caaac15b383af9115c5586779dec5.png" style="width: 400px; height: 333px;" />
 * Input: vals = [1,3,2,1,3], edges = [[0,1],[0,2],[2,3],[2,4]]
 * Output: 6
 * Explanation: There are 5 good paths consisting of a single node.
 * There is 1 additional good path: 1 -> 0 -> 2 -> 4.
 * (The reverse path 4 -> 2 -> 0 -> 1 is treated as the same as 1 -> 0 -> 2 -> 4.)
 * Note that 0 -> 2 -> 3 is not a good path because vals[2] > vals[0].
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/08/04/149d3065ec165a71a1b9aec890776ff.png" style="width: 273px; height: 350px;" />
 * Input: vals = [1,1,2,2,3], edges = [[0,1],[1,2],[2,3],[2,4]]
 * Output: 7
 * Explanation: There are 5 good paths consisting of a single node.
 * There are 2 additional good paths: 0 -> 1 and 2 -> 3.
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/08/04/31705e22af3d9c0a557459bc7d1b62d.png" style="width: 100px; height: 88px;" />
 * Input: vals = [1], edges = []
 * Output: 1
 * Explanation: The tree consists of only one node, so there is one good path.
 *
 *  
 * Constraints:
 *
 *     n == vals.length
 *     1 <= n <= 3 * 10^4
 *     0 <= vals[i] <= 10^5
 *     edges.length == n - 1
 *     edges[i].length == 2
 *     0 <= ai, bi < n
 *     ai != bi
 *     edges represents a valid tree.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
		fn find(set: &mut [i32], a: i32) -> i32 {
			if set[a as usize] != a {
				set[a as usize] = find(set, set[a as usize]);
			}
			set[a as usize]
		}
		fn union(set: &mut [i32], a: i32, b: i32) {
			let a = find(set, a);
			set[a as usize] = find(set, b);
		}
		let mut set = (0..=vals.len() as i32).collect::<Vec<_>>();
		let mut neigs = vals.iter().map(|i| (*i, Vec::new())).collect::<Vec<_>>();
		for e in edges {
			neigs[e[0] as usize].1.push(e[1]);
			neigs[e[1] as usize].1.push(e[0]);
		}
		let mut vmap = std::collections::BTreeMap::new();
		let mut ans = vals.len() as i32;
		for (v, i) in vals.into_iter().zip(0..) {
			vmap.entry(v).or_insert_with(Vec::new).push(i);
		}
		for (v, nodes) in vmap {
			for &no in &nodes {
				for &nei in &neigs[no as usize].1 {
					if v >= neigs[nei as usize].0 {
						union(&mut set, no, nei);
					}
				}
			}
			let mut cnts = std::collections::HashMap::new();
			for no in nodes {
				*cnts.entry(find(&mut set, no)).or_insert(0) += 1;
			}
			for n in cnts.into_values() {
				ans += n * (n - 1) / 2;
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2421() {
		assert_eq!(
			Solution::number_of_good_paths(
				vec![1, 3, 2, 1, 3],
				matrix![[0, 1], [1, 2], [2, 3], [2, 4]]
			),
			6
		);
	}
}
