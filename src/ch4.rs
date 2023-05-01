/// # Maximum subarray from chapter 4

fn find_max_crossing_subarray<T>(
    array: &[T],
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, T)
where
    T: std::ops::Add<Output = T> + PartialOrd + Copy,
{
    let mut max_left: usize = 0;
    let mut max_right: usize = 0;

    let mut left_sum: Option<T> = None;
    let mut sum: Option<T> = None;
    for i in (low..=mid).rev() {
        sum = sum.map_or(Some(array[i]), |sum| Some(sum + array[i]));
        if left_sum.is_none() || sum > left_sum {
            left_sum = sum;
            max_left = i;
        };
    }

    let mut right_sum: Option<T> = None;
    let mut sum: Option<T> = None;
    for j in mid + 1..=high {
        sum = sum.map_or(Some(array[j]), |sum| Some(sum + array[j]));
        if right_sum.is_none() || sum > right_sum {
            right_sum = sum;
            max_right = j;
        };
    }

    (max_left, max_right, left_sum.unwrap() + right_sum.unwrap())
}

pub fn find_max_subarray<T>(
    array: &[T],
    low: usize,
    high: usize,
) -> (usize, usize, T)
where
    T: std::ops::Add<Output = T> + PartialOrd + Copy + std::fmt::Debug,
{
    if high == low {
        return (low, high, array[low]);
    };
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
        let array: [i32; 16] = [
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        assert_eq!(find_max_subarray(&array, 0, 15), (7, 10, 43));
    }
}
