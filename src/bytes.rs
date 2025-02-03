use dbutils::leb128::*;

use crate::{LengthDelimitedDecoder, LengthDelimitedEncoder};

macro_rules! bytes_impl {
  ($($ty:ty:$from_bytes:ident), +$(,)?) => {
    bytes_impl!(@encoder $($ty), +);
    bytes_impl!(@decoder $($ty:$from_bytes), +);
  };
  (@encoder $($ty:ty), +$(,)?) => {
    $(
      impl $crate::LengthDelimitedEncoder for $ty {
        type Error = $crate::InsufficientBuffer;

        fn encoded_len(&self) -> usize {
          let buf: &[u8] = self.as_ref();
          buf.len()
        }

        fn encoded_length_delimited_len(&self) -> usize {
          let buf: &[u8] = self.as_ref();
          let len = buf.len();
          encoded_u64_varint_len(len as u64) + len
        }

        fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          let dst_len = buf.len();
          let this: &[u8] = self.as_ref();
          let len = this.len();

          let written = match encode_u64_varint(len as u64, buf) {
            Ok(written) => written,
            Err(_) => {
              return Err(InsufficientBuffer::with_information(
                self.encoded_length_delimited_len() as u64,
                dst_len as u64,
              ));
            }
          };

          let required = written + len;
          if required > dst_len {
            return Err(InsufficientBuffer::with_information(required as u64, dst_len as u64));
          }

          buf[written..required].copy_from_slice(this);
          Ok(required)
        }

        fn encode(
          &self,
          buf: &mut [u8],
        ) -> Result<usize, Self::Error> {
          let dst_len = buf.len();
          let this: &[u8] = self.as_ref();
          let len = this.len();

          let required = len;
          if required > dst_len {
            return Err(InsufficientBuffer::with_information(required as u64, dst_len as u64));
          }

          buf[..required].copy_from_slice(this);
          Ok(required)
        }
      }
    )*
  };
  (@decoder $($ty:ty:$from_bytes:ident), +$(,)?) => {
    $(
      impl $crate::LengthDelimitedDecoder for $ty {
        type Error = $crate::DecodeBytesError;

        fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized
        {
          Ok((src.len(), Self::$from_bytes(src)))
        }

        fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized {
          let (read, val) = decode_u64_varint(src)?;
          let len = val as usize;
          let required = read + len;
          if required > src.len() {
            return Err(IncompleteBuffer::with_information(required as u64, src.len() as u64).into());
          }

          let data = Self::$from_bytes(&src[read..required]);
          Ok((required, data))
        }
      }
    )*
  };
}

bytes_impl!(@encoder &[u8]);

#[cfg(any(feature = "std", feature = "alloc"))]
bytes_impl!(std::vec::Vec<u8>:from, std::boxed::Box<[u8]>:from, std::sync::Arc<[u8]>:from, std::rc::Rc<[u8]>:from);

#[cfg(all(feature = "triomphe01", any(feature = "std", feature = "alloc")))]
bytes_impl!(triomphe01::Arc<[u8]>:from,);

#[cfg(all(feature = "bytes1", any(feature = "std", feature = "alloc")))]
bytes_impl!(bytes1::Bytes:copy_from_slice,);

#[cfg(all(feature = "bstr1", any(feature = "std", feature = "alloc")))]
bytes_impl!(bstr1::BString:from,);

#[cfg(feature = "bstr1")]
bytes_impl!(@encoder &bstr1::BStr,);

impl<const N: usize> LengthDelimitedEncoder for [u8; N] {
  type Error = InsufficientBuffer;

  fn encoded_len(&self) -> usize {
    N
  }

  fn encoded_length_delimited_len(&self) -> usize {
    N
  }

  fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
    self.encode(buf)
  }

  fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
    if buf.len() < N {
      return Err(InsufficientBuffer::with_information(
        N as u64,
        buf.len() as u64,
      ));
    }

    buf[..N].copy_from_slice(self);
    Ok(N)
  }
}

impl<const N: usize> LengthDelimitedDecoder for [u8; N] {
  type Error = DecodeBytesError;

  fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
  where
    Self: Sized,
  {
    let len = src.len();
    if len < N {
      return Err(DecodeBytesError::IncompleteBuffer(
        IncompleteBuffer::with_information(N as u64, len as u64),
      ));
    }

    let mut dst = [0; N];
    dst.copy_from_slice(&src[..N]);
    Ok((N, dst))
  }

  fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
  where
    Self: Sized,
  {
    Self::decode(src)
  }
}

/// The error that can be returned when decoding bytes.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeBytesError {
  /// Returned when there is not enough data to decode the full type.
  #[error(transparent)]
  IncompleteBuffer(#[from] IncompleteBuffer),
  /// Returned when the length delimited overflows.
  #[error("length delimited overflow")]
  Overflow,
}

impl From<DecodeVarintError> for DecodeBytesError {
  fn from(e: DecodeVarintError) -> Self {
    match e {
      DecodeVarintError::IncompleteBuffer(e) => Self::IncompleteBuffer(e),
      DecodeVarintError::Overflow => Self::Overflow,
    }
  }
}
