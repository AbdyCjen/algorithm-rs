/**
 * [2528] Maximize the Minimum Powered City
 *
 * You are given a 0-indexed integer array stations of length n, where stations[i] represents the number of power stations in the i^th city.
 * Each power station can provide power to every city in a fixed range. In other words, if the range is denoted by r, then a power station at city i can provide power to all cities j such that |i - j| <= r and 0 <= i, j <= n - 1.
 *
 *     Note that |x| denotes absolute value. For example, |7 - 5| = 2 and |3 - 10| = 7.
 *
 * The power of a city is the total number of power stations it is being provided power from.
 * The government has sanctioned building k more power stations, each of which can be built in any city, and have the same range as the pre-existing ones.
 * Given the two integers r and k, return the maximum possible minimum power of a city, if the additional power stations are built optimally.
 * Note that you can build the k power stations in multiple cities.
 *  
 * <strong class="example">Example 1:
 *
 * Input: stations = [1,2,4,5,0], r = 1, k = 2
 * Output: 5
 * Explanation:
 * One of the optimal ways is to install both the power stations at city 1.
 * So stations will become [1,4,4,5,0].
 * - City 0 is provided by 1 + 4 = 5 power stations.
 * - City 1 is provided by 1 + 4 + 4 = 9 power stations.
 * - City 2 is provided by 4 + 4 + 5 = 13 power stations.
 * - City 3 is provided by 5 + 4 = 9 power stations.
 * - City 4 is provided by 5 + 0 = 5 power stations.
 * So the minimum power of a city is 5.
 * Since it is not possible to obtain a larger power, we return 5.
 *
 * <strong class="example">Example 2:
 *
 * Input: stations = [4,4,4,4], r = 0, k = 3
 * Output: 4
 * Explanation:
 * It can be proved that we cannot make the minimum power of a city greater than 4.
 *
 *  
 * Constraints:
 *
 *     n == stations.length
 *     1 <= n <= 10^5
 *     0 <= stations[i] <= 10^5
 *     0 <= r <= n - 1
 *     0 <= k <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_power(stat: Vec<i32>, r: i32, k: i32) -> i64 {
		let len = stat.len();
		let mut sum = vec![0_i64; len + 1];
		let mut prv = 0;
		for (s, p) in sum[1..].iter_mut().zip(stat) {
			prv += p as i64;
			*s = prv;
		}
		let mut power = vec![0_i64; len];
		let sum = |i: usize, j: usize| (sum[(j + 1).min(len)] - sum[i]);
		for (i, p) in power.iter_mut().enumerate() {
			*p = sum(i.saturating_sub(r as usize), i + r as usize);
		}

		let mut lb = *power.iter().min().unwrap();
		let mut ub = lb + k as i64;
		while lb <= ub {
			let mid = (lb + ub) / 2;
			if Self::check(&power, r * 2, k, mid) {
				lb = mid + 1;
			} else {
				ub = mid - 1;
			}
		}
		ub
	}

	fn check(power: &[i64], r: i32, mut k: i32, ans: i64) -> bool {
		let mut q = std::collections::VecDeque::<(usize, i64)>::new();
		let mut extra = 0;
		for (j, &p) in power.iter().enumerate() {
			while let Some(front) = q.front() {
				if front.0 < j {
					extra -= front.1;
					q.pop_front();
				} else {
					break;
				}
			}
			if ans > p + extra {
				let more = ans - extra - p;
				if more > k as i64 {
					return false;
				} else {
					q.push_back((j + r as usize, more));
					extra += more;
					k -= more as i32;
				}
			}
		}
		true
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2528() {
		assert_eq!(Solution::max_power(vec![4, 4, 4, 4], 0, 3), 4);
		assert_eq!(Solution::max_power(vec![1, 2, 4, 5, 0], 1, 2), 5);
	}
}
