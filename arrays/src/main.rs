mod alternate_swap;
mod find_unique;
mod find_duplicate;
mod array_intersection;
mod pair_sum;

fn main() {
    let mut arr1: [i32; 8] = [ 5, 2, 9, 4, 7, 6, 1, 0 ];
    alternate_swap::alternate_swap(&mut arr1);
    println!("{:?}", arr1);
    let mut arr2: [i32; 5] = [ 7, 2, 3, 7, 3 ];
    println!("{} appears uniquely", find_unique::find_unique(&mut arr2));
    let mut arr3: [i32; 6] = [ 5, 1, 2, 3, 4, 2 ];
    println!("{} is the duplicate", find_duplicate::find_duplicate(&mut arr3));
    let arr41: [i32; 3] = [ 1, 4, 5 ];
    let arr42: [i32; 3] = [ 3, 4, 5 ];
    println!("The intersection vector is: {:?}", array_intersection::intersection(arr41, arr42));
    let vec5: Vec<i32> = vec![2, -3, 3, 3, -2];
    println!("The pairs are: {:?}", pair_sum::get_pairs(vec5, 0));
}
