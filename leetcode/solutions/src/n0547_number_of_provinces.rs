/**
 * [547] Number of Provinces
 *
 * There are n cities. Some of them are connected, while some are not. If city a is connected directly with city b, and city b is connected directly with city c, then city a is connected indirectly with city c.
 * A province is a group of directly or indirectly connected cities and no other cities outside of the group.
 * You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the i^th city and the j^th city are directly connected, and isConnected[i][j] = 0 otherwise.
 * Return the total number of provinces.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/24/graph1.jpg" style="width: 222px; height: 142px;" />
 * Input: isConnected = [[1,1,0],[1,1,0],[0,0,1]]
 * Output: 2
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/24/graph2.jpg" style="width: 222px; height: 142px;" />
 * Input: isConnected = [[1,0,0],[0,1,0],[0,0,1]]
 * Output: 3
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 200
 *     n == isConnected.length
 *     n == isConnected[i].length
 *     isConnected[i][j] is 1 or 0.
 *     isConnected[i][i] == 1
 *     isConnected[i][j] == isConnected[j][i]
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
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
		let n = is_connected.len() as i32;
		let mut set: Vec<_> = (0..n * n).collect();
		for (i, row) in (0..).zip(is_connected) {
			for (j, c) in (0..).zip(row) {
				if c == 1 {
					union(&mut set, i, j);
				}
			}
		}
		(0..n).filter(|&i| find(&mut set, i) == i).count() as i32
	}
}

// submission codes end
