/**
 * [1626] Best Team With No Conflicts
 *
 * You are the manager of a basketball team. For the upcoming tournament, you want to choose the team with the highest overall score. The score of the team is the sum of scores of all the players in the team.
 * However, the basketball team is not allowed to have conflicts. A conflict exists if a younger player has a strictly higher score than an older player. A conflict does not occur between players of the same age.
 * Given two lists, scores and ages, where each scores[i] and ages[i] represents the score and age of the i^th player, respectively, return the highest overall score of all possible basketball teams.
 *  
 * <strong class="example">Example 1:
 *
 * Input: scores = [1,3,5,10,15], ages = [1,2,3,4,5]
 * Output: 34
 * Explanation: You can choose all the players.
 *
 * <strong class="example">Example 2:
 *
 * Input: scores = [4,5,6,5], ages = [2,1,2,1]
 * Output: 16
 * Explanation: It is best to choose the last 3 players. Notice that you are allowed to choose multiple people of the same age.
 *
 * <strong class="example">Example 3:
 *
 * Input: scores = [1,2,3,5], ages = [8,9,10,1]
 * Output: 6
 * Explanation: It is best to choose the first 3 players.
 *
 *  
 * Constraints:
 *
 *     1 <= scores.length, ages.length <= 1000
 *     scores.length == ages.length
 *     1 <= scores[i] <= 10^6
 *     1 <= ages[i] <= 1000
 *
 */
pub struct Solution {}

// submission codes start here

use std::collections::HashMap;
impl Solution {
	pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
		let mut players = ages.into_iter().zip(scores).collect::<Vec<_>>();
		players.sort_unstable();

		Self::sovle2(&players, 0, &mut HashMap::new())
	}

	fn sovle2(players: &[(i32, i32)], lb: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
		if players.is_empty() {
			return 0;
		} else if let Some(&ans) = cache.get(&(players.len() as i32, lb)) {
			return ans;
		}
		let mut ans = 0;
		for (i, &(_, score)) in players.iter().enumerate() {
			if score >= lb {
				ans = ans.max(score + Self::sovle2(&players[i + 1..], score, cache));
			}
		}

		cache.insert((players.len() as i32, lb), ans);

		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_1626() {
		assert_eq!(
			Solution::best_team_score(vec![1, 2, 3, 5], vec![8, 9, 10, 1]),
			6
		);
		assert_eq!(
			Solution::best_team_score(vec![4, 5, 6, 5], vec![2, 1, 2, 1]),
			16
		);
		assert_eq!(
			Solution::best_team_score(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5]),
			34
		);
	}
}
