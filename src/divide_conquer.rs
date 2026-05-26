use std::result;

fn main() {
    let a: [i32; 6] = [1, 3, 5, 2, 4, 6];
    count_inversions(&a);
}
pub fn count_inversions(arr: &[i32]) -> i32 {
    //calculate the number of inversions in the array A
    let mut result = 0;

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if arr[i] > arr[j] {
                result += 1;
            }
        }
    }

    return result;
}
