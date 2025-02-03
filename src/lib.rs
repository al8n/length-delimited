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

mod sealed {
  macro_rules! doc {
    (@bounds { $($item:item)* }) => {
      $(
        /// A trait bound that bounds `Send`, `Sync`, and `'static` in possible combinations
        ///
        /// which can be configured with features.
        $item
      )*
    };
    (@error_bounds { $($item:item)* }) => {
      $(
        /// A trait bound that bounds `Debug`, `Display`, `Error`, `Send`, `Sync`, and `'static` in possible combinations
        ///
        /// which can be configured with features.
        $item
      )*
    };
  }

  doc!(@bounds {
    #[cfg(not(any(feature = "send", feature = "sync", feature = "static")))]
    pub trait Bounds {}

    #[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
    pub trait Bounds: Sync {}

    #[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
    pub trait Bounds: Send {}

    #[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
    pub trait Bounds: 'static {}

    #[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
    pub trait Bounds: Send + Sync {}

    #[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
    pub trait Bounds: Sync + 'static {}

    #[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
    pub trait Bounds: Send + 'static {}

    #[cfg(all(feature = "send", feature = "sync", feature = "static"))]
    pub trait Bounds: Send + Sync + 'static {}
  });

  #[cfg(not(any(feature = "send", feature = "sync", feature = "static")))]
  impl<T> Bounds for T {}

  #[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
  impl<T> Bounds for T where T: Sync {}

  #[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
  impl<T> Bounds for T where T: Send {}

  #[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
  impl<T> Bounds for T where T: 'static {}

  #[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
  impl<T> Bounds for T where T: Send + Sync {}

  #[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
  impl<T> Bounds for T where T: Sync + 'static {}

  #[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
  impl<T> Bounds for T where T: Send + 'static {}

  #[cfg(all(feature = "send", feature = "sync", feature = "static"))]
  impl<T> Bounds for T where T: Send + Sync + 'static {}

  #[cfg(not(any(feature = "debug", feature = "display", feature = "error")))]
  pub trait Printable {}

  #[cfg(not(any(feature = "debug", feature = "display", feature = "error")))]
  impl<T> Printable for T {}

  #[cfg(all(feature = "debug", not(any(feature = "display", feature = "error"))))]
  pub trait Printable: core::fmt::Debug {}

  #[cfg(all(feature = "debug", not(any(feature = "display", feature = "error"))))]
  impl<T> Printable for T where T: core::fmt::Debug {}

  #[cfg(all(feature = "display", not(any(feature = "debug", feature = "error"))))]
  pub trait Printable: core::fmt::Display {}

  #[cfg(all(feature = "display", not(any(feature = "debug", feature = "error"))))]
  impl<T> Printable for T where T: core::fmt::Display {}

  #[cfg(all(feature = "display", feature = "debug", not(any(feature = "error"))))]
  pub trait Printable: core::fmt::Display + core::fmt::Debug {}

  #[cfg(all(feature = "display", feature = "debug", not(any(feature = "error"))))]
  impl<T> Printable for T where T: core::fmt::Display + core::fmt::Debug {}

  #[cfg(feature = "error")]
  pub trait Printable: core::error::Error {}

  #[cfg(feature = "error")]
  impl<T> Printable for T where T: core::error::Error {}

  #[cfg(not(any(feature = "send", feature = "sync", feature = "static",)))]
  impl<T> ErrorBounds for T where T: Printable {}

  #[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
  impl<T> ErrorBounds for T where T: Printable + Sync {}

  #[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
  impl<T> ErrorBounds for T where T: Printable + Send {}

  #[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
  impl<T> ErrorBounds for T where T: Printable + 'static {}

  #[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
  impl<T> ErrorBounds for T where T: Printable + Send + Sync {}

  #[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
  impl<T> ErrorBounds for T where T: Printable + Sync + 'static {}

  #[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
  impl<T> ErrorBounds for T where T: Printable + Send + 'static {}

  #[cfg(all(feature = "send", feature = "sync", feature = "static"))]
  impl<T> ErrorBounds for T where T: Printable + Send + Sync + 'static {}

  doc!(@error_bounds {
    #[cfg(not(any(feature = "send", feature = "sync", feature = "static",)))]
    pub trait ErrorBounds: Printable {}

    #[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
    pub trait ErrorBounds: Printable + Sync {}

    #[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
    pub trait ErrorBounds: Printable + Send {}

    #[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
    pub trait ErrorBounds: Printable + 'static {}

    #[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
    pub trait ErrorBounds: Printable + Send + Sync {}

    #[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
    pub trait ErrorBounds: Printable + Sync + 'static {}

    #[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
    pub trait ErrorBounds: Printable + Send + 'static {}

    #[cfg(all(feature = "send", feature = "sync", feature = "static"))]
    pub trait ErrorBounds: Printable + Send + Sync + 'static {}
  });
}

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
  use core::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddrV4},
    num::NonZeroUsize,
  };

  use super::*;
  use quickcheck_macros::quickcheck;

  #[cfg(feature = "std")]
  extern crate std;

  #[cfg(all(not(feature = "std"), feature = "alloc"))]
  extern crate alloc as std;

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
      if let Ok((read, decoded)) = T::decode_length_delimited(&buffer[..written]) {
        return read == written && value == decoded;
      }
    }

    false
  }

  // Primitive type roundtrip tests
  #[quickcheck]
  fn fuzz_u8_roundtrip(x: u8) -> bool {
    test_roundtrip(x)
  }

  #[quickcheck]
  fn fuzz_u16_roundtrip(x: u16) -> bool {
    test_roundtrip(x)
  }

  #[quickcheck]
  fn fuzz_u32_roundtrip(x: u32) -> bool {
    test_roundtrip(x)
  }

  #[quickcheck]
  fn fuzz_u64_roundtrip(x: u64) -> bool {
    test_roundtrip(x)
  }

  #[quickcheck]
  fn fuzz_i8_roundtrip(x: i8) -> bool {
    test_roundtrip(x)
  }

  #[quickcheck]
  fn fuzz_i16_roundtrip(x: i16) -> bool {
    test_roundtrip(x)
  }

  #[quickcheck]
  fn fuzz_i32_roundtrip(x: i32) -> bool {
    test_roundtrip(x)
  }

  #[quickcheck]
  fn fuzz_i64_roundtrip(x: i64) -> bool {
    test_roundtrip(x)
  }

  #[quickcheck]
  fn fuzz_bool_roundtrip(x: bool) -> bool {
    test_roundtrip(x)
  }

  #[quickcheck]
  fn fuzz_char_roundtrip(x: char) -> bool {
    test_roundtrip(x)
  }

  // String roundtrip test
  #[cfg(any(feature = "std", feature = "alloc"))]
  #[quickcheck]
  fn fuzz_string_roundtrip(s: String) -> bool {
    test_roundtrip(s)
  }

  // Network type roundtrip tests
  #[quickcheck]
  fn fuzz_ipv4_roundtrip(val: Ipv4Addr) -> bool {
    test_roundtrip(val)
  }

  #[quickcheck]
  #[allow(clippy::too_many_arguments)]
  fn fuzz_ipv6_roundtrip(val: Ipv6Addr) -> bool {
    test_roundtrip(val)
  }

  #[quickcheck]
  fn fuzz_socketv4_roundtrip(a: u8, b: u8, c: u8, d: u8, port: u16) -> bool {
    test_roundtrip(SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), port))
  }

  // Byte array roundtrip tests
  #[quickcheck]
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn fuzz_bytes_roundtrip(bytes: Vec<u8>) -> bool {
    test_roundtrip(bytes)
  }

  // Fixed size array roundtrip test
  #[quickcheck]
  fn fuzz_fixed_array_roundtrip(a: u8, b: u8, c: u8, d: u8) -> bool {
    test_roundtrip([a, b, c, d])
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
