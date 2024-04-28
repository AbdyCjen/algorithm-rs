/**
 * [752] Open the Lock
 *
 * You have a lock in front of you with 4 circular wheels. Each wheel has 10 slots: '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'. The wheels can rotate freely and wrap around: for example we can turn '9' to be '0', or '0' to be '9'. Each move consists of turning one wheel one slot.
 * The lock initially starts at '0000', a string representing the state of the 4 wheels.
 * You are given a list of deadends dead ends, meaning if the lock displays any of these codes, the wheels of the lock will stop turning and you will be unable to open it.
 * Given a target representing the value of the wheels that will unlock the lock, return the minimum total number of turns required to open the lock, or -1 if it is impossible.
 *  
 * <strong class="example">Example 1:
 *
 * Input: deadends = ["0201","0101","0102","1212","2002"], target = "0202"
 * Output: 6
 * Explanation:
 * A sequence of valid moves would be "0000" -> "1000" -> "1100" -> "1200" -> "1201" -> "1202" -> "0202".
 * Note that a sequence like "0000" -> "0001" -> "0002" -> "0102" -> "0202" would be invalid,
 * because the wheels of the lock become stuck after the display becomes the dead end "0102".
 *
 * <strong class="example">Example 2:
 *
 * Input: deadends = ["8888"], target = "0009"
 * Output: 1
 * Explanation: We can turn the last wheel in reverse to move from "0000" -> "0009".
 *
 * <strong class="example">Example 3:
 *
 * Input: deadends = ["8887","8889","8878","8898","8788","8988","7888","9888"], target = "8888"
 * Output: -1
 * Explanation: We cannot reach the target without getting stuck.
 *
 *  
 * Constraints:
 *
 *     1 <= deadends.length <= 500
 *     deadends[i].length == 4
 *     target.length == 4
 *     target will not be in the list deadends.
 *     target and deadends[i] consist of digits only.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
		let tar = target.parse::<i32>().unwrap();
		let mut vis = [false; 10000];
		for dead in deadends {
			vis[dead.parse::<usize>().unwrap()] = true;
		}
		let mut push = |i: i32, st: &mut Vec<i32>| {
			if !vis[i as usize] {
				vis[i as usize] = true;
				st.push(i);
			}
		};
		let mut dep = 0;
		let mut st = vec![];
		push(0, &mut st);
		while !st.is_empty() {
			for n in std::mem::take(&mut st) {
				if n == tar {
					return dep;
				}
				for mul in [1, 10, 100, 1000] {
					match (n / mul) % 10 {
						0 => {
							push(n + mul, &mut st);
							push(n + mul * 9, &mut st);
						}
						9 => {
							push(n - mul, &mut st);
							push(n - mul * 9, &mut st);
						}
						_ => {
							push(n + mul, &mut st);
							push(n - mul, &mut st);
						}
					}
				}
			}
			dep += 1;
		}
		-1
	}
}

// submission codes end

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_752() {
		assert_eq!(
			Solution::open_lock(vec_string!["0000"], "8888".to_owned()),
			-1
		);
		assert_eq!(
			Solution::open_lock(
				vec_string!["0201", "0101", "0102", "1212", "2002"],
				"0202".to_owned()
			),
			6
		);
		assert_eq!(
			Solution::open_lock(vec_string!["8888"], "0009".to_owned()),
			1
		);
		assert_eq!(
			Solution::open_lock(
				vec_string!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"],
				"8888".to_owned()
			),
			-1
		);
	}
}
