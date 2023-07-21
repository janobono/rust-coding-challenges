use crate::value_of_a_hand_of_cards::Hand;

mod calculate_the_median;
mod find_unique_items;
mod print_any_text_type;
mod case_insensitive_sort;
mod convert_text_to_morse_code;
mod value_of_a_hand_of_cards;
mod sum_a_list_of_numbers_with_missing_values;
mod calculate_the_number_of_weeks_between_two_dates;
mod validate_an_isbn_number;
mod check_if_a_file_exists;
mod run_length_encoding;

pub fn calculate_the_median(data: &[f32]) -> Option<f32> {
    calculate_the_median::calculate_the_median(data)
}

pub fn find_unique_items<T>(data: &[T]) -> Vec<T>
    where
        T: PartialEq + Clone
{
    find_unique_items::find_unique_items(data)
}

pub fn print_any_text_type<T: std::fmt::Display + ?Sized>(text: &T) {
    print_any_text_type::print_any_text_type(text);
}

pub fn case_insensitive_sort<T: AsRef<str>>(data: &mut Vec<T>) {
    case_insensitive_sort::case_insensitive_sort(data);
}

pub fn print_morse_code(data: &str) {
    convert_text_to_morse_code::print_morse_code(data);
}

pub fn count_value(data: &Hand) -> u8 {
    data.value()
}

pub fn sum_with_missing(data: &Vec<Option<i32>>) -> i32 {
    sum_a_list_of_numbers_with_missing_values::sum_with_missing(data)
}

pub fn weeks_between(start: &str, end: &str) -> i32 {
    calculate_the_number_of_weeks_between_two_dates::weeks_between(start, end)
}

pub fn validate_isbn(data: &str) -> bool {
    validate_an_isbn_number::validate_isbn(data)
}

pub fn encode(text: &str) -> String {
    run_length_encoding::encode(text)
}

pub fn decode(text: &str) -> String {
    run_length_encoding::decode(text)
}
