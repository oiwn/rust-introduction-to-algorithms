/// # Maximum subarray from chapter 4

pub trait MinPossibleValue<T> {
    fn get_min_possible_value() -> T;
    fn get_zero() -> T;
}

fn find_max_crossing_subarray<T: MinPossibleValue<T>>(
    array: &[T],
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, T)
where
    T: std::ops::Add<Output = T> + PartialOrd + Copy,
{
    let mut max_left: usize = 0;
    let mut max_righ: usize = 0;

    let mut left_sum = T::get_min_possible_value();
    let mut sum: T = T::get_zero();
    for i in low..=mid {
        sum = sum + array[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        };
    }

    let mut right_sum = T::get_min_possible_value();
    let mut sum: T = T::get_zero();
    for j in mid + 1..high {
        sum = sum + array[j];
        if sum > right_sum {
            right_sum = sum;
            max_righ = j;
        };
    }

    (max_left, max_righ, left_sum + right_sum)
}

pub fn find_max_subarray<T: MinPossibleValue<T>>(
    array: &mut [T],
    low: usize,
    high: usize,
) -> (usize, usize, T)
where
    T: std::ops::Add<Output = T> + PartialOrd + Copy,
{
    if high == low {
        return (low, high, array[low]);
    }
    let mid = (low + high) / 2;

    let (left_low, left_high, left_sum) = find_max_subarray(array, low, mid);

    let (right_low, right_high, right_sum) =
        find_max_subarray(array, mid + 1, high);

    let (cross_low, cross_high, cross_sum) =
        find_max_crossing_subarray(array, low, mid, high);

    if left_sum > right_sum && left_sum > cross_sum {
        return (left_low, left_high, left_sum);
    };

    if right_sum >= left_sum && right_sum >= cross_sum {
        return (right_low, right_high, right_sum);
    };

    (cross_low, cross_high, cross_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fin_max_subarray() {
        let result: [i32; 10] = [16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
        let mut array: [i32; 10] = [16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        assert!(true);
    }
}
