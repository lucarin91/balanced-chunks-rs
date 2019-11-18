use std::mem;

// Iterators definition

#[derive(Debug)]
pub struct BalancedChunks<'a, T: 'a> {
    v: &'a [T],
    chunk_size: usize,
    remainder: usize,
}

impl<'a, T> Iterator for BalancedChunks<'a, T> {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.v.is_empty() {
            None
        } else {
            let sz = self.chunk_size
                + if self.remainder > 0 {
                    self.remainder -= 1;
                    1
                } else {
                    0
                };
            let tmp = mem::replace(&mut self.v, &[]);
            let (head, tail) = tmp.split_at(sz);
            self.v = tail;
            Some(head)
        }
    }
}

#[derive(Debug)]
pub struct BalancedChunksMut<'a, T: 'a> {
    v: &'a mut [T],
    chunk_size: usize,
    remainder: usize,
}

impl<'a, T> Iterator for BalancedChunksMut<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<&'a mut [T]> {
        if self.v.is_empty() {
            None
        } else {
            let sz = self.chunk_size
                + if self.remainder > 0 {
                    self.remainder -= 1;
                    1
                } else {
                    0
                };
            let tmp = mem::replace(&mut self.v, &mut []);
            let (head, tail) = tmp.split_at_mut(sz);
            self.v = tail;
            Some(head)
        }
    }
}

// Traits Implements for Vec and Slice

pub trait BalancedChunksExt<T> {
    fn balanced_chunks(&self, chunk_num: usize) -> BalancedChunks<'_, T>;
}

impl<T> BalancedChunksExt<T> for Vec<T> {
    #[inline]
    fn balanced_chunks(&self, chunk_num: usize) -> BalancedChunks<'_, T> {
        assert!(chunk_num != 0);
        let iter_size = self.len();
        let chunk_size = iter_size / chunk_num;
        let remainder = iter_size % chunk_num;
        BalancedChunks {
            v: self,
            chunk_size,
            remainder,
        }
    }
}

impl<T> BalancedChunksExt<T> for &[T] {
    #[inline]
    fn balanced_chunks(&self, chunk_num: usize) -> BalancedChunks<'_, T> {
        assert!(chunk_num != 0);
        let iter_size = self.len();
        let chunk_size = iter_size / chunk_num;
        let remainder = iter_size % chunk_num;
        BalancedChunks {
            v: self,
            chunk_size,
            remainder,
        }
    }
}

pub trait BalancedChunksMutExt<T> {
    fn balanced_chunks_mut(&mut self, chunk_num: usize) -> BalancedChunksMut<'_, T>;
}

impl<T> BalancedChunksMutExt<T> for Vec<T> {
    #[inline]
    fn balanced_chunks_mut(&mut self, chunk_num: usize) -> BalancedChunksMut<'_, T> {
        assert!(chunk_num != 0);
        let iter_size = self.len();
        let chunk_size = iter_size / chunk_num;
        let remainder = iter_size % chunk_num;
        BalancedChunksMut {
            v: self,
            chunk_size,
            remainder,
        }
    }
}

impl<T> BalancedChunksMutExt<T> for &mut [T] {
    #[inline]
    fn balanced_chunks_mut(&mut self, chunk_num: usize) -> BalancedChunksMut<'_, T> {
        assert!(chunk_num != 0);
        let iter_size = self.len();
        let chunk_size = iter_size / chunk_num;
        let remainder = iter_size % chunk_num;
        BalancedChunksMut {
            v: self,
            chunk_size,
            remainder,
        }
    }
}
