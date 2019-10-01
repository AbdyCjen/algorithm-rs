#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let (s, p) = ("abcdefghijklmnopqrst", "ijklmno");
        let find = super::gen_kmp(p.as_bytes());
        let m_pos = find(s.as_bytes());
        assert_eq!(p, match m_pos{
            None => panic!("NO Match"),
            Some(i) => &s[i..i+p.len()],
        });
    }
}
pub fn gen_kmp<T>(p :&[T]) -> impl Fn(&[T]) -> Option<usize> 
    where T: std::clone::Clone + PartialEq {
    let p :Vec<T> = Vec::from(p);
    let mut v : Vec<usize> = vec![0;p.len() + 1];
    let mut j :usize = 0;
    for i in 2..p.len() {
        while j > 0 && p[j] != p[i-1] { j = v[j] }
        if p[j] == p[i-1] { j+=1 }
        v[i] = if p[j] == p[i] {v[j]} else {j}
    }

    move |s :&[T]| {
        if p.len() == 0 {return Some(0);}
        let mut j: usize = 0;
        for i in 0..s.len() {
            while j > 0 && p[j] != s[i] {j = v[j]}
            if p[j] == s[i] { j+= 1}
            if j == p.len() { return Some(i - j + 1) }
        }
        None
    }
}
