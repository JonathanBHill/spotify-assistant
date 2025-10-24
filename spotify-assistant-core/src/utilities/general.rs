/// Calculates the total number of combinations based on the given number of unique items.
///
/// This function takes the number of unique items (`uniques`) as input and computes
/// the square of that number to determine the total number of possible combinations.
/// It also prints the calculated number of combinations to the console.
///
/// # Arguments
///
/// * `uniques` - A `u32` representing the number of unique items.
///
/// # Returns
///
/// * A `u32` representing the total number of combinations.
///
/// # Example
///
/// ```
/// use spotify_assistant_core::utilities::general::uniques;
/// let unique_items = 4;
/// let total_combinations = uniques(unique_items);
/// assert_eq!(total_combinations, 16);
/// // This will also print: "# of combos: 16"
/// ```
pub fn uniques(uniques: u32) -> u32 {
    let combos = uniques.pow(2);
    println!("# of combos: {combos}");
    combos
}
/// Generates a vector of all unique pairs (tuples) of indices from a range.
///
/// The function takes a `number_of_uniques` integer which determines the range `[0, number_of_uniques)`.
/// It computes all possible combinations of unique pairs `(i, j)` where `i < j` within the range.
///
/// # Arguments
///
/// * `number_of_uniques` - The size of the range `[0, number_of_uniques)` for which unique pairs are generated.
///
/// # Returns
///
/// A `Vec<(usize, usize)>` containing all unique pairs `(i, j)` where each `i < j`
/// and both `i` and `j` belong to the range `[0, number_of_uniques)`.
///
/// # Examples
///
/// ```rust
/// use spotify_assistant_core::utilities::general::pair_vector;
/// let pairs = pair_vector(4);
/// // Returns: vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]
/// assert_eq!(pairs, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
/// ```
pub fn pair_vector(number_of_uniques: usize) -> Vec<(usize, usize)> {
    (0..number_of_uniques)
        .flat_map(|i| {
            (i + 1..number_of_uniques)
                .map(|j| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}
/// Prints a horizontal separator line to the terminal.
///
/// The function attempts to determine the width of the terminal using the `term_size` crate.
/// If the terminal's width can be determined, it prints a line of dashes (`-`) that spans the entire width of the terminal.
/// If the terminal's dimensions cannot be determined (e.g., when running in an environment without a terminal),
/// the function defaults to printing a line of dashes that is 80 characters wide.
pub trait TerminalSizeProvider {
    fn dimensions(&self) -> Option<(usize, usize)>;
}

struct DefaultTerminalSizeProvider;

impl TerminalSizeProvider for DefaultTerminalSizeProvider {
    fn dimensions(&self) -> Option<(usize, usize)> {
        term_size::dimensions()
    }
}

fn separator_line<P: TerminalSizeProvider>(provider: &P) -> String {
    let width = provider.dimensions().map(|(width, _)| width).unwrap_or(80);
    "-".repeat(width)
}

pub fn print_separator() {
    let separator = separator_line(&DefaultTerminalSizeProvider);
    println!("{separator}");
}

pub fn format_duration(duration: chrono::TimeDelta) -> String {
    let total_seconds = duration.num_seconds();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let dur = format!("{minutes:02}:{seconds:02}");
    println!("Formatted Duration: {dur}");
    dur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_vector_with_zero_items_is_empty() {
        assert!(pair_vector(0).is_empty());
        assert!(pair_vector(1).is_empty());
    }

    #[test]
    fn pair_vector_with_multiple_items_contains_all_unique_pairs() {
        let pairs = pair_vector(5);
        let expected = vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 4),
            (3, 4),
        ];
        assert_eq!(pairs, expected);
    }

    #[test]
    fn pair_vector_handles_larger_ranges() {
        let size = 100;
        let pairs = pair_vector(size);
        let expected_len = size * (size - 1) / 2;
        assert_eq!(pairs.len(), expected_len);
        assert_eq!(pairs.first().copied(), Some((0, 1)));
        assert_eq!(pairs.last().copied(), Some((98, 99)));
    }

    struct FixedSizeProvider(Option<(usize, usize)>);

    impl TerminalSizeProvider for FixedSizeProvider {
        fn dimensions(&self) -> Option<(usize, usize)> {
            self.0
        }
    }

    #[test]
    fn separator_uses_terminal_width_when_available() {
        let provider = FixedSizeProvider(Some((5, 10)));
        assert_eq!(separator_line(&provider), "-".repeat(5));
    }

    #[test]
    fn separator_falls_back_to_default_width_when_unavailable() {
        let provider = FixedSizeProvider(None);
        assert_eq!(separator_line(&provider), "-".repeat(80));
    }
}
