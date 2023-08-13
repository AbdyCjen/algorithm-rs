/**
 * [1575] Count All Possible Routes
 *
 * You are given an array of distinct positive integers locations where locations[i] represents the position of city i. You are also given integers start, finish and fuel representing the starting city, ending city, and the initial amount of fuel you have, respectively.
 * At each step, if you are at city i, you can pick any city j such that j != i and 0 <= j < locations.length and move to city j. Moving from city i to city j reduces the amount of fuel you have by |locations[i] - locations[j]|. Please notice that |x| denotes the absolute value of x.
 * Notice that fuel cannot become negative at any point in time, and that you are allowed to visit any city more than once (including start and finish).
 * Return the count of all possible routes from start to finish. Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: locations = [2,3,6,8,4], start = 1, finish = 3, fuel = 5
 * Output: 4
 * Explanation: The following are all possible routes, each uses 5 units of fuel:
 * 1 -> 3
 * 1 -> 2 -> 3
 * 1 -> 4 -> 3
 * 1 -> 4 -> 2 -> 3
 *
 * <strong class="example">Example 2:
 *
 * Input: locations = [4,3,1], start = 1, finish = 0, fuel = 6
 * Output: 5
 * Explanation: The following are all possible routes:
 * 1 -> 0, used fuel = 1
 * 1 -> 2 -> 0, used fuel = 5
 * 1 -> 2 -> 1 -> 0, used fuel = 5
 * 1 -> 0 -> 1 -> 0, used fuel = 3
 * 1 -> 0 -> 1 -> 0 -> 1 -> 0, used fuel = 5
 *
 * <strong class="example">Example 3:
 *
 * Input: locations = [5,2,1], start = 0, finish = 2, fuel = 3
 * Output: 0
 * Explanation: It is impossible to get from 0 to 2 using only 3 units of fuel since the shortest route needs 4 units of fuel.
 *
 *  
 * Constraints:
 *
 *     2 <= locations.length <= 100
 *     1 <= locations[i] <= 10^9
 *     All integers in locations are distinct.
 *     0 <= start, finish < locations.length
 *     1 <= fuel <= 200
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_routes(mut pos: Vec<i32>, start: i32, end: i32, fuel: i32) -> i32 {
		let (start, end) = (pos[start as usize], pos[end as usize]);
		if (end - start).abs() > fuel {
			return 0;
		}
		pos.sort_unstable();
		let start = pos.binary_search(&start).unwrap();
		let end = pos.binary_search(&end).unwrap();
		Self::solve(
			&pos,
			start,
			end,
			fuel,
			&mut vec![vec![-1; fuel as usize + 1]; pos.len() + 1],
		)
	}

	fn solve(pos: &[i32], start: usize, end: usize, fuel: i32, cache: &mut [Vec<i32>]) -> i32 {
		const MO: i32 = 1e9 as i32 + 7;
		if cache[start][fuel as usize] >= 0 {
			return cache[start][fuel as usize];
		}
		let mut ans = 0;
		if start == end {
			ans += 1;
		}
		for (i, &n) in pos.iter().enumerate().skip(start + 1) {
			if let fuel_rest @ 0.. = fuel - (n - pos[start]) {
				ans = (ans + Self::solve(pos, i, end, fuel_rest, cache)) % MO;
			} else {
				break;
			}
		}
		for (i, &n) in pos[..start].iter().enumerate().rev() {
			if let fuel_rest @ 0.. = fuel - (pos[start] - n) {
				ans = (ans + Self::solve(pos, i, end, fuel_rest, cache)) % MO;
			} else {
				break;
			}
		}
		cache[start][fuel as usize] = ans;
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1575() {
		assert_eq!(Solution::count_routes(vec![1, 2, 5], 1, 1, 3), 2);
		assert_eq!(Solution::count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5), 4);
		assert_eq!(Solution::count_routes(vec![4, 3, 1], 1, 0, 6), 5);
		assert_eq!(Solution::count_routes(vec![5, 2, 1], 0, 2, 3), 0);
	}
}
