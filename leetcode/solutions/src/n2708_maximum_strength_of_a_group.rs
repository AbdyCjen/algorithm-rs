/**
 * [2708] Maximum Strength of a Group
 *
 * You are given a 0-indexed integer array nums representing the score of students in an exam. The teacher would like to form one non-empty group of students with maximal strength, where the strength of a group of students of indices i0, i1, i2, ... , ik is defined as nums[i0] * nums[i1] * nums[i2] * ... * nums[ik​].
 * Return the maximum strength of a group the teacher can create.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [3,-1,-5,2,5,-9]
 * Output: 1350
 * Explanation: One way to form a group of maximal strength is to group the students at indices [0,2,3,4,5]. Their strength is 3 * (-5) * 2 * 5 * (-9) = 1350, which we can show is optimal.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [-4,-5,-4]
 * Output: 20
 * Explanation: Group the students at indices [0, 1] . Then, we&rsquo;ll have a resulting strength of 20. We cannot achieve greater strength.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 13
 *     -9 <= nums[i] <= 9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn max_strength(nums: Vec<i32>) -> i64 {
		nums[1..]
			.iter()
			.copied()
			.fold((nums[0] as i64, nums[0] as i64), |(min, max), n| {
				let n = n as i64;
				(
					min.min(min * n).min(max * n).min(n),
					max.max(max * n).max(min * n).max(n),
				)
			})
			.1
	}
	pub fn max_strength1(mut nums: Vec<i32>) -> i64 {
		nums.sort_unstable();
		let mut ans = 1;
		for w in nums.chunks_exact(2).take_while(|w| w[1] < 0) {
			ans *= (w[1] * w[0]) as i64;
		}
		for &i in nums.iter().filter(|i| **i > 0) {
			ans *= i as i64;
		}
		match nums.as_slice() {
			[.., 0] if ans == 1 => 0,
			[.., n @ ..=-1] if ans == 1 => *n as i64,
			_ => ans,
		}
	}
}

// submission codes end
