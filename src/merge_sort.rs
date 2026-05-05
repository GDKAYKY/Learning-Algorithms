// Entry point - demonstrates merge sort on a sample array
fn main() {
    // Create a mutable vector of integers to sort
    let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Original array: {:?}", arr);

    // Sort the array in-place using merge sort
    merge_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

// Recursively divides the array into smaller pieces and merges them back sorted
fn merge_sort(arr: &mut [i32]) {
    // Base case: arrays with 0 or 1 element are already sorted
    if arr.len() <= 1 {
        return;
    }

    // Find the middle point to divide the array into two halves
    let mid = arr.len() / 2;

    // Recursively sort the left half (from start to mid)
    merge_sort(&mut arr[..mid]);

    // Recursively sort the right half (from mid to end)
    merge_sort(&mut arr[mid..]);

    // Merge the two sorted halves back together
    merge(arr, mid);
}

// Merges two sorted halves of an array back together in sorted order
fn merge(arr: &mut [i32], mid: usize) {
    // Create temporary copies of the left and right halves
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    // Index for left array
    let mut i = 0;
    // Index for right array
    let mut j = 0;
    // Index for the merged array
    let mut k = 0;

    // Compare elements from left and right, placing smaller one into merged array
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            // Left element is smaller, copy it to merged array
            arr[k] = left[i];
            i += 1;
        } else {
            // Right element is smaller, copy it to merged array
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // Copy any remaining elements from the left array
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    // Copy any remaining elements from the right array
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
