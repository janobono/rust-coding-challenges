pub fn calculate_the_median(mut data: Vec<f32>) -> Option<f32> {
    if data.is_empty() { return None; }

    data.sort_by(|a, b| {
        a.partial_cmp(b).unwrap()
    });

    let middle = data.len() / 2;

    if data.len() % 2 == 0 {
        Some((data[middle] + data[middle - 1]) / 2.0)
    } else {
        Some(data[middle])
    }
}

#[cfg(test)]
mod test {
    use crate::calculate_the_median::calculate_the_median;

    #[test]
    fn empty_list() {
        let mut data = vec![];
        assert_eq!(calculate_the_median(data), None);
    }

    #[test]
    fn sorted_list() {
        let mut data = vec![1.0, 4.0, 5.0];
        assert_eq!(calculate_the_median(data), Some(4.0f32));
    }

    #[test]
    fn unsorted_list() {
        let mut data = vec![3.0, 1.5, 8.8, 5.0];
        assert_eq!(calculate_the_median(data), Some(4.0f32));
    }
}
