/**
 * [665] Non-decreasing Array
 *
 * Given an array nums with n integers, your task is to check if it could become non-decreasing by modifying at most one element.
 * We define an array is non-decreasing if nums[i] <= nums[i + 1] holds for every i (0-based) such that (0 <= i <= n - 2).
 *  
 * Example 1:
 *
 * Input: nums = [4,2,3]
 * Output: true
 * Explanation: You could modify the first 4 to 1 to get a non-decreasing array.
 *
 * Example 2:
 *
 * Input: nums = [4,2,1]
 * Output: false
 * Explanation: You can't get a non-decreasing array by modify at most one element.
 *
 *  
 * Constraints:
 *
 *     n == nums.length
 *     1 <= n <= 10^4
 *     -10^5 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn check_possibility(nums: Vec<i32>) -> bool {
		fn is_sorted<T: std::cmp::Ord, I: Iterator<Item = T>>(mut it: I) -> bool {
			it.next()
				.map(|mut prev| {
					it.all(move |mut o| {
						std::mem::swap(&mut o, &mut prev);
						o <= prev
					})
				})
				.unwrap_or(true)
		}

		if nums.len() <= 2 {
			return true;
		}

		let mut prev = std::i32::MIN;
		for (i, &v) in nums.iter().enumerate() {
			if v < prev {
				return (prev <= nums.get(i + 1).copied().unwrap_or(std::i32::MAX)
					|| nums
						.get(i.wrapping_sub(2))
						.copied()
						.unwrap_or(std::i32::MIN)
						<= v) && is_sorted(nums[i..].iter());
			}
			prev = v;
		}

		true
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_665() {
		assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
		assert_eq!(Solution::check_possibility(vec![1, 2, 7, 4, 5]), true);
		assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
		assert_eq!(Solution::check_possibility(vec![1, 4, 1, 2]), true);
		assert_eq!(Solution::check_possibility(vec![1, 2, -1]), true);
	}
}
