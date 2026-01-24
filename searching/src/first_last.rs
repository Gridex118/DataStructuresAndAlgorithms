fn get_first_occurance(vec: &[i32], key: i32) -> Option<usize> {
    let mut start: usize = 0;
    let mut end: usize = vec.len() - 1;
    let mut ans: Option<usize> = None;
    while start <= end {
        let mid: usize = start + (end - start) / 2;
        if vec[mid] == key {
            ans = Some(mid);
            end = mid - 1;
        } else if vec[mid] > key {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    ans
}

fn get_last_occurance(vec: &[i32], key: i32) -> Option<usize> {
    let mut start: usize = 0;
    let mut end: usize = vec.len() - 1;
    let mut ans: Option<usize> = None;
    while start <= end {
        let mid: usize = start + (end - start) / 2;
        if vec[mid] == key {
            ans = Some(mid);
            start = mid + 1;
        } else if vec[mid] > key {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    ans
}

pub fn get_occurances(vec: &[i32], key: i32) -> (Option<usize>, Option<usize>) {
    (get_first_occurance(vec, key), get_last_occurance(vec, key))
}
