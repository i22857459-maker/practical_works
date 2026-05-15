// https://www.hackerrank.com/challenges/bon-appetit/problem

pub fn bon_appetit_calculation_function(bill_items_costs: &[i32], excluded_item_index: i32, charged_amount: i32) -> Option<i32> {
    let mut total_shared_cost: i64 = 0;

    let excluded_index_usize: usize = excluded_item_index as usize;

    for (current_index, &current_item_cost) in bill_items_costs.iter().enumerate() {
        if current_index != excluded_index_usize {
            total_shared_cost += current_item_cost as i64;
        }
    }

    let correct_amount_per_person: i64 = total_shared_cost / 2;
    let charged_amount_i64: i64 = charged_amount as i64;

    if charged_amount_i64 == correct_amount_per_person {
        None
    } else {
        Some((charged_amount_i64 - correct_amount_per_person) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit_overcharged_case() {
        let bill_items_costs = vec![3, 10, 2, 9];
        let result = bon_appetit_calculation_function(&bill_items_costs, 1, 12);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_bon_appetit_fair_split_case() {
        let bill_items_costs = vec![3, 10, 2, 9];
        let result = bon_appetit_calculation_function(&bill_items_costs, 1, 7);
        assert_eq!(result, None);
    }
}