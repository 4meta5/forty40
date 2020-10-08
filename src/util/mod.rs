pub mod perm;
pub mod shift;
use perm::Permutation;
use shift::*;

const MAX_BUCKET_SIZE: usize = 16;

pub trait MergeShuffle<T> {
    type Perm;
    fn merge_shuffle(&self) -> Vec<T>;
    fn permute(&self) -> Option<Self::Perm>;
}

impl<T: Clone> MergeShuffle<T> for Permutation<T> {
    type Perm = Vec<Vec<T>>;
    fn merge_shuffle(&self) -> Vec<T> {
        todo!()
    }
    fn permute(&self) -> Option<Self::Perm> {
        todo!()
    }
}
