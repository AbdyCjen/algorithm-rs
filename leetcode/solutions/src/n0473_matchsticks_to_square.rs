/**
 * [473] Matchsticks to Square
 *
 * You are given an integer array matchsticks where matchsticks[i] is the length of the i^th matchstick. You want to use all the matchsticks to make one square. You should not break any stick, but you can link them up, and each matchstick must be used exactly one time.
 * Return true if you can make this square and false otherwise.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/matchsticks1-grid.jpg" style="width: 253px; height: 253px;" />
 * Input: matchsticks = [1,1,2,2,2]
 * Output: true
 * Explanation: You can form a square with length 2, one side of the square came two sticks with length 1.
 *
 * Example 2:
 *
 * Input: matchsticks = [3,3,3,3,4]
 * Output: false
 * Explanation: You cannot find a way to form a square with all the matchsticks.
 *
 *  
 * Constraints:
 *
 *     1 <= matchsticks.length <= 15
 *     0 <= matchsticks[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
		fn makesquare_inner(nums: &mut [i32], start: usize, side: i32, cnt: i32, sum: i32) -> bool {
			if cnt == 3 {
				return true;
			}
			if sum == side {
				return makesquare_inner(nums, 0, side, cnt + 1, 0);
			}

			for i in start..nums.len() {
				if nums[i] != 0 {
					let n = std::mem::replace(&mut nums[i], 0);
					if makesquare_inner(nums, i + 1, side, cnt, sum + n) {
						return true;
					}
					nums[i] = n;
				}
			}

			false
		}

		let s = matchsticks.iter().sum::<i32>();
		let side = s / 4;
		if s % 4 != 0 || s == 0 || matchsticks.iter().any(|i| *i > side) {
			return false;
		}
		makesquare_inner(&mut matchsticks, 0, side, 0, 0)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_473() {
		assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 2]), true);
		assert_eq!(Solution::makesquare(vec![3, 3, 3, 3, 4]), false);
		assert_eq!(
			Solution::makesquare(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
			true
		);
	}
}
