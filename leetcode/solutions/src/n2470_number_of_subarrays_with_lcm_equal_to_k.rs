/**
 * [2470] Number of Subarrays With LCM Equal to K
 *
 * Given an integer array nums and an integer k, return the number of subarrays of nums where the least common multiple of the subarray's elements is k.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 * The least common multiple of an array is the smallest positive integer that is divisible by all the array elements.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [3,6,2,7,1], k = 6
 * Output: 4
 * Explanation: The subarrays of nums where 6 is the least common multiple of all the subarray's elements are:
 * - [<u>3</u>,<u>6</u>,2,7,1]
 * - [<u>3</u>,<u>6</u>,<u>2</u>,7,1]
 * - [3,<u>6</u>,2,7,1]
 * - [3,<u>6</u>,<u>2</u>,7,1]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3], k = 2
 * Output: 0
 * Explanation: There are no subarrays of nums where 2 is the least common multiple of all the subarray's elements.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 1000
 *     1 <= nums[i], k <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
		let (mut i, mut j) = (0, 0);
		let mut ans = 0;
		while j < nums.len() {
			if k % nums[j] == 0 {
				for idx in (i..=j).rev() {
					if Self::gcd(nums[idx..=j].iter().map(|i| k / i)) == 1 {
						ans += 1;
					}
				}
			} else {
				i = j + 1;
			}
			j += 1;
		}
		ans
	}

	fn gcd<I: Iterator<Item = i32>>(mut it: I) -> i32 {
		let mut l = it.next().unwrap();
		for r in it {
			l = Self::gcd2(l, r);
			if l == 1 {
				break;
			}
		}
		l
	}
	fn gcd2(a: i32, b: i32) -> i32 {
		if b == 0 {
			a
		} else {
			Self::gcd2(b, a % b)
		}
	}
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2470() {
		assert_eq!(Solution::subarray_lcm(vec![3, 6, 2, 7, 1], 6), 4);
		assert_eq!(Solution::subarray_lcm(vec![3], 2), 0);
	}
}
