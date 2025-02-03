use dbutils::error::{IncompleteBuffer, InsufficientBuffer};

use crate::{LengthDelimitedDecoder, LengthDelimitedEncoder};

macro_rules! numbers_impl_length_delimited {
  ($($ty:ident), +$(,)?) => {
    paste::paste! {
      $(
        impl $crate::LengthDelimitedEncoder for $ty {
          type Error = InsufficientBuffer;

          fn encoded_len(&self) -> usize {
            dbutils::leb128::[< encoded_ $ty _varint_len >](*self)
          }

          fn encoded_length_delimited_len(&self) -> usize {
            <Self as $crate::LengthDelimitedEncoder>::encoded_len(self)
          }

          fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            <Self as $crate::LengthDelimitedEncoder>::encode(self, buf)
          }

          fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            dbutils::leb128::Varint::encode(self, buf).map_err(|_| InsufficientBuffer::new())
          }
        }

        impl $crate::LengthDelimitedDecoder for $ty {
          type Error = $crate::DecodeVarintError;

          fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
          where
            Self: Sized,
          {
            dbutils::leb128::[< decode_ $ty _varint >](src)
          }

          fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
          where
            Self: Sized,
          {
            <Self as $crate::LengthDelimitedDecoder>::decode(src)
          }
        }
      )*
    }
  };
  (@fixed $($ty:ident), +$(,)?) => {
    paste::paste! {
      $(
        impl $crate::LengthDelimitedEncoder for $ty {
          type Error = InsufficientBuffer;

          fn encoded_len(&self) -> usize {
            core::mem::size_of::<$ty>()
          }

          fn encoded_length_delimited_len(&self) -> usize {
            <Self as $crate::LengthDelimitedEncoder>::encoded_len(self)
          }

          fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            <Self as $crate::LengthDelimitedEncoder>::encode(self, buf)
          }

          fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            const SIZE: usize = core::mem::size_of::<$ty>();

            if buf.len() < SIZE {
              return Err(InsufficientBuffer::with_information(SIZE as u64, buf.len() as u64));
            }

            buf[..SIZE].copy_from_slice(&self.to_le_bytes());
            Ok(SIZE)
          }
        }

        impl $crate::LengthDelimitedDecoder for $ty {
          type Error = IncompleteBuffer;

          fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
          where
            Self: Sized,
          {
            const SIZE: usize = core::mem::size_of::<$ty>();

            if src.len() < SIZE {
              return Err(IncompleteBuffer::with_information(SIZE as u64, src.len() as u64));
            }

            let mut bytes = [0u8; SIZE];
            bytes.copy_from_slice(&src[..SIZE]);
            Ok((SIZE, Self::from_le_bytes(bytes)))
          }

          fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
          where
            Self: Sized,
          {
            <Self as $crate::LengthDelimitedDecoder>::decode(src)
          }
        }
      )*
    }
  }
}

numbers_impl_length_delimited!(u16, i16, u32, i32, u64, i64, u128, i128,);

numbers_impl_length_delimited!(@fixed f32, f64,);

impl_length_delimited!(i8 as u8,);

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
  /// Invalid character value
  #[error("invalid character value")]
  InvalidChar,
}

impl From<dbutils::leb128::DecodeVarintError> for DecodeCharError {
  fn from(e: dbutils::leb128::DecodeVarintError) -> Self {
    match e {
      dbutils::leb128::DecodeVarintError::Underflow => {
        DecodeCharError::IncompleteBuffer(IncompleteBuffer::new())
      }
      dbutils::leb128::DecodeVarintError::Overflow => DecodeCharError::InvalidChar,
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
    let (size, code) = <u32 as LengthDelimitedDecoder>::decode(src)?;
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_u8_encode_decode() {
    let value: u8 = 42;
    let mut buffer = [0u8; 1];

    // Test encoding
    let encoded_size = value.encode(&mut buffer).unwrap();
    assert_eq!(encoded_size, 1);
    assert_eq!(buffer[0], 42);

    // Test decoding
    let (decoded_size, decoded_value) = u8::decode(&buffer).unwrap();
    assert_eq!(decoded_size, 1);
    assert_eq!(decoded_value, 42);
  }

  #[test]
  fn test_u16_encode_decode() {
    let value: u16 = 12345;
    let mut buffer = [0u8; 3]; // u16 can take up to 3 bytes in LEB128

    // Test encoding
    let encoded_size = value.encode(&mut buffer).unwrap();
    assert!(encoded_size <= 3);

    // Test decoding
    let (decoded_size, decoded_value) = u16::decode(&buffer).unwrap();
    assert_eq!(decoded_size, encoded_size);
    assert_eq!(decoded_value, 12345);
  }

  #[test]
  fn test_bool_encode_decode() {
    let value = true;
    let mut buffer = [0u8; 1];

    // Test encoding
    let encoded_size = value.encode(&mut buffer).unwrap();
    assert_eq!(encoded_size, 1);
    assert_eq!(buffer[0], 1);

    // Test decoding
    let (decoded_size, decoded_value) = bool::decode(&buffer).unwrap();
    assert_eq!(decoded_size, 1);
    assert!(decoded_value);
  }

  #[test]
  fn test_char_encode_decode() {
    let value = '🦀';
    let mut buffer = [0u8; 4]; // char is encoded as u32

    let written = value.encode(&mut buffer).unwrap();
    let (read, decoded) = char::decode(&buffer[..written]).unwrap();
    assert_eq!(value, decoded);
    assert_eq!(written, read);
  }

  #[test]
  fn test_u8_encode_insufficient_buffer() {
    let value: u8 = 42;
    let mut buffer = [0u8; 0]; // Empty buffer

    // Test encoding with insufficient buffer
    let result = value.encode(&mut buffer);
    assert!(matches!(result, Err(InsufficientBuffer { .. })));
  }

  #[test]
  fn test_u8_decode_incomplete_buffer() {
    let buffer = []; // Empty buffer

    // Test decoding with incomplete buffer
    let result = u8::decode(&buffer);
    assert!(matches!(result, Err(IncompleteBuffer { .. })));
  }

  #[test]
  fn test_char_decode_invalid_char() {
    let mut buffer = [0u8; 128];

    // Encode an invalid Unicode scalar value
    let invalid_char_value: u32 = 0x110000; // First value outside valid Unicode range
    let written = invalid_char_value.encode(&mut buffer).unwrap();

    assert!(matches!(
      char::decode(&buffer[..written]),
      Err(DecodeCharError::InvalidChar)
    ));
  }

  #[test]
  fn test_u16_decode_incomplete_buffer() {
    let buffer = [0x80]; // Incomplete LEB128 encoding

    // Test decoding with incomplete buffer
    let result = u16::decode(&buffer);
    assert!(matches!(result, Err(crate::DecodeVarintError::Underflow)));
  }

  #[test]
  fn test_bool_decode_incomplete_buffer() {
    let buffer = []; // Empty buffer

    // Test decoding with incomplete buffer
    let result = bool::decode(&buffer);
    assert!(matches!(result, Err(IncompleteBuffer { .. })));
  }
}
