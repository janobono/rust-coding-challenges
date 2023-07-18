pub fn print_any_text_type<T: std::fmt::Display + ?Sized>(text: &T) {
    println!("{}", text);
}

#[cfg(test)]
mod test {
    use crate::print_any_text_type::print_any_text_type;

    #[test]
    fn print_slice() {
        print_any_text_type("slice");
    }

    #[test]
    fn print_string() {
        print_any_text_type(&String::from("string"));
    }
}
