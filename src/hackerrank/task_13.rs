// https://www.hackerrank.com/challenges/divisible-sum-pairs/problem

fn count_divisible_sum_pairs_with_given_divisor(array_length: i32, divisor_value: i32, integer_array: &[i32]) -> i32 {
    let mut valid_pair_counter: i32 = 0;

    let array_length_usize = array_length as usize;

    for first_index in 0..array_length_usize {
        for second_index in (first_index + 1)..array_length_usize {
            let pair_sum_value = integer_array[first_index] + integer_array[second_index];

            if pair_sum_value % divisor_value == 0 {
                valid_pair_counter += 1;
            }
        }
    }

    valid_pair_counter
}

#[cfg(test)]
mod unit_tests_for_divisible_sum_pairs {
    use super::*;

    #[test]
    fn test_sample_case_from_task_description() {
        let array_length_value = 6;
        let divisor_value = 3;
        let integer_array_values = vec![1, 3, 2, 6, 1, 2];

        let result_value = count_divisible_sum_pairs_with_given_divisor(
            array_length_value,
            divisor_value,
            &integer_array_values,
        );

        assert_eq!(result_value, 5);
    }

    #[test]
    fn test_simple_case_with_single_valid_pair() {
        let array_length_value = 4;
        let divisor_value = 5;
        let integer_array_values = vec![1, 4, 7, 2];

        let result_value = count_divisible_sum_pairs_with_given_divisor(
            array_length_value,
            divisor_value,
            &integer_array_values,
        );

        assert_eq!(result_value, 1);
    }

    #[test]
    fn test_no_valid_pairs_case() {
        let array_length_value = 3;
        let divisor_value = 10;
        let integer_array_values = vec![1, 2, 3];

        let result_value = count_divisible_sum_pairs_with_given_divisor(
            array_length_value,
            divisor_value,
            &integer_array_values,
        );

        assert_eq!(result_value, 0);
    }
}