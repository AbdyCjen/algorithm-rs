/**
 * [57] Insert Interval
 *
 * You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the i^th interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.
 * Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).
 * Return intervals after the insertion.
 *  
 * <strong class="example">Example 1:
 *
 * Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
 * Output: [[1,5],[6,9]]
 *
 * <strong class="example">Example 2:
 *
 * Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
 * Output: [[1,2],[3,10],[12,16]]
 * Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
 *
 *  
 * Constraints:
 *
 *     0 <= intervals.length <= 10^4
 *     intervals[i].length == 2
 *     0 <= starti <= endi <= 10^5
 *     intervals is sorted by starti in ascending order.
 *     newInterval.length == 2
 *     0 <= start <= end <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn insert(intervals: Vec<Vec<i32>>, mut new: Vec<i32>) -> Vec<Vec<i32>> {
		let start = intervals.partition_point(|intv| intv[1] < new[0]);
		let mut intvs = intervals.into_iter().peekable();
		let mut ans: Vec<_> = intvs.by_ref().take(start).collect();
		while let Some(intv) = intvs.next_if(|intv| intv[0] <= new[1]) {
			new[1] = new[1].max(intv[1]);
			new[0] = new[0].min(intv[0]);
		}
		ans.push(new);
		ans.extend(intvs);
		ans
	}
	pub fn insert_01(intervals: Vec<Vec<i32>>, new: Vec<i32>) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		let mut new = (new[0], new[1]);
		let mut pushed = Some(());
		for intv in intervals {
			if intv[0] > new.1 {
				if pushed.take().is_some() {
					ans.push(vec![new.0, new.1]);
				}
				ans.push(intv);
			} else if intv[1] < new.0 {
				ans.push(intv);
			} else {
				new = (new.0.min(intv[0]), new.1.max(intv[1]));
			}
		}
		if pushed.take().is_some() {
			ans.push(vec![new.0, new.1]);
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_57() {
		assert_eq!(Solution::insert(matrix![], vec![2, 3]), matrix![[2, 3]]);
		assert_eq!(
			Solution::insert(matrix![[1, 5]], vec![6, 8]),
			matrix![[1, 5], [6, 8]]
		);
		assert_eq!(
			Solution::insert(matrix![[1, 5]], vec![2, 3]),
			matrix![[1, 5]]
		);
	}
}
