/**
 * [947] Most Stones Removed with Same Row or Column
 *
 * On a 2D plane, we place n stones at some integer coordinate points. Each coordinate point may have at most one stone.
 * A stone can be removed if it shares either the same row or the same column as another stone that has not been removed.
 * Given an array stones of length n where stones[i] = [xi, yi] represents the location of the i^th stone, return the largest possible number of stones that can be removed.
 *  
 * <strong class="example">Example 1:
 *
 * Input: stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]
 * Output: 5
 * Explanation: One way to remove 5 stones is as follows:
 * 1. Remove stone [2,2] because it shares the same row as [2,1].
 * 2. Remove stone [2,1] because it shares the same column as [0,1].
 * 3. Remove stone [1,2] because it shares the same row as [1,0].
 * 4. Remove stone [1,0] because it shares the same column as [0,0].
 * 5. Remove stone [0,1] because it shares the same row as [0,0].
 * Stone [0,0] cannot be removed since it does not share a row/column with another stone still on the plane.
 *
 * <strong class="example">Example 2:
 *
 * Input: stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]
 * Output: 3
 * Explanation: One way to make 3 moves is as follows:
 * 1. Remove stone [2,2] because it shares the same row as [&2,0].
 * 2. Remove stone [2,0] because it shares the same column as [0,0].
 * 3. Remove stone [0,2] because it shares the same row as [0,0].
 * Stones [0,0] and [1,1] cannot be removed since they do not share a row/column with another stone still on the plane.
 *
 * <strong class="example">Example 3:
 *
 * Input: stones = [[0,0]]
 * Output: 0
 * Explanation: [0,0] is the only stone on the plane, so you cannot remove it.
 *
 *  
 * Constraints:
 *
 *     1 <= stones.length <= 1000
 *     0 <= xi, yi <= 10^4
 *     No two stones are at the same coordinate point.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
		let mut col_row = std::collections::HashMap::new();
		let n = stones.len();

		for (i, stone) in stones.into_iter().enumerate() {
			for k in stone.into_iter().enumerate() {
				col_row.entry(k).or_insert_with(Vec::new).push(i as i32);
			}
		}

		let mut set: Vec<_> = (0..n as i32).collect();
		for col in col_row.into_values() {
			let root = col[0];
			for i in col.into_iter().skip(1) {
				Self::union(&mut set, i, root);
			}
		}

		let mut ans = n as i32;
		for (i, j) in set.into_iter().enumerate() {
			if i as i32 == j {
				ans -= 1;
			}
		}
		ans
	}

	fn union(set: &mut [i32], a: i32, b: i32) {
		let a = Self::find(set, a);
		set[a as usize] = Self::find(set, b);
	}

	fn find(set: &mut [i32], a: i32) -> i32 {
		if set[a as usize] != a {
			set[a as usize] = Self::find(set, set[a as usize]);
		}
		set[a as usize]
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_947() {
		assert_eq!(
			Solution::remove_stones(matrix![[3, 2], [3, 1], [4, 4], [1, 1], [0, 2], [4, 0]]),
			4
		);
		assert_eq!(Solution::remove_stones(matrix![[0, 1], [1, 0], [1, 1]]), 2);
		assert_eq!(
			Solution::remove_stones(matrix![[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]]),
			5
		);
		assert_eq!(
			Solution::remove_stones(matrix![[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]]),
			3
		);
		assert_eq!(Solution::remove_stones(matrix![[0, 0]]), 0);
	}
}
