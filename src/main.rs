use rust_coding_challenges::calculate_the_median;

fn main() {
    // Menu is missing
    let data = vec![1.0, 1.4, 1.5];
    println!("median is {}", calculate_the_median(data).unwrap_or(-1.0));
}
