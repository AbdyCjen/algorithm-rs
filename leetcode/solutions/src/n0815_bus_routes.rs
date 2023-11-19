/**
 * [815] Bus Routes
 *
 * You are given an array routes representing bus routes where routes[i] is a bus route that the i^th bus repeats forever.
 *
 *     For example, if routes[0] = [1, 5, 7], this means that the 0^th bus travels in the sequence 1 -> 5 -> 7 -> 1 -> 5 -> 7 -> 1 -> ... forever.
 *
 * You will start at the bus stop source (You are not on any bus initially), and you want to go to the bus stop target. You can travel between bus stops by buses only.
 * Return the least number of buses you must take to travel from source to target. Return -1 if it is not possible.
 *  
 * <strong class="example">Example 1:
 *
 * Input: routes = [[1,2,7],[3,6,7]], source = 1, target = 6
 * Output: 2
 * Explanation: The best strategy is take the first bus to the bus stop 7, then take the second bus to the bus stop 6.
 *
 * <strong class="example">Example 2:
 *
 * Input: routes = [[7,12],[4,5,15],[6],[15,19],[9,12,13]], source = 15, target = 12
 * Output: -1
 *
 *  
 * Constraints:
 *
 *     1 <= routes.length <= 500.
 *     1 <= routes[i].length <= 10^5
 *     All the values of routes[i] are unique.
 *     sum(routes[i].length) <= 10^5
 *     0 <= routes[i][j] < 10^6
 *     0 <= source, target < 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn num_buses_to_destination(mut routes: Vec<Vec<i32>>, from: i32, to: i32) -> i32 {
		// station => bus routes
		let n = *routes.iter().flatten().max().unwrap();
		let mut neis = vec![vec![]; n as usize + 1];
		for (bus, route) in (0..).zip(&routes) {
			for &st in route {
				neis[st as usize].push(bus);
			}
		}
		let mut ans = 0;
		let mut dq = vec![from];
		let mut vis = std::collections::HashSet::new();
		while !dq.is_empty() {
			for st in std::mem::take(&mut dq) {
				if st == to {
					return ans;
				}
				for bus in std::mem::take(&mut neis[st as usize]) {
					for nei in routes[bus as usize].drain(0..) {
						if vis.insert(nei) {
							dq.push(nei);
						}
					}
				}
			}
			ans += 1;
		}
		-1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_815() {
		assert_eq!(
			Solution::num_buses_to_destination(
				matrix![[7, 12], [4, 5, 15], [6], [15, 19], [9, 12, 13]],
				15,
				12
			),
			-1
		);
		assert_eq!(
			Solution::num_buses_to_destination(matrix![[1, 2, 7], [3, 6, 7]], 1, 6),
			2
		);
	}
}
