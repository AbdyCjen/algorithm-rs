/**
 * [805] Split Array With Same Average
 *
 * You are given an integer array nums.
 * You should move each element of nums into one of the two arrays A and B such that A and B are non-empty, and average(A) == average(B).
 * Return true if it is possible to achieve that and false otherwise.
 * Note that for an array arr, average(arr) is the sum of all the elements of arr over the length of arr.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,3,4,5,6,7,8]
 * Output: true
 * Explanation: We can split the array into [1,4,5,8] and [2,3,6,7], and both of them have an average of 4.5.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,1]
 * Output: false
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 30
 *     0 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn split_array_same_average(nums: Vec<i32>) -> bool {
		let sum = nums.iter().sum::<i32>();
		let len = nums.len() as i32;
		if len < 2 {
			return false;
		}
		use std::collections::*;
		let (mut lower, mut upper) = (vec![], vec![]);
		for i in nums {
			match i * len - sum {
				0 => return true,
				i @ 1.. => upper.push(i),
				i => lower.push(-i),
			}
		}
		let mut low_sum = HashSet::new();
		let low_s = lower.iter().sum::<i32>();
		for n in lower {
			low_sum.extend(low_sum.iter().map(|i| i + n).chain([n]).collect::<Vec<_>>());
		}
		low_sum.remove(&low_s);

		let mut upper_sum = HashSet::new();
		for n in upper {
			let new_sum = upper_sum
				.iter()
				.map(|i| i + n)
				.chain([n])
				.collect::<Vec<_>>();
			if new_sum.iter().any(|i| low_sum.contains(i)) {
				return true;
			}
			upper_sum.extend(new_sum);
		}
		false
	}

	pub fn split_array_same_average_01(nums: Vec<i32>) -> bool {
		use std::collections::{HashMap, HashSet};
		let n = nums.len();
		let sum = nums.iter().sum::<i32>();
		let (left, right) = nums.split_at(n / 2);
		let mut map: HashMap<i32, HashSet<usize>> = HashMap::new();
		// 枚举左半部分所有(total, count)组合
		for i in 0..(1 << left.len()) {
			let (mut tot, mut cnt) = (0, 0);
			for (j, cur) in left.iter().enumerate() {
				if (1 << j) & i > 0 {
					tot += cur;
					cnt += 1;
				}
			}
			map.entry(tot).or_default().insert(cnt);
		}

		// 遍历右半部分所有(total, count) 组合
		for i in 0..(1 << right.len()) {
			let (mut tot, mut cnt) = (0, 0);
			for (j, cur) in right.iter().enumerate() {
				if (1 << j) & i > 0 {
					tot += cur;
					cnt += 1;
				}
			}
			// 对于当前的(total, count)组合, 想办法从前半部分找到一组(total', count')满足(total + total') / (count + count') = sum / n
			// 假设k = count + count', 遍历所有k的可能取值
			for k in 1.max(cnt)..n {
				if k * sum as usize % n != 0 {
					continue;
				}
				let t = (k * sum as usize / n) as i32;
				if map
					.get(&(t - tot))
					.and_then(|cnts| cnts.get(&(k - cnt)))
					.is_some()
				{
					return true;
				}
			}
		}

		false
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_805() {
		assert!(Solution::split_array_same_average(vec![5, 3, 11, 19, 2]));
		assert!(Solution::split_array_same_average(vec![
			1, 2, 3, 4, 5, 6, 7, 8
		]));
		assert!(!Solution::split_array_same_average(vec![3, 1]));
		assert!(!Solution::split_array_same_average(vec![0]));
	}
}
