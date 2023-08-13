/**
 * [1326] Minimum Number of Taps to Open to Water a Garden
 *
 * There is a one-dimensional garden on the x-axis. The garden starts at the point 0 and ends at the point n. (i.e The length of the garden is n).
 * There are n + 1 taps located at points [0, 1, ..., n] in the garden.
 * Given an integer n and an integer array ranges of length n + 1 where ranges[i] (0-indexed) means the i-th tap can water the area [i - ranges[i], i + ranges[i]] if it was open.
 * Return the minimum number of taps that should be open to water the whole garden, If the garden cannot be watered return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/16/1685_example_1.png" style="width: 525px; height: 255px;" />
 * Input: n = 5, ranges = [3,4,1,1,0,0]
 * Output: 1
 * Explanation: The tap at point 0 can cover the interval [-3,3]
 * The tap at point 1 can cover the interval [-3,5]
 * The tap at point 2 can cover the interval [1,3]
 * The tap at point 3 can cover the interval [2,4]
 * The tap at point 4 can cover the interval [4,4]
 * The tap at point 5 can cover the interval [5,5]
 * Opening Only the second tap will water the whole garden [0,5]
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 3, ranges = [0,0,0,0]
 * Output: -1
 * Explanation: Even if you activate all the four taps you cannot water the whole garden.
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 10^4
 *     ranges.length == n + 1
 *     0 <= ranges[i] <= 100
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_taps(n: i32, mut rng: Vec<i32>) -> i32 {
		for i in 0..rng.len() {
			let l = i.saturating_sub(rng[i] as usize);
			rng[l] = rng[l].max(i as i32 + rng[i]);
		}
		rng.pop();
		let (mut stp, mut cur_reach, mut next_reach) = (0, 0, 0);
		for (i, reach) in (0..).zip(rng) {
			next_reach = reach.max(next_reach);
			if i == cur_reach {
				cur_reach = next_reach;
				stp += 1;
			}
		}
		if cur_reach >= n {
			stp
		} else {
			-1
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1326() {
		assert_eq!(Solution::min_taps(5, vec![3, 0, 1, 1, 0, 0]), -1);
		assert_eq!(Solution::min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
	}
}
