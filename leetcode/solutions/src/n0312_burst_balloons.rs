/**
 * [312] Burst Balloons
 *
 * You are given n balloons, indexed from 0 to n - 1. Each balloon is painted with a number on it represented by an array nums. You are asked to burst all the balloons.
 * If you burst the i^th balloon, you will get nums[i - 1] * nums[i] * nums[i + 1] coins. If i - 1 or i + 1 goes out of bounds of the array, then treat it as if there is a balloon with a 1 painted on it.
 * Return the maximum coins you can collect by bursting the balloons wisely.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [3,1,5,8]
 * Output: 167
 * Explanation:
 * nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
 * coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,5]
 * Output: 10
 *
 *  
 * Constraints:
 *
 *     n == nums.length
 *     1 <= n <= 300
 *     0 <= nums[i] <= 100
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_coins(nums: Vec<i32>) -> i32 {
		Self::solve(
			&nums,
			1,
			1,
			0,
			&mut vec![vec![-1; nums.len() + 1]; nums.len() + 1],
		)
	}
	fn solve(nums: &[i32], prv: i32, post: i32, start: usize, cache: &mut [Vec<i32>]) -> i32 {
		if cache[start][nums.len()] >= 0 {
			return cache[start][nums.len()];
		}
		match nums {
			[] => 0,
			[n] => prv * n * post,
			nums => {
				let mut ans = 0;
				// NOTE: pick a last balloon to burst
				for (i, &n) in nums.iter().enumerate() {
					let cur = Self::solve(&nums[..i], prv, n, start, cache)
						+ Self::solve(&nums[i + 1..], n, post, start + i + 1, cache)
						+ n * prv * post;
					ans = ans.max(cur);
				}
				cache[start][nums.len()] = ans;
				ans
			}
		}
	}
}

// submission codes end
