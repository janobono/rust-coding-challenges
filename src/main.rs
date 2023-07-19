use {
    chrono::Local,
    crate::has_a_deadline_been_reached::{Deadline, ImportantEvent},
    rust_coding_challenges::*,
};

mod has_a_deadline_been_reached;

fn main() {
    let data = [1.0, 1.4, 1.5];
    println!("calculate_the_median({:?}) = {}", &data, calculate_the_median(&data).unwrap_or(-1.0));

    let data = [4, 4, 3, 2, 1, 3];
    println!("find_unique_items({:?})={:?}", &data, find_unique_items(&data));

    print_any_text_type("Text as slice");
    print_any_text_type(&String::from("Text as String"));

    let mut data = vec!["Todd", "amy"];
    case_insensitive_sort(&mut data);
    println!("Sorted data {:?}", data);

    print_morse_code("Hello world!");

    let event = ImportantEvent {
        what: String::from("Hello event"),
        when: Local::now().naive_local().date(),
    };
    println!("This event {} is today {} is passed {}", &event.what, event.is_today(), event.is_passed());
}
