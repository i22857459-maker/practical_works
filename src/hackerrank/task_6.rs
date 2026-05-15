// https://www.hackerrank.com/challenges/kangaroo/problem

pub fn kangaroo(starting_position_first_kangaroo: i32, jump_distance_first_kangaroo: i32, starting_position_second_kangaroo: i32, jump_distance_second_kangaroo: i32) -> String {
    if jump_distance_first_kangaroo == jump_distance_second_kangaroo {
        if starting_position_first_kangaroo == starting_position_second_kangaroo {
            return "YES".to_string();
        } else {
            return "NO".to_string();
        }
    }

    let distance_between_starting_positions: i32 = starting_position_second_kangaroo - starting_position_first_kangaroo;
    let relative_jump_difference: i32 = jump_distance_first_kangaroo - jump_distance_second_kangaroo;

    if relative_jump_difference <= 0 {
        return "NO".to_string();
    }

    if distance_between_starting_positions % relative_jump_difference == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[cfg(test)]
mod kangaroo_tests {
    use super::*;

    #[test]
    fn test_kangaroo_meeting_case_returns_yes() {
        let result_from_kangaroo = kangaroo(0, 3, 4, 2);
        assert_eq!(result_from_kangaroo, "YES");
    }

    #[test]
    fn test_kangaroo_never_meeting_case_returns_no() {
        let result_from_kangaroo = kangaroo(0, 2, 5, 3);
        assert_eq!(result_from_kangaroo, "NO");
    }

    #[test]
    fn test_kangaroo_same_start_same_speed_returns_yes() {
        let result_from_kangaroo = kangaroo(5, 2, 5, 2);
        assert_eq!(result_from_kangaroo, "YES");
    }
}