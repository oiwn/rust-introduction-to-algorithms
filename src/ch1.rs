/// # Two types of sort algorythms from chapter 2

/// Sorts the elements of a mutable slice in ascending order using the insertion sort algorithm.
///
/// ## Examples
///
/// ```
/// let mut array = [5, 2, 4, 6, 1, 3];
/// insertion_sort(&mut array);
/// assert_eq!(array, [1, 2, 3, 4, 5, 6]);
/// ```
///
/// ## Parameters
///
/// * `array` - A mutable slice of elements to be sorted
///
/// ## Type Parameters
///
/// * `T` - The type of the elements in the slice. `T` must implement
/// the `PartialOrd` trait.
pub fn insertion_sort<T>(array: &mut [T])
where
    T: PartialOrd,
{
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Sorts the elements of a mutable slice in ascending order using the merge sort algorithm.
///
/// ## Examples
///
/// ```
/// let mut array = [5, 2, 4, 6, 1, 3];
/// merge_sort(&mut array);
/// assert_eq!(array, [1, 2, 3, 4, 5, 6]);
/// ```
///
/// ## Parameters
///
/// * `array` - A mutable slice of elements to be sorted
///
/// ## Type Parameters
///
/// * `T` - The type of the elements in the slice. `T` must implement the `PartialOrd` and `Copy` traits.
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

/// Merges two slices into a single slice in ascending order.
///
/// ## Examples
///
/// ```
/// let arr1 = [1, 3, 5];
/// let arr2 = [2, 4, 6];
/// let mut ret = [0; 6];
///
/// merge(&arr1, &arr2, &mut ret);
/// assert_eq!(ret, [1, 2, 3, 4, 5, 6]);
/// ```
///
/// ## Parameters
///
/// * `arr1` - A slice of elements to be merged
/// * `arr2` - A slice of elements to be merged
/// * `ret` - A mutable slice to store the merged elements
///
/// ## Type Parameters
///
/// * `T` - The type of the elements in the slice. `T` must implement the `PartialOrd` and `Copy` traits.
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
    };
    if right < arr2.len() {
        ret[index..].copy_from_slice(&arr2[right..]);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_test() {
        let mut array: [i32; 6] = [5, 2, 4, 6, 1, 3];
        insertion_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5, 6]);

        let mut vec = vec![5.0, 2.3, 4.2, 6.0, 1.4, 3.6];
        insertion_sort(&mut vec);
        assert_eq!(vec, [1.4, 2.3, 3.6, 4.2, 5.0, 6.0]);

        let mut vec: Vec<u32> = vec![];
        insertion_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn merge_sort_test() {
        let result: [i32; 6] = [1, 2, 3, 4, 5, 6];
        let mut array: [i32; 6] = [5, 2, 4, 6, 1, 3];
        merge_sort(&mut array);
        assert_eq!(array, result)
    }
}
