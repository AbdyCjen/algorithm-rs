/**
 * [207] Course Schedule
 *
 * There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
 *
 *     For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
 *
 * Return true if you can finish all courses. Otherwise, return false.
 *  
 * <strong class="example">Example 1:
 *
 * Input: numCourses = 2, prerequisites = [[1,0]]
 * Output: true
 * Explanation: There are a total of 2 courses to take.
 * To take course 1 you should have finished course 0. So it is possible.
 *
 * <strong class="example">Example 2:
 *
 * Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
 * Output: false
 * Explanation: There are a total of 2 courses to take.
 * To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.
 *
 *  
 * Constraints:
 *
 *     1 <= numCourses <= 2000
 *     0 <= prerequisites.length <= 5000
 *     prerequisites[i].length == 2
 *     0 <= ai, bi < numCourses
 *     All the pairs prerequisites[i] are unique.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
		let mut neigs = vec![(vec![], 0); num_courses as usize];
		for pre in prerequisites {
			neigs[pre[0] as usize].0.push(pre[1]);
			neigs[pre[1] as usize].1 += 1;
		}
		let mut st: Vec<i32> = neigs
			.iter()
			.zip(0..)
			.filter(|v| v.0 .1 == 0)
			.map(|(_, i)| i)
			.collect();
		while let Some(cur) = st.pop() {
			for nei in std::mem::take(&mut neigs[cur as usize].0) {
				neigs[nei as usize].1 -= 1;
				if neigs[nei as usize].1 == 0 {
					st.push(nei);
				}
			}
		}
		neigs.iter().all(|v| v.0.is_empty())
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_207() {
		assert!(Solution::can_finish(2, matrix![[1, 0]]));
		assert!(!Solution::can_finish(2, matrix![[0, 1], [1, 0]]));
		assert!(Solution::can_finish(3, matrix![[0, 1], [0, 2], [1, 2]]));
		assert!(!Solution::can_finish(
			4,
			matrix![[0, 1], [3, 1], [1, 3], [3, 2]]
		));
	}
}
