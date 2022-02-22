/**
 * [391] Perfect Rectangle
 *
 * Given an array rectangles where rectangles[i] = [xi, yi, ai, bi] represents an axis-aligned rectangle. The bottom-left point of the rectangle is (xi, yi) and the top-right point of it is (ai, bi).
 * Return true if all the rectangles together form an exact cover of a rectangular region.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perectrec1-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]]
 * Output: true
 * Explanation: All 5 rectangles together form an exact cover of a rectangular region.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perfectrec2-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]
 * Output: false
 * Explanation: Because there is a gap between the two rectangular regions.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perfectrec3-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[3,2,4,4]]
 * Output: false
 * Explanation: Because there is a gap in the top center.
 *
 * Example 4:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perfecrrec4-plane.jpg" style="width: 300px; height: 294px;" />
 * Input: rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]
 * Output: false
 * Explanation: Because two of the rectangles overlap with each other.
 *
 *  
 * Constraints:
 *
 *     1 <= rectangles.length <= 2 * 10^4
 *     rectangles[i].length == 4
 *     -10^5 <= xi, yi, ai, bi <= 10^5
 *
 */
pub struct Solution1 {}

// submission codes start here

// 完美矩形的充要条件
//	1. 所有矩形之间没有重叠
//	2. 所有矩形定位点，取最左下和最右上两个点A B(先横后纵，不要求绝对左下右上)
//		1. 所有矩形都在A B 限定的矩形内
//		2. 所有矩形面积之和等于A B 包裹的矩形面积
#[allow(dead_code)]
impl Solution1 {
	#[allow(non_snake_case)]
	pub fn is_rectangle_cover(rects: Vec<Vec<i32>>) -> bool {
		let mut rects = rects
			.into_iter()
			.map(|r| ((r[0], r[1]), (r[2], r[3])))
			.collect::<Vec<_>>();
		rects.sort_unstable();

		let A = rects.first().copied().unwrap().0;
		let B = rects.iter().map(|r| r.1).max().unwrap();

		let s_all: i64 = rects
			.iter()
			.map(|&r| ((r.1).0 - (r.0).0) as i64 * ((r.1).1 - (r.0).1) as i64)
			.sum();
		let s_m: i64 = (B.0 - A.0) as i64 * (B.1 - A.1) as i64;
		if s_all != s_m {
			return false;
		}

		let y_rng = A.1..=B.1;
		if !rects
			.iter()
			.all(move |r| y_rng.contains(&(r.0).1) && y_rng.contains(&(r.1).1))
		{
			return false;
		}

		for (i, r1) in rects.iter().enumerate() {
			for r2 in rects[(i + 1)..].iter().take_while(|r2| (r2.0).0 < (r1.1).0) {
				if (r1.0).1 < (r2.1).1 && (r1.1).1 > (r2.0).1 {
					return false;
				}
			}
		}
		true
	}
}

struct Solution {}

// 看到另一个题解, 完美矩形的充要条件如下:
// 1. 所有成员矩形的顶点两两重合,除了最后组成完美矩形的四角节点
// 2. 所有成员矩形的面积之和等于完美矩形的面积
#[allow(dead_code)]
impl Solution {
	pub fn is_rectangle_cover(rects: Vec<Vec<i32>>) -> bool {
		use std::collections::HashSet;
		let mut corners = HashSet::new();
		let mut area = 0;
		for r in rects {
			let p1 = (r[0], r[1]);
			let p2 = (r[0], r[3]);
			let p3 = (r[2], r[1]);
			let p4 = (r[2], r[3]);
			for p in &[p1, p2, p3, p4] {
				if !corners.insert(*p) {
					corners.remove(p);
				}
			}
			area += (r[2] - r[0]) * (r[3] - r[1]);
		}
		if corners.len() != 4 {
			return false;
		}

		let x0 = corners.iter().map(|r| r.0).min().unwrap();
		let x1 = corners.iter().map(|r| r.0).max().unwrap();
		let y0 = corners.iter().map(|r| r.1).min().unwrap();
		let y1 = corners.iter().map(|r| r.1).max().unwrap();

		area == (x1 - x0) * (y1 - y0)
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_391() {
		assert!(Solution::is_rectangle_cover(vec![
			vec![1, 1, 3, 3],
			vec![3, 1, 4, 2],
			vec![3, 2, 4, 4],
			vec![1, 3, 2, 4],
			vec![2, 3, 3, 4]
		]));
		assert!(!Solution::is_rectangle_cover(vec![
			vec![1, 1, 2, 3],
			vec![1, 3, 2, 4],
			vec![3, 1, 4, 2],
			vec![3, 2, 4, 4]
		]));
		assert!(!Solution::is_rectangle_cover(vec![
			vec![1, 1, 3, 3],
			vec![3, 1, 4, 2],
			vec![1, 3, 2, 4],
			vec![3, 2, 4, 4]
		]));
		assert!(!Solution::is_rectangle_cover(vec![
			vec![1, 1, 3, 3],
			vec![3, 1, 4, 2],
			vec![1, 3, 2, 4],
			vec![2, 2, 4, 4]
		]));
		//assert!(Solution1::overlap(&((0, 1), (3, 2)), &((1, 0), (2, 2))));
		assert!(!Solution::is_rectangle_cover(vec![
			vec![0, 0, 1, 1],
			vec![0, 1, 3, 2],
			vec![1, 0, 2, 2]
		]));
	}
}
