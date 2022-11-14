/**
 * [220] Contains Duplicate III
 *
 * You are given an integer array nums and two integers indexDiff and valueDiff.
 * Find a pair of indices (i, j) such that:
 *
 *     i != j,
 *     abs(i - j) <= indexDiff.
 *     abs(nums[i] - nums[j]) <= valueDiff, and
 *
 * Return true if such pair exists or false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,3,1], indexDiff = 3, valueDiff = 0
 * Output: true
 * Explanation: We can choose (i, j) = (0, 3).
 * We satisfy the three conditions:
 * i != j --> 0 != 3
 * abs(i - j) <= indexDiff --> abs(0 - 3) <= 3
 * abs(nums[i] - nums[j]) <= valueDiff --> abs(1 - 1) <= 0
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,5,9,1,5,9], indexDiff = 2, valueDiff = 3
 * Output: false
 * Explanation: After trying all the possible pairs (i, j), we cannot satisfy the three conditions, so we return false.
 *
 *  
 * Constraints:
 *
 *     2 <= nums.length <= 10^5
 *     -10^9 <= nums[i] <= 10^9
 *     1 <= indexDiff <= nums.length
 *     0 <= valueDiff <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, id: i32, vd: i32) -> bool {
		let mut buckets = std::collections::HashMap::new();
		let get_bucket = |n| n / (vd + 1) - (n < 0) as i32;

		for (i, &n) in nums.iter().enumerate() {
			if i > id as usize {
				buckets.remove(&get_bucket(nums[i - id as usize - 1]));
			}

			let bucket = get_bucket(n);
			if buckets.insert(bucket, n).is_some() {
				return true;
			}

			if let Some(pn) = buckets.get(&(bucket - 1)) {
				if n - pn <= vd {
					return true;
				}
			}

			if let Some(pn) = buckets.get(&(bucket + 1)) {
				if pn - n <= vd {
					return true;
				}
			}
		}

		false
	}
	pub fn contains_nearby_almost_duplicate_01(nums: Vec<i32>, k: i32, v: i32) -> bool {
		let mut set = std::collections::BTreeSet::new();
		for (i, n) in nums.iter().enumerate() {
			if i > k as usize {
				set.remove(&nums[i - k as usize - 1]);
			}
			if set.range(n - v..=n + v).next().is_some() {
				return true;
			}
			set.insert(n);
		}
		false
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_220() {
		assert!(!Solution::contains_nearby_almost_duplicate(
			vec![1, 5, 9, 1, 5, 9],
			2,
			3
		));
		assert!(!Solution::contains_nearby_almost_duplicate(
			vec![-3, 3],
			2,
			4
		));
	}
}
