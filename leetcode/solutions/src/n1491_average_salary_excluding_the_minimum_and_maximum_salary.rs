/**
 * [1491] Average Salary Excluding the Minimum and Maximum Salary
 *
 * You are given an array of unique integers salary where salary[i] is the salary of the i^th employee.
 * Return the average salary of employees excluding the minimum and maximum salary. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * <strong class="example">Example 1:
 *
 * Input: salary = [4000,3000,1000,2000]
 * Output: 2500.00000
 * Explanation: Minimum salary and maximum salary are 1000 and 4000 respectively.
 * Average salary excluding minimum and maximum salary is (2000+3000) / 2 = 2500
 *
 * <strong class="example">Example 2:
 *
 * Input: salary = [1000,2000,3000]
 * Output: 2000.00000
 * Explanation: Minimum salary and maximum salary are 1000 and 3000 respectively.
 * Average salary excluding minimum and maximum salary is (2000) / 1 = 2000
 *
 *  
 * Constraints:
 *
 *     3 <= salary.length <= 100
 *     1000 <= salary[i] <= 10^6
 *     All the integers of salary are unique.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn average(salary: Vec<i32>) -> f64 {
		let n = (salary.len() - 2) as f64;
		let (min, max, sum) = salary
			.into_iter()
			.fold((i32::MAX, i32::MIN, 0), |(min, max, sum), v| {
				(min.min(v), max.max(v), sum + v)
			});
		(sum - max - min) as f64 / n
	}
}

// submission codes end
