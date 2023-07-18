pub fn calculate_the_median(data: &[f32]) -> Option<f32> {
    if data.is_empty() { return None; }

    let mut sorted = data.to_vec();

    sorted.sort_by(|a, b| {
        a.partial_cmp(b).unwrap()
    });

    let middle = sorted.len() / 2;

    if sorted.len() % 2 == 0 {
        Some((sorted[middle] + sorted[middle - 1]) / 2.0)
    } else {
        Some(sorted[middle])
    }
}

#[cfg(test)]
mod test {
    use crate::calculate_the_median::calculate_the_median;

    #[test]
    fn empty_list() {
        let data: [f32; 0] = [];
        assert_eq!(calculate_the_median(&data), None);
    }

    #[test]
    fn sorted_list() {
        let data = [1.0, 4.0, 5.0];
        assert_eq!(calculate_the_median(&data), Some(4.0f32));
    }

    #[test]
    fn unsorted_list() {
        let data = [3.0, 1.5, 8.8, 5.0];
        assert_eq!(calculate_the_median(&data), Some(4.0f32));
    }
}
