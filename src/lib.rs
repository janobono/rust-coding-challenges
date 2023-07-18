mod calculate_the_median;
mod find_unique_items;

pub fn calculate_the_median(data: &[f32]) -> Option<f32> {
    calculate_the_median::calculate_the_median(data)
}

pub fn find_unique_items<T>(data: &[T]) -> Vec<T>
    where
        T: PartialEq + Clone
{
    find_unique_items::find_unique_items(data)
}
