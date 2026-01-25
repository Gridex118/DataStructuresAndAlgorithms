mod binary_search;
mod first_last;
mod mountain_array;
mod rotation_pivot;

fn main() {
    let vec1: Vec<i32> = vec![ 2, 4, 6, 8, 12, 18 ];
    let key: i32 = 12;
    match binary_search::search(&vec1, key) {
        Some(index) => println!("{key} found at index {index}"),
        None => println!("{key} not found"),
    }

    let vec2: Vec<i32> = vec![ 1, 4, 4, 4, 4, 5, 6 ];
    let key: i32 = 4;
    match first_last::get_occurances(&vec2, key) {
        (Some(first), Some(last)) => println!("First: {first}, Last: {last}"),
        _ => println!("Key not found"),
    }

    let vec3: Vec<i32> = vec![ 0, 10, 5, 2 ];
    let peak_index: usize = mountain_array::get_peak(&vec3);
    println!("The peak is at index {peak_index}: {}", vec3[peak_index]);

    let vec4: Vec<i32> = vec![ 7, 9, 2, 3 ];
    let pivot_index: usize = rotation_pivot::get_pivot_index(&vec4);
    println!("The pivot is at index {pivot_index}: {}", vec4[pivot_index]);
}
