use blake2::{
    digest::{typenum::U32, FixedOutput, Update},
    Blake2bMac,
};
use std::convert::TryInto;
use std::io::{Error, ErrorKind};

use crate::constants::DISCOVERY_NS_BUF;
use crate::DiscoveryKey;

/// Calculate the discovery key of a key.
///
/// The discovery key is a 32 byte namespaced hash of the key.
pub fn discovery_key(key: &[u8]) -> DiscoveryKey {
    let mut hasher = Blake2bMac::<U32>::new_with_salt_and_personal(key, &[], &[]).unwrap();
    hasher.update(DISCOVERY_NS_BUF);
    hasher.finalize_fixed().as_slice().try_into().unwrap()
}

pub(crate) fn pretty_hash(key: &[u8]) -> String {
    pretty_hash::fmt(key).unwrap_or_else(|_| "<invalid>".into())
}

pub(crate) fn map_channel_err<T>(err: async_channel::SendError<T>) -> Error {
    Error::new(
        ErrorKind::BrokenPipe,
        format!("Cannot forward on channel: {err}"),
    )
}

pub(crate) const UINT_24_LENGTH: usize = 3;

#[inline]
pub(crate) fn wrap_uint24_le(data: &Vec<u8>) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![0; 3];
    let n = data.len();
    write_uint24_le(n, &mut buf);
    buf.extend(data);
    buf
}

#[inline]
pub(crate) fn write_uint24_le(n: usize, buf: &mut [u8]) {
    buf[0] = (n & 255) as u8;
    buf[1] = ((n >> 8) & 255) as u8;
    buf[2] = ((n >> 16) & 255) as u8;
}

#[inline]
pub(crate) fn stat_uint24_le(buffer: &[u8]) -> Option<(usize, u64)> {
    if buffer.len() >= 3 {
        let len =
            ((buffer[0] as u32) | ((buffer[1] as u32) << 8) | ((buffer[2] as u32) << 16)) as u64;
        Some((UINT_24_LENGTH, len))
    } else {
        None
    }
}
