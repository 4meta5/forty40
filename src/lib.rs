mod list;
mod perm;
mod set;
pub use list::List;
pub use perm::Permutations;
pub use set::MultiSet;

/// Pass in non-increasing multiset, consumed by .tail() on multiset
pub fn init<T>(tail: Vec<T>) -> (usize, usize, usize) {
    todo!()
}
