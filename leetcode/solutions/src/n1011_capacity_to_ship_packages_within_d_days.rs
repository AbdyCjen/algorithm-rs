/**
 * [1011] Capacity To Ship Packages Within D Days
 *
 * A conveyor belt has packages that must be shipped from one port to another within days days.
 *
 * The i^th package on the conveyor belt has a weight of weights[i]. Each day, we load the ship with packages on the conveyor belt (in the order given by weights). We may not load more weight than the maximum weight capacity of the ship.
 *
 * Return the least weight capacity of the ship that will result in all the packages on the conveyor belt being shipped within days days.
 *
 *  
 * <strong class="example">Example 1:
 *
 *
 * Input: weights = [1,2,3,4,5,6,7,8,9,10], days = 5
 * Output: 15
 * Explanation: A ship capacity of 15 is the minimum to ship all the packages in 5 days like this:
 * 1st day: 1, 2, 3, 4, 5
 * 2nd day: 6, 7
 * 3rd day: 8
 * 4th day: 9
 * 5th day: 10
 *
 * Note that the cargo must be shipped in the order given, so using a ship of capacity 14 and splitting the packages into parts like (2, 3, 4, 5), (1, 6, 7), (8), (9), (10) is not allowed.
 *
 *
 * <strong class="example">Example 2:
 *
 *
 * Input: weights = [3,2,2,4,1,4], days = 3
 * Output: 6
 * Explanation: A ship capacity of 6 is the minimum to ship all the packages in 3 days like this:
 * 1st day: 3, 2
 * 2nd day: 2, 4
 * 3rd day: 1, 4
 *
 *
 * <strong class="example">Example 3:
 *
 *
 * Input: weights = [1,2,3,1,1], days = 4
 * Output: 3
 * Explanation:
 * 1st day: 1
 * 2nd day: 2
 * 3rd day: 3
 * 4th day: 1, 1
 *
 *
 *  
 * Constraints:
 *
 *
 *     1 <= days <= weights.length <= 5 * 10^4
 *     1 <= weights[i] <= 500
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
		let mut l = weights.iter().max().copied().unwrap();
		let mut r = weights.iter().sum::<i32>();
		while l <= r {
			let mid = (r - l) / 2 + l;
			if Self::ship(&weights, mid, days) {
				r = mid - 1;
			} else {
				l = mid + 1;
			}
		}
		l
	}

	fn ship(weights: &[i32], cap: i32, days: i32) -> bool {
		let mut acc = 0;
		let mut ans = 1;
		for &i in weights {
			if ans > days {
				return false;
			}
			if acc + i > cap {
				acc = i;
				ans += 1;
			} else {
				acc += i;
			}
		}
		ans <= days
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1011() {
		assert_eq!(Solution::ship_within_days(vec![500; 502], 1), 500 * 502);
		assert_eq!(
			Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
			15
		);
		assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
		assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
	}
}
