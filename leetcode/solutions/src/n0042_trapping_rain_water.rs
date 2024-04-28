/**
 * [42] Trapping Rain Water
 *
 * Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
 *  
 * Example 1:
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png" style="width: 412px; height: 161px;" />
 * Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
 * Output: 6
 * Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
 *
 * Example 2:
 *
 * Input: height = [4,2,0,3,2,5]
 * Output: 9
 *
 *  
 * Constraints:
 *
 *     n == height.length
 *     0 <= n <= 3 * 10^4
 *     0 <= height[i] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn trap(height: Vec<i32>) -> i32 {
		let mut st = vec![];
		let mut max = 0;
		for h in &height {
			max = max.max(*h);
			st.push(max);
		}
		max = 0;
		st.iter()
			.zip(height)
			.rev()
			.map(|(&lmax, h)| {
				max = max.max(h);
				(max.min(lmax) - h).max(0)
			})
			.sum()
	}
	pub fn trap1(height: Vec<i32>) -> i32 {
		if height.len() < 3 {
			return 0;
		}
		let mut prv_base = 0;
		let mut trap_sum = 0;
		let (mut l, mut r) = (0, height.len() - 1);
		while l < r {
			let m = height[r].min(height[l]);
			prv_base = prv_base.max(m);
			trap_sum += prv_base - m;

			if height[l] > height[r] {
				r -= 1;
			} else {
				l += 1;
			}
		}

		trap_sum
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_42() {
		assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
		assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
		assert_eq!(Solution::trap(vec![5, 5, 1, 7, 1, 1, 5, 2, 7, 6]), 23);
	}
}
