fn is_feasible(stalls: &[i32], k: i32, mid: i32) -> bool {
    let mut cows: i32 = 1;
    let mut begin: i32 = stalls[0];
    for stall in stalls.iter().skip(1) {
        if *stall - begin >= mid {
            begin = *stall;
            cows += 1;
            if cows == k {
                return true;
            }
        }
    }
    false
}

pub fn get_max_minimum_dist(mut stalls: Vec<i32>, k: i32) -> i32 {
    stalls.sort();
    let mut distance: i32 = 0;
    let mut start = 0;
    let mut end = stalls[stalls.len() - 1];
    while start <= end {
        let mid: i32 = start + (end - start) / 2;
        if is_feasible(&stalls, k, mid) {
            distance = mid;
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    distance
}
