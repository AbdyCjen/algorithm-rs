/**
 * [442] Find All Duplicates in an Array
 *
 * Given an integer array nums of length n where all the integers of nums are in the range [1, n] and each integer appears once or twice, return an array of all the integers that appears twice.
 * You must write an algorithm that runs in O(n) time and uses only constant extra space.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [4,3,2,7,8,2,3,1]
 * Output: [2,3]
 * <strong class="example">Example 2:
 * Input: nums = [1,1,2]
 * Output: [1]
 * <strong class="example">Example 3:
 * Input: nums = [1]
 * Output: []
 *  
 * Constraints:
 *
 *     n == nums.length
 *     1 <= n <= 10^5
 *     1 <= nums[i] <= n
 *     Each element in nums appears once or twice.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
		let mut ans = vec![];
		let n = nums.len();
		nums.push(0);
		nums.swap(0, n);
		for i in 1..nums.len() {
			while nums[i] != nums[nums[i] as usize] {
				let ni = nums[i];
				nums.swap(i, ni as usize);
			}
		}
		for (i, n) in (0..).zip(nums).skip(1) {
			if n != i {
				ans.push(n);
			}
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_442() {
		assert_eq!(
			Solution::find_duplicates(vec![10, 2, 5, 10, 9, 1, 1, 4, 3, 7]),
			[1, 10]
		);
	}
}
