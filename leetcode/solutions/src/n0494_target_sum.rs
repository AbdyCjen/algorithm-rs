/**
 * [494] Target Sum
 *
 * You are given an integer array nums and an integer target.
 * You want to build an expression out of nums by adding one of the symbols '+' and '-' before each integer in nums and then concatenate all the integers.
 *
 *     For example, if nums = [2, 1], you can add a '+' before 2 and a '-' before 1 and concatenate them to build the expression "+2-1".
 *
 * Return the number of different expressions that you can build, which evaluates to target.
 *  
 * Example 1:
 *
 * Input: nums = [1,1,1,1,1], target = 3
 * Output: 5
 * Explanation: There are 5 ways to assign symbols to make the sum of nums be target 3.
 * -1 + 1 + 1 + 1 + 1 = 3
 * +1 - 1 + 1 + 1 + 1 = 3
 * +1 + 1 - 1 + 1 + 1 = 3
 * +1 + 1 + 1 - 1 + 1 = 3
 * +1 + 1 + 1 + 1 - 1 = 3
 *
 * Example 2:
 *
 * Input: nums = [1], target = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 20
 *     0 <= nums[i] <= 1000
 *     0 <= sum(nums[i]) <= 1000
 *     -1000 <= target <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
		let sum = nums.iter().sum::<i32>() as usize;
		if target.abs() > sum as i32 {
			return 0;
		}

		let mut dp0 = vec![0; sum * 2 + 1];
		dp0[sum] += 1;
		for num in nums {
			let mut dp1 = vec![0; sum * 2 + 1];
			for (i, v) in dp0.into_iter().enumerate() {
				if v > 0 {
					dp1[i - num as usize] += v;
					dp1[i + num as usize] += v;
				}
			}
			dp0 = dp1;
		}
		dp0[(sum as i32 + target) as usize]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_494() {
		assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
		assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
	}
}
