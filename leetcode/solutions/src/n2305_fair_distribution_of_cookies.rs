/**
 * [2305] Fair Distribution of Cookies
 *
 * You are given an integer array cookies, where cookies[i] denotes the number of cookies in the i^th bag. You are also given an integer k that denotes the number of children to distribute all the bags of cookies to. All the cookies in the same bag must go to the same child and cannot be split up.
 * The unfairness of a distribution is defined as the maximum total cookies obtained by a single child in the distribution.
 * Return the minimum unfairness of all distributions.
 *  
 * <strong class="example">Example 1:
 *
 * Input: cookies = [8,15,10,20,8], k = 2
 * Output: 31
 * Explanation: One optimal distribution is [8,15,8] and [10,20]
 * - The 1^st child receives [8,15,8] which has a total of 8 + 15 + 8 = 31 cookies.
 * - The 2^nd child receives [10,20] which has a total of 10 + 20 = 30 cookies.
 * The unfairness of the distribution is max(31,30) = 31.
 * It can be shown that there is no distribution with an unfairness less than 31.
 *
 * <strong class="example">Example 2:
 *
 * Input: cookies = [6,1,3,2,2,4,1,2], k = 3
 * Output: 7
 * Explanation: One optimal distribution is [6,1], [3,2,2], and [4,1,2]
 * - The 1^st child receives [6,1] which has a total of 6 + 1 = 7 cookies.
 * - The 2^nd child receives [3,2,2] which has a total of 3 + 2 + 2 = 7 cookies.
 * - The 3^rd child receives [4,1,2] which has a total of 4 + 1 + 2 = 7 cookies.
 * The unfairness of the distribution is max(7,7,7) = 7.
 * It can be shown that there is no distribution with an unfairness less than 7.
 *
 *  
 * Constraints:
 *
 *     2 <= cookies.length <= 8
 *     1 <= cookies[i] <= 10^5
 *     2 <= k <= cookies.length
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
	pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
		let mut ans = i32::MAX;
		Self::solve(&cookies, &mut vec![0; k as usize], &mut ans, 0, k as usize);
		ans
	}
	fn solve(cookies: &[i32], dist: &mut [i32], ans: &mut i32, max: i32, empty: usize) {
		match cookies {
			_ if cookies.len() < empty => {}
			[cookies @ .., c] => {
				for i in 0..dist.len() {
					if dist[i] + c <= *ans {
						let empty = empty - (dist[i] == 0) as usize;
						dist[i] += c;
						Self::solve(cookies, dist, ans, max.max(dist[i]), empty);
						dist[i] -= c;
					}
				}
			}
			[] => *ans = (*ans).min(max),
		}
	}
	pub fn distribute_cookies1(cookies: Vec<i32>, k: i32) -> i32 {
		Self::solve1(&cookies, &mut vec![0; k as usize], i32::MIN)
	}
	fn solve1(cookies: &[i32], dist: &mut [i32], max: i32) -> i32 {
		match cookies {
			[c, cookies @ ..] => {
				let mut ans = i32::MAX;
				for i in 0..dist.len() {
					dist[i] += c;
					ans = ans.min(Self::solve1(cookies, dist, dist[i].max(max)));
					dist[i] -= c;
				}
				ans
			}
			[] => max,
		}
	}
}

// submission codes end
