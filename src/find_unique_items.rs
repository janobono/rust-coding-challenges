pub fn find_unique_items<T>(data: &[T]) -> Vec<T>
    where
        T: PartialEq + Clone
{
    let mut result: Vec<T> = Vec::new();
    for item in data {
        if result.contains(item) {
            continue;
        }
        result.push(item.clone());
    }
    result
}

#[cfg(test)]
mod test {
    use crate::find_unique_items::find_unique_items;

    #[test]
    fn no_duplicates_list() {
        let data = [1, 4, 5];
        assert_eq!(find_unique_items(&data), &data);
    }

    #[test]
    fn duplicates_list() {
        assert_eq!(find_unique_items(&[1, 1, 3]), &[1, 3]);
    }

    #[test]
    fn same_order() {
        assert_eq!(find_unique_items(&[6, 2, 2, 1, 1, 3, 6]), &[6, 2, 1, 3]);
    }
}
