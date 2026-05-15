// https://www.hackerrank.com/challenges/diagonal-difference/problem

pub fn diagonal_difference(square_integer_matrix: &[Vec<i32>]) -> i32 {
    let mut primary_diagonal_sum: i32 = 0;
    let mut secondary_diagonal_sum: i32 = 0;

    let matrix_dimension_size: usize = square_integer_matrix.len();

    for current_index in 0..matrix_dimension_size {
        primary_diagonal_sum += square_integer_matrix[current_index][current_index];
        secondary_diagonal_sum += square_integer_matrix[current_index][matrix_dimension_size - 1 - current_index];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference_sample_case() {
        let test_square_integer_matrix = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];

        let computed_result = diagonal_difference(&test_square_integer_matrix);

        assert_eq!(computed_result, 15);
    }

    #[test]
    fn test_diagonal_difference_single_element_matrix() {
        let test_square_integer_matrix = vec![vec![7]];

        let computed_result = diagonal_difference(&test_square_integer_matrix);

        assert_eq!(computed_result, 0);
    }
}