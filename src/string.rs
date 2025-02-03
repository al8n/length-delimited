use dbutils::{
  error::{IncompleteBuffer, InsufficientBuffer},
  leb128::*,
};

macro_rules! string_impl {
  ($($ty:ty:$converter:ident),+$(,)?) => {
    string_impl!(@encoder $($ty),+);
    string_impl!(@decoder $($ty:$converter),+);
  };
  (@encoder $($ty:ty),+$(,)?) => {
    $(
      impl crate::LengthDelimitedEncoder for $ty {
        type Error = InsufficientBuffer;

        fn encoded_len(&self) -> usize {
          <&[u8] as crate::LengthDelimitedEncoder>::encoded_len(&self.as_bytes())
        }

        fn encoded_length_delimited_len(&self) -> usize {
          <&[u8] as crate::LengthDelimitedEncoder>::encoded_length_delimited_len(&self.as_bytes())
        }

        fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          <&[u8] as crate::LengthDelimitedEncoder>::encode_length_delimited(&self.as_bytes(), buf)
        }

        fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          <&[u8] as crate::LengthDelimitedEncoder>::encode(&self.as_bytes(), buf)
        }
      }
    )*
  };
  (@decoder $($ty:ty:$converter:ident),+$(,)?) => {
    $(
      impl crate::LengthDelimitedDecoder for $ty {
        type Error = DecodeUtf8BytesError;

        fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized,
        {
          core::str::from_utf8(&src).map_err(Into::into).map(|s| (src.len(), Self::$converter(s)))
        }

        fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized,
        {
          let (read, bytes) = decode_u64_varint(src)?;
          let len = bytes as usize;
          let required = read + len;
          if required > src.len() {
            return Err(IncompleteBuffer::with_information(required as u64, src.len() as u64).into());
          }

          Self::decode(&src[read..required])
            .map(|(bytes, this)| (read + bytes, this))
            .map_err(|e| match e {
              Self::Error::IncompleteBuffer(_) => IncompleteBuffer::with_information(
                required as u64,
                src.len() as u64,
              ).into(),
              e => e,
            })
        }
      }
    )*
  };
}

/// The error that can be returned when decoding utf8 bytes.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeUtf8BytesError {
  /// Returned when there is not enough data to decode the full type.
  #[error(transparent)]
  IncompleteBuffer(#[from] IncompleteBuffer),
  /// Returned when the length delimited overflows.
  #[error("length delimited overflow")]
  Overflow,
  /// Returned when the string is not valid UTF-8.
  #[error(transparent)]
  Utf8(#[from] core::str::Utf8Error),
}

impl From<DecodeVarintError> for DecodeUtf8BytesError {
  fn from(e: DecodeVarintError) -> Self {
    match e {
      DecodeVarintError::Underflow => Self::IncompleteBuffer(IncompleteBuffer::new()),
      DecodeVarintError::Overflow => Self::Overflow,
    }
  }
}

impl From<crate::DecodeBytesError> for DecodeUtf8BytesError {
  fn from(e: crate::DecodeBytesError) -> Self {
    match e {
      crate::DecodeBytesError::IncompleteBuffer(e) => Self::IncompleteBuffer(e),
      crate::DecodeBytesError::Overflow => Self::Overflow,
    }
  }
}

string_impl!(@encoder &str);

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(std::string::String:from, std::sync::Arc<str>:from, std::boxed::Box<str>:from, std::rc::Rc<str>:from);

#[cfg(all(feature = "triomphe01", any(feature = "std", feature = "alloc")))]
string_impl!(triomphe01::Arc<str>:from,);

#[cfg(all(feature = "smol_str03", any(feature = "std", feature = "alloc")))]
string_impl!(smol_str03::SmolStr:new,);

#[cfg(all(feature = "faststr02", any(feature = "std", feature = "alloc")))]
string_impl!(faststr02::FastStr:new,);
