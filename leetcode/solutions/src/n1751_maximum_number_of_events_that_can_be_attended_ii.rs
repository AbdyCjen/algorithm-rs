/**
 * [1751] Maximum Number of Events That Can Be Attended II
 *
 * You are given an array of events where events[i] = [startDayi, endDayi, valuei]. The i^th event starts at startDayi and ends at endDayi, and if you attend this event, you will receive a value of valuei. You are also given an integer k which represents the maximum number of events you can attend.
 * You can only attend one event at a time. If you choose to attend an event, you must attend the entire event. Note that the end day is inclusive: that is, you cannot attend two events where one of them starts and the other ends on the same day.
 * Return the maximum sum of values that you can receive by attending events.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60048-pm.png" style="width: 400px; height: 103px;" />
 *
 * Input: events = [[1,2,4],[3,4,3],[2,3,1]], k = 2
 * Output: 7
 * Explanation: Choose the green events, 0 and 1 (0-indexed) for a total value of 4 + 3 = 7.
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60150-pm.png" style="width: 400px; height: 103px;" />
 *
 * Input: events = [[1,2,4],[3,4,3],[2,3,10]], k = 2
 * Output: 10
 * Explanation: Choose event 2 for a total value of 10.
 * Notice that you cannot attend any other event as they overlap, and that you do not have to attend k events.
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60703-pm.png" style="width: 400px; height: 126px;" />
 *
 * Input: events = [[1,1,1],[2,2,2],[3,3,3],[4,4,4]], k = 3
 * Output: 9
 * Explanation: Although the events do not overlap, you can only attend 3 events. Pick the highest valued three.
 *  
 * Constraints:
 *
 *     1 <= k <= events.length
 *     1 <= k * events.length <= 10^6
 *     1 <= startDayi <= endDayi <= 10^9
 *     1 <= valuei <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
		events.sort_unstable();
		Self::solve(
			&events,
			k,
			&mut vec![vec![-1; events.len() + 1]; 1 + k as usize],
		)
	}

	fn solve(events: &[Vec<i32>], k: i32, cache: &mut [Vec<i32>]) -> i32 {
		if cache[k as usize][events.len()] >= 0 {
			return cache[k as usize][events.len()];
		} else if k == 0 {
			return 0;
		}
		let len = events.len();
		match events {
			[event, events @ ..] => {
				let mut rest = events;
				while let [e, rrest @ ..] = rest {
					if e[0] > event[1] {
						break;
					} else {
						rest = rrest;
					}
				}
				let ans =
					Self::solve(events, k, cache).max(Self::solve(rest, k - 1, cache) + event[2]);
				cache[k as usize][len] = ans;
				ans
			}
			_ => 0,
		}
	}
}

// submission codes end
