use chrono::NaiveDate;

static FORMAT: &str = "%Y-%m-%d";

pub fn weeks_between(start: &str, end: &str) -> i32 {
    let start_date = NaiveDate::parse_from_str(start, FORMAT).unwrap();
    let end_date = NaiveDate::parse_from_str(end, FORMAT).unwrap();

    ((end_date - start_date).num_days() / 7) as i32
}

#[cfg(test)]
mod test {
    use crate::calculate_the_number_of_weeks_between_two_dates::weeks_between;

    #[test]
    fn start_is_before_end() {
        let start = "2023-07-10";
        let end = "2023-07-30";
        assert_eq!(weeks_between(start, end), 2);
    }

    #[test]
    fn start_is_after_end() {
        let start = "2023-07-30";
        let end = "2023-07-10";
        assert_eq!(weeks_between(start, end), -2);
    }
}
