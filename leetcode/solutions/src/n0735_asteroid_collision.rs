/**
 * [735] Asteroid Collision
 *
 * We are given an array asteroids of integers representing asteroids in a row.
 * For each asteroid, the absolute value represents its size, and the sign represents its direction (positive meaning right, negative meaning left). Each asteroid moves at the same speed.
 * Find out the state of the asteroids after all collisions. If two asteroids meet, the smaller one will explode. If both are the same size, both will explode. Two asteroids moving in the same direction will never meet.
 *  
 * <strong class="example">Example 1:
 *
 * Input: asteroids = [5,10,-5]
 * Output: [5,10]
 * Explanation: The 10 and -5 collide resulting in 10. The 5 and 10 never collide.
 *
 * <strong class="example">Example 2:
 *
 * Input: asteroids = [8,-8]
 * Output: []
 * Explanation: The 8 and -8 collide exploding each other.
 *
 * <strong class="example">Example 3:
 *
 * Input: asteroids = [10,2,-5]
 * Output: [10]
 * Explanation: The 2 and -5 collide resulting in -5. The 10 and -5 collide resulting in 10.
 *
 *  
 * Constraints:
 *
 *     2 <= asteroids.length <= 10^4
 *     -1000 <= asteroids[i] <= 1000
 *     asteroids[i] != 0
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	#[allow(dropping_copy_types)]
	pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
		let mut st = vec![];
		for i in asteroids {
			match i {
				0.. => st.push(i),
				_ => loop {
					match &st[..] {
						[] | [.., i32::MIN..=0] => break st.push(i),
						[.., s] if *s > -i => break,
						[.., s] if *s == -i => break drop(st.pop()),
						[.., _] => drop(st.pop()),
					}
				},
			}
		}
		st
	}
}

// submission codes end
