use core::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

use dbutils::{
  error::{IncompleteBuffer, InsufficientBuffer},
  leb128::DecodeVarintError,
};

use super::{LengthDelimitedDecoder, LengthDelimitedEncoder};

macro_rules! net_impl {
  (@ip $($ty:ident($inner:ident)), + $(,)?) => {
    $(
      impl LengthDelimitedEncoder for $ty {
        type Error = InsufficientBuffer;

        fn encoded_len(&self) -> usize {
          self.to_bits().encoded_len()
        }

        fn encoded_length_delimited_len(&self) -> usize {
          Self::encoded_len(self)
        }

        fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          Self::encode(self, buf)
        }

        fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          self.to_bits().encode(buf)
        }
      }

      impl LengthDelimitedDecoder for $ty {
        type Error = DecodeVarintError;

        fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized,
        {
          let (read, inner) = <$inner as LengthDelimitedDecoder>::decode(src)?;
          Ok((read, $ty::from_bits(inner)))
        }

        fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized,
        {
          Self::decode(src)
        }
      }
    )*
  };
  (@sock $($ty:ident($inner:ident $(, $($defaults:literal), +$(,)? )? )),+$(,)?) => {
    $(
      impl LengthDelimitedEncoder for $ty {
        type Error = InsufficientBuffer;

        fn encoded_len(&self) -> usize {
          self.ip().encoded_len() + 2
        }

        fn encoded_length_delimited_len(&self) -> usize {
          Self::encoded_len(self)
        }

        fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          Self::encode(self, buf)
        }

        fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          let encoded_len = self.encoded_len();
          if buf.len() < encoded_len {
            return Err(InsufficientBuffer::with_information(encoded_len as u64, buf.len() as u64));
          }

          self.ip().encode(buf)?;
          let port = self.port();
          buf[encoded_len - 2..encoded_len].copy_from_slice(&port.to_le_bytes());
          Ok(encoded_len)
        }
      }

      impl LengthDelimitedDecoder for $ty {
        type Error = DecodeSocketAddrError;

        fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized,
        {
          let (read, ip) = $inner::decode(src)?;
          if read + 2 > src.len() {
            return Err(Self::Error::IncompleteBuffer(IncompleteBuffer::with_information((read + 2) as u64, src.len() as u64)));
          }

          let port = u16::from_le_bytes([src[read], src[read + 1]]);
          Ok((read + 2, Self::new(ip, port $(, $($defaults),* )? )))
        }

        fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized,
        {
          Self::decode(src)
        }
      }
    )*
  }
}

net_impl!(@ip Ipv4Addr(u32), Ipv6Addr(u128),);
net_impl!(@sock SocketAddrV4(Ipv4Addr), SocketAddrV6(Ipv6Addr, 0, 0));

/// An error that can occur when decoding a `SocketAddrV4` or `SocketAddrV6` from bytes.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeSocketAddrError {
  /// The buffer is incomplete.
  #[error(transparent)]
  IncompleteBuffer(#[from] IncompleteBuffer),
  /// The IP address could not be decoded.
  #[error("value is not a valid IP address")]
  InvalidIpAddress,
}

impl From<DecodeVarintError> for DecodeSocketAddrError {
  fn from(e: DecodeVarintError) -> Self {
    match e {
      DecodeVarintError::Underflow => {
        DecodeSocketAddrError::IncompleteBuffer(IncompleteBuffer::new())
      }
      _ => DecodeSocketAddrError::InvalidIpAddress,
    }
  }
}
