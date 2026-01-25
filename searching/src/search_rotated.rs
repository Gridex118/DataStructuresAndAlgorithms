use crate::rotation_pivot::get_pivot_index;
use crate::binary_search::search;

pub fn get_index(vec: &[i32], key: i32) -> Option<usize> {
    let pivot_index: usize = get_pivot_index(vec);
    let rightmost: i32 = vec[vec.len() - 1];
    if vec[pivot_index] <= key && key <= rightmost {
        search(vec, key, Some(pivot_index), None)
    } else {
        search(vec, key, None, Some(pivot_index - 1))
    }
}
