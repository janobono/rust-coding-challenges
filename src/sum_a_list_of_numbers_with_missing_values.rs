pub fn sum_with_missing(data: &Vec<Option<i32>>) -> i32 {
    data.iter()
        .map(|x| x.unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::sum_a_list_of_numbers_with_missing_values::sum_with_missing;

    #[test]
    fn empty_data() {
        assert_eq!(sum_with_missing(&vec![]), 0);
    }

    #[test]
    fn no_missing_values_data() {
        assert_eq!(sum_with_missing(&vec![Some(1), Some(2), Some(3)]), 6);
    }

    #[test]
    fn missing_values_data() {
        assert_eq!(sum_with_missing(&vec![Some(4), None, Some(1)]), 5);
    }

    #[test]
    fn only_missing_values_data() {
        assert_eq!(sum_with_missing(&vec![None, None, None]), 0);
    }
}
