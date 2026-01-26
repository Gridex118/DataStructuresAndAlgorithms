pub fn square_root(x: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = x;
    while (right - left) > 1 {
        let root: i32 = left + (right - left) / 2;
        let square: i32 = root * root;
        if square == x {
            return root
        } else if square < x {
            left = root;
        } else {
            right = root;
        }
    }
    left
}
