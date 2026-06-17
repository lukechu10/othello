/// Represents the position on the game board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Play(pub u8);

impl Play {
    /// Create a new `Play` with specified `row` and `col`.
    pub fn new(row: u8, col: u8) -> Self {
        debug_assert!(row < 8);
        debug_assert!(col < 8);

        Self(row * 8 + col)
    }

    pub fn coords(&self) -> Option<(u8, u8)> {
        if self.0 < 64 {
            Some((self.0 / 8, self.0 % 8))
        } else {
            None
        }
    }
}

impl std::fmt::Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((row, col)) = self.coords() {
            let row = row + 1; // Convert to 1-based index for display
            let col = "abcdefgh".chars().nth(col as usize).unwrap(); // Convert to letter for display
            write!(f, "{row}{col}")
        } else {
            write!(f, "pass")
        }
    }
}
