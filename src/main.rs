mod ch1;
mod ch6;

fn main() {
    // insertion sort
    let mut array: [i32; 6] = [5, 2, 4, 6, 1, 3];
    println!("Insertion sort for array:  {:?}", array);
    ch1::insertion_sort(&mut array);
    println!("Sorted array: {:?}", array);

    // merge sort
    let mut array: [i32; 6] = [5, 2, 4, 6, 1, 3];
    println!("Merge sort for array:  {:?}", array);
    ch1::merge_sort(&mut array);
    println!("Sorted array: {:?}", array);

    // heap sort
    let mut array: [i32; 6] = [5, 2, 4, 6, 1, 3];
    println!("Heap sort for array:  {:?}", array);
    ch6::heap_sort(&mut array);
    println!("Sorted array: {:?}", array);
}
