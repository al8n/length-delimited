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
  #[cfg(not(any(feature = "send", feature = "sync", feature = "static")))]
  pub trait Bounds {}

  #[cfg(not(any(feature = "send", feature = "sync", feature = "static")))]
  impl<T> Bounds for T {}

  #[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
  pub trait Bounds: Sync {}

  #[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
  impl<T> Bounds for T where T: Sync {}

  #[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
  pub trait Bounds: Send {}

  #[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
  impl<T> Bounds for T where T: Send {}

  #[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
  pub trait Bounds: 'static {}

  #[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
  impl<T> Bounds for T where T: 'static {}

  #[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
  pub trait Bounds: Send + Sync {}

  #[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
  impl<T> Bounds for T where T: Send + Sync {}

  #[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
  pub trait Bounds: Sync + 'static {}

  #[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
  impl<T> Bounds for T where T: Sync + 'static {}

  #[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
  pub trait Bounds: Send + 'static {}

  #[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
  impl<T> Bounds for T where T: Send + 'static {}

  /// A trait bound that bounds `Send`, `Sync`, and `'static` in possible combinations
  ///
  /// which can be configured with features.
  #[cfg(all(feature = "send", feature = "sync", feature = "static"))]
  pub trait Bounds: Send + Sync + 'static {}

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
  pub trait ErrorBounds: Printable {}

  #[cfg(not(any(feature = "send", feature = "sync", feature = "static",)))]
  impl<T> ErrorBounds for T where T: Printable {}

  #[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
  pub trait ErrorBounds: Printable + Sync {}

  #[cfg(all(feature = "sync", not(any(feature = "send", feature = "static"))))]
  impl<T> ErrorBounds for T where T: Printable + Sync {}

  #[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
  pub trait ErrorBounds: Printable + Send {}

  #[cfg(all(feature = "send", not(any(feature = "sync", feature = "static"))))]
  impl<T> ErrorBounds for T where T: Printable + Send {}

  #[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
  pub trait ErrorBounds: Printable + 'static {}

  #[cfg(all(feature = "static", not(any(feature = "send", feature = "sync"))))]
  impl<T> ErrorBounds for T where T: Printable + 'static {}

  #[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
  pub trait ErrorBounds: Printable + Send + Sync {}

  #[cfg(all(feature = "sync", feature = "send", not(feature = "static")))]
  impl<T> ErrorBounds for T where T: Printable + Send + Sync {}

  #[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
  pub trait ErrorBounds: Printable + Sync + 'static {}

  #[cfg(all(feature = "sync", feature = "static", not(feature = "send")))]
  impl<T> ErrorBounds for T where T: Printable + Sync + 'static {}

  #[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
  pub trait ErrorBounds: Printable + Send + 'static {}

  #[cfg(all(feature = "send", feature = "static", not(feature = "sync")))]
  impl<T> ErrorBounds for T where T: Printable + Send + 'static {}

  /// A trait bound that bounds `Debug`, `Display`, `Error`, `Send`, `Sync`, and `'static` in possible combinations
  ///
  /// which can be configured with features.
  #[cfg(all(feature = "send", feature = "sync", feature = "static"))]
  pub trait ErrorBounds: Printable + Send + Sync + 'static {}

  #[cfg(all(feature = "send", feature = "sync", feature = "static"))]
  impl<T> ErrorBounds for T where T: Printable + Send + Sync + 'static {}
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
