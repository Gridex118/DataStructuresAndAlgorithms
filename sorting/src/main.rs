mod selection;
mod bubble;

fn main() {
    let vec1: Vec<i32> = vec![ 29, 72, 98, 13, 87, 66, 52, 51, 36 ];
    println!("Original sequence: {:?}", vec1);
    let vec_selection = selection::sort(&vec1);
    println!("Selection sort result: {:?}", vec_selection);
    let vec_bubble = bubble::sort(&vec1);
    println!("Bubble sort result: {:?}", vec_bubble);
}
