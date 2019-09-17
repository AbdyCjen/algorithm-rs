use rand::prelude::*;
use std::cmp::Ordering;

fn qsort<T>(iv: &mut [T])
where
    T: std::cmp::Ord + Copy,
{
    if iv.len() <= 1 {
        return;
    }
    let o = iv[0];
    let (mut i, mut j, mut k) = (0, 0, iv.len() - 1);
    while j <= k {
        match iv[j].cmp(&o) {
            Ordering::Less => {
                iv.swap(j, i);
                i += 1;
                j += 1;
            }
            Ordering::Greater => {
                iv.swap(j, k);
                k -= 1;
            }
            Ordering::Equal => j += 1,
        }
    }
    qsort(&mut iv[..i]);
    qsort(&mut iv[k + 1..]);
    return;
}

fn main() {
    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rand::thread_rng());
    qsort(&mut nums);
    for i in &nums {
        print!("{},", i);
    }
    println!("");
}
