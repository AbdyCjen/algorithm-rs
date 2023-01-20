/**
 * [974] Subarray Sums Divisible by K
 *
 * Given an integer array nums and an integer k, return the number of non-empty subarrays that have a sum divisible by k.
 * A subarray is a contiguous part of an array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [4,5,0,-2,-3,1], k = 5
 * Output: 7
 * Explanation: There are 7 subarrays with a sum divisible by k = 5:
 * [4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [5], k = 9
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 3 * 10^4
 *     -10^4 <= nums[i] <= 10^4
 *     2 <= k <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
		let mut cnts = vec![0; k as usize];
		let mut ans = 0;
		let mut offset = 0;
		for n in nums {
			cnts[offset as usize] += 1;
			offset = (offset + k - n).rem_euclid(k);
			ans += cnts[offset as usize];
		}
		ans
	}
	// simple but tle
	pub fn subarrays_div_by_k_00(nums: Vec<i32>, k: i32) -> i32 {
		let mut cnts = std::collections::HashMap::<i32, _>::new();
		let mut ans = 0;
		for n in nums {
			for (m, cnt) in std::mem::take(&mut cnts) {
				cnts.insert((m + n).rem_euclid(k), cnt);
			}
			*cnts.entry(n.rem_euclid(k)).or_insert(0) += 1;
			ans += cnts.get(&0).unwrap_or(&0);
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_974() {
		assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
		assert_eq!(Solution::subarrays_div_by_k(vec![5], 9), 0);
		assert_eq!(Solution::subarrays_div_by_k(vec![0, -5], 10), 1);
	}
}
