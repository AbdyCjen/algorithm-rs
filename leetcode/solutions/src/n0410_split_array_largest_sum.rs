/**
 * [410] Split Array Largest Sum
 *
 * Given an array nums which consists of non-negative integers and an integer m, you can split the array into m non-empty continuous subarrays.
 * Write an algorithm to minimize the largest sum among these m subarrays.
 *  
 * Example 1:
 *
 * Input: nums = [7,2,5,10,8], m = 2
 * Output: 18
 * Explanation:
 * There are four ways to split nums into two subarrays.
 * The best way is to split it into [7,2,5] and [10,8],
 * where the largest sum among the two subarrays is only 18.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4,5], m = 2
 * Output: 9
 *
 * Example 3:
 *
 * Input: nums = [1,4,4], m = 3
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 1 <= nums.length <= 1000
 * 0 <= nums[i] <= 10^6
 * 1 <= m <= min(50, nums.length)
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
		let max: i32 = nums.iter().max().copied().unwrap_or(0);
		let sum: i32 = nums.iter().sum();
		if m == 1 {
			return sum;
		}

		let (mut l, mut r) = (max, sum);
		while l <= r {
			let mid = l + (r - l) / 2;
			if is_valid(mid, &nums, m) {
				r = mid - 1;
			} else {
				l = mid + 1;
			}
		}
		return l;

		fn is_valid(target: i32, nums: &[i32], m: i32) -> bool {
			nums.iter()
				.fold((1, 0), |(cnt, sum), &i| {
					if sum + i > target {
						(cnt + 1, i)
					} else {
						(cnt, sum + i)
					}
				})
				.0 <= m
		}
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_410() {
		assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
		assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 1), 15);
		assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
		assert_eq!(Solution::split_array(vec![1, 4, 4], 3), 4);
	}
}
