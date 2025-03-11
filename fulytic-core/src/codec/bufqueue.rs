use std::marker::PhantomData;

use bytes::BytesMut;

use super::Codec;

pub struct BufQueue<T: Codec> {
    queue: BytesMut,
    missed: bool,
    _phantom: PhantomData<T>,
}

impl<T: Codec> BufQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: BytesMut::new(),
            missed: false,
            _phantom: PhantomData,
        }
    }

    pub fn push(&mut self, item: T) {
        match item.encode() {
            Ok(encoded) => self.queue.extend_from_slice(&encoded),
            Err(_) => self.missed = true,
        }
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.queue.extend_from_slice(bytes);
    }

    pub fn split(&mut self) -> Option<BytesMut> {
        if self.missed {
            self.missed = false;
            Some(self.queue.split())
        } else {
            None
        }
    }
}

impl<T: Codec> Default for BufQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}
