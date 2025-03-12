use std::ops::{Deref, DerefMut};

use bytes::{Buf, BytesMut};

use super::Codec;

pub struct BaseBufQueue {
    queue: BytesMut,
    missed: bool,
}

impl Default for BaseBufQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for BaseBufQueue {
    type Target = BytesMut;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl DerefMut for BaseBufQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}

impl BaseBufQueue {
    pub fn new() -> Self {
        Self {
            queue: BytesMut::new(),
            missed: false,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.queue.as_ref()
    }

    pub fn as_buf(&self) -> &BytesMut {
        &self.queue
    }

    pub fn mut_buf(&mut self) -> &mut BytesMut {
        &mut self.queue
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn advance(&mut self, cnt: usize) {
        self.queue.advance(cnt);
    }

    pub fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.queue.extend_from_slice(bytes);
    }

    pub fn missed(&mut self) {
        self.missed = true
    }

    pub fn is_missed(&self) -> bool {
        self.missed
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

#[derive(Default)]
pub struct BufQueue {
    queue: BaseBufQueue,
}

impl BufQueue {
    pub fn encode<T: Codec>(&mut self, item: T) {
        let result = item.encode();
        match result {
            Ok(value) => {
                self.extend_from_slice(&value);
            }
            Err(err) => {
                if let bincode::error::EncodeError::UnexpectedEnd = err {
                    log::error!("Unexpected end while encoding on BufQueue");
                }
                self.missed();
            }
        }
    }

    // when returns error with true, close connection
    pub fn decode<T: Codec>(&mut self) -> Result<T, bool> {
        let result = T::decode(self.as_ref());
        match result {
            Ok((item, cnt)) => {
                self.advance(cnt);
                Ok(item)
            }
            Err(err) => {
                if let bincode::error::DecodeError::UnexpectedEnd { .. } = err {
                    Err(false)
                } else {
                    self.missed();
                    Err(true)
                }
            }
        }
    }
}

impl Deref for BufQueue {
    type Target = BaseBufQueue;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl DerefMut for BufQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}

#[derive(Default)]
pub struct TypedBufQueue<T: Codec> {
    queue: BufQueue,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Codec> TypedBufQueue<T> {
    pub fn encode(&mut self, item: T) {
        self.queue.encode(item);
    }

    pub fn decode(&mut self) -> Result<T, bool> {
        self.queue.decode()
    }
}

impl<T: Codec> Deref for TypedBufQueue<T> {
    type Target = BaseBufQueue;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl<T: Codec> DerefMut for TypedBufQueue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}
