/**
 * [1140] Stone Game II
 *
 * Alice and Bob continue their games with piles of stones.  There are a number of piles arranged in a row, and each pile has a positive integer number of stones piles[i].  The objective of the game is to end with the most stones.
 * Alice and Bob take turns, with Alice starting first.  Initially, M = 1.
 * On each player's turn, that player can take all the stones in the first X remaining piles, where 1 <= X <= 2M.  Then, we set M = max(M, X).
 * The game continues until all the stones have been taken.
 * Assuming Alice and Bob play optimally, return the maximum number of stones Alice can get.
 *  
 * <strong class="example">Example 1:
 *
 * Input: piles = [2,7,9,4,4]
 * Output: 10
 * Explanation:  If Alice takes one pile at the beginning, Bob takes two piles, then Alice takes 2 piles again. Alice can get 2 + 4 + 4 = 10 piles in total. If Alice takes two piles at the beginning, then Bob can take all three piles left. In this case, Alice get 2 + 7 = 9 piles in total. So we return 10 since it's larger.
 *
 * <strong class="example">Example 2:
 *
 * Input: piles = [1,2,3,4,5,100]
 * Output: 104
 *
 *  
 * Constraints:
 *
 *     1 <= piles.length <= 100
 *     1 <= piles[i] <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
	pub fn stone_game_ii(mut sum: Vec<i32>) -> i32 {
		let n = sum.len();
		for i in (0..n - 1).rev() {
			sum[i] += sum[i + 1];
		}
		Self::solve(&sum, 1, &mut vec![vec![-1; n + 1]; n + 1])
	}
	fn solve(sum: &[i32], m: i32, cache: &mut [Vec<i32>]) -> i32 {
		if sum.is_empty() {
			return 0;
		} else if cache[m as usize][sum.len()] >= 0 {
			return cache[m as usize][sum.len()];
		}
		let mut ans = 0;
		for x in 0..sum.len().min(2 * m as usize) {
			ans = ans.max(sum[0] - Self::solve(&sum[x + 1..], m.max(1 + x as i32), cache));
		}
		cache[m as usize][sum.len()] = ans;
		ans
	}
	pub fn stone_game_ii_1(piles: Vec<i32>) -> i32 {
		Self::solve1(&piles, 0, 1, &mut HashMap::new())[0]
	}

	fn solve1(
		piles: &[i32],
		op: u8,
		m: i32,
		cache: &mut HashMap<(i32, u8, usize), [i32; 2]>,
	) -> [i32; 2] {
		if piles.is_empty() {
			return [0; 2];
		} else if let Some(&ans) = cache.get(&(m, op, piles.len())) {
			return ans;
		}
		let mut s = 0;
		let mut ans = [0; 2];
		for x in 0..(2 * m as usize).min(piles.len()) {
			s += piles[x];
			let mut sub = Self::solve1(&piles[x + 1..], op ^ 1, m.max(1 + x as i32), cache);
			sub[op as usize] += s;
			if sub[op as usize] > ans[op as usize] {
				ans = sub;
			}
		}
		cache.insert((m, op, piles.len()), ans);
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1140() {
		assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
		assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
	}
}
