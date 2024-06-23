#![feature(type_alias_impl_trait)]

use std::io::Write;

use bytes::Bytes;
use std::future::Future;

pub trait KvIterator {
    type NextFuture<'a>: Future<Output = Option<(&'a [u8], &'a [u8])>>
    where
        Self: 'a;

    /// Get the next item from the iterator.
    fn next<'x>(&'x mut self) -> Self::NextFuture<'_>;
}

pub struct TestIterator {
    idx: usize,
    to_idx: usize,
    key: Vec<u8>,
    value: Vec<u8>,
}

impl TestIterator {
    pub fn new(from_idx: usize, to_idx: usize) -> Self {
        Self {
            idx: from_idx,
            to_idx,
            key: Vec::new(),
            value: Vec::new(),
        }
    }
}
impl KvIterator for TestIterator {
    type NextFuture<'a> = impl Future<Output = Option<(&'a [u8], &'a [u8])>> where Self: 'a;

    fn next<'x>(&'x mut self) -> Self::NextFuture<'_> {
        async move {
            if self.idx >= self.to_idx {
                return None;
            }

            // Zero-allocation key value manipulation

            self.key.clear();
            write!(&mut self.key, "key_{:05}", self.idx).unwrap();

            self.value.clear();
            write!(&mut self.value, "value_{:05}", self.idx).unwrap();

            self.idx += 1;
            Some((&self.key[..], &self.value[..]))
        }
    }
}

pub struct ConcatIterator<Iter: KvIterator> {
    iters: Vec<Iter>,
    current_idx: usize,
}

impl<Iter: KvIterator> ConcatIterator<Iter> {
    pub fn new(iters: Vec<Iter>) -> Self {
        Self {
            iters,
            current_idx: 0,
        }
    }
}

impl<Iter: KvIterator> KvIterator for ConcatIterator<Iter> {
    type NextFuture<'a> = impl Future<Output = Option<(&'a [u8], &'a [u8])>> where Self: 'a;

    fn next<'x>(&'x mut self) -> Self::NextFuture<'_> {
        let l = self.iters.len();

        async move {
            loop {
                if self.current_idx >= l {
                    return None;
                }

                let iter = &mut self.iters[self.current_idx];

                let res = iter.next().await;

                if res.is_some() {
                    return res;
                }

                self.current_idx += 1;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut iter = TestIterator::new(0, 10);
    while let Some((key, value)) = iter.next().await {
        println!(
            "{:?} {:?}",
            Bytes::copy_from_slice(key),
            Bytes::copy_from_slice(value)
        );
    }
}
