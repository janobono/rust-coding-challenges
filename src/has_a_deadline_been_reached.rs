use {
    chrono::Local,
    chrono::naive::NaiveDate,
};

pub struct ImportantEvent {
    pub what: String,
    pub when: NaiveDate,
}

pub trait Deadline {
    fn is_passed(&self) -> bool;
    fn is_today(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.when < Local::now().naive_local().date()
    }

    fn is_today(&self) -> bool {
        self.when == Local::now().naive_local().date()
    }
}

#[cfg(test)]
mod test {
    use {
        chrono::Local,
        chrono::naive::Days,
        crate::has_a_deadline_been_reached::*,
    };

    #[test]
    fn future_date() {
        let now = Local::now().naive_local().date();
        let event = ImportantEvent {
            what: String::from("you have a lot of time"),
            when: now.checked_add_days(Days::new(1)).unwrap(),
        };
        assert_eq!(event.is_passed(), false);
        assert_eq!(event.is_today(), false);
    }

    #[test]
    fn today_date() {
        let now = Local::now().naive_local().date();
        let event = ImportantEvent {
            what: String::from("it's today"),
            when: now,
        };
        assert_eq!(event.is_passed(), false);
        assert_eq!(event.is_today(), true);
    }

    #[test]
    fn past_date() {
        let now = Local::now().naive_local().date();
        let event = ImportantEvent {
            what: String::from("you missed it"),
            when: now.checked_sub_days(Days::new(1)).unwrap(),
        };
        assert_eq!(event.is_passed(), true);
        assert_eq!(event.is_today(), false);
    }
}
