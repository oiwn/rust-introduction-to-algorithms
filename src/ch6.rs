// heap sort

fn max_heapify(array: &mut [i32], i: usize, heap_size: usize) {
    let l: usize = (2 * i) + 1;
    let r: usize = (2 * i) + 2;
    let mut largest: usize;

    if l < heap_size && array[l] > array[i] {
        largest = l;
    } else {
        largest = i;
    }

    if r < heap_size && array[r] > array[largest] {
        largest = r;
    }

    if largest != i {
        array.swap(i, largest);
        max_heapify(array, largest, heap_size)
    }
}

fn build_max_heap(array: &mut [i32]) {
    let heap_size = array.len();
    for i in (0..heap_size / 2).rev() {
        max_heapify(array, i, heap_size);
    }
}

pub fn heap_sort(array: &mut [i32]) {
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
