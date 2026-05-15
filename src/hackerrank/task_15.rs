// https://www.hackerrank.com/challenges/drawing-book/problem

pub fn page_count(total_number_of_pages_in_book: i32, target_page_number: i32) -> i32 {
    let number_of_page_turns_from_front = target_page_number / 2;
    let number_of_page_turns_from_back = (total_number_of_pages_in_book / 2) - (target_page_number / 2);

    if number_of_page_turns_from_front < number_of_page_turns_from_back {
        number_of_page_turns_from_front
    } else {
        number_of_page_turns_from_back
    }
}

#[cfg(test)]
mod page_count_tests {
    use super::*;

    #[test]
    fn test_when_target_page_is_in_the_middle_of_book() {
        let total_number_of_pages_in_book = 6;
        let target_page_number = 2;
        let expected_minimum_number_of_turns = 1;

        let actual_result = page_count(total_number_of_pages_in_book, target_page_number);

        assert_eq!(actual_result, expected_minimum_number_of_turns);
    }

    #[test]
    fn test_when_target_page_is_near_end_of_book() {
        let total_number_of_pages_in_book = 5;
        let target_page_number = 4;
        let expected_minimum_number_of_turns = 0;

        let actual_result = page_count(total_number_of_pages_in_book, target_page_number);

        assert_eq!(actual_result, expected_minimum_number_of_turns);
    }
}