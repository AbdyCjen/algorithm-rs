/**
 * [2527] Find Xor-Beauty of Array
 *
 * You are given a 0-indexed integer array nums.
 * The effective value of three indices i, j, and k is defined as ((nums[i] | nums[j]) &amp; nums[k]).
 * The xor-beauty of the array is the XORing of the effective values of all the possible triplets of indices (i, j, k) where 0 <= i, j, k < n.
 * Return the xor-beauty of nums.
 * Note that:
 *
 *     val1 | val2 is bitwise OR of val1 and val2.
 *     val1 &amp; val2 is bitwise AND of val1 and val2.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,4]
 * Output: 5
 * Explanation:
 * The triplets and their corresponding effective values are listed below:
 * - (0,0,0) with effective value ((1 | 1) &amp; 1) = 1
 * - (0,0,1) with effective value ((1 | 1) &amp; 4) = 0
 * - (0,1,0) with effective value ((1 | 4) &amp; 1) = 1
 * - (0,1,1) with effective value ((1 | 4) &amp; 4) = 4
 * - (1,0,0) with effective value ((4 | 1) &amp; 1) = 1
 * - (1,0,1) with effective value ((4 | 1) &amp; 4) = 4
 * - (1,1,0) with effective value ((4 | 4) &amp; 1) = 0
 * - (1,1,1) with effective value ((4 | 4) &amp; 4) = 4
 * Xor-beauty of array will be bitwise XOR of all beauties = 1 ^ 0 ^ 1 ^ 4 ^ 1 ^ 4 ^ 0 ^ 4 = 5.
 * <strong class="example">Example 2:
 *
 * Input: nums = [15,45,20,2,34,35,5,44,32,30]
 * Output: 34
 * Explanation: The xor-beauty of the given array is 34.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn xor_beauty(nums: Vec<i32>) -> i32 {
		let mut cnt = [0; 32];
		for n in &nums {
			for (c, i) in cnt.iter_mut().zip(0..) {
				if (1 << i) & n > 0 {
					*c += 1;
				}
			}
		}
		let l = nums.len();
		let mut cnt2 = [0; 32];
		for n in nums {
			for i in 0..32 {
				if n & (1 << i) > 0 {
					let c = cnt[i];
					cnt2[i] += l * l - (l - c) * (l - c);
				}
			}
		}
		cnt2.iter()
			.rev()
			.fold(0, |acc, v| (acc << 1) + (v % 2 == 1) as i32)
	}
}
