/**
 * [2225] Find Players With Zero or One Losses
 *
 * You are given an integer array matches where matches[i] = [winneri, loseri] indicates that the player winneri defeated player loseri in a match.
 * Return a list answer of size 2 where:
 *
 *     answer[0] is a list of all players that have not lost any matches.
 *     answer[1] is a list of all players that have lost exactly one match.
 *
 * The values in the two lists should be returned in increasing order.
 * Note:
 *
 *     You should only consider the players that have played at least one match.
 *     The testcases will be generated such that no two matches will have the same outcome.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: matches = [[1,3],[2,3],[3,6],[5,6],[5,7],[4,5],[4,8],[4,9],[10,4],[10,9]]
 * Output: [[1,2,10],[4,5,7,8]]
 * Explanation:
 * Players 1, 2, and 10 have not lost any matches.
 * Players 4, 5, 7, and 8 each have lost one match.
 * Players 3, 6, and 9 each have lost two matches.
 * Thus, answer[0] = [1,2,10] and answer[1] = [4,5,7,8].
 *
 * <strong class="example">Example 2:
 *
 * Input: matches = [[2,3],[1,3],[5,4],[6,4]]
 * Output: [[1,2,5,6],[]]
 * Explanation:
 * Players 1, 2, 5, and 6 have not lost any matches.
 * Players 3 and 4 each have lost two matches.
 * Thus, answer[0] = [1,2,5,6] and answer[1] = [].
 *
 *  
 * Constraints:
 *
 *     1 <= matches.length <= 10^5
 *     matches[i].length == 2
 *     1 <= winneri, loseri <= 10^5
 *     winneri != loseri
 *     All matches[i] are unique.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let mut cnt = std::collections::BTreeMap::new();
		for mat in matches {
			cnt.entry(mat[0]).or_insert(0);
			*cnt.entry(mat[1]).or_insert(0) += 1;
		}
		let (mut all_win, mut one_lose) = (vec![], vec![]);
		for (i, c) in cnt {
			match c {
				0 => all_win.push(i),
				1 => one_lose.push(i),
				_ => {}
			}
		}
		vec![all_win, one_lose]
	}
}

// submission codes end
