
use pofk_algorithm::list_algorithms::*;

fn main() {
    // Linear Search
    let arr = [10, 20, 30];
    println!("Linear search for 20: {:?}", linear_search::linear_search(&arr, &20));

    // Binary Search
    let arr = [1, 3, 5, 7, 9];
    println!("Binary search for 7: {:?}", binary_search::binary_search(&arr, &7));

    // Bubble Sort
    let mut arr = [5, 2, 4, 6, 1, 3];
    bubble_sort::bubble_sort(&mut arr);
    println!("Bubble sorted: {:?}", arr);

    // Selection Sort
    let mut arr = [64, 25, 12, 22, 11];
    selection_sort::selection_sort(&mut arr);
    println!("Selection sorted: {:?}", arr);

    // Insertion Sort
    let mut arr = [5, 2, 4, 6, 1, 3];
    insertion_sort::insertion_sort(&mut arr);
    println!("Insertion sorted: {:?}", arr);

    // Merge Sort
    let mut arr = vec![5, 2, 4, 6, 1, 3];
    merge_sort::merge_sort(&mut arr);
    println!("Merge sorted: {:?}", arr);

    // Quick Sort
    let mut arr = [5, 2, 4, 6, 1, 3];
    quick_sort::quick_sort(&mut arr);
    println!("Quick sorted: {:?}", arr);

    // Counting Sort
    let mut arr = [4u32, 2, 2, 8, 3, 3, 1];
    counting_sort::counting_sort(&mut arr, 8u32);
    println!("Counting sorted: {:?}", arr);

    // Reverse List
    let mut arr = [1, 2, 3, 4, 5];
    reverse_list::reverse_list(&mut arr);
    println!("Reversed: {:?}", arr);

    // Find Max/Min
    let arr = [3, 1, 4, 1, 5, 9];
    println!("Min/Max: {:?}", find_max_min::find_max_min(&arr));

    // Find Duplicates
    let arr = [1, 2, 2, 3, 4, 4, 4, 5];
    println!("Duplicates: {:?}", find_duplicates::find_duplicates(&arr));

    // Kadane's Algorithm
    let arr = [1, -2, 3, 4, -1, 2, 1, -5, 4];
    println!("Kadane's max sum: {:?}", kadanes_algorithm::kadane(&arr));

    // Sliding Window
    let arr = [2, 1, 5, 1, 3, 2];
    println!("Sliding window max sum (k=3): {:?}", sliding_window::sliding_window_max_sum(&arr, 3));

    // Remove Duplicates
    let arr = [1, 2, 2, 3, 4, 4, 5];
    println!("Remove duplicates: {:?}", remove_duplicates::remove_duplicates(&arr));

    // Rotate Array Right
    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    rotate_array_right::rotate_array_right(&mut arr, 3);
    println!("Rotated right by 3: {:?}", arr);

    // Prefix Sum
    let arr = [1, 2, 3, 4];
    println!("Prefix sum: {:?}", prefix_sum::prefix_sum(&arr));
}
