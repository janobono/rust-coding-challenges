use {
    chrono::Local,
    crate::convert_between_celsius_and_fahrenheit::{Scale, Temperature},
    crate::has_a_deadline_been_reached::{Deadline, ImportantEvent},
    rust_coding_challenges::*,
};

mod has_a_deadline_been_reached;
mod convert_between_celsius_and_fahrenheit;

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

    let temperature = Temperature::new(30.0, Scale::Celsius);
    println!("Temperature {} Celsius = {} Fahrenheit", temperature.to_celsius(), temperature.to_fahrenheit());
    let temperature = Temperature::new(86.0, Scale::Fahrenheit);
    println!("Temperature {} Celsius = {} Fahrenheit", temperature.to_celsius(), temperature.to_fahrenheit());

    let data = vec![Some(4), None, Some(1)];
    println!("Sum of {:?} = {}", &data, sum_with_missing(&data));

    let start = "2023-07-10";
    let end = "2023-07-30";
    println!("Weeks between {} - {} = {}", start, end, weeks_between(start, end));
    println!("Weeks between {} - {} = {}", end, start, weeks_between(end, start));

    let data = "978-3-16-148410-0";
    println!("Isbn number {} is valid {}", data, validate_isbn(data));
}
