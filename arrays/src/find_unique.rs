pub fn find_unique<const N: usize>(arr: &mut [i32; N]) -> i32 {
    arr.iter()
        .fold(0i32, |unique_number, value| unique_number ^ value)
}
