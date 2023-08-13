/**
 * [646] Maximum Length of Pair Chain
 *
 * You are given an array of n pairs pairs where pairs[i] = [lefti, righti] and lefti < righti.
 * A pair p2 = [c, d] follows a pair p1 = [a, b] if b < c. A chain of pairs can be formed in this fashion.
 * Return the length longest chain which can be formed.
 * You do not need to use up all the given intervals. You can select pairs in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: pairs = [[1,2],[2,3],[3,4]]
 * Output: 2
 * Explanation: The longest chain is [1,2] -> [3,4].
 *
 * <strong class="example">Example 2:
 *
 * Input: pairs = [[1,2],[7,8],[4,5]]
 * Output: 3
 * Explanation: The longest chain is [1,2] -> [4,5] -> [7,8].
 *
 *  
 * Constraints:
 *
 *     n == pairs.length
 *     1 <= n <= 1000
 *     -1000 <= lefti < righti <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
		pairs.sort_unstable_by_key(|v| v[1]);
		let mut prv = i32::MIN;
		let mut ans = 0;
		for v in pairs {
			if v[0] > prv {
				ans += 1;
				prv = v[1];
			}
		}
		ans
	}
}

// submission codes end
