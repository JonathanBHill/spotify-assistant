pub fn print_separator() {
    if let Some((width, _)) = term_size::dimensions() {
        let separator = "-".repeat(width);
        println!("{}", separator);
    } else {
        // Fallback if the terminal size can't be determined
        println!("{}", "-".repeat(80));
    }
}
