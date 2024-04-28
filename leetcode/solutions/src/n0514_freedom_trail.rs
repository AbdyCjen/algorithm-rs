/**
 * [514] Freedom Trail
 *
 * In the video game Fallout 4, the quest "Road to Freedom" requires players to reach a metal dial called the "Freedom Trail Ring" and use the dial to spell a specific keyword to open the door.
 * Given a string ring that represents the code engraved on the outer ring and another string key that represents the keyword that needs to be spelled, return the minimum number of steps to spell all the characters in the keyword.
 * Initially, the first character of the ring is aligned at the "12:00" direction. You should spell all the characters in key one by one by rotating ring clockwise or anticlockwise to make each character of the string key aligned at the "12:00" direction and then by pressing the center button.
 * At the stage of rotating the ring to spell the key character key[i]:
 * <ol>
 *     You can rotate the ring clockwise or anticlockwise by one place, which counts as one step. The final purpose of the rotation is to align one of ring's characters at the "12:00" direction, where this character must equal key[i].
 *     If the character key[i] has been aligned at the "12:00" direction, press the center button to spell, which also counts as one step. After the pressing, you could begin to spell the next character in the key (next stage). Otherwise, you have finished all the spelling.
 * </ol>
 *  
 * <strong class="example">Example 1:
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/ring.jpg" style="width: 450px; height: 450px;" />
 * Input: ring = "godding", key = "gd"
 * Output: 4
 * Explanation:
 * For the first key character 'g', since it is already in place, we just need 1 step to spell this character.
 * For the second key character 'd', we need to rotate the ring "godding" anticlockwise by two steps to make it become "ddinggo".
 * Also, we need 1 more step for spelling.
 * So the final output is 4.
 *
 * <strong class="example">Example 2:
 *
 * Input: ring = "godding", key = "godding"
 * Output: 13
 *
 *  
 * Constraints:
 *
 *     1 <= ring.length, key.length <= 100
 *     ring and key consist of only lower case English letters.
 *     It is guaranteed that key could always be spelled by rotating ring.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn find_rotate_steps(ring: String, key: String) -> i32 {
		let mut map: [Vec<_>; 26] = Default::default();
		for (i, c) in (0..).zip(ring.bytes()) {
			map[(c - b'a') as usize].push(i);
		}
		Self::solve(
			&map,
			key.as_bytes(),
			ring.len() as i32,
			0,
			&mut vec![vec![-1; ring.len() + 1]; key.len() + 1],
		)
	}

	fn solve(ring: &[Vec<i32>; 26], key: &[u8], rl: i32, i: i32, cache: &mut [Vec<i32>]) -> i32 {
		if cache[key.len() as usize][i as usize] != -1 {
			return cache[key.len() as usize][i as usize];
		}
		let ans = match key {
			[c, kr @ ..] => match ring[(*c - b'a') as usize].binary_search(&i) {
				Ok(_) => Self::solve(ring, kr, rl, i, cache) + 1,
				Err(j) => {
					let len = ring[(*c - b'a') as usize].len();
					let l = ring[(*c - b'a') as usize][(j + len - 1) % len];
					let ans = Self::solve(ring, kr, rl, l, cache) + (i - l).rem_euclid(rl) + 1;
					let r = ring[(*c - b'a') as usize][j % len];
					ans.min(Self::solve(ring, kr, rl, r, cache) + (r - i).rem_euclid(rl) + 1)
				}
			},
			[] => 0,
		};
		cache[key.len() as usize][i as usize] = ans;
		ans
	}
}

// submission codes end
