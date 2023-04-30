/// # Chapter 6 heap sort implementation

/// Maintains the max heap property for a given node in the heap.
///
/// ## Examples
///
/// ```ignore
/// let mut array = [16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
/// max_heapify(&mut array, 1, array.len());
/// assert_eq!(array, [16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
/// ```
///
/// ## Parameters
///
/// * `array` - A mutable slice representing the heap
/// * `i` - The index of the node to be heapified
/// * `heap_size` - The size of the heap
///
/// ## Type Parameters
///
/// * `T` - The type of the elements in the slice. `T` must implement the `PartialOrd` trait.
fn max_heapify<T>(array: &mut [T], i: usize, heap_size: usize)
where
    T: PartialOrd,
{
    let l: usize = (2 * i) + 1;
    let r: usize = (2 * i) + 2;
    let mut largest: usize;

    if l < heap_size && array[l] > array[i] {
        largest = l;
    } else {
        largest = i;
    };

    if r < heap_size && array[r] > array[largest] {
        largest = r;
    };

    if largest != i {
        array.swap(i, largest);
        max_heapify(array, largest, heap_size)
    };
}

/// Builds a max heap from an unordered slice.
///
/// ## Examples
///
/// ```ignore
/// let mut array = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
/// build_max_heap(&mut array);
/// assert_eq!(array, [16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
/// ```
///
/// ## Parameters
///
/// * `array` - A mutable slice to be transformed into a max heap
///
/// ## Type Parameters
///
/// * `T` - The type of the elements in the slice. `T` must implement the `PartialOrd` trait.
fn build_max_heap<T>(array: &mut [T])
where
    T: PartialOrd,
{
    let heap_size = array.len();
    for i in (0..heap_size / 2).rev() {
        max_heapify(array, i, heap_size);
    }
}

/// Sorts the elements of a mutable slice in ascending order using the heap sort algorithm.
///
/// ## Examples
///
/// ```
/// use kormen::ch6::heap_sort;
/// let mut array = [16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
/// heap_sort(&mut array);
/// assert_eq!(array, [1, 2, 3, 4, 7, 8, 9, 10, 14, 16]);
/// ```
///
/// ## Parameters
///
/// * `array` - A mutable slice of elements to be sorted
///
/// ## Type Parameters
///
/// * `T` - The type of the elements in the slice. `T` must implement the `PartialOrd` trait.
pub fn heap_sort<T>(array: &mut [T])
where
    T: PartialOrd,
{
    let mut heap_size: usize = array.len();
    build_max_heap(array);
    for i in (1..array.len()).rev() {
        array.swap(0, i);
        heap_size -= 1;
        max_heapify(array, 0, heap_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_heapify_test() {
        let result: [i32; 10] = [16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        let mut array: [i32; 10] = [16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        max_heapify(&mut array, 1, 10);
        assert_eq!(array, result);
    }

    #[test]
    fn build_max_heap_test() {
        let result: [i32; 10] = [16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        let mut array: [i32; 10] = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        build_max_heap(&mut array);
        assert_eq!(array, result);
    }

    #[test]
    fn heap_sort_test() {
        let result: [i32; 6] = [1, 2, 3, 4, 5, 6];
        let mut array: [i32; 6] = [5, 2, 4, 6, 1, 3];
        heap_sort(&mut array);
        assert_eq!(array, result)
    }
}
