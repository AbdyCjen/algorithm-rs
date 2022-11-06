/**
 * [2136] Earliest Possible Day of Full Bloom
 *
 * You have n flower seeds. Every seed must be planted first before it can begin to grow, then bloom. Planting a seed takes time and so does the growth of a seed. You are given two 0-indexed integer arrays plantTime and growTime, of length n each:
 *
 *     plantTime[i] is the number of full days it takes you to plant the i^th seed. Every day, you can work on planting exactly one seed. You do not have to work on planting the same seed on consecutive days, but the planting of a seed is not complete until you have worked plantTime[i] days on planting it in total.
 *     growTime[i] is the number of full days it takes the i^th seed to grow after being completely planted. After the last day of its growth, the flower blooms and stays bloomed forever.
 *
 * From the beginning of day 0, you can plant the seeds in any order.
 * Return the earliest possible day where all seeds are blooming.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/21/1.png" style="width: 453px; height: 149px;" />
 * Input: plantTime = [1,4,3], growTime = [2,3,1]
 * Output: 9
 * Explanation: The grayed out pots represent planting days, colored pots represent growing days, and the flower represents the day it blooms.
 * One optimal way is:
 * On day 0, plant the 0^th seed. The seed grows for 2 full days and blooms on day 3.
 * On days 1, 2, 3, and 4, plant the 1^st seed. The seed grows for 3 full days and blooms on day 8.
 * On days 5, 6, and 7, plant the 2^nd seed. The seed grows for 1 full day and blooms on day 9.
 * Thus, on day 9, all the seeds are blooming.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/21/2.png" style="width: 454px; height: 184px;" />
 * Input: plantTime = [1,2,3,2], growTime = [2,1,2,1]
 * Output: 9
 * Explanation: The grayed out pots represent planting days, colored pots represent growing days, and the flower represents the day it blooms.
 * One optimal way is:
 * On day 1, plant the 0^th seed. The seed grows for 2 full days and blooms on day 4.
 * On days 0 and 3, plant the 1^st seed. The seed grows for 1 full day and blooms on day 5.
 * On days 2, 4, and 5, plant the 2^nd seed. The seed grows for 2 full days and blooms on day 8.
 * On days 6 and 7, plant the 3^rd seed. The seed grows for 1 full day and blooms on day 9.
 * Thus, on day 9, all the seeds are blooming.
 *
 * <strong class="example">Example 3:
 *
 * Input: plantTime = [1], growTime = [1]
 * Output: 2
 * Explanation: On day 0, plant the 0^th seed. The seed grows for 1 full day and blooms on day 2.
 * Thus, on day 2, all the seeds are blooming.
 *
 *  
 * Constraints:
 *
 *     n == plantTime.length == growTime.length
 *     1 <= n <= 10^5
 *     1 <= plantTime[i], growTime[i] <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
		let mut time_arr = plant_time.into_iter().zip(grow_time).collect::<Vec<_>>();
		time_arr.sort_unstable_by_key(|(_, grow_time)| *grow_time);

		let mut cur = 0;
		let mut ans = 0;
		for (plant, grow) in time_arr.into_iter().rev() {
			cur += plant;
			ans = ans.max(cur + grow);
		}

		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2136() {
		assert_eq!(
			Solution::earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]),
			9
		);
		assert_eq!(
			Solution::earliest_full_bloom(vec![1, 2, 3, 2], vec![2, 1, 2, 1]),
			9
		);
		assert_eq!(Solution::earliest_full_bloom(vec![1], vec![1]), 2);
	}
}
