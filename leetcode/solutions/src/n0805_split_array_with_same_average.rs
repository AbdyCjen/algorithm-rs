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
	// 按照大于还是小于平均值, 将数组分为两个部分
	// 枚举 两个部分元素对平均值的差的 绝对值组合 之和
	// 然后判断交集存在性
	pub fn split_array_same_average(nums: Vec<i32>) -> bool {
		use std::collections::BTreeSet;
		let sum = nums.iter().sum::<i32>();
		let n = nums.len();
		if n < 2 {
			return false;
		}
		let (mut neg, mut pos) = (vec![], vec![]);
		for i in nums {
			match (i * n as i32) - sum {
				0 => return true,
				i if i < 0 => neg.push(-i),
				i => pos.push(i),
			}
		}

		fn perm(nums: &[i32]) -> BTreeSet<i32> {
			let mut sums = BTreeSet::new();

			for &i in nums {
				let sums_old = sums.clone();
				sums.insert(i);
				for j in sums_old {
					sums.insert(i + j);
				}
			}

			// 去掉全集
			sums.remove(&nums.iter().sum::<i32>());
			sums
		}

		perm(&neg).intersection(&perm(&pos)).next().is_some()
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
