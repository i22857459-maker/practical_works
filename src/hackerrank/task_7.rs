// https://www.hackerrank.com/challenges/between-two-sets/problem

fn greatest_common_divisor(mut first_number: i64, mut second_number: i64) -> i64 {
    while second_number != 0 {
        let temporary_value = second_number;
        second_number = first_number % second_number;
        first_number = temporary_value;
    }
    first_number.abs()
}

fn least_common_multiple(first_number: i64, second_number: i64) -> i64 {
    (first_number / greatest_common_divisor(first_number, second_number)) * second_number
}

pub fn get_total_x(first_array: &[i32], second_array: &[i32]) -> i32 {
    let mut least_common_multiple_value = first_array[0] as i64;

    for &value in first_array.iter().skip(1) {
        least_common_multiple_value =
            least_common_multiple(least_common_multiple_value, value as i64);
    }

    let mut greatest_common_divisor_value = second_array[0] as i64;

    for &value in second_array.iter().skip(1) {
        greatest_common_divisor_value =
            greatest_common_divisor(greatest_common_divisor_value, value as i64);
    }

    let mut valid_numbers_count = 0;
    let mut candidate_value = least_common_multiple_value;

    while candidate_value <= greatest_common_divisor_value {
        if greatest_common_divisor_value % candidate_value == 0 {
            valid_numbers_count += 1;
        }
        candidate_value += least_common_multiple_value;
    }

    valid_numbers_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let first_array = vec![2, 4];
        let second_array = vec![16, 32, 96];
        assert_eq!(get_total_x(&first_array, &second_array), 3);
    }

    #[test]
    fn test_sample_case() {
        let first_array = vec![2, 6];
        let second_array = vec![24, 36];
        assert_eq!(get_total_x(&first_array, &second_array), 2);
    }
}