/**
 * [2537] Count the Number of Good Subarrays
 *
 * Given an integer array nums and an integer k, return the number of good subarrays of nums.
 * A subarray arr is good if it there are at least k pairs of indices (i, j) such that i < j and arr[i] == arr[j].
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,1,1,1,1], k = 10
 * Output: 1
 * Explanation: The only good subarray is the array nums itself.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,1,4,3,2,2,4], k = 2
 * Output: 4
 * Explanation: There are 4 different good subarrays:
 * - [3,1,4,3,2,2] that has 2 pairs.
 * - [3,1,4,3,2,2,4] that has 3 pairs.
 * - [1,4,3,2,2,4] that has 2 pairs.
 * - [4,3,2,2,4] that has 2 pairs.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i], k <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn count_good(nums: Vec<i32>, mut k: i32) -> i64 {
		let (mut i, mut cnts) = (0, std::collections::HashMap::new());
		nums.iter()
			.map(|n| {
				let e = cnts.entry(n).or_insert(0);
				k -= *e;
				*e += 1;
				while k <= 0 {
					let e = cnts.get_mut(&nums[i]).unwrap();
					i += 1;
					*e -= 1;
					k += *e;
				}
				i as i64
			})
			.sum()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2537() {
		assert_eq!(
			Solution::count_good(vec![2, 3, 1, 3, 2, 3, 3, 3, 1, 1, 3, 2, 2, 2], 18),
			9
		);
	}
}
