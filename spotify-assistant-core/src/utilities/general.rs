pub fn uniques(uniques: u32) -> u32 {
    let combos = uniques.pow(2);
    println!("# of combos: {}", combos);
    combos
}
pub fn pair_vector(number_of_uniques: usize) -> Vec<(usize, usize)> {
    (0..number_of_uniques).flat_map(|i| {
        (i + 1..number_of_uniques).map(|j| {
            (i, j)
        }).collect::<Vec<(usize, usize)>>()
    }).collect()
}
pub fn print_separator() {
    if let Some((width, _)) = term_size::dimensions() {
        let separator = "-".repeat(width);
        println!("{}", separator);
    } else {
        // Fallback if the terminal size can't be determined
        println!("{}", "-".repeat(80));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_vec() {
        // let test = Testing::new(1, 2);

        assert_eq!(pair_vector(8).len(), 16);
    }
}
