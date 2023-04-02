/**
 * [2316] Count Unreachable Pairs of Nodes in an Undirected Graph
 *
 * You are given an integer n. There is an undirected graph with n nodes, numbered from 0 to n - 1. You are given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists an undirected edge connecting nodes ai and bi.
 * Return the number of pairs of different nodes that are unreachable from each other.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/05/tc-3.png" style="width: 267px; height: 169px;" />
 * Input: n = 3, edges = [[0,1],[0,2],[1,2]]
 * Output: 0
 * Explanation: There are no pairs of nodes that are unreachable from each other. Therefore, we return 0.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/05/tc-2.png" style="width: 295px; height: 269px;" />
 * Input: n = 7, edges = [[0,2],[0,5],[2,4],[1,6],[5,4]]
 * Output: 14
 * Explanation: There are 14 pairs of nodes that are unreachable from each other:
 * [[0,1],[0,3],[0,6],[1,2],[1,3],[1,4],[1,5],[2,3],[2,6],[3,4],[3,5],[3,6],[4,6],[5,6]].
 * Therefore, we return 14.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^5
 *     0 <= edges.length <= 2 * 10^5
 *     edges[i].length == 2
 *     0 <= ai, bi < n
 *     ai != bi
 *     There are no repeated edges.
 *
 */
pub struct Solution {}

impl Solution {
	pub fn count_pairs(mut n: i32, edges: Vec<Vec<i32>>) -> i64 {
		fn find(set: &mut [i32], x: i32) -> i32 {
			if set[x as usize] != x {
				set[x as usize] = find(set, set[x as usize]);
			}
			set[x as usize]
		}
		fn union(set: &mut [i32], a: i32, b: i32) {
			let a = find(set, a);
			set[a as usize] = find(set, b);
		}

		let mut set = (0..n).collect::<Vec<_>>();
		for r in edges {
			union(&mut set, r[0], r[1]);
		}
		let mut cnts = std::collections::HashMap::new();
		for i in (0..n).map(|i| find(&mut set, i)) {
			*cnts.entry(i).or_insert(0) += 1;
		}
		let mut ans = 0;
		for c in cnts.into_values() {
			n -= c;
			ans += c as i64 * n as i64;
		}
		ans
	}
}
