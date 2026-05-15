// https://www.hackerrank.com/challenges/staircase/problem

fn staircase(staircase_height: i32) {
    let height_as_usize = staircase_height as usize;

    for current_level in 1..=height_as_usize {
        let number_of_spaces = height_as_usize - current_level;
        let number_of_hashes = current_level;

        let line_output = format!(
            "{}{}",
            " ".repeat(number_of_spaces),
            "#".repeat(number_of_hashes)
        );

        println!("{}", line_output);
    }
}

#[cfg(test)]
mod staircase_tests {
    use super::*;

    #[test]
    fn staircase_prints_correct_number_of_lines() {
        staircase(5);
    }

    #[test]
    fn staircase_handles_minimum_input() {
        staircase(1);
    }

    #[test]
    fn staircase_handles_larger_input() {
        staircase(10);
    }
}