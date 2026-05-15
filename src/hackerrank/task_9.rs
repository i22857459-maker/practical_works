// https://www.hackerrank.com/challenges/migratory-birds/problem

use std::collections::HashMap;

pub fn migratory_birds(bird_type_identifiers: &[i32]) -> i32 {
    let mut bird_frequency_map: HashMap<i32, i32> = HashMap::new();

    for bird_type_identifier in bird_type_identifiers {
        *bird_frequency_map.entry(*bird_type_identifier).or_insert(0) += 1;
    }

    let mut most_frequent_bird_type: i32 = i32::MAX;
    let mut highest_bird_frequency: i32 = 0;

    for (bird_type_identifier, bird_frequency) in bird_frequency_map {
        if bird_frequency > highest_bird_frequency
            || (bird_frequency == highest_bird_frequency && bird_type_identifier < most_frequent_bird_type)
        {
            highest_bird_frequency = bird_frequency;
            most_frequent_bird_type = bird_type_identifier;
        }
    }

    most_frequent_bird_type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case_with_multiple_max_frequencies() {
        let bird_type_identifiers = vec![1, 1, 2, 2, 3];
        assert_eq!(migratory_birds(&bird_type_identifiers), 1);
    }

    #[test]
    fn test_sample_case_with_clear_winner() {
        let bird_type_identifiers = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
        assert_eq!(migratory_birds(&bird_type_identifiers), 3);
    }

    #[test]
    fn test_single_element() {
        let bird_type_identifiers = vec![7];
        assert_eq!(migratory_birds(&bird_type_identifiers), 7);
    }

    #[test]
    fn test_all_same_elements() {
        let bird_type_identifiers = vec![5, 5, 5, 5];
        assert_eq!(migratory_birds(&bird_type_identifiers), 5);
    }
}