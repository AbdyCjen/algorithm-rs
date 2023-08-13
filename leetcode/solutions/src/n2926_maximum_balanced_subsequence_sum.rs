/**
 * [2926] Maximum Balanced Subsequence Sum
 *
 * You are given a 0-indexed integer array nums.
 * A subsequence of nums having length k and consisting of indices i0 < i1 < ... < ik-1 is balanced if the following holds:
 *
 *     nums[ij] - nums[ij-1] >= ij - ij-1, for every j in the range [1, k - 1].
 *
 * A subsequence of nums having length 1 is considered balanced.
 * Return an integer denoting the maximum possible sum of elements in a balanced subsequence of nums.
 * A subsequence of an array is a new non-empty array that is formed from the original array by deleting some (possibly none) of the elements without disturbing the relative positions of the remaining elements.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [3,3,5,6]
 * Output: 14
 * Explanation: In this example, the subsequence [3,5,6] consisting of indices 0, 2, and 3 can be selected.
 * nums[2] - nums[0] >= 2 - 0.
 * nums[3] - nums[2] >= 3 - 2.
 * Hence, it is a balanced subsequence, and its sum is the maximum among the balanced subsequences of nums.
 * The subsequence consisting of indices 1, 2, and 3 is also valid.
 * It can be shown that it is not possible to get a balanced subsequence with a sum greater than 14.
 * <strong class="example">Example 2:
 *
 * Input: nums = [5,-1,-3,8]
 * Output: 13
 * Explanation: In this example, the subsequence [5,8] consisting of indices 0 and 3 can be selected.
 * nums[3] - nums[0] >= 3 - 0.
 * Hence, it is a balanced subsequence, and its sum is the maximum among the balanced subsequences of nums.
 * It can be shown that it is not possible to get a balanced subsequence with a sum greater than 13.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [-2,-1]
 * Output: -1
 * Explanation: In this example, the subsequence [-1] can be selected.
 * It is a balanced subsequence, and its sum is the maximum among the balanced subsequences of nums.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     -10^9 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// submission codes start here

struct Tree {
	val: i64,
	set: i64,
	ch: [Option<Box<Self>>; 2],
}

impl Tree {
	fn new(val: i64) -> Self {
		Self {
			val,
			set: i64::MIN,
			ch: [None, None],
		}
	}
	fn get_max(&self, l: i32, r: i32, s: i32, t: i32) -> i64 {
		if l <= s && t <= r {
			return self.val;
		}

		let m = s + (t - s) / 2;
		let mut a = self.set;
		if l <= m {
			if let Some(ch) = self.ch[0].as_ref() {
				a = ch.get_max(l, r, s, m).max(a);
			}
		}
		if r > m {
			if let Some(ch) = self.ch[1].as_ref() {
				a = ch.get_max(l, r, m + 1, t).max(a);
			}
		}
		a
	}
	fn set_max(&mut self, i: i32, s: i32, t: i32, max: i64) {
		if s == t {
			self.set = self.set.max(max);
			self.val = self.val.max(max);
			return;
		}

		let m = s + (t - s) / 2;
		let set = self.set;
		if i <= m {
			self.ch[0]
				.get_or_insert_with(|| Box::new(Self::new(set)))
				.set_max(i, s, m, max);
		} else {
			self.ch[1]
				.get_or_insert_with(|| Box::new(Self::new(set)))
				.set_max(i, m + 1, t, max);
		}

		self.val = self.val.max(max);
	}
}

impl Solution {
	pub fn max_balanced_subsequence_sum(nums: Vec<i32>) -> i64 {
		let (s, t) = (0..)
			.zip(&nums)
			.map(|v| v.1 - v.0)
			.fold((i32::MAX, i32::MIN), |(l, r), n| (l.min(n), r.max(n)));
		let mut tr = Tree::new(i64::MIN);
		for (i, n) in (0..).zip(nums) {
			tr.set_max(n - i, s, t, tr.get_max(s, n - i, s, t).max(0) + n as i64);
		}

		tr.get_max(s, t, s, t)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_2926() {
		assert_eq!(
			Solution::max_balanced_subsequence_sum(vec![-5, -9, 1, 3, -5]),
			4
		);
		assert_eq!(Solution::max_balanced_subsequence_sum(vec![3, 3, 5, 6]), 14);
		assert_eq!(Solution::max_balanced_subsequence_sum(vec![-2, -1]), -1);
		assert_eq!(Solution::max_balanced_subsequence_sum(vec![2, 7]), 9);
		assert_eq!(
			Solution::max_balanced_subsequence_sum(vec![5, -1, -3, 8]),
			13
		);
	}
}
