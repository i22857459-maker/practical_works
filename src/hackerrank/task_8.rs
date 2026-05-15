// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem

pub fn breaking_records(game_scores: &[i32]) -> Vec<i32> {
    if game_scores.is_empty() {
        return vec![0, 0];
    }

    let mut highest_score_record = game_scores[0];
    let mut lowest_score_record = game_scores[0];
    let mut highest_score_record_break_count = 0;
    let mut lowest_score_record_break_count = 0;

    for &current_game_score in game_scores.iter().skip(1) {
        if current_game_score > highest_score_record {
            highest_score_record = current_game_score;
            highest_score_record_break_count += 1;
        }

        if current_game_score < lowest_score_record {
            lowest_score_record = current_game_score;
            lowest_score_record_break_count += 1;
        }
    }

    vec![highest_score_record_break_count, lowest_score_record_break_count]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records_sample_case_one() {
        let game_scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&game_scores), vec![2, 4]);
    }

    #[test]
    fn test_breaking_records_sample_case_two() {
        let game_scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breaking_records(&game_scores), vec![4, 0]);
    }

    #[test]
    fn test_breaking_records_single_game() {
        let game_scores = vec![100];
        assert_eq!(breaking_records(&game_scores), vec![0, 0]);
    }
}