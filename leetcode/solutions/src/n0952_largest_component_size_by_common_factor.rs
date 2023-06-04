/**
 * [952] Largest Component Size by Common Factor
 *
 * You are given an integer array of unique positive integers nums. Consider the following graph:
 *
 *     There are nums.length nodes, labeled nums[0] to nums[nums.length - 1],
 *     There is an undirected edge between nums[i] and nums[j] if nums[i] and nums[j] share a common factor greater than 1.
 *
 * Return the size of the largest connected component in the graph.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/01/ex1.png" style="width: 500px; height: 97px;" />
 * Input: nums = [4,6,15,35]
 * Output: 4
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/01/ex2.png" style="width: 500px; height: 85px;" />
 * Input: nums = [20,50,9,63]
 * Output: 2
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/01/ex3.png" style="width: 500px; height: 260px;" />
 * Input: nums = [2,3,6,7,4,12,21,39]
 * Output: 8
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 2 * 10^4
 *     1 <= nums[i] <= 10^5
 *     All the values of nums are unique.
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	// 很难过:(, 一开始分解质因数的写法不对, 直接遍历质数求解tle走了很多弯路
	fn primes(n: i32) -> Vec<i32> {
		let mut ans = vec![2];
		for i in (3..n).step_by(2) {
			let lim = (i as f32).sqrt() as i32;
			if ans.iter().take_while(|p| **p <= lim).all(|pr| i % pr != 0) {
				ans.push(i);
			}
		}
		ans
	}
	fn find(set: &mut [i32], i: i32) -> i32 {
		if set[i as usize] != i {
			set[i as usize] = Self::find(set, set[i as usize]);
		}
		set[i as usize]
	}
	fn union(set: &mut [i32], i: i32, mut j: i32) {
		j = Self::find(set, j);
		set[j as usize] = Self::find(set, i);
	}

	pub fn largest_component_size(nums: Vec<i32>) -> i32 {
		let primes = Self::primes(1e5_f32.sqrt() as i32);
		let mut set = (0..(nums.iter().copied().max().unwrap() + 1)).collect::<Vec<_>>();
		let set = set.as_mut_slice();

		for &n in &nums {
			let mut nn = n;
			for p in primes.iter().copied().take_while(|&p| p * p <= n) {
				while nn % p == 0 {
					nn /= p;
					Self::union(set, n, p);
				}
			}
			if nn > 1 {
				Self::union(set, n, nn);
			}
		}
		let mut cnts = vec![0; set.len()];
		nums.into_iter()
			.map(|i| {
				let j = Self::find(set, i as _);
				cnts[j as usize] += 1;
				cnts[j as usize]
			})
			.max()
			.unwrap_or(0)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_952() {
		assert_eq!(Solution::largest_component_size(vec![4, 6, 15, 35]), 4);
		assert_eq!(Solution::largest_component_size(vec![20, 50, 9, 63]), 2);
		assert_eq!(
			Solution::largest_component_size(vec![83, 99, 39, 11, 19, 30, 31]),
			4
		);
		assert_eq!(
			Solution::largest_component_size(vec![
				2, 7, 522, 526, 535, 26, 944, 35, 519, 45, 48, 567, 266, 68, 74, 591, 81, 86, 602,
				93, 610, 621, 111, 114, 629, 641, 131, 651, 142, 659, 669, 161, 674, 163, 180, 187,
				190, 194, 195, 206, 207, 218, 737, 229, 240, 757, 770, 260, 778, 270, 272, 785,
				274, 290, 291, 292, 296, 810, 816, 314, 829, 833, 841, 349, 880, 369, 147, 897,
				387, 390, 905, 405, 406, 407, 414, 416, 417, 425, 938, 429, 432, 926, 959, 960,
				449, 963, 966, 929, 457, 463, 981, 985, 79, 487, 1000, 494, 508
			]),
			84
		);
		assert_eq!(
			Solution::largest_component_size(vec![2, 3, 6, 7, 4, 12, 21, 39]),
			8
		);
	}
}
