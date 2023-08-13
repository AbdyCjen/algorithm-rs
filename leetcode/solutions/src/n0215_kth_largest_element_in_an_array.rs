/**
 * [215] Kth Largest Element in an Array
 *
 * Given an integer array nums and an integer k, return the k^th largest element in the array.
 * Note that it is the k^th largest element in the sorted order, not the k^th distinct element.
 * Can you solve it without sorting?
 *  
 * <strong class="example">Example 1:
 * Input: nums = [3,2,1,5,6,4], k = 2
 * Output: 5
 * <strong class="example">Example 2:
 * Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
 * Output: 4
 *  
 * Constraints:
 *
 *     1 <= k <= nums.length <= 10^5
 *     -10^4 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_kth_largest(mut nums: Vec<i32>, mut kth: i32) -> i32 {
		let mut nums = &mut nums[..];
		while nums.len() > 1 {
			let (mut i, mut j, mut k) = (0, 0, nums.len() - 1);
			let n = nums[nums.len() / 2];
			while j <= k {
				use std::cmp::Ordering::*;
				match nums[j].cmp(&n) {
					Greater => {
						nums.swap(i, j);
						i += 1;
						j += 1;
					}
					Less => {
						nums.swap(j, k);
						k -= 1;
					}
					_ => j += 1,
				}
			}
			if kth <= i as i32 {
				nums = &mut nums[..i];
			} else if kth > j as i32 {
				nums = &mut nums[j..];
				kth -= j as i32;
			} else {
				return n;
			}
		}
		nums[0]
	}
	pub fn find_kth_largest1(nums: Vec<i32>, k: i32) -> i32 {
		let mut bh = std::collections::BinaryHeap::new();
		for n in nums {
			bh.push(-n);
			if bh.len() > k as usize {
				bh.pop();
			}
		}
		-bh.pop().unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_215() {
		assert_eq!(Solution::find_kth_largest(vec![7, 6, 5, 4, 3, 2, 1], 2), 6);
	}
}
