// N numbers from the range 1..N-1, only one of which is duplicated
// Case 1: 5 1 2 3 4 2, 6
// -> 2
// Case 2: 8 7 2 5 4 7 1 3 6, 9
// -> 7

pub fn find_duplicate<const N: usize>(arr: &mut [i32; N]) -> i32 {
    let max: i32 = N as i32;
    let auxilary: Vec<i32> = (1..max).collect();
    arr.iter()
        .chain(auxilary.iter())
        .fold(0i32, |res, val| res ^ val)
}
