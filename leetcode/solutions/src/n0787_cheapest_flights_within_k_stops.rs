/**
 * [787] Cheapest Flights Within K Stops
 *
 * There are n cities connected by some number of flights. You are given an array flights where flights[i] = [fromi, toi, pricei] indicates that there is a flight from city fromi to city toi with cost pricei.
 * You are also given three integers src, dst, and k, return the cheapest price from src to dst with at most k stops. If there is no such route, return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-3drawio.png" style="width: 332px; height: 392px;" />
 * Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
 * Output: 700
 * Explanation:
 * The graph is shown above.
 * The optimal path with at most 1 stop from city 0 to 3 is marked in red and has cost 100 + 600 = 700.
 * Note that the path through cities [0,1,2,3] is cheaper but is invalid because it uses 2 stops.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-1drawio.png" style="width: 332px; height: 242px;" />
 * Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 1
 * Output: 200
 * Explanation:
 * The graph is shown above.
 * The optimal path with at most 1 stop from city 0 to 2 is marked in red and has cost 100 + 100 = 200.
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-2drawio.png" style="width: 332px; height: 242px;" />
 * Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 0
 * Output: 500
 * Explanation:
 * The graph is shown above.
 * The optimal path with no stops from city 0 to 2 is marked in red and has cost 500.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 100
 *     0 <= flights.length <= (n * (n - 1) / 2)
 *     flights[i].length == 3
 *     0 <= fromi, toi < n
 *     fromi != toi
 *     1 <= pricei <= 10^4
 *     There will not be any multiple flights between two cities.
 *     0 <= src, dst, k < n
 *     src != dst
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_cheapest_price(n: i32, flis: Vec<Vec<i32>>, src: i32, dst: i32, mut k: i32) -> i32 {
		let mut fli = vec![vec![i32::MAX; n as usize]; n as usize];
		for f in flis {
			let e = &mut fli[f[0] as usize][f[1] as usize];
			*e = f[2].min(*e);
		}
		let fli: Vec<Vec<_>> = fli
			.into_iter()
			.map(|nei| (0..).zip(nei).filter(|v| v.1 != i32::MAX).collect())
			.collect();
		let mut dis = vec![i32::MAX; n as usize];
		dis[src as usize] = 0;
		use std::iter::FromIterator;
		let mut st = std::collections::HashMap::<i32, i32>::from_iter([(src, 0)]);
		while !st.is_empty() && k >= 0 {
			for (cur, amt) in std::mem::take(&mut st) {
				for &(to, pr) in &fli[cur as usize] {
					if pr < i32::MAX && dis[to as usize] > amt + pr {
						dis[to as usize] = amt + pr;
						st.insert(to, amt + pr);
					}
				}
			}
			k -= 1;
		}
		match dis[dst as usize] {
			i32::MAX => -1,
			n => n,
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_787() {
		assert_eq!(
			Solution::find_cheapest_price(
				5,
				// 0 3 2 1 4
				matrix![
					[0, 1, 100],
					[0, 2, 100],
					[0, 3, 10],
					[1, 2, 100],
					[1, 4, 10],
					[2, 1, 10],
					[2, 3, 100],
					[2, 4, 100],
					[3, 2, 10],
					[3, 4, 100]
				],
				0,
				4,
				3
			),
			40
		);
		assert_eq!(
			Solution::find_cheapest_price(
				4,
				matrix![
					[0, 1, 100],
					[1, 2, 100],
					[2, 0, 100],
					[1, 3, 600],
					[2, 3, 200]
				],
				0,
				3,
				1
			),
			700
		);
	}
}
