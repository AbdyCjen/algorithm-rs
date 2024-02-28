/**
 * [909] Snakes and Ladders
 *
 * You are given an n x n integer matrix board where the cells are labeled from 1 to n^2 in a <a href="https://en.wikipedia.org/wiki/Boustrophedon" target="_blank">Boustrophedon style</a> starting from the bottom left of the board (i.e. board[n - 1][0]) and alternating direction each row.
 * You start on square 1 of the board. In each move, starting from square curr, do the following:
 *
 *     Choose a destination square next with a label in the range [curr + 1, min(curr + 6, n^2)].
 *     
 *         This choice simulates the result of a standard 6-sided die roll: i.e., there are always at most 6 destinations, regardless of the size of the board.
 *     
 *     
 *     If next has a snake or ladder, you must move to the destination of that snake or ladder. Otherwise, you move to next.
 *     The game ends when you reach the square n^2.
 *
 * A board square on row r and column c has a snake or ladder if board[r][c] != -1. The destination of that snake or ladder is board[r][c]. Squares 1 and n^2 do not have a snake or ladder.
 * Note that you only take a snake or ladder at most once per move. If the destination to a snake or ladder is the start of another snake or ladder, you do not follow the subsequent snake or ladder.
 *
 *     For example, suppose the board is [[-1,4],[-1,3]], and on the first move, your destination square is 2. You follow the ladder to square 3, but do not follow the subsequent ladder to 4.
 *
 * Return the least number of moves required to reach the square n^2. If it is not possible to reach the square, return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/09/23/snakes.png" style="width: 500px; height: 394px;" />
 * Input: board = [[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]]
 * Output: 4
 * Explanation:
 * In the beginning, you start at square 1 (at row 5, column 0).
 * You decide to move to square 2 and must take the ladder to square 15.
 * You then decide to move to square 17 and must take the snake to square 13.
 * You then decide to move to square 14 and must take the ladder to square 35.
 * You then decide to move to square 36, ending the game.
 * This is the lowest possible number of moves to reach the last square, so return 4.
 *
 * <strong class="example">Example 2:
 *
 * Input: board = [[-1,-1],[-1,3]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 *     n == board.length == board[i].length
 *     2 <= n <= 20
 *     grid[i][j] is either -1 or in the range [1, n^2].
 *     The squares labeled 1 and n^2 do not have any ladders or snakes.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn snakes_and_ladders(bd: Vec<Vec<i32>>) -> i32 {
		let n = bd.len();
		let mut board = vec![-1];
		let size = n * n;
		for (r, i) in bd.into_iter().rev().zip(0..) {
			if i % 2 == 0 {
				board.extend(r.into_iter());
			} else {
				board.extend(r.into_iter().rev());
			}
		}
		let mut visited = vec![false; size + 1];
		let mut step = 0;
		let mut st = vec![1];
		while !st.is_empty() {
			for cur in std::mem::take(&mut st) {
				if cur == size {
					return step;
				}
				if !visited[cur] {
					visited[cur] = true;
					for (next, &b) in (cur + 1..).zip(&board[cur + 1..]).take(6) {
						if b > 0 {
							st.push(b as usize);
						} else {
							st.push(next);
						}
					}
				}
			}
			step += 1;
		}
		-1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_909() {
		assert_eq!(
			Solution::snakes_and_ladders(matrix![
				[-1, -1, -1, -1, 48, 5, -1],
				[12, 29, 13, 9, -1, 2, 32],
				[-1, -1, 21, 7, -1, 12, 49],
				[42, 37, 21, 40, -1, 22, 12],
				[42, -1, 2, -1, -1, -1, 6],
				[39, -1, 35, -1, -1, 39, -1],
				[-1, 36, -1, -1, -1, -1, 5]
			]),
			3
		);
		assert_eq!(Solution::snakes_and_ladders(matrix![[-1, -1], [-1, 3]]), 1);
		assert_eq!(
			Solution::snakes_and_ladders(matrix![
				[-1, -1, -1, -1, -1, -1],
				[-1, -1, -1, -1, -1, -1],
				[-1, -1, -1, -1, -1, -1],
				[-1, 35, -1, -1, 13, -1],
				[-1, -1, -1, -1, -1, -1],
				[-1, 15, -1, -1, -1, -1]
			]),
			4
		);
	}
}
