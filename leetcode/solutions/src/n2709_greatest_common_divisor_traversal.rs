/**
 * [2709] Greatest Common Divisor Traversal
 *
 * You are given a 0-indexed integer array nums, and you are allowed to traverse between its indices. You can traverse between index i and index j, i != j, if and only if gcd(nums[i], nums[j]) > 1, where gcd is the greatest common divisor.
 * Your task is to determine if for every pair of indices i and j in nums, where i < j, there exists a sequence of traversals that can take us from i to j.
 * Return true if it is possible to traverse between all such pairs of indices, or false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [2,3,6]
 * Output: true
 * Explanation: In this example, there are 3 possible pairs of indices: (0, 1), (0, 2), and (1, 2).
 * To go from index 0 to index 1, we can use the sequence of traversals 0 -> 2 -> 1, where we move from index 0 to index 2 because gcd(nums[0], nums[2]) = gcd(2, 6) = 2 > 1, and then move from index 2 to index 1 because gcd(nums[2], nums[1]) = gcd(6, 3) = 3 > 1.
 * To go from index 0 to index 2, we can just go directly because gcd(nums[0], nums[2]) = gcd(2, 6) = 2 > 1. Likewise, to go from index 1 to index 2, we can just go directly because gcd(nums[1], nums[2]) = gcd(3, 6) = 3 > 1.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [3,9,5]
 * Output: false
 * Explanation: No sequence of traversals can take us from index 0 to index 2 in this example. So, we return false.
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [4,3,12,8]
 * Output: true
 * Explanation: There are 6 possible pairs of indices to traverse between: (0, 1), (0, 2), (0, 3), (1, 2), (1, 3), and (2, 3). A valid sequence of traversals exists for each pair, so we return true.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 10^5
 *     1 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
		fn find(set: &mut [i32], x: i32) -> i32 {
			if set[x as usize] != x {
				set[x as usize] = find(set, set[x as usize]);
			}
			set[x as usize]
		}
		fn union(set: &mut [i32], a: i32, b: i32) {
			let a = find(set, a);
			set[a as usize] = find(set, b);
		}
		if nums.len() == 1 {
			return true;
		} else if nums.contains(&1) {
			return false;
		}
		// primes in 0..1e5.sqrt()
		const PRIMES: &[i32] = &[
			2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
			89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
			181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
			277, 281, 283, 293, 307, 311, 313, 317,
		];
		let max = nums.iter().copied().max().unwrap();
		let mut set: Vec<_> = (0..=max).collect();
		for &i in &nums {
			let mut ii = i;
			for &pr in PRIMES.iter().take_while(move |&&pr| pr * pr <= i) {
				if ii % pr == 0 {
					union(&mut set, ii, pr);
					while ii % pr == 0 {
						ii /= pr;
					}
				}
			}
			if ii > 1 {
				union(&mut set, ii, i);
			}
		}
		nums.iter()
			.all(|&i| find(&mut set, i) == find(&mut set, nums[0]))
	}
}

// submission codes end
