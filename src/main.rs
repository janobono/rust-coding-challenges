use rust_coding_challenges::*;

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
}
