// https://www.hackerrank.com/challenges/apple-and-orange/problem

pub fn compute_apples_and_oranges_falling_on_house(
    house_start_position: i32,
    house_end_position: i32,
    apple_tree_position: i32,
    orange_tree_position: i32,
    apple_fall_distances_from_tree: &[i32],
    orange_fall_distances_from_tree: &[i32],
) -> (i32, i32) {
    let mut apples_falling_on_house_counter = 0;
    let mut oranges_falling_on_house_counter = 0;

    for apple_fall_distance in apple_fall_distances_from_tree {
        let apple_landing_position = apple_tree_position + apple_fall_distance;

        if apple_landing_position >= house_start_position
            && apple_landing_position <= house_end_position
        {
            apples_falling_on_house_counter += 1;
        }
    }

    for orange_fall_distance in orange_fall_distances_from_tree {
        let orange_landing_position = orange_tree_position + orange_fall_distance;

        if orange_landing_position >= house_start_position
            && orange_landing_position <= house_end_position
        {
            oranges_falling_on_house_counter += 1;
        }
    }

    (apples_falling_on_house_counter, oranges_falling_on_house_counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_case() {
        let (apples_count, oranges_count) = compute_apples_and_oranges_falling_on_house(
            7,
            11,
            5,
            15,
            &[-2, 2, 1],
            &[5, -6],
        );

        assert_eq!(apples_count, 1);
        assert_eq!(oranges_count, 1);
    }

    #[test]
    fn test_no_fruits_fall_on_house() {
        let (apples_count, oranges_count) = compute_apples_and_oranges_falling_on_house(
            7,
            11,
            5,
            15,
            &[-100, -200],
            &[100, 200],
        );

        assert_eq!(apples_count, 0);
        assert_eq!(oranges_count, 0);
    }
}