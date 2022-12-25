/**
 * [2518] Number of Great Partitions
 *
 * You are given an array nums consisting of positive integers and an integer k.
 * Partition the array into two ordered groups such that each element is in exactly one group. A partition is called great if the sum of elements of each group is greater than or equal to k.
 * Return the number of distinct great partitions. Since the answer may be too large, return it modulo 10^9 + 7.
 * Two partitions are considered distinct if some element nums[i] is in different groups in the two partitions.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,3,4], k = 4
 * Output: 6
 * Explanation: The great partitions are: ([1,2,3], [4]), ([1,3], [2,4]), ([1,4], [2,3]), ([2,3], [1,4]), ([2,4], [1,3]) and ([4], [1,2,3]).
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,3,3], k = 4
 * Output: 0
 * Explanation: There are no great partitions for this array.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [6,6], k = 2
 * Output: 2
 * Explanation: We can either put nums[0] in the first partition or in the second partition.
 * The great partitions will be ([6], [6]) and ([6], [6]).
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length, k <= 1000
 *     1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

const MO: i32 = 1e9 as i32 + 7;
impl Solution {
	pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
		let mut cnt = std::collections::HashMap::new();
		let s: i64 = nums.iter().map(|i| *i as i64).sum();
		let tot = Self::total(nums.len() as i32);
		for i in nums {
			let mut diff = vec![];
			for (&j, &c) in &cnt {
				if i + j < k {
					diff.push((i + j, c));
				}
			}
			for (n, c) in diff {
				let e = cnt.entry(n).or_insert(0);
				*e = (*e + c) % MO;
			}
			if i < k {
				*cnt.entry(i).or_insert(0) += 1;
			}
		}
		let lt = cnt
			.into_iter()
			.map(|(i, c)| {
				if s - i as i64 >= k as i64 {
					(c * 2) % MO
				} else {
					c
				}
			})
			.fold(0, |acc, i| (acc + i) % MO);
		(tot + MO - lt - 1 - (s >= k as i64) as i32) % MO
	}

	fn total(n: i32) -> i32 {
		let mut ans = 1;
		for _ in 0..n {
			ans = (ans << 1) % MO;
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2518() {
		assert_eq!(Solution::count_partitions(vec![1, 2, 4, 2, 1, 5], 817), 0);
		assert_eq!(Solution::count_partitions(vec![1, 2, 3, 4], 4), 6);
		assert_eq!(Solution::count_partitions(vec![3, 3, 3], 4), 0);
		assert_eq!(Solution::count_partitions(vec![6, 6], 4), 2);
	}
}
