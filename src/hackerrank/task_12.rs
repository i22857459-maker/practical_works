// https://www.hackerrank.com/challenges/birthday-cake-candles/problem

pub fn birthday_cake_candles(candle_heights_list: &[i32]) -> i32 {
    let mut maximum_candle_height_value: i32 = i32::MIN;
    let mut number_of_tallest_candles: i32 = 0;

    for &single_candle_height in candle_heights_list.iter() {
        if single_candle_height > maximum_candle_height_value {
            maximum_candle_height_value = single_candle_height;
            number_of_tallest_candles = 1;
        } else if single_candle_height == maximum_candle_height_value {
            number_of_tallest_candles += 1;
        }
    }

    number_of_tallest_candles
}

#[cfg(test)]
mod birthday_cake_candles_tests {
    use super::*;

    #[test]
    fn test_example_case_with_multiple_tallest_candles() {
        let candle_heights_input_values = vec![3, 2, 1, 3];
        let result_value = birthday_cake_candles(&candle_heights_input_values);
        assert_eq!(result_value, 2);
    }

    #[test]
    fn test_single_candle_case() {
        let candle_heights_input_values = vec![5];
        let result_value = birthday_cake_candles(&candle_heights_input_values);
        assert_eq!(result_value, 1);
    }

    #[test]
    fn test_all_candles_same_height_case() {
        let candle_heights_input_values = vec![4, 4, 4, 4];
        let result_value = birthday_cake_candles(&candle_heights_input_values);
        assert_eq!(result_value, 4);
    }
}