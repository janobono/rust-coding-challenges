use rust_coding_challenges::{calculate_the_median, find_unique_items};

fn main() {
    let data = [1.0, 1.4, 1.5];
    println!("calculate_the_median({:?}) = {}", &data, calculate_the_median(&data).unwrap_or(-1.0));

    let data = [4, 4, 3, 2, 1, 3];
    println!("find_unique_items({:?})={:?}", &data, find_unique_items(&data));
}
