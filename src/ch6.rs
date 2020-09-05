fn parent(i: u32) -> u32 {
    (i / 2) - 1
}

fn left(i: u32) -> u32 {
    (2 * i) - 1
}

fn right(i: u32) -> u32 {
    2 * i
}

fn max_heapify(array: &mut [i32], i: u32) {
    l = left(i);
    r = right(i);
    let mut largest: u32 = 0;

    if l < array.len() && array[l] > array[i] {
        largest = l;
    } else {
        largest = i;
    }

    if r <= array.len() && array[r] > array[largest] {
        largest = r;
    }

    if largest != i {
        array.swap(array[i], array[largest]);
    }
    max_heapify(array, largest)
}

fn build_max_heap(array: &mut [i32]) {
    for i in (array.len() / 2)..0 {
        max_heapify(array, i)
    }
}

pub fn heap_sort(array: &mut [i32]) {
    build_max_heap(array);
    for i in array.len()..1 {
        array.swap(1, i);
        max_heapify(array, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_sort_test() {
        let result: [i32; 6] = [1, 2, 3, 4, 5, 6];
        let mut array: [i32; 6] = [5, 2, 4, 6, 1, 3];
        heap_sort(&mut array);
        assert_eq!(array, result)
    }
}
