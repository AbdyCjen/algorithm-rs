/**
 * [1319] Number of Operations to Make Network Connected
 *
 * There are n computers numbered from 0 to n - 1 connected by ethernet cables connections forming a network where connections[i] = [ai, bi] represents a connection between computers ai and bi. Any computer can reach any other computer directly or indirectly through the network.
 * You are given an initial computer network connections. You can extract certain cables between two directly connected computers, and place them between any pair of disconnected computers to make them directly connected.
 * Return the minimum number of times you need to do this in order to make all the computers connected. If it is not possible, return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/02/sample_1_1677.png" style="width: 500px; height: 148px;" />
 * Input: n = 4, connections = [[0,1],[0,2],[1,2]]
 * Output: 1
 * Explanation: Remove cable between computer 1 and 2 and place between computers 1 and 3.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/02/sample_2_1677.png" style="width: 500px; height: 129px;" />
 * Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
 * Output: 2
 *
 * <strong class="example">Example 3:
 *
 * Input: n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
 * Output: -1
 * Explanation: There are not enough cables.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^5
 *     1 <= connections.length <= min(n * (n - 1) / 2, 10^5)
 *     connections[i].length == 2
 *     0 <= ai, bi < n
 *     ai != bi
 *     There are no repeated connections.
 *     No two computers are connected by more than one cable.
 *
 */
pub struct Solution {}

impl Solution {
	pub fn make_connected(n: i32, conn: Vec<Vec<i32>>) -> i32 {
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
		if n - 1 > conn.len() as i32 {
			return -1;
		}
		let mut set = (0..n).collect::<Vec<_>>();
		for c in conn {
			union(&mut set, c[0], c[1]);
		}
		(0..n)
			.map(|i| find(&mut set, i))
			.collect::<std::collections::HashSet<_>>()
			.len() as i32
			- 1
	}
}
