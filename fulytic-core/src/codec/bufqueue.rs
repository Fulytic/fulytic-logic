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

impl BaseBufQueue {
    pub fn new() -> Self {
        Self {
            queue: BytesMut::new(),
            missed: false,
        }
    }

    pub fn with(buf: BytesMut) -> Self {
        Self {
            queue: buf,
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

    pub fn split(&mut self) -> BytesMut {
        self.queue.split()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.queue.reserve(additional);
    }

    pub fn as_buf_queue(&self) -> &BufQueue {
        unsafe { &*(self as *const BaseBufQueue as *const BufQueue) }
    }

    pub fn as_mut_buf_queue(&mut self) -> &mut BufQueue {
        unsafe { &mut *(self as *mut BaseBufQueue as *mut BufQueue) }
    }

    pub fn as_typed_buf_queue<T: Codec>(&self) -> &TypedBufQueue<T> {
        unsafe { &*(self as *const BaseBufQueue as *const TypedBufQueue<T>) }
    }

    pub fn as_mut_typed_buf_queue<T: Codec>(&mut self) -> &mut TypedBufQueue<T> {
        unsafe { &mut *(self as *mut BaseBufQueue as *mut TypedBufQueue<T>) }
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct BufQueue {
    queue: BaseBufQueue,
}

impl BufQueue {
    pub fn with(buf: BytesMut) -> Self {
        Self {
            queue: BaseBufQueue::with(buf),
        }
    }

    pub fn encode<T: Codec>(&mut self, item: T) {
        if self.is_missed() {
            return;
        }
        match item.encode() {
            Some(value) => {
                self.extend_from_slice(&value);
            }
            None => {
                self.missed();
            }
        }
    }

    // when returns error with true, close connection
    pub fn decode<T: Codec>(&mut self) -> Result<T, bool> {
        if self.is_missed() {
            return Err(true);
        }
        match T::decode(self.as_slice()) {
            Ok((item, cnt)) => {
                self.advance(cnt);
                Ok(item)
            }
            Err(is_missed) => {
                if is_missed {
                    self.missed();
                }
                Err(is_missed)
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
#[repr(transparent)]
pub struct TypedBufQueue<T: Codec> {
    queue: BufQueue,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Codec> TypedBufQueue<T> {
    pub fn with(buf: BytesMut) -> Self {
        Self {
            queue: BufQueue::with(buf),
            _phantom: std::marker::PhantomData,
        }
    }

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
