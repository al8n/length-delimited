#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

pub use dbutils::{
  error::{IncompleteBuffer, InsufficientBuffer},
  leb128::*,
};

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

pub use sealed::{Bounds, ErrorBounds};

mod sealed;

/// A type can be encode to a buffer.
pub trait LengthDelimitedEncoder {
  /// The encode error type that can be returned when encoding the type.
  type Error;

  /// Returns the encoded length of the message without a length delimiter.
  fn encoded_len(&self) -> usize;

  /// Returns the encoded length of the message with a length delimiter.
  fn encoded_length_delimited_len(&self) -> usize;

  /// Encodes the message with a length-delimiter to a buffer.
  ///
  /// An error will be returned if the buffer does not have sufficient capacity.
  fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error>;

  /// Encodes the message to a buffer.
  ///
  /// An error will be returned if the buffer does not have sufficient capacity.
  fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

/// A type can be encode to a buffer.
pub trait LengthDelimitedDecoder: core::fmt::Debug {
  /// The decode error type that can be returned when decoding the type.
  type Error;

  /// Decodes an instance of the message from a buffer.
  ///
  /// The entire buffer will be consumed.
  fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
  where
    Self: Sized;

  /// Decodes a length-delimited instance of the message from the buffer.
  fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
  where
    Self: Sized;
}

macro_rules! impl_length_delimited {
  ($($src:ty as $dst:ty), +$(,)?) => {
    impl_length_delimited!(@encoder $($src as $dst), +);
    impl_length_delimited!(@decoder $($src as $dst), +);
  };
  (@encoder $($src:ty as $dst:ty), +$(,)?) => {
    $(
      impl $crate::LengthDelimitedEncoder for $src {
        type Error = <$dst as LengthDelimitedEncoder>::Error;

        fn encoded_len(&self) -> usize {
          <$dst as LengthDelimitedEncoder>::encoded_len(&(*self as $dst))
        }

        fn encoded_length_delimited_len(&self) -> usize {
          <$dst as LengthDelimitedEncoder>::encoded_length_delimited_len(&(*self as $dst))
        }

        fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          <$dst as LengthDelimitedEncoder>::encode_length_delimited(&(*self as $dst), buf)
        }

        fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          <$dst as LengthDelimitedEncoder>::encode(&(*self as $dst), buf)
        }
      }
    )*
  };
  (@decoder $($src:ty as $dst:ty), +$(,)?) => {
    $(
      impl $crate::LengthDelimitedDecoder for $src {
        type Error = <$dst as $crate::LengthDelimitedDecoder>::Error;

        fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized,
        {
          <$dst as LengthDelimitedDecoder>::decode(src).map(|(read, val)| (read, val as $src))
        }

        fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized,
        {
          <$dst as LengthDelimitedDecoder>::decode_length_delimited(src).map(|(read, val)| (read, val as $src))
        }
      }
    )*
  };
}

mod bytes;
mod net;
mod primitives;
mod string;

pub use bytes::*;
pub use primitives::*;
pub use string::*;

#[cfg(test)]
mod fuzz_tests {
  use core::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

  use super::*;
  use quickcheck_macros::quickcheck;

  #[cfg(feature = "std")]
  extern crate std;

  #[cfg(all(not(feature = "std"), feature = "alloc"))]
  extern crate alloc as std;

  #[cfg(any(feature = "std", feature = "alloc"))]
  use core::num::NonZeroUsize;
  #[cfg(any(feature = "std", feature = "alloc"))]
  use std::{string::String, vec, vec::Vec};

  // Helper function to test roundtrip property
  fn test_roundtrip<T>(value: T) -> bool
  where
    T: LengthDelimitedEncoder + LengthDelimitedDecoder + std::fmt::Debug + PartialEq,
    T: Clone,
    <T as LengthDelimitedDecoder>::Error: std::fmt::Debug,
    <T as LengthDelimitedEncoder>::Error: std::fmt::Debug,
  {
    let mut buffer = vec![0u8; value.encoded_length_delimited_len()];

    // Test length delimited encoding/decoding
    if let Ok(written) = value.encode_length_delimited(&mut buffer) {
      if written != value.encoded_length_delimited_len() {
        return false;
      }

      if let Ok((read, decoded)) = T::decode_length_delimited(&buffer[..written]) {
        return read == written && value == decoded;
      }
    }

    false
  }

  macro_rules! roundtrip {
    ($(
      $(#[$meta:meta])*
      $ty:ty
    ), +$(,)?) => {
      paste::paste! {
        $(
          #[quickcheck]
          $(#[$meta])*
          fn [<fuzz_ $ty:snake _roundtrip>](val: $ty) -> bool {
            test_roundtrip(val)
          }
        )*
      }
    };
  }

  roundtrip!(
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
    f32, f64,
    bool,
    char,
    Ipv4Addr, Ipv6Addr,
    SocketAddrV4, SocketAddrV6,
    #[cfg(any(feature = "std", feature = "alloc"))]
    String,
  );

  #[quickcheck]
  fn fuzz_fixed_array_roundtrip(a: u8, b: u8, c: u8, d: u8) -> bool {
    test_roundtrip([a, b, c, d])
  }

  #[quickcheck]
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn fuzz_bytes_roundtrip(bytes: Vec<u8>) -> bool {
    test_roundtrip(bytes)
  }

  // Test encoding with insufficient buffer
  #[quickcheck]
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn fuzz_insufficient_buffer(value: String, trim: NonZeroUsize) -> bool {
    let required_len = value.encoded_length_delimited_len();
    let trim: usize = trim.into();
    if trim >= required_len {
      return true; // Skip test if trim would make buffer larger than needed
    }

    let mut buffer = vec![0u8; required_len - trim];
    value.encode_length_delimited(&mut buffer).is_err()
  }

  // Test decoding with incomplete data
  #[quickcheck]
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn fuzz_incomplete_decode(value: String, trim: NonZeroUsize) -> bool {
    let mut buffer = vec![0u8; value.encoded_length_delimited_len()];
    let trim: usize = trim.into();
    if let Ok(written) = value.encode_length_delimited(&mut buffer) {
      if trim >= written {
        return true; // Skip test if trim would remove entire buffer
      }

      <String>::decode_length_delimited(&buffer[..written - trim]).is_err()
    } else {
      false
    }
  }

  // Test with random buffer content
  #[quickcheck]
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn fuzz_random_buffer(buffer: Vec<u8>) -> bool {
    // Attempt to decode various types from random buffer
    // Should either succeed and roundtrip correctly, or fail gracefully
    let string_result = <String>::decode_length_delimited(&buffer);
    let u64_result = u64::decode_length_delimited(&buffer);
    let ipv4_result = Ipv4Addr::decode_length_delimited(&buffer);

    match (string_result, u64_result, ipv4_result) {
      (Ok((_, s)), _, _) => {
        // If we successfully decoded a string, verify roundtrip
        let mut new_buffer = vec![0u8; s.encoded_length_delimited_len()];
        if let Ok(written) = s.encode_length_delimited(&mut new_buffer) {
          if let Ok((read, decoded)) = String::decode_length_delimited(&new_buffer[..written]) {
            return read == written && s == decoded;
          }
        }
        false
      }
      (Err(_), _, _) => true, // Error is acceptable for random data
    }
  }
}
