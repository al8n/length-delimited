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

  #[cfg(all(feature = "send", feature = "sync", feature = "static"))]
  pub trait ErrorBounds: Printable + Send + Sync + 'static {}

  #[cfg(all(feature = "send", feature = "sync", feature = "static"))]
  impl<T> ErrorBounds for T where T: Printable + Send + Sync + 'static {}

  pub trait Sealed {}

  macro_rules! impl_sealed {
    ($($ty:ident), +$(,)?) => {
      $(
        impl Sealed for $ty {}
      )*
    };
  }

  impl_sealed!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128,);
}

/// A type can be encode to a buffer.
pub trait LengthDelimitedEncoder: sealed::Bounds {
  /// The error type that can be returned when encoding the type.
  type Error: sealed::ErrorBounds;

  /// Returns the encoded length of the type.
  fn length_delimited<V>(&self) -> Result<usize, <V as TryFrom<usize>>::Error>
  where
    V: Varint;

  /// Encodes the type into the buffer.
  ///
  /// Returns the number of bytes written to the buffer.
  fn encode<V>(
    &self,
    buf: &mut [u8],
  ) -> Result<usize, LengthDelimitedError<<V as TryFrom<usize>>::Error, Self::Error>>
  where
    V: Varint;
}

/// The type can be encoded as a varint.
pub trait Varint: TryFrom<usize> + Copy + sealed::Sealed {
  /// The maximum value of this type.
  const MAX: Self;
  /// The maximum length of the varint.
  const MAX_VARINT_LEN: usize;

  /// Returns the fn that used to calculate the length of the varint.
  fn len() -> impl Fn(Self) -> usize
  where
    Self: Sized;

  /// Returns the encoder fn that used to encode the varint to the buffer.
  fn encoder() -> impl Fn(Self, &mut [u8]) -> Result<usize, InsufficientBuffer>
  where
    Self: Sized;

  /// Returns the decoder fn that used to decode the varint from the buffer.
  fn decoder() -> impl Fn(&[u8]) -> Result<(usize, Self), DecodeVarintError>
  where
    Self: Sized;
}

impl Varint for u8 {
  const MAX: Self = Self::MAX;
  const MAX_VARINT_LEN: usize = 1;

  #[inline]
  fn len() -> impl Fn(Self) -> usize {
    |_| 1
  }

  #[inline]
  fn encoder() -> impl Fn(Self, &mut [u8]) -> Result<usize, InsufficientBuffer> {
    |val, buf| {
      if buf.is_empty() {
        Err(InsufficientBuffer::new())
      } else {
        buf[0] = val;
        Ok(1)
      }
    }
  }

  #[inline]
  fn decoder() -> impl Fn(&[u8]) -> Result<(usize, Self), DecodeVarintError> {
    |buf| {
      if buf.is_empty() {
        Err(DecodeVarintError::IncompleteBuffer(
          IncompleteBuffer::with_information(1, 0),
        ))
      } else {
        Ok((1, buf[0]))
      }
    }
  }
}

impl Varint for i8 {
  const MAX: Self = Self::MAX;
  const MAX_VARINT_LEN: usize = 1;

  #[inline]
  fn len() -> impl Fn(Self) -> usize {
    |_| 1
  }

  #[inline]
  fn encoder() -> impl Fn(Self, &mut [u8]) -> Result<usize, InsufficientBuffer> {
    |val, buf| {
      if buf.is_empty() {
        Err(InsufficientBuffer::new())
      } else {
        buf[0] = val as u8;
        Ok(1)
      }
    }
  }

  #[inline]
  fn decoder() -> impl Fn(&[u8]) -> Result<(usize, Self), DecodeVarintError> {
    |buf| {
      if buf.is_empty() {
        Err(DecodeVarintError::IncompleteBuffer(
          IncompleteBuffer::with_information(1, 0),
        ))
      } else {
        Ok((1, buf[0] as i8))
      }
    }
  }
}

macro_rules! impl_varint {
  ($($ty:ident), +$(,)?) => {
    $(
      paste::paste! {
        impl Varint for $ty {
          const MAX: Self = Self::MAX;
          const MAX_VARINT_LEN: usize = dbutils::leb128::[<encoded_ $ty _varint_len>](Self::MAX);

          fn len() -> impl Fn(Self) -> usize {
            dbutils::leb128::[<encoded_ $ty _varint_len>]
          }

          fn encoder() -> impl Fn(Self, &mut [u8]) -> Result<usize, InsufficientBuffer> {
            dbutils::leb128::[<encode_ $ty _varint>]
          }

          fn decoder() -> impl Fn(&[u8]) -> Result<(usize, Self), DecodeVarintError> {
            dbutils::leb128::[<decode_ $ty _varint>]
          }
        }
      }
    )*
  };
}

impl_varint!(u16, u32, u64, u128, i16, i32, i64, i128);

/// The error that can be returned when encoding the length delimited type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LengthDelimitedError<V, L> {
  /// Failed converting from usize to the varint type.
  Varint(V),
  /// The encoding error.
  Encode(L),
}

impl<V, L> core::fmt::Display for LengthDelimitedError<V, L>
where
  V: core::fmt::Display,
  L: core::fmt::Display,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::Varint(e) => write!(f, "{e}"),
      Self::Encode(e) => write!(f, "{e}"),
    }
  }
}

impl<V, L> core::error::Error for LengthDelimitedError<V, L>
where
  V: core::fmt::Debug + core::fmt::Display,
  L: core::fmt::Debug + core::fmt::Display,
{
}

macro_rules! impl_length_delimited {
  (@bytes) => {
    type Error = InsufficientBuffer;

    fn length_delimited<V>(&self) -> Result<usize, <V as TryFrom<usize>>::Error>
    where
      V: Varint,
    {
      let buf: &[u8] = self.as_ref();
      let len = buf.len();
      len.try_into().map(|len_len| V::len()(len_len) + len)
    }

    fn encode<V>(
      &self,
      buf: &mut [u8],
    ) -> Result<usize, LengthDelimitedError<<V as TryFrom<usize>>::Error, Self::Error>>
    where
      V: Varint,
    {
      let dst_len = buf.len();
      let this: &[u8] = self.as_ref();
      let len = this.len();

      match len.try_into() {
        Ok(len_len) => {
          let written = match V::encoder()(len_len, buf) {
            Ok(written) => written,
            Err(_) => {
              return Err(LengthDelimitedError::Encode(
                InsufficientBuffer::with_information(
                  (V::len()(len_len) + len) as u64,
                  dst_len as u64,
                ),
              ));
            }
          };

          let required = written + len;
          if required > dst_len {
            return Err(LengthDelimitedError::Encode(
              InsufficientBuffer::with_information(required as u64, dst_len as u64),
            ));
          }

          buf[written..required].copy_from_slice(this);
          Ok(required)
        }
        Err(e) => Err(LengthDelimitedError::Varint(e)),
      }
    }
  };
  (@numbers $($ty:ident), +$(,)?) => {
    paste::paste! {
      $(
        impl LengthDelimitedEncoder for $ty {
          type Error = InsufficientBuffer;

          fn length_delimited<V>(&self) -> Result<usize, <V as TryFrom<usize>>::Error>
          where
            V: Varint,
          {
            let len = dbutils::leb128::[< encoded_ $ty _varint_len >](*self);
            V::try_from(len).map(|_| len)
          }

          fn encode<V>(
            &self,
            buf: &mut [u8],
          ) -> Result<usize, LengthDelimitedError<<V as TryFrom<usize>>::Error, Self::Error>>
          where
            V: Varint,
          {
            dbutils::leb128::[< encode_ $ty _varint >](*self, buf).map_err(|_| {
              LengthDelimitedError::Encode(InsufficientBuffer::with_information(
                dbutils::leb128::[< encoded_ $ty _varint_len >](*self) as u64,
                buf.len() as u64,
              ))
            })
          }
        }
      )*
    }
  };
  ($src:ty as $target:ty) => {
    impl LengthDelimitedEncoder for $src {
      type Error = <$target as LengthDelimitedEncoder>::Error;

      fn length_delimited<V>(&self) -> Result<usize, <V as TryFrom<usize>>::Error>
      where
        V: Varint,
      {
        <$target as LengthDelimitedEncoder>::length_delimited::<V>(&(*self as $target))
      }

      fn encode<V>(
        &self,
        buf: &mut [u8],
      ) -> Result<usize, LengthDelimitedError<<V as TryFrom<usize>>::Error, Self::Error>>
      where
        V: Varint,
      {
        <$target as LengthDelimitedEncoder>::encode::<V>(&(*self as $target), buf)
      }
    }
  };
}

impl_length_delimited!(@numbers u16, u32, u64, u128, i16, i32, i64, i128);
impl_length_delimited!(bool as u8);
impl_length_delimited!(i8 as u8);
impl_length_delimited!(char as u32);
impl_length_delimited!(f32 as u32);
impl_length_delimited!(f64 as u64);

impl LengthDelimitedEncoder for u8 {
  type Error = InsufficientBuffer;

  fn length_delimited<V>(&self) -> Result<usize, <V as TryFrom<usize>>::Error>
  where
    V: Varint,
  {
    V::try_from(1).map(|_| 1)
  }

  fn encode<V>(
    &self,
    buf: &mut [u8],
  ) -> Result<usize, LengthDelimitedError<<V as TryFrom<usize>>::Error, Self::Error>>
  where
    V: Varint,
  {
    if buf.is_empty() {
      return Err(LengthDelimitedError::Encode(
        InsufficientBuffer::with_information(1, 0),
      ));
    }

    buf[0] = *self;
    Ok(1)
  }
}

impl<const N: usize> LengthDelimitedEncoder for [u8; N] {
  impl_length_delimited!(@bytes);
}

macro_rules! impl_length_delimited_for_bytes {
  ($($ty:ty), +$(,)?) => {
    $(
      impl LengthDelimitedEncoder for $ty {
        impl_length_delimited!(@bytes);
      }
    )*
  };
}

#[cfg(feature = "static")]
impl_length_delimited_for_bytes!(&'static str, &'static [u8]);

#[cfg(not(feature = "static"))]
impl_length_delimited_for_bytes!(&str, &[u8]);

#[cfg(feature = "bytes1")]
impl_length_delimited_for_bytes!(bytes1::Bytes);

#[cfg(feature = "smol_str03")]
impl_length_delimited_for_bytes!(smol_str03::SmolStr);

#[cfg(all(feature = "bstr1", feature = "static"))]
impl_length_delimited_for_bytes!(&'static bstr1::BStr,);

#[cfg(all(feature = "bstr1", not(feature = "static")))]
impl_length_delimited_for_bytes!(&bstr1::BStr,);

#[cfg(all(feature = "bstr1", any(feature = "alloc", feature = "std")))]
impl_length_delimited_for_bytes!(bstr1::BString,);

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::string::String;
  use std::{boxed::Box, sync::Arc, vec::Vec};

  impl_length_delimited_for_bytes!(Vec<u8>, Box<[u8]>, Arc<[u8]>, String);

  #[cfg(not(any(feature = "send", feature = "sync")))]
  impl_length_delimited_for_bytes!(std::rc::Rc<[u8]>);
};

#[cfg(feature = "triomphe01")]
impl_length_delimited_for_bytes!(triomphe01::Arc<[u8]>);

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(any(feature = "std", feature = "alloc"))]
  use std::{boxed::Box, string::String, vec};

  #[test]
  fn test_length_delimited_u8_array() {
    let data = [1, 2, 3, 4, 5];
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], &data[..]);
  }

  #[test]
  fn test_length_delimited_empty_slice() {
    let data: &[u8] = &[];
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], data);
  }

  #[test]
  fn test_length_delimited_insufficient_buffer() {
    let data = [1, 2, 3, 4, 5];
    let mut buf = [0; 4]; // Buffer too small
    let result = data.encode::<u32>(&mut buf);
    assert!(matches!(result, Err(LengthDelimitedError::Encode(_))));
  }

  #[test]
  fn test_length_delimited_str() {
    let data = "hello";
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], data.as_bytes());
  }

  #[test]
  fn test_length_delimited_array() {
    let data = b"hello";
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], data);
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  #[test]
  fn test_length_delimited_string() {
    let data = String::from("hello");
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], data.as_bytes());
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  #[test]
  fn test_length_delimited_vec() {
    let data = vec![1, 2, 3, 4, 5];
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], &data[..]);
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  #[test]
  fn test_length_delimited_boxed_slice() {
    let data = Box::new([1, 2, 3, 4, 5]);
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], &data[..]);
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  #[test]
  fn test_length_delimited_arc_slice() {
    let data = std::sync::Arc::new([1, 2, 3, 4, 5]);
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], &data[..]);
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  #[test]
  fn test_length_delimited_rc_slice() {
    let data = std::rc::Rc::new([1, 2, 3, 4, 5]);
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], &data[..]);
  }

  #[cfg(feature = "bytes1")]
  #[test]
  fn test_length_delimited_bytes() {
    let data = bytes1::Bytes::from_static(b"hello");
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], data.as_ref());
  }

  #[cfg(feature = "smol_str03")]
  #[test]
  fn test_length_delimited_smol_str() {
    let data = smol_str03::SmolStr::new("hello");
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], data.as_bytes());
  }

  #[cfg(feature = "bstr1")]
  #[test]
  fn test_length_delimited_bstr() {
    let data = bstr1::BStr::new("hello");
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    let b: &[u8] = data.as_ref();
    assert_eq!(&buf[1..len], b);
  }

  #[cfg(all(feature = "bstr1", any(feature = "alloc", feature = "std")))]
  #[test]
  fn test_length_delimited_bstring() {
    let data = bstr1::BString::from("hello");
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    let b: &[u8] = data.as_ref();
    assert_eq!(&buf[1..len], b);
  }

  #[test]
  #[cfg(feature = "triomphe01")]
  fn test_length_delimited_triomphe_arc() {
    let data = triomphe01::Arc::new([1, 2, 3, 4, 5]);
    let mut buf = [0; 10];
    let len = data.length_delimited::<u32>().unwrap();
    let written = data.encode::<u32>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], &data[..]);
  }

  #[test]
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn test_length_delimited_max_varint() {
    let data = vec![0; u16::MAX as usize];
    let mut buf = vec![0; data.len() + 3]; // Enough space for the varint and data
    let len = data.length_delimited::<u16>().unwrap();
    let written = data.encode::<u16>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[3..len], &data[..]);

    let data = vec![0; u8::MAX as usize];
    let mut buf = vec![0; data.len() + 1]; // Enough space for the varint and data
    let len = data.length_delimited::<u8>().unwrap();
    let written = data.encode::<u8>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], &data[..]);

    let data = vec![0; i8::MAX as usize];
    let mut buf = vec![0; data.len() + 1]; // Enough space for the varint and data
    let len = data.length_delimited::<i8>().unwrap();
    let written = data.encode::<i8>(&mut buf).unwrap();
    assert_eq!(len, written);
    assert_eq!(&buf[1..len], &data[..]);
  }

  #[test]
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn test_length_delimited_varint_conversion_error() {
    let data = vec![0; u16::MAX as usize + 1];
    let result = data.length_delimited::<u16>();
    assert!(result.is_err());

    let data = vec![0; u8::MAX as usize + 1];
    let result = data.length_delimited::<u8>();
    assert!(result.is_err());

    let data = vec![0; i8::MAX as usize + 1];
    let result = data.length_delimited::<i8>();
    assert!(result.is_err());
  }

  #[test]
  fn test_u8_varint() {
    let val = 42u8;
    let mut buf = [0u8; 1];
    let encoder = u8::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 1);
    assert_eq!(buf[0], val);

    let decoder = u8::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 1);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_u8_varint_max() {
    let val = u8::MAX;
    let mut buf = [0u8; u8::MAX_VARINT_LEN];
    let encoder = u8::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 1);
    assert_eq!(buf[0], val);

    let decoder = u8::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 1);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_u8_varint_insufficient_buffer() {
    let val = 42u8;
    let mut buf = [];
    let encoder = u8::encoder();
    let result = encoder(val, &mut buf);
    assert!(result.is_err());
  }

  #[test]
  fn test_u8_varint_incomplete_buffer() {
    let buf = [];
    let decoder = u8::decoder();
    let result = decoder(&buf);
    assert!(matches!(
      result,
      Err(DecodeVarintError::IncompleteBuffer(_))
    ));
  }

  #[test]
  fn test_i8_varint() {
    let val = -42i8;
    let mut buf = [0u8; 1];
    let encoder = i8::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 1);
    assert_eq!(buf[0], val as u8);

    let decoder = i8::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 1);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_i8_varint_max() {
    let val = i8::MAX;
    let mut buf = [0u8; i8::MAX_VARINT_LEN];
    let encoder = i8::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 1);
    assert_eq!(buf[0], val as u8);

    let decoder = i8::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 1);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_i8_varint_min() {
    let val = i8::MIN;
    let mut buf = [0u8; 1];
    let encoder = i8::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 1);
    assert_eq!(buf[0], val as u8);

    let decoder = i8::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 1);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_i8_varint_insufficient_buffer() {
    let val = 42i8;
    let mut buf = [];
    let encoder = i8::encoder();
    let result = encoder(val, &mut buf);
    assert!(result.is_err());
  }

  #[test]
  fn test_i8_varint_incomplete_buffer() {
    let buf = [];
    let decoder = i8::decoder();
    let result = decoder(&buf);
    assert!(matches!(
      result,
      Err(DecodeVarintError::IncompleteBuffer(_))
    ));
  }

  #[test]
  fn test_u16_varint() {
    let val = 1234u16;
    let mut buf = [0u8; 2];
    let encoder = u16::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 2);

    let decoder = u16::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 2);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_u16_varint_max() {
    let val = u16::MAX;
    let mut buf = [0u8; u16::MAX_VARINT_LEN];
    let encoder = u16::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 3);

    let decoder = u16::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 3);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_u32_varint() {
    let val = 123456u32;
    let mut buf = [0u8; 4];
    let encoder = u32::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 3);

    let decoder = u32::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 3);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_u32_varint_max() {
    let val = u32::MAX;
    let mut buf = [0u8; u32::MAX_VARINT_LEN];
    let encoder = u32::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 5);

    let decoder = u32::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 5);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_u64_varint() {
    let val = 1234567890u64;
    let mut buf = [0u8; 8];
    let encoder = u64::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 5);

    let decoder = u64::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 5);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_u64_varint_max() {
    let val = u64::MAX;
    let mut buf = [0u8; u64::MAX_VARINT_LEN];
    let encoder = u64::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 10);

    let decoder = u64::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 10);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_i16_varint() {
    let val = -1234i16;
    let mut buf = [0u8; 2];
    let encoder = i16::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 2);

    let decoder = i16::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 2);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_i16_varint_max() {
    let val = i16::MAX;
    let mut buf = [0u8; i16::MAX_VARINT_LEN];
    let encoder = i16::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 3);

    let decoder = i16::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 3);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_i32_varint() {
    let val = -123456i32;
    let mut buf = [0u8; 4];
    let encoder = i32::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 3);

    let decoder = i32::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 3);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_i32_varint_max() {
    let val = i32::MAX;
    let mut buf = [0u8; i32::MAX_VARINT_LEN];
    let encoder = i32::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 5);

    let decoder = i32::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 5);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_i64_varint() {
    let val = -1234567890i64;
    let mut buf = [0u8; 8];
    let encoder = i64::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 5);

    let decoder = i64::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 5);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_i64_varint_max() {
    let val = i64::MAX;
    let mut buf = [0u8; i64::MAX_VARINT_LEN];
    let encoder = i64::encoder();
    let written = encoder(val, &mut buf).unwrap();
    assert_eq!(written, 10);

    let decoder = i64::decoder();
    let (read, decoded_val) = decoder(&buf).unwrap();
    assert_eq!(read, 10);
    assert_eq!(decoded_val, val);
  }

  #[test]
  fn test_length_delimited_insufficient_buffer_varint() {
    // Create a buffer that is too small to hold the varint
    let data = [1, 2, 3, 4, 5]; // 5 bytes of data
    let mut buf = [0; 0]; // Buffer is too small for even the varint

    // Attempt to encode the data
    let result = data.encode::<u32>(&mut buf);

    // Verify that the error is returned
    assert!(matches!(
      result,
      Err(LengthDelimitedError::Encode(InsufficientBuffer { .. }))
    ));
  }

  #[test]
  fn test_length_delimited_insufficient_buffer_data() {
    // Create a buffer that is large enough for the varint but too small for the data
    let data = [1, 2, 3, 4, 5]; // 5 bytes of data
    let mut buf = [0; 2]; // Buffer is too small for the data (varint requires 1 byte, data requires 5)

    // Attempt to encode the data
    let result = data.encode::<u8>(&mut buf);

    // Verify that the error is returned
    assert!(matches!(
      result,
      Err(LengthDelimitedError::Encode(InsufficientBuffer { .. }))
    ));
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  #[test]
  fn test_length_delimited_insufficient_buffer_large_data() {
    // Create a large data slice that requires a multi-byte varint
    let data = vec![0; 300]; // 300 bytes of data
    let mut buf = [0; 1]; // Buffer is too small for the varint (requires 2 bytes for u16 varint)

    // Attempt to encode the data
    let result = data.encode::<u16>(&mut buf);

    // Verify that the error is returned
    assert!(matches!(
      result,
      Err(LengthDelimitedError::Encode(InsufficientBuffer { .. }))
    ));
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  #[test]
  fn test_length_delimited_insufficient_buffer_max_varint() {
    // Create a data slice that requires the maximum varint size
    let data = vec![0; u16::MAX as usize]; // Data size is u32::MAX
    let mut buf = [0; 2]; // Buffer is too small for the varint (requires 5 bytes for u32 varint)

    // Attempt to encode the data
    let result = data.encode::<u16>(&mut buf);

    // Verify that the error is returned
    assert!(matches!(
      result,
      Err(LengthDelimitedError::Encode(InsufficientBuffer { .. }))
    ));
  }
}
