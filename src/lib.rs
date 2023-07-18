mod calculate_the_median;
mod find_unique_items;
mod print_any_text_type;
mod case_insensitive_sort;

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
