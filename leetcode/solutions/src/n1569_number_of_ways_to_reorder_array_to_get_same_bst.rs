/**
 * [1569] Number of Ways to Reorder Array to Get Same BST
 *
 * Given an array nums that represents a permutation of integers from 1 to n. We are going to construct a binary search tree (BST) by inserting the elements of nums in order into an initially empty BST. Find the number of different ways to reorder nums so that the constructed BST is identical to that formed from the original array nums.
 *
 *     For example, given nums = [2,1,3], we will have 2 as the root, 1 as a left child, and 3 as a right child. The array [2,3,1] also yields the same BST but [3,2,1] yields a different BST.
 *
 * Return the number of ways to reorder nums such that the BST formed is identical to the original BST formed from nums.
 * Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/12/bb.png" style="width: 121px; height: 101px;" />
 * Input: nums = [2,1,3]
 * Output: 1
 * Explanation: We can reorder nums to be [2,3,1] which will yield the same BST. There are no other ways to reorder nums which will yield the same BST.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/12/ex1.png" style="width: 241px; height: 161px;" />
 * Input: nums = [3,4,5,1,2]
 * Output: 5
 * Explanation: The following 5 arrays will yield the same BST:
 * [3,1,2,4,5]
 * [3,1,4,2,5]
 * [3,1,4,5,2]
 * [3,4,1,2,5]
 * [3,4,1,5,2]
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/12/ex4.png" style="width: 121px; height: 161px;" />
 * Input: nums = [1,2,3]
 * Output: 0
 * Explanation: There are no other orderings of nums that will yield the same BST.
 *
 *  
 * Constraints:
 *
 *     1 <= nums.length <= 1000
 *     1 <= nums[i] <= nums.length
 *     All integers in nums are distinct.
 *
 */
pub struct Solution {}

// submission codes start here

struct Tree {
	val: i32,
	cnt: i32,
	ch: [Option<Box<Self>>; 2],
}

impl Tree {
	fn new(val: i32) -> Self {
		Self {
			val,
			cnt: 1,
			ch: [None, None],
		}
	}

	fn insert(&mut self, val: i32) {
		self.cnt += 1;
		let ch = if self.val > val {
			&mut self.ch[0]
		} else {
			&mut self.ch[1]
		};
		match ch {
			Some(ch) => ch.insert(val),
			ch => *ch = Some(Box::new(Self::new(val))),
		}
	}
	fn solve(&self, table: &[Vec<i32>]) -> i64 {
		const MO: i64 = 1e9 as i64 + 7;
		match &self.ch {
			[Some(l), Some(r)] => {
				let mul = l.solve(table) * r.solve(table) % MO;
				mul * table[(l.cnt + r.cnt) as usize][l.cnt as usize] as i64 % MO
			}
			[Some(ch), _] | [_, Some(ch)] => ch.solve(table),
			_ => 1,
		}
	}
}

impl Solution {
	pub fn num_of_ways(mut nums: Vec<i32>) -> i32 {
		let mut table = vec![vec![1]];
		const MO: i32 = 1e9 as i32 + 7;
		for i in 1..nums.len() {
			table.push(vec![1; i + 1]);
			for j in 1..i {
				table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % MO;
			}
		}
		Self::solve(&mut nums, &table) as i32 - 1
	}
	fn solve(nums: &mut [i32], table: &[Vec<i32>]) -> i64 {
		match nums {
			[n, nums @ ..] => {
				let (mut i, mut r) = (0, vec![]);
				for j in 0..nums.len() {
					if nums[j] > *n {
						r.push(nums[j]);
					} else {
						nums[i] = nums[j];
						i += 1;
					}
				}
				let l = &mut nums[..i];
				const MO: i64 = 1e9 as i64 + 7;
				let ans = Self::solve(l, table) * Self::solve(&mut r, table) % MO;
				(ans * table[l.len() + r.len()][l.len()] as i64) % MO
			}
			_ => 1,
		}
	}
	pub fn num_of_ways1(nums: Vec<i32>) -> i32 {
		let mut tr = Tree::new(nums[0]);
		for &n in &nums[1..] {
			tr.insert(n);
		}
		const MO: i32 = 1e9 as i32 + 7;
		let mut table = vec![vec![1]];
		for i in 1..nums.len() {
			table.push(vec![1; i + 1]);
			for j in 1..i {
				table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % MO;
			}
		}
		tr.solve(&table) as i32 - 1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1569() {
		assert_eq!(Solution::num_of_ways(vec![6, 1, 3, 2, 4, 5]), 2);
		assert_eq!(
			Solution::num_of_ways(vec![6, 9, 1, 3, 2, 7, 8, 4, 5, 10]),
			1133
		);
	}
}
