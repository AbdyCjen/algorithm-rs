/**
 * [699] Falling Squares
 *
 * There are several squares being dropped onto the X-axis of a 2D plane.
 * You are given a 2D integer array positions where positions[i] = [lefti, sideLengthi] represents the i^th square with a side length of sideLengthi that is dropped with its left edge aligned with X-coordinate lefti.
 * Each square is dropped one at a time from a height above any landed squares. It then falls downward (negative Y direction) until it either lands on the top side of another square or on the X-axis. A square brushing the left/right side of another square does not count as landing on it. Once it lands, it freezes in place and cannot be moved.
 * After each square is dropped, you must record the height of the current tallest stack of squares.
 * Return an integer array ans where ans[i] represents the height described above after dropping the i^th square.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/fallingsq1-plane.jpg" style="width: 500px; height: 505px;" />
 * Input: positions = [[1,2],[2,3],[6,1]]
 * Output: [2,5,5]
 * Explanation:
 * After the first drop, the tallest stack is square 1 with a height of 2.
 * After the second drop, the tallest stack is squares 1 and 2 with a height of 5.
 * After the third drop, the tallest stack is still squares 1 and 2 with a height of 5.
 * Thus, we return an answer of [2, 5, 5].
 *
 * <strong class="example">Example 2:
 *
 * Input: positions = [[100,100],[200,100]]
 * Output: [100,100]
 * Explanation:
 * After the first drop, the tallest stack is square 1 with a height of 100.
 * After the second drop, the tallest stack is either square 1 or square 2, both with heights of 100.
 * Thus, we return an answer of [100, 100].
 * Note that square 2 only brushes the right side of square 1, which does not count as landing on it.
 *
 *  
 * Constraints:
 *
 *     1 <= positions.length <= 1000
 *     1 <= lefti <= 10^8
 *     1 <= sideLengthi <= 10^6
 *
 */
pub struct Solution {}

// submission codes start here

struct Tree {
	// val, set, ch
	nodes: Vec<(i32, i32, [usize; 2])>,
}

impl Tree {
	fn new(set: i32) -> Self {
		Self {
			nodes: vec![(set, set, [0, 0])],
		}
	}

	fn insert(&mut self, set: i32) -> usize {
		self.nodes.push((set, set, [0, 0]));
		self.nodes.len() - 1
	}
	fn set_max(&mut self, l: i32, r: i32, s: i32, t: i32, p: usize, max: i32) {
		self.nodes[p].0 = self.nodes[p].0.max(max);
		if l <= s && t <= r {
			self.nodes[p].1 = self.nodes[p].1.max(max);
			return;
		}
		let m = s + (t - s) / 2;
		let set = self.nodes[p].1;
		if l <= m {
			if 0 == self.nodes[p].2[0] {
				self.nodes[p].2[0] = self.insert(set);
			}
			self.set_max(l, r, s, m, self.nodes[p].2[0], max);
		}
		if r > m {
			if 0 == self.nodes[p].2[1] {
				self.nodes[p].2[1] = self.insert(set);
			}
			self.set_max(l, r, m + 1, t, self.nodes[p].2[1], max);
		}
	}

	fn get_max(&mut self, l: i32, r: i32, s: i32, t: i32, p: usize) -> i32 {
		if l <= s && t <= r {
			return self.nodes[p].0;
		}

		let m = s + (t - s) / 2;
		let mut max = self.nodes[p].1;
		if l <= m && self.nodes[p].2[0] > 0 {
			max = max.max(self.get_max(l, r, s, m, self.nodes[p].2[0]));
		}
		if r > m && self.nodes[p].2[1] > 0 {
			max = max.max(self.get_max(l, r, m + 1, t, self.nodes[p].2[1]));
		}
		max
	}
}

impl Solution {
	pub fn falling_squares(pos: Vec<Vec<i32>>) -> Vec<i32> {
		let mut tr = Tree::new(0);
		let (s, t) = pos.iter().fold((i32::MAX, i32::MIN), |(s, t), p| {
			(s.min(p[0]), t.max(p[0] + p[1]))
		});

		let mut ans = vec![];
		for p in pos {
			let (l, r) = (p[0], p[0] + p[1] - 1);
			let h = tr.get_max(l, r, s, t, 0);
			tr.set_max(l, r, s, t, 0, h + p[1]);
			ans.push(tr.get_max(s, t, s, t, 0));
		}
		ans
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_699() {
		assert_eq!(Solution::falling_squares(matrix![[6, 1], [2, 4]]), [1, 4]);
	}
}
