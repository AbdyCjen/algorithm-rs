/**
 * [879] Profitable Schemes
 *
 * There is a group of n members, and a list of various crimes they could commit. The i^th crime generates a profit[i] and requires group[i] members to participate in it. If a member participates in one crime, that member can't participate in another crime.
 * Let's call a profitable scheme any subset of these crimes that generates at least minProfit profit, and the total number of members participating in that subset of crimes is at most n.
 * Return the number of schemes that can be chosen. Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 *
 * Input: n = 5, minProfit = 3, group = [2,2], profit = [2,3]
 * Output: 2
 * Explanation: To make a profit of at least 3, the group could either commit crimes 0 and 1, or just crime 1.
 * In total, there are 2 schemes.
 * <strong class="example">Example 2:
 *
 * Input: n = 10, minProfit = 5, group = [2,3,5], profit = [6,7,8]
 * Output: 7
 * Explanation: To make a profit of at least 5, the group could commit any crimes, as long as they commit one.
 * There are 7 possible schemes: (0), (1), (2), (0,1), (0,2), (1,2), and (0,1,2).
 *  
 * Constraints:
 *
 *     1 <= n <= 100
 *     0 <= minProfit <= 100
 *     1 <= group.length <= 100
 *     1 <= group[i] <= 100
 *     profit.length == group.length
 *     0 <= profit[i] <= 100
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
		Self::solve(
			n,
			min_profit,
			&group,
			&profit,
			&mut vec![vec![vec![-1; profit.len() + 1]; min_profit as usize + 1]; n as usize + 1],
		)
	}
	fn solve(
		n: i32,
		min_profit: i32,
		group: &[i32],
		profit: &[i32],
		cache: &mut [Vec<Vec<i32>>],
	) -> i32 {
		if n == 0 {
			return (min_profit <= 0) as i32;
		} else if cache[n as usize][min_profit.max(0) as usize][group.len()] >= 0 {
			return cache[n as usize][min_profit.max(0) as usize][group.len()];
		}
		match (group, profit) {
			([g, group @ ..], [p, profit @ ..]) => {
				let mut ans = Self::solve(n, min_profit, group, profit, cache);
				if *g <= n {
					ans = (ans + Self::solve(n - g, min_profit - p, group, profit, cache))
						% (1e9 as i32 + 7);
				}
				let e = &mut cache[n as usize][min_profit.max(0) as usize][group.len() + 1];
				*e = ((*e).max(0) + ans) % (1e9 as i32 + 7);
				ans
			}
			_ => (min_profit <= 0) as i32,
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_879() {
		assert_eq!(
			Solution::profitable_schemes(
				100,
				100,
				vec![
					24, 23, 7, 4, 26, 3, 7, 11, 1, 7, 1, 3, 5, 26, 26, 1, 13, 12, 2, 1, 7, 4, 1,
					27, 13, 16, 26, 18, 6, 1, 1, 7, 16, 1, 6, 2, 5, 9, 19, 28, 1, 23, 2, 1, 3, 4,
					4, 3, 22, 1, 1, 3, 5, 34, 2, 1, 22, 16, 8, 5, 3, 21, 1, 8, 14, 2, 1, 3, 8, 12,
					40, 6, 4, 2, 2, 14, 1, 11, 9, 1, 7, 1, 1, 1, 6, 6, 4, 1, 1, 7, 8, 10, 20, 2,
					14, 31, 1, 13, 1, 9
				],
				vec![
					5, 2, 38, 25, 4, 17, 5, 1, 4, 0, 0, 8, 13, 0, 20, 0, 28, 1, 22, 7, 10, 32, 6,
					37, 0, 11, 6, 11, 23, 20, 13, 13, 6, 2, 36, 1, 0, 9, 4, 5, 6, 14, 20, 1, 13, 6,
					33, 0, 22, 1, 17, 12, 10, 1, 19, 13, 8, 1, 0, 17, 20, 9, 8, 6, 2, 2, 1, 4, 22,
					11, 3, 2, 6, 0, 40, 0, 0, 7, 1, 0, 25, 5, 12, 7, 19, 4, 12, 7, 4, 4, 1, 15, 33,
					14, 2, 1, 1, 61, 4, 5
				]
			),
			653387801
		);
	}
}
