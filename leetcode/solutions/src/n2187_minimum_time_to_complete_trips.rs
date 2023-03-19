/**
 * [2187] Minimum Time to Complete Trips
 *
 * You are given an array time where time[i] denotes the time taken by the i^th bus to complete one trip.
 * Each bus can make multiple trips successively; that is, the next trip can start immediately after completing the current trip. Also, each bus operates independently; that is, the trips of one bus do not influence the trips of any other bus.
 * You are also given an integer totalTrips, which denotes the number of trips all buses should make in total. Return the minimum time required for all buses to complete at least totalTrips trips.
 *  
 * <strong class="example">Example 1:
 *
 * Input: time = [1,2,3], totalTrips = 5
 * Output: 3
 * Explanation:
 * - At time t = 1, the number of trips completed by each bus are [1,0,0].
 *   The total number of trips completed is 1 + 0 + 0 = 1.
 * - At time t = 2, the number of trips completed by each bus are [2,1,0].
 *   The total number of trips completed is 2 + 1 + 0 = 3.
 * - At time t = 3, the number of trips completed by each bus are [3,1,1].
 *   The total number of trips completed is 3 + 1 + 1 = 5.
 * So the minimum time needed for all buses to complete at least 5 trips is 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: time = [2], totalTrips = 1
 * Output: 2
 * Explanation:
 * There is only one bus, and it will complete its first trip at t = 2.
 * So the minimum time needed to complete 1 trip is 2.
 *
 *  
 * Constraints:
 *
 *     1 <= time.length <= 10^5
 *     1 <= time[i], totalTrips <= 10^7
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn minimum_time(time: Vec<i32>, total: i32) -> i64 {
		let max = *time.iter().max().unwrap() as i64;
		let min = *time.iter().min().unwrap() as i64;
		let mut l = (total as i64) * min / time.len() as i64;
		let mut r = ((total as i64) * max / time.len() as i64 + 1).max(max);
		while l <= r {
			let m = (l + r) / 2;
			let s: i64 = time.iter().map(|v| m / *v as i64).sum();
			if s < total as i64 {
				l = m + 1;
			} else {
				r = m - 1;
			}
		}
		l
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2187() {
		assert_eq!(Solution::minimum_time(vec![9, 7, 10, 9, 10, 9, 10], 1), 7);
		assert_eq!(Solution::minimum_time(vec![1, 2, 3], 5), 3);
		assert_eq!(Solution::minimum_time(vec![2], 1), 2);
	}
}
