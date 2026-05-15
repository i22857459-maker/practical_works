// https://www.hackerrank.com/challenges/sock-merchant/problem

pub fn sock_merchant(_total_number_of_socks: i32, sock_color_values: &[i32]) -> i32 {
    let mut color_frequency_counter: [i32; 101] = [0; 101];

    for &sock_color_value in sock_color_values {
        color_frequency_counter[sock_color_value as usize] += 1;
    }

    let mut total_number_of_matching_pairs: i32 = 0;

    for frequency_of_single_color in color_frequency_counter.iter() {
        total_number_of_matching_pairs += frequency_of_single_color / 2;
    }

    total_number_of_matching_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case_from_description() {
        let sock_color_values = vec![1, 2, 1, 2, 1, 3, 2];
        let total_number_of_socks = sock_color_values.len() as i32;

        let result = sock_merchant(total_number_of_socks, &sock_color_values);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_sample_input_case() {
        let sock_color_values = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        let total_number_of_socks = sock_color_values.len() as i32;

        let result = sock_merchant(total_number_of_socks, &sock_color_values);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_single_pair_only() {
        let sock_color_values = vec![5, 5];
        let total_number_of_socks = sock_color_values.len() as i32;

        let result = sock_merchant(total_number_of_socks, &sock_color_values);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_no_pairs() {
        let sock_color_values = vec![1, 2, 3, 4, 5];
        let total_number_of_socks = sock_color_values.len() as i32;

        let result = sock_merchant(total_number_of_socks, &sock_color_values);

        assert_eq!(result, 0);
    }
}