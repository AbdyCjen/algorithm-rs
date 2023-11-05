/**
 * [2448] Minimum Cost to Make Array Equal
 *
 * You are given two 0-indexed arrays nums and cost consisting each of n positive integers.
 * You can do the following operation any number of times:
 *
 *     Increase or decrease any element of the array nums by 1.
 *
 * The cost of doing one operation on the i^th element is cost[i].
 * Return the minimum total cost such that all the elements of the array nums become equal.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,3,5,2], cost = [2,3,1,14]
 * Output: 8
 * Explanation: We can make all the elements equal to 2 in the following way:
 * - Increase the 0^th element one time. The cost is 2.
 * - Decrease the 1^<span style="font-size: 10.8333px;">st</span> element one time. The cost is 3.
 * - Decrease the 2^nd element three times. The cost is 1 + 1 + 1 = 3.
 * The total cost is 2 + 3 + 3 = 8.
 * It can be shown that we cannot make the array equal with a smaller cost.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [2,2,2,2,2], cost = [4,2,8,1,3]
 * Output: 0
 * Explanation: All the elements are already equal, so no operations are needed.
 *
 *  
 * Constraints:
 *
 *     n == nums.length == cost.length
 *     1 <= n <= 10^5
 *     1 <= nums[i], cost[i] <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
		let mut nums: Vec<_> = nums.into_iter().zip(cost).collect();
		nums.sort_unstable();
		let tot = nums.iter().map(|v| v.1 as i64).sum::<i64>();
		let mut rest = nums
			.iter()
			.map(|v| (v.0 - nums[0].0) as i64 * v.1 as i64)
			.sum::<i64>();
		let mut ans = rest;
		let mut prv = (nums[0].0, 0_i64, 0_i64);
		for (n, c) in nums {
			let dif = (n - prv.0) as i64;
			let cur = (n, prv.1 + c as i64, prv.2 + dif * prv.1);
			rest -= (tot - prv.1) * dif;
			ans = ans.min(cur.2 + rest);
			prv = cur;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2448() {
		assert_eq!(Solution::min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 14]), 8);
		assert_eq!(Solution::min_cost(vec![1, 2, 3, 5], vec![2, 14, 3, 1]), 8);
		assert_eq!(
			Solution::min_cost(vec![2, 2, 2, 2, 2], vec![4, 2, 8, 1, 3]),
			0
		);
	}
}
