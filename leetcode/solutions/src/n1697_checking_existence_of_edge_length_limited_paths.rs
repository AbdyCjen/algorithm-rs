/**
 * [1697] Checking Existence of Edge Length Limited Paths
 *
 * An undirected graph of n nodes is defined by edgeList, where edgeList[i] = [ui, vi, disi] denotes an edge between nodes ui and vi with distance disi. Note that there may be multiple edges between two nodes.
 * Given an array queries, where queries[j] = [pj, qj, limitj], your task is to determine for each queries[j] whether there is a path between pj and qj such that each edge on the path has a distance strictly less than limitj .
 * Return a boolean array answer, where answer.length == queries.length and the j^th value of answer is true if there is a path for queries[j] is true, and false otherwise.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/08/h.png" style="width: 267px; height: 262px;" />
 * Input: n = 3, edgeList = [[0,1,2],[1,2,4],[2,0,8],[1,0,16]], queries = [[0,1,2],[0,2,5]]
 * Output: [false,true]
 * Explanation: The above figure shows the given graph. Note that there are two overlapping edges between 0 and 1 with distances 2 and 16.
 * For the first query, between 0 and 1 there is no path where each distance is less than 2, thus we return false for this query.
 * For the second query, there is a path (0 -> 1 -> 2) of two edges with distances less than 5, thus we return true for this query.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/08/q.png" style="width: 390px; height: 358px;" />
 * Input: n = 5, edgeList = [[0,1,10],[1,2,5],[2,3,9],[3,4,13]], queries = [[0,4,14],[1,4,13]]
 * Output: [true,false]
 * Exaplanation: The above figure shows the given graph.
 *
 *  
 * Constraints:
 *
 *     2 <= n <= 10^5
 *     1 <= edgeList.length, queries.length <= 10^5
 *     edgeList[i].length == 3
 *     queries[j].length == 3
 *     0 <= ui, vi, pj, qj <= n - 1
 *     ui != vi
 *     pj != qj
 *     1 <= disi, limitj <= 10^9
 *     There may be multiple edges between two nodes.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn distance_limited_paths_exist(
		n: i32,
		mut edges: Vec<Vec<i32>>,
		queries: Vec<Vec<i32>>,
	) -> Vec<bool> {
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
		edges.sort_unstable_by_key(|v| -v[2]);
		let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
		queries.sort_unstable_by_key(|e| e.1[2]);

		let mut set: Vec<i32> = (0..n).collect();
		let mut ans = vec![true; queries.len()];
		for (i, q) in queries {
			while let Some(e) = edges.last() {
				if e[2] < q[2] {
					union(&mut set, e[0], e[1]);
					edges.pop();
				} else {
					break;
				}
			}
			ans[i] = find(&mut set, q[0]) == find(&mut set, q[1]);
		}

		ans
	}
}

// submission codes end
