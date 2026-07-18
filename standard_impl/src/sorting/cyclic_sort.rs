/// * `slice` is a sequence of numbers of length N,
/// containing all numbers in the range 1..N exactly once
pub fn cyclic_sort<T>(slice: &mut [T])
where T: Copy + Into<usize>
{
    let n = slice.len();
    for i in 0..n {
        while slice[i].into() != i + 1 {
            slice.swap(i, slice[i].into() - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_for_empty_slice() {
        let mut v = vec![];
        cyclic_sort::<u8>(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn works_for_one_to_n_slices() {
        for v_original in [
            vec![1, 3, 2, 5, 6, 4],
            vec![1], vec![3, 2, 1],
        ] {
            let mut v_copy = v_original.clone();
            cyclic_sort::<u8>(&mut v_copy);
            assert!(v_copy.is_sorted());
        }
    }
}
