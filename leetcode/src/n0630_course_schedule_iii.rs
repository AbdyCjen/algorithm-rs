/**
 * [630] Course Schedule III
 *
 * There are n different online courses numbered from 1 to n. You are given an array courses where courses[i] = [durationi, lastDayi] indicate that the i^th course should be taken continuously for durationi days and must be finished before or on lastDayi.
 * You will start on the 1^st day and you cannot take two or more courses simultaneously.
 * Return the maximum number of courses that you can take.
 *  
 * Example 1:
 *
 * Input: courses = [[100,200],[200,1300],[1000,1250],[2000,3200]]
 * Output: 3
 * Explanation:
 * There are totally 4 courses, but you can take 3 courses at most:
 * First, take the 1^st course, it costs 100 days so you will finish it on the 100^th day, and ready to take the next course on the 101^st day.
 * Second, take the 3^rd course, it costs 1000 days so you will finish it on the 1100^th day, and ready to take the next course on the 1101^st day.
 * Third, take the 2^nd course, it costs 200 days so you will finish it on the 1300^th day.
 * The 4^th course cannot be taken now, since you will finish it on the 3300^th day, which exceeds the closed date.
 *
 * Example 2:
 *
 * Input: courses = [[1,2]]
 * Output: 1
 *
 * Example 3:
 *
 * Input: courses = [[3,2],[4,3]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 *     1 <= courses.length <= 10^4
 *     1 <= durationi, lastDayi <= 10^4
 *
 */
pub struct Solution {}

// submission codes start here
// 思路参照题注提示
#[allow(dead_code)]
impl Solution {
	pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
		let mut courses: Vec<_> = courses.into_iter().map(|v| (v[0], v[1])).collect();
		courses.sort_unstable_by_key(|i| i.1);
		let mut selected_course = std::collections::BTreeMap::new();
		let mut cur_sum = 0;
		for course in courses {
			if course.0 + cur_sum <= course.1 {
				cur_sum += course.0;
				*selected_course.entry(course.0).or_insert(0) += 1;
			} else {
				let max_dur = *selected_course.iter().next_back().unwrap_or((&0, &0)).0;
				if max_dur > course.0 {
					cur_sum -= max_dur;
					cur_sum += course.0;
					*selected_course.entry(course.0).or_default() += 1;
					*selected_course.entry(max_dur).or_default() -= 1;
					if selected_course[&max_dur] == 0 {
						selected_course.remove(&max_dur);
					}
				}
			}
		}
		selected_course.iter().map(|(_, i)| i).sum()
	}
}

// submission codes end
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_630() {
		assert_eq!(
			Solution::schedule_course(vec![
				vec![100, 200],
				vec![200, 1300],
				vec![1000, 1250],
				vec![2000, 3200]
			]),
			3
		);
		assert_eq!(Solution::schedule_course(vec![vec![1, 2]]), 1);
		assert_eq!(Solution::schedule_course(vec![vec![3, 2], vec![4, 3]]), 0);
		assert_eq!(
			Solution::schedule_course(vec![vec![3, 3], vec![3, 6], vec![2, 7], vec![1, 7]]),
			3
		);
	}
}
