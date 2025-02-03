use dbutils::leb128::*;

use crate::{LengthDelimitedDecoder, LengthDelimitedEncoder};

macro_rules! numbers_impl_length_delimited {
  ($($ty:ident), +$(,)?) => {
    $(
      paste::paste! {
        impl $crate::LengthDelimitedEncoder for $ty {
          type Error = InsufficientBuffer;

          fn encoded_len(&self) -> usize {
            [< encoded_ $ty _varint_len >](*self)
          }

          fn encoded_length_delimited_len(&self) -> usize {
            self.encoded_len()
          }

          fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            self.encode(buf)
          }

          fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            [< encode_ $ty _varint >](*self, buf)
          }
        }

        impl $crate::LengthDelimitedDecoder for $ty {
          type Error = $crate::DecodeVarintError;

          fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
          where
            Self: Sized,
          {
            [< decode_ $ty _varint >](src)
          }

          fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
          where
            Self: Sized,
          {
            Self::decode(src)
          }
        }
      }
    )*
  };
}

numbers_impl_length_delimited!(u16, i16, u32, i32, u64, i64, u128, i128,);

impl_length_delimited!(i8 as u8, f32 as u32, f64 as u64,);

impl LengthDelimitedEncoder for u8 {
  type Error = InsufficientBuffer;

  fn encoded_len(&self) -> usize {
    1
  }

  fn encoded_length_delimited_len(&self) -> usize {
    self.encoded_len()
  }

  fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
    self.encode(buf)
  }

  fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
    if buf.is_empty() {
      return Err(InsufficientBuffer::with_information(1, 0));
    }

    buf[0] = *self;
    Ok(1)
  }
}

impl LengthDelimitedDecoder for u8 {
  type Error = IncompleteBuffer;

  fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
  where
    Self: Sized,
  {
    if src.is_empty() {
      Err(IncompleteBuffer::with_information(1, 0))
    } else {
      Ok((1, src[0]))
    }
  }

  fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
  where
    Self: Sized,
  {
    Self::decode(src)
  }
}

impl_length_delimited!(@encoder bool as u8,);

impl LengthDelimitedDecoder for bool {
  type Error = IncompleteBuffer;

  fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
  where
    Self: Sized,
  {
    if src.is_empty() {
      Err(IncompleteBuffer::with_information(1, 0))
    } else {
      Ok((1, src[0] != 0))
    }
  }

  fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
  where
    Self: Sized,
  {
    Self::decode(src)
  }
}

/// Error when decoding a character
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeCharError {
  /// Incomplete buffer
  #[error(transparent)]
  IncompleteBuffer(#[from] IncompleteBuffer),
  /// Invalid UTF-8 character
  #[error("invalid UTF-8 character")]
  InvalidChar,
}

impl From<DecodeVarintError> for DecodeCharError {
  fn from(e: DecodeVarintError) -> Self {
    match e {
      DecodeVarintError::IncompleteBuffer(e) => DecodeCharError::IncompleteBuffer(e),
      DecodeVarintError::Overflow => DecodeCharError::InvalidChar,
    }
  }
}

impl_length_delimited!(@encoder char as u32,);

impl LengthDelimitedDecoder for char {
  type Error = DecodeCharError;

  fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
  where
    Self: Sized,
  {
    let (size, code) = u32::decode(src)?;
    let c = core::char::from_u32(code).ok_or(DecodeCharError::InvalidChar)?;
    Ok((size, c))
  }

  fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
  where
    Self: Sized,
  {
    Self::decode(src)
  }
}
