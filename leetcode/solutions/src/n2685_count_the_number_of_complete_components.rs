/**
 * [2685] Count the Number of Complete Components
 *
 * You are given an integer n. There is an undirected graph with n vertices, numbered from 0 to n - 1. You are given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists an undirected edge connecting vertices ai and bi.
 * Return the number of complete connected components of the graph.
 * A connected component is a subgraph of a graph in which there exists a path between any two vertices, and no vertex of the subgraph shares an edge with a vertex outside of the subgraph.
 * A connected component is said to be complete if there exists an edge between every pair of its vertices.
 *  
 * <strong class="example">Example 1:
 * <strong class="example"><img alt="" src="https://assets.leetcode.com/uploads/2023/04/11/screenshot-from-2023-04-11-23-31-23.png" style="width: 671px; height: 270px;" />
 *
 * Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4]]
 * Output: 3
 * Explanation: From the picture above, one can see that all of the components of this graph are complete.
 *
 * <strong class="example">Example 2:
 * <strong class="example"><img alt="" src="https://assets.leetcode.com/uploads/2023/04/11/screenshot-from-2023-04-11-23-32-00.png" style="width: 671px; height: 270px;" />
 *
 * Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4],[3,5]]
 * Output: 1
 * Explanation: The component containing vertices 0, 1, and 2 is complete since there is an edge between every pair of two vertices. On the other hand, the component containing vertices 3, 4, and 5 is not complete since there is no edge between vertices 4 and 5. Thus, the number of complete components in this graph is 1.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 50
 *     0 <= edges.length <= n * (n - 1) / 2
 *     edges[i].length == 2
 *     0 <= ai, bi <= n - 1
 *     ai != bi
 *     There are no repeated edges.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
		fn find(set: &mut [(i32, i32)], x: i32) -> (i32, i32) {
			if set[x as usize].0 != x {
				set[x as usize] = find(set, set[x as usize].0);
			}
			set[x as usize]
		}
		fn union(set: &mut [(i32, i32)], a: i32, b: i32) {
			let a = find(set, a);
			let b = find(set, b);
			set[a.0 as usize] = b;
			if a.0 != b.0 {
				set[b.0 as usize].1 += a.1 + 1;
			} else {
				set[b.0 as usize].1 += 1;
			}
		}
		let mut set = vec![];
		for i in 0..n {
			set.push((i, 0));
		}
		for e in edges {
			union(&mut set, e[0], e[1]);
		}
		let mut cnts = std::collections::HashMap::new();
		for i in 0..n {
			let a = find(&mut set, i);
			let e = cnts.entry(a.0).or_insert((0, 0));
			e.0 = a.1;
			e.1 += 1;
		}
		cnts.into_values()
			.filter(|&(c, e)| c == e * (e - 1) / 2)
			.count() as i32
	}
}

// submission codes end
