/**
 * [1335] Minimum Difficulty of a Job Schedule
 *
 * You want to schedule a list of jobs in d days. Jobs are dependent (i.e To work on the i^th job, you have to finish all the jobs j where 0 <= j < i).
 * You have to finish at least one task every day. The difficulty of a job schedule is the sum of difficulties of each day of the d days. The difficulty of a day is the maximum difficulty of a job done on that day.
 * You are given an integer array jobDifficulty and an integer d. The difficulty of the i^th job is jobDifficulty[i].
 * Return the minimum difficulty of a job schedule. If you cannot find a schedule for the jobs return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/16/untitled.png" style="width: 365px; height: 370px;" />
 * Input: jobDifficulty = [6,5,4,3,2,1], d = 2
 * Output: 7
 * Explanation: First day you can finish the first 5 jobs, total difficulty = 6.
 * Second day you can finish the last job, total difficulty = 1.
 * The difficulty of the schedule = 6 + 1 = 7
 *
 * <strong class="example">Example 2:
 *
 * Input: jobDifficulty = [9,9,9], d = 4
 * Output: -1
 * Explanation: If you finish a job per day you will still have a free day. you cannot find a schedule for the given jobs.
 *
 * <strong class="example">Example 3:
 *
 * Input: jobDifficulty = [1,1,1], d = 3
 * Output: 3
 * Explanation: The schedule is one job per day. total difficulty will be 3.
 *
 *  
 * Constraints:
 *
 *     1 <= jobDifficulty.length <= 300
 *     0 <= jobDifficulty[i] <= 1000
 *     1 <= d <= 10
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::*;
impl Solution {
	pub fn min_difficulty(nums: Vec<i32>, d: i32) -> i32 {
		if (nums.len() as i32) < d {
			return -1;
		} else {
			Self::solve(&nums, d, &mut HashMap::new())
		}
	}
	fn solve(mut nums: &[i32], d: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
		if let Some(a) = cache.get(&(nums.len() as i32, d)) {
			return *a;
		} else if nums.is_empty() {
			return 0;
		} else if d == 1 {
			return *cache
				.entry((nums.len() as i32, d))
				.or_insert(*nums.iter().max().unwrap());
		}
		let len = nums.len() as i32;
		let mut acc = 0;
		let mut ans = i32::MAX;
		while let [n, rest @ ..] = nums {
			acc = acc.max(*n);
			if rest.len() as i32 >= d - 1 {
				ans = ans.min(Self::solve(rest, d - 1, cache) + acc);
			} else {
				break;
			}
			nums = rest;
		}
		cache.insert((len, d), ans);
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn test_1335() {
		assert_eq!(
			Solution::min_difficulty(
				vec![
					380, 302, 102, 681, 863, 676, 243, 671, 651, 612, 162, 561, 394, 856, 601, 30,
					6, 257, 921, 405, 716, 126, 158, 476, 889, 699, 668, 930, 139, 164, 641, 801,
					480, 756, 797, 915, 275, 709, 161, 358, 461, 938, 914, 557, 121, 964, 315
				],
				10
			),
			3807
		);
		assert_eq!(Solution::min_difficulty(vec![1; 3], 3), 3);
		assert_eq!(Solution::min_difficulty(vec![9; 3], 4), -1);
	}
}
