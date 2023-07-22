use chrono::{NaiveDate, TimeZone};

fn is_year(field: &str) -> bool {
    field.len() == 4 && field.chars().all(|x| x.is_ascii_digit())
}

pub fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let text = text.trim();

    if !text.bytes().any(|x| x.is_ascii_digit()) {
        return None;
    }

    let fields: Vec<_> = text
        .split(['/', '-', '.', ' '].as_slice())
        .collect();

    let mut year = None;
    let mut month = None;
    let mut day = None;

    for field in fields.iter() {
        if field.len() < 3 {
            continue;
        }

        let m = match &field.to_lowercase()[..3] {
            "jan" => 1,
            "feb" => 2,
            "mar" => 3,
            "apr" => 4,
            "may" => 5,
            "jun" => 6,
            "jul" => 7,
            "aug" => 8,
            "sep" => 9,
            "oct" => 10,
            "nov" => 11,
            "dec" => 12,
            _ => continue,
        };

        month = Some(m)
    }

    for field in fields.iter() {
        if is_year(field) {
            year = field.parse::<i32>().ok();
            continue;
        }

        if month.is_some() {
            day = field.parse::<u32>().ok();
        } else {
            month = field.parse::<u32>().ok();
        }
    }

    match (year, month, day) {
        (Some(y), Some(m), None) => NaiveDate::from_ymd_opt(y, m, 1),
        (Some(y), Some(m), Some(d)) => NaiveDate::from_ymd_opt(y, m, d),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use {
        chrono::NaiveDate,
        crate::handle_inconsistent_dates::flexible_date_parse,
    };

    #[test]
    fn ymd_hyphen() {
        assert_eq!(flexible_date_parse("2010-12-11"), NaiveDate::from_ymd_opt(2010, 12, 11));
    }

    #[test]
    fn ymd_slash() {
        assert_eq!(flexible_date_parse("1999/Mar/02"), NaiveDate::from_ymd_opt(1999, 3, 2));
    }

    #[test]
    fn dmy_dot() {
        assert_eq!(flexible_date_parse("01.Mar.2021"), NaiveDate::from_ymd_opt(2021, 3, 1));
    }

    #[test]
    fn mdy_dot() {
        assert_eq!(flexible_date_parse("Apr.05.2021"), NaiveDate::from_ymd_opt(2021, 4, 5));
    }

    #[test]
    fn invalid() {
        assert_eq!(flexible_date_parse("not a date"), None);
    }
}
