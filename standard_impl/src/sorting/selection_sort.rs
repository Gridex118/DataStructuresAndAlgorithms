fn max_index(arr: &[i32], start: usize, end: usize) -> usize {
    let mut i_max = start;
    for i in start..end {
        if arr[i] > arr[i_max] {
            i_max = i;
        }
    }
    i_max
}

pub fn selection_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    while n > 1 {
        let i_max = max_index(arr, 0, n);
        arr.swap(i_max, n - 1);
        n -= 1;
    }
}
