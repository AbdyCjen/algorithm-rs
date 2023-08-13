/**
 * [2916] Subarrays Distinct Element Sum of Squares II
 *
 * You are given a 0-indexed integer array nums.
 * The distinct count of a subarray of nums is defined as:
 *
 *     Let nums[i..j] be a subarray of nums consisting of all the indices from i to j such that 0 <= i <= j < nums.length. Then the number of distinct values in nums[i..j] is called the distinct count of nums[i..j].
 *
 * Return the sum of the squares of distinct counts of all subarrays of nums.
 * Since the answer may be very large, return it modulo 10^9 + 7.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,1]
 * Output: 15
 * Explanation: Six possible subarrays are:
 * [1]: 1 distinct value
 * [2]: 1 distinct value
 * [1]: 1 distinct value
 * [1,2]: 2 distinct values
 * [2,1]: 2 distinct values
 * [1,2,1]: 2 distinct values
 * The sum of the squares of the distinct counts in all subarrays is equal to 1^2 + 1^2 + 1^2 + 2^2 + 2^2 + 2^2 = 15.
 *
 * Example 2:
 *
 * Input: nums = [2,2]
 * Output: 3
 * Explanation: Three possible subarrays are:
 * [2]: 1 distinct value
 * [2]: 1 distinct value
 * [2,2]: 1 distinct value
 * The sum of the squares of the distinct counts in all subarrays is equal to 1^2 + 1^2 + 1^2 = 3.
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

struct Tree {
	nodes: Vec<(i64, i32)>,
	len: i32,
}

impl Tree {
	fn new(len: i32) -> Self {
		Self {
			nodes: vec![(0, 0); len as usize * 4],
			len,
		}
	}
	fn add_and_fetch(&mut self, l: i32, r: i32) -> i64 { self.add1(l, r, 1, self.len, 0, 1) }
	fn add1(&mut self, l: i32, r: i32, s: i32, t: i32, mut dif: i64, p: usize) -> i64 {
		// included by current node
		if l <= s && t <= r {
			let a = self.nodes[p].0 + (t - s + 1) as i64 * dif;
			self.nodes[p].0 += (t - s + 1) as i64;
			self.nodes[p].1 += 1;
			return a;
		}
		let m = (s + t) / 2;
		dif += self.nodes[p].1 as i64;
		let mut sum = 0;
		if l <= m {
			sum = self.add1(l, r, s, m, dif, p * 2);
		}
		if r > m {
			sum += self.add1(l, r, m + 1, t, dif, p * 2 + 1);
		}
		self.nodes[p].0 = (self.nodes[p * 2].0 + self.nodes[p * 2 + 1].0)
			+ (self.nodes[p].1 * (t - s + 1)) as i64;
		return sum;
	}
}

impl Solution {
	pub fn sum_counts(nums: Vec<i32>) -> i32 {
		const MO: i64 = 1e9 as i64 + 7;
		let mut prv = std::collections::HashMap::new();
		let mut ans = 0_i64;
		let mut tr = Tree::new(nums.len() as i32);
		let mut cur = 0_i64; //(dis_cnt(0, i) ^ 2 + dis_cnt(1, i) ^ 2 ..dis_cnt(i, i) ^ 2

		// start from 1 to fit segment tree
		for (i, n) in (1..).zip(&nums) {
			let pr = prv.entry(n).or_insert(0);
			// only dis_cnt(pr + 1, i), dis_cnt(pr + 2, i) .. dis_cnt(i, i)
			// will differ from dis_cnt(pr + 1, i - 1), dis_cnt(pr + 2, i - 1) .. dis_cnt(i, i - 1)by 1
			cur += 2 * tr.add_and_fetch(*pr + 1, i) + (i - *pr) as i64;
			ans = (cur + ans) % MO;
			*pr = i;
		}
		ans as i32
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2916() {
		assert_eq!(
			Solution::sum_counts(vec![5, 2, 4, 2, 1, 3, 2, 4, 3, 1]),
			578
		);
		assert_eq!(Solution::sum_counts(vec![2, 2, 5, 5]), 22);
		assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
	}
}
