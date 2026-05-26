pub mod integer_multiplication;
mod merge_sort;
pub mod divide_conquer;

fn main() {
    let a: [i32; 6] = [1, 3, 5, 2, 4, 6];
    println!("Number of inversions: {}", divide_conquer::count_inversions(&a));
}
