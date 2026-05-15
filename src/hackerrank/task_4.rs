// https://www.hackerrank.com/challenges/grading/problem

fn grading_students_with_rounding_logic(student_grades: &[i32]) -> Vec<i32> {
    let mut rounded_student_grades = Vec::with_capacity(student_grades.len());

    for original_student_grade in student_grades {
        if *original_student_grade < 38 {
            rounded_student_grades.push(*original_student_grade);
            continue;
        }

        let next_multiple_of_five = ((*original_student_grade / 5) + 1) * 5;
        let difference_to_next_multiple_of_five = next_multiple_of_five - original_student_grade;

        if difference_to_next_multiple_of_five < 3 {
            rounded_student_grades.push(next_multiple_of_five);
        } else {
            rounded_student_grades.push(*original_student_grade);
        }
    }

    rounded_student_grades
}

#[cfg(test)]
mod grading_students_rounding_tests {
    use super::*;

    #[test]
    fn test_grades_that_round_up_to_next_multiple_of_five() {
        let input_student_grades = vec![84, 29, 57];
        let expected_result = vec![85, 29, 57];

        assert_eq!(
            grading_students_with_rounding_logic(&input_student_grades),
            expected_result
        );
    }

    #[test]
    fn test_grades_below_failing_threshold_do_not_round() {
        let input_student_grades = vec![37, 38];
        let expected_result = vec![37, 40];

        assert_eq!(
            grading_students_with_rounding_logic(&input_student_grades),
            expected_result
        );
    }

    #[test]
    fn test_mixed_grades_behavior() {
        let input_student_grades = vec![73, 67, 38, 33];
        let expected_result = vec![75, 67, 40, 33];

        assert_eq!(
            grading_students_with_rounding_logic(&input_student_grades),
            expected_result
        );
    }
}