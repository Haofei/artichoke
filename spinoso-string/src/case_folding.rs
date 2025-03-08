/// Tracks whether a case-folding operation changed any bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseFoldingEffect {
    /// No bytes have changed so far.
    Unchanged,
    /// We have changed at least one byte.
    Modified,
}

impl CaseFoldingEffect {
    /// Returns `true` if at least one byte was changed.
    pub const fn changed(&self) -> bool {
        matches!(self, Self::Modified)
    }
}
