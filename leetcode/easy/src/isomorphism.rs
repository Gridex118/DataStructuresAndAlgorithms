pub fn are_isomorphic(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();
    if s.len() != t.len() { return false; }
    let n = s.len();
    let mut d1 = [0; 256];
    let mut d2 = [0; 256];
    for i in 0..n {
        let (a, b) = (s[i] as usize, t[i] as usize);
        if d1[a] != d2[b] { return false; }
        (d1[a], d2[b]) = (i + 1, i + 1);
    }
    true
}
