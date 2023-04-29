/// Two types of sort algorythms from chapter 2

/// Sorts the elements of a mutable slice in ascending order using the insertion sort algorithm.
///
/// # Examples
///
/// ```
/// let mut array = [5, 2, 4, 6, 1, 3];
/// insertion_sort(&mut array);
/// assert_eq!(array, [1, 2, 3, 4, 5, 6]);
/// ```
///
/// # Parameters
///
/// * `array` - A mutable slice of elements to be sorted
///
/// # Type Parameters
///
/// * `T` - The type of the elements in the slice. `T` must implement
/// the `PartialOrd` trait.
pub fn insertion_sort<T>(array: &mut [T])
where
    T: PartialOrd,
{
    // Insertion sort
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn merge<T>(arr1: &[T], arr2: &[T], ret: &mut [T])
where
    T: PartialOrd + Copy,
{
    // merge two arrays
    let mut left = 0;
    let mut right = 0;
    let mut index = 0;

    while left < arr1.len() && right < arr2.len() {
        if arr1[left] <= arr2[right] {
            ret[index] = arr1[left];
            index += 1;
            left += 1;
        } else {
            ret[index] = arr2[right];
            index += 1;
            right += 1;
        }
    }

    if left < arr1.len() {
        ret[index..].copy_from_slice(&arr1[left..]);
    }
    if right < arr2.len() {
        ret[index..].copy_from_slice(&arr2[right..]);
    }
}

pub fn merge_sort<T>(array: &mut [T])
where
    T: PartialOrd + Copy,
{
    // merge sort
    let mid = array.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut array[..mid]);
    merge_sort(&mut array[mid..]);

    let mut ret = array.to_vec();
    merge(&array[..mid], &array[mid..], &mut ret[..]);
    array.copy_from_slice(&ret);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_test() {
        let result: [i32; 6] = [1, 2, 3, 4, 5, 6];
        let mut array: [i32; 6] = [5, 2, 4, 6, 1, 3];
        insertion_sort(&mut array);
        assert_eq!(array, result)
    }

    #[test]
    fn merge_sort_test() {
        let result: [i32; 6] = [1, 2, 3, 4, 5, 6];
        let mut array: [i32; 6] = [5, 2, 4, 6, 1, 3];
        merge_sort(&mut array);
        assert_eq!(array, result)
    }
}
