pub fn alternate_swap<const N: usize>(arr: &mut [i32; N]) {
    for i in (0..N).step_by(2) {
        if i < N {
            arr.swap(i, i + 1);
        }
    }
}
