/**
 * [540] Single Element in a Sorted Array
 *
 * You are given a sorted array consisting of only integers where every element appears exactly twice, except for one element which appears exactly once. Find this single element that appears only once.
 *  
 * Example 1:
 *
 * Input: [1,1,2,3,3,4,4,8,8]
 * Output: 2
 *
 * Example 2:
 *
 * Input: [3,3,7,7,10,11,11]
 * Output: 10
 *
 *  
 * Note: Your solution should run in O(log n) time and O(1) space.
 *
 */
pub struct Solution {}

// submission codes start here

// FIXME: 比较绝望, 二分实在gg, 观察一下这个万能式呢
#[allow(dead_code)]
impl Solution {
	pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
		if nums.len() < 5 {
			return nums.into_iter().take(3).fold(0, |acc, n| acc ^ n);
		}
		let (mut lb, mut ub) = (0, nums.len() - 1);
		while lb < ub {
			let mid = (ub + lb) / 2;
			//dbg!(lb, ub, mid);
			//dbg!(nums[lb], nums[ub], nums[mid]);
			let neighbor = if mid % 2 == 0 { mid + 1 } else { mid - 1 };
			if nums[mid] == nums[neighbor] {
				lb = mid + 1;
			} else {
				ub = mid;
			}
		}
		nums[lb]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_540() {
		assert_eq!(
			Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
			2
		);
		assert_eq!(
			Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
			10
		);
		assert_eq!(
			Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 10, 11, 11, 12]),
			12
		);
		assert_eq!(
			Solution::single_non_duplicate(vec![3, 7, 7, 10, 10, 11, 11]),
			3
		);
		assert_eq!(Solution::single_non_duplicate(vec![3]), 3);
		assert_eq!(Solution::single_non_duplicate(vec![3, 4, 4]), 3);
		assert_eq!(Solution::single_non_duplicate(vec![1, 1, 2, 3, 3]), 2);
	}
}
