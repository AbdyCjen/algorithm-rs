/**
 * [435] Non-overlapping Intervals
 *
 * Given an array of intervals intervals where intervals[i] = [starti, endi], return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.
 *  
 * <strong class="example">Example 1:
 *
 * Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
 * Output: 1
 * Explanation: [1,3] can be removed and the rest of the intervals are non-overlapping.
 *
 * <strong class="example">Example 2:
 *
 * Input: intervals = [[1,2],[1,2],[1,2]]
 * Output: 2
 * Explanation: You need to remove two [1,2] to make the rest of the intervals non-overlapping.
 *
 * <strong class="example">Example 3:
 *
 * Input: intervals = [[1,2],[2,3]]
 * Output: 0
 * Explanation: You don't need to remove any of the intervals since they're already non-overlapping.
 *
 *  
 * Constraints:
 *
 *     1 <= intervals.length <= 10^5
 *     intervals[i].length == 2
 *     -5 * 10^4 <= starti < endi <= 5 * 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
		intervals.sort_unstable();
		Self::solve(&intervals, &mut vec![-1; intervals.len() + 1])
	}

	fn solve(ints: &[Vec<i32>], cache: &mut [i32]) -> i32 {
		if cache[ints.len()] >= 0 {
			return cache[ints.len()];
		}
		match ints {
			[int, rest @ ..] => {
				let mut ans = Self::solve(rest, cache) + 1;
				let idx = rest.partition_point(|i| i[0] < int[1]);
				ans = ans.min(Self::solve(&rest[idx..], cache) + idx as i32);
				cache[ints.len()] = ans;
				ans
			}
			[] => 0,
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_435() {
		assert_eq!(
			Solution::erase_overlap_intervals(matrix![[1, 2], [2, 3], [3, 4], [1, 3]]),
			1
		);
		assert_eq!(
			Solution::erase_overlap_intervals(matrix![[1, 2], [1, 2], [1, 2]]),
			2
		);
		assert_eq!(
			Solution::erase_overlap_intervals(matrix![[1, 2], [2, 3]]),
			0
		);
	}
}
