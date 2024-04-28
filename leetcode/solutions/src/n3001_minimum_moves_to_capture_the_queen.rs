/**
 * [3001] Minimum Moves to Capture The Queen
 *
 * There is a 1-indexed 8 x 8 chessboard containing 3 pieces.
 * You are given 6 integers a, b, c, d, e, and f where:
 *
 *     (a, b) denotes the position of the white rook.
 *     (c, d) denotes the position of the white bishop.
 *     (e, f) denotes the position of the black queen.
 *
 * Given that you can only move the white pieces, return the minimum number of moves required to capture the black queen.
 * Note that:
 *
 *     Rooks can move any number of squares either vertically or horizontally, but cannot jump over other pieces.
 *     Bishops can move any number of squares diagonally, but cannot jump over other pieces.
 *     A rook or a bishop can capture the queen if it is located in a square that they can move to.
 *     The queen does not move.
 *
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/12/21/ex1.png" style="width: 600px; height: 600px; padding: 10px; background: #fff; border-radius: .5rem;" />
 * Input: a = 1, b = 1, c = 8, d = 8, e = 2, f = 3
 * Output: 2
 * Explanation: We can capture the black queen in two moves by moving the white rook to (1, 3) then to (2, 3).
 * It is impossible to capture the black queen in less than two moves since it is not being attacked by any of the pieces at the beginning.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/12/21/ex2.png" style="width: 600px; height: 600px;padding: 10px; background: #fff; border-radius: .5rem;" />
 * Input: a = 5, b = 3, c = 3, d = 4, e = 5, f = 2
 * Output: 1
 * Explanation: We can capture the black queen in a single move by doing one of the following:
 * - Move the white rook to (5, 2).
 * - Move the white bishop to (5, 2).
 *
 *  
 * Constraints:
 *
 *     1 <= a, b, c, d, e, f <= 8
 *     No two pieces are on the same square.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
		if (a == e || f == b) && !Self::between((a, b), (e, f), (c, d)) {
			return 1;
		}
		if (e - c == f - d || e - c == d - f) && !Self::between((c, d), (e, f), (a, b)) {
			return 1;
		}
		2
	}

	fn between(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> bool {
		(Self::dist(a, c) + Self::dist(b, c)) - Self::dist(a, b) < 0.001
	}

	fn dist(a: (i32, i32), b: (i32, i32)) -> f32 {
		let (h, v) = (a.0 - b.0, a.1 - b.1);
		((h * h + v * v) as f32).sqrt()
	}

	pub fn min_moves_to_capture_the_queen1(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
		if a == e && !(a == c && ((b..f).contains(&d) || (f..b).contains(&d))) {
			return 1;
		}
		if f == b && !(b == d && ((a..e).contains(&c) || (e..a).contains(&c))) {
			return 1;
		}
		if (e - c) == (f - d)
			&& !((a - c == b - d)
				&& ((0..f - d).contains(&(b - d)) || (f - d..=0).contains(&(b - d))))
		{
			return 1;
		}
		if (e - c) == (d - f)
			&& !((a - c == d - b)
				&& ((0..f - d).contains(&(b - d)) || (f - d..=0).contains(&(b - d))))
		{
			return 1;
		}
		2
	}
}

// submission codes end
