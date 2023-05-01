/// # Maximum subarray from chapter 4

pub trait NumericValues<T> {
    fn get_zero() -> T;
}

fn find_max_crossing_subarray<T: NumericValues<T>>(
    array: &[T],
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, T)
where
    T: std::ops::Add<Output = T> + PartialOrd + Copy + std::fmt::Debug,
{
    let mut max_left: usize = 0;
    let mut max_right: usize = 0;

    let mut left_sum: Option<T> = None;
    let mut sum: T = T::get_zero();
    for i in (low..=mid).rev() {
        println!(
            "Left sum: {:?} Sum: {:?} array[i]: {:?}",
            left_sum, sum, array[i]
        );
        sum = sum + array[i];
        if left_sum.is_none() || Some(sum) > left_sum {
            left_sum = Some(sum);
            max_left = i;
        };
    }

    let mut right_sum: Option<T> = None;
    let mut sum: T = T::get_zero();
    for j in mid + 1..=high {
        println!(
            "Right sum: {:?} Sum: {:?} array[j]: {:?}",
            left_sum, sum, array[j]
        );
        sum = sum + array[j];
        if right_sum.is_none() || Some(sum) > right_sum {
            right_sum = Some(sum);
            max_right = j;
        };
    }

    (
        max_left,
        max_right,
        left_sum.unwrap_or(T::get_zero()) + right_sum.unwrap_or(T::get_zero()),
    )
}

pub fn find_max_subarray<T: NumericValues<T>>(
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
    println!(
        "Left subarray: {:?} {:?} {:?}",
        left_low, left_high, left_sum
    );

    let (right_low, right_high, right_sum) =
        find_max_subarray(array, mid + 1, high);
    println!(
        "Right subarray: {:?} {:?} {:?}",
        right_low, right_high, right_sum
    );

    let (cross_low, cross_high, cross_sum) =
        find_max_crossing_subarray(array, low, mid, high);
    println!(
        "Cross subarray: {:?} {:?} {:?}",
        cross_low, cross_high, cross_sum
    );

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

    impl NumericValues<i32> for i32 {
        fn get_zero() -> i32 {
            0
        }
    }

    #[test]
    fn fin_max_subarray() {
        let array: [i32; 16] = [
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        assert_eq!(find_max_subarray(&array, 0, 15), (7, 10, 43));
    }
}
