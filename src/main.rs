use {
    chrono::Local,
    crate::check_if_a_file_exists::FileMetaData,
    crate::convert_between_celsius_and_fahrenheit::{Scale, Temperature},
    crate::has_a_deadline_been_reached::{Deadline, ImportantEvent},
    crate::travel_planner::{Graph, shortest_path},
    rust_coding_challenges::*,
    std::path::Path,
};

mod has_a_deadline_been_reached;
mod convert_between_celsius_and_fahrenheit;
mod check_if_a_file_exists;
mod interpret_an_rgb_hex_color;
mod travel_planner;

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

    let path = Path::new("./test.txt");
    println!("File {:?} exists = {}, is readable = {}, is writeable = {}", path, path.exists(), path.is_readable(), path.is_writeable());

    let text = "text";
    let encoded = encode(text);
    let decoded = decode(&encoded);
    println!("Data = {} Encoded = {} Decoded = {}", text, &encoded, &decoded);

    let dates = [
        "2002 Feb 02",
        "2010-12-11",
        "1999/March/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }

    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(
        &g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };

    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JRKUC
    ";
    let plaintext = decrypt(&ciphertext, key);
    println!("{}", plaintext);
}
