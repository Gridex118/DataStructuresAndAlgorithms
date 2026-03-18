use std::collections::HashSet;

fn digital_root_square(mut num: i32) -> i32 {
    let mut sum = 0;
    while num != 0 {
        let rem = num % 10;
        sum += rem * rem;
        num /= 10;
    }
    sum
}

pub fn is_happy(mut num: i32) -> bool {
    let mut used: HashSet<i32> = HashSet::new();
    while num != 1 {
        if !used.insert(num) {
            return false;
        } else {
            num = digital_root_square(num);
        }
    }
    true
}
