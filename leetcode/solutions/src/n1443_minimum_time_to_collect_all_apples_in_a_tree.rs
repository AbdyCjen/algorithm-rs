/**
 * [1443] Minimum Time to Collect All Apples in a Tree
 *
 * Given an undirected tree consisting of n vertices numbered from 0 to n-1, which has some apples in their vertices. You spend 1 second to walk over one edge of the tree. Return the minimum time in seconds you have to spend to collect all apples in the tree, starting at vertex 0 and coming back to this vertex.
 * The edges of the undirected tree are given in the array edges, where edges[i] = [ai, bi] means that exists an edge connecting the vertices ai and bi. Additionally, there is a boolean array hasApple, where hasApple[i] = true means that vertex i has an apple; otherwise, it does not have any apple.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/23/min_time_collect_apple_1.png" style="width: 300px; height: 212px;" />
 * Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,true,false,true,true,false]
 * Output: 8
 * Explanation: The figure above represents the given tree where red vertices have an apple. One optimal path to collect all apples is shown by the green arrows.  
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/23/min_time_collect_apple_2.png" style="width: 300px; height: 212px;" />
 * Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,true,false,false,true,false]
 * Output: 6
 * Explanation: The figure above represents the given tree where red vertices have an apple. One optimal path to collect all apples is shown by the green arrows.  
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,false,false,false,false,false]
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^5
 *     edges.length == n - 1
 *     edges[i].length == 2
 *     0 <= ai < bi <= n - 1
 *     fromi < toi
 *     hasApple.length == n
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_time(_: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
		let mut neig = has_apple
			.into_iter()
			.map(|v| (v, vec![]))
			.collect::<Vec<_>>();
		for e in edges {
			neig[e[0] as usize].1.push(e[1]);
			neig[e[1] as usize].1.push(e[0]);
		}
		Self::dfs(0, 0, &neig).unwrap_or(0)
	}

	fn dfs(no: i32, par: i32, tree: &[(bool, Vec<i32>)]) -> Option<i32> {
		let mut ans = 0;
		for &ne in &tree[no as usize].1 {
			if ne != par {
				if let Some(l) = Self::dfs(ne, no, tree) {
					ans += l + 2;
				}
			}
		}
		if ans > 0 || tree[no as usize].0 {
			Some(ans)
		} else {
			None
		}
	}
}
