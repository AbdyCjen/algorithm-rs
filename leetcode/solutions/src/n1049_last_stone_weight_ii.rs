/**
 * [1049] Last Stone Weight II
 *
 * You are given an array of integers stones where stones[i] is the weight of the i^th stone.
 * We are playing a game with the stones. On each turn, we choose any two stones and smash them together. Suppose the stones have weights x and y with x <= y. The result of this smash is:
 *
 *     If x == y, both stones are destroyed, and
 *     If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
 *
 * At the end of the game, there is at most one stone left.
 * Return the smallest possible weight of the left stone. If there are no stones left, return 0.
 *  
 * Example 1:
 *
 * Input: stones = [2,7,4,1,8,1]
 * Output: 1
 * Explanation:
 * We can combine 2 and 4 to get 2, so the array converts to [2,7,1,8,1] then,
 * we can combine 7 and 8 to get 1, so the array converts to [2,1,1,1] then,
 * we can combine 2 and 1 to get 1, so the array converts to [1,1,1] then,
 * we can combine 1 and 1 to get 0, so the array converts to [1], then that's the optimal value.
 *
 * Example 2:
 *
 * Input: stones = [31,26,33,21,40]
 * Output: 5
 *
 * Example 3:
 *
 * Input: stones = [1,2]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     1 <= stones.length <= 30
 *     1 <= stones[i] <= 100
 *
 */
pub struct Solution {}

// submission codes start here

#[allow(dead_code)]
impl Solution {
	pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
		use std::collections::BTreeSet;
		let mut dp0 = BTreeSet::new();
		dp0.insert(stones[0]);

		for m in stones.into_iter().skip(1) {
			let mut dp1 = BTreeSet::new();
			for n in dp0 {
				dp1.insert((m - n).abs());
				dp1.insert(m + n);
			}
			dp0 = dp1;
		}
		dp0.into_iter().min().unwrap()
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1049() {
		assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
		assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
		assert_eq!(Solution::last_stone_weight_ii(vec![1, 1]), 0);
		assert_eq!(Solution::last_stone_weight_ii(vec![1, 2]), 1);
	}
}
