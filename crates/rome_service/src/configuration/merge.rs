/// Simple trait to merge two configuration types
pub trait MergeWith<T> {
    /// Merges one configuration type with another
    fn merge_with(&mut self, other: T);

    /// Merges one configuration type with another open
    fn merge_with_if(&mut self, other: T, condition: bool) {
        if condition {
            self.merge_with(other)
        }
    }
}
