/**
 * [611] Valid Triangle Number
 *
 * Given an integer array nums, return the number of triplets chosen from the array that can make triangles if we take them as side lengths of a triangle.
 *  
 * Example 1:
 *
 * Input: nums = [2,2,3,4]
 * Output: 3
 * Explanation: Valid combinations are:
 * 2,3,4 (using the first 2)
 * 2,3,4 (using the second 2)
 * 2,2,3
 *
 * Example 2:
 *
 * Input: nums = [4,2,3,4]
 * Output: 4
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 1000
 *     0 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

//	Select three length from sorted array: i, j ,k (i < j < k)
//	Whether selected lengths can form a triangle is equivant to whether:
//		nums[i] + nums[j] > nums[k]
//	For a given k, if nums[i] + nums[j] > nums[k],
//	apparently, for any i' in [i,j), nums[i'] + nums[j] > nums[k];
#[allow(dead_code)]
impl Solution {
	pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
		nums.sort_unstable();
		let mut ans = 0;
		for (mut j, &kv) in (1..).zip(nums.iter().skip(2)) {
			let mut i = 0;
			while i < j {
				let sum = nums[i] + nums[j];
				if sum <= kv {
					i += 1;
				} else {
					ans += (j - i) as i32;
					j -= 1;
				}
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
	fn test_611() {
		assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
		assert_eq!(Solution::triangle_number(vec![2, 3, 4, 4]), 4);
		assert_eq!(Solution::triangle_number(vec![0, 1, 0]), 0);
		assert_eq!(Solution::triangle_number(vec![0, 1, 1]), 0);
		assert_eq!(Solution::triangle_number(vec![1, 1, 1, 1]), 4);
		assert_eq!(Solution::triangle_number(vec![1]), 0);
	}
}
