pub fn case_insensitive_sort<T: AsRef<str>>(data: &mut Vec<T>) {
    data.sort_by_cached_key(|value| { value.as_ref().to_lowercase() });
    // data.sort_by(|a, b| { a.as_ref().to_lowercase().cmp(&b.as_ref().to_lowercase()) });
}

#[cfg(test)]
mod test {
    use crate::case_insensitive_sort::case_insensitive_sort;

    #[test]
    fn case_insensitive_sort_works() {
        let mut data = vec!["Todd", "amy"];
        case_insensitive_sort(&mut data);
        assert_eq!(data, vec!["amy", "Todd"]);
    }
}
