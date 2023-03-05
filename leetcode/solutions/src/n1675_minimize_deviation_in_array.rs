/**
 * [1675] Minimize Deviation in Array
 *
 * You are given an array nums of n positive integers.
 * You can perform two types of operations on any element of the array any number of times:
 *
 *     If the element is even, divide it by 2.
 *     
 *         For example, if the array is [1,2,3,4], then you can do this operation on the last element, and the array will be [1,2,3,<u>2</u>].
 *     
 *     
 *     If the element is odd, multiply it by 2.
 *     
 *         For example, if the array is [1,2,3,4], then you can do this operation on the first element, and the array will be [<u>2</u>,2,3,4].
 *     
 *     
 *
 * The deviation of the array is the maximum difference between any two elements in the array.
 * Return the minimum deviation the array can have after performing some number of operations.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,3,4]
 * Output: 1
 * Explanation: You can transform the array to [1,2,3,<u>2</u>], then to [<u>2</u>,2,3,2], then the deviation will be 3 - 2 = 1.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [4,1,5,20,3]
 * Output: 3
 * Explanation: You can transform the array after two operations to [4,<u>2</u>,5,<u>5</u>,3], then the deviation will be 5 - 2 = 3.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [2,10,8]
 * Output: 3
 *
 *  
 * Constraints:
 *
 *     n == nums.length
 *     2 <= n <= 5 * 10^<span style="font-size: 10.8333px;">4</span>
 *     1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn minimum_deviation(mut nums: Vec<i32>) -> i32 {
		for n in nums.iter_mut().filter(|n| **n % 2 == 1) {
			*n *= 2;
		}
		let mut min = *nums.iter().min().unwrap();
		let mut bh = std::collections::BinaryHeap::from(nums);
		let mut ans = i32::MAX;
		while *bh.peek().unwrap() % 2 == 0 {
			let mut t = bh.peek_mut().unwrap();
			ans = ans.min(*t - min);
			min = min.min(*t / 2);
			*t /= 2;
		}
		ans.min(bh.pop().unwrap() - min)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1675() {
		assert_eq!(Solution::minimum_deviation(vec![2, 10, 8]), 3);
		assert_eq!(Solution::minimum_deviation(vec![4, 8, 12]), 1);
		assert_eq!(Solution::minimum_deviation(vec![1, 2, 3, 4]), 1);
		assert_eq!(Solution::minimum_deviation(vec![4, 1, 5, 20, 3]), 3);
	}
}
