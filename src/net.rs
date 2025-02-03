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
          self.to_bits().encoded_length_delimited_len()
        }

        fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          self.to_bits().encode_length_delimited(buf)
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
          let (read, inner) = <$inner as LengthDelimitedDecoder>::decode_length_delimited(src)?;
          Ok((read, $ty::from_bits(inner)))
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
          self.ip().encoded_length_delimited_len() + 2
        }

        fn encode_length_delimited(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          let encoded_len = self.encoded_length_delimited_len();
          if buf.len() < encoded_len {
            return Err(InsufficientBuffer::with_information(encoded_len as u64, buf.len() as u64));
          }

          self.ip().encode_length_delimited(buf)?;
          let port = self.port();
          buf[encoded_len - 2..encoded_len].copy_from_slice(&port.to_be_bytes());
          Ok(encoded_len)
        }

        fn encode(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
          let encoded_len = self.encoded_len();
          if buf.len() < encoded_len {
            return Err(InsufficientBuffer::with_information(encoded_len as u64, buf.len() as u64));
          }

          self.ip().encode(buf)?;
          let port = self.port();
          buf[encoded_len - 2..encoded_len].copy_from_slice(&port.to_be_bytes());
          Ok(encoded_len)
        }
      }

      impl LengthDelimitedDecoder for $ty {
        type Error = DecodeVarintError;

        fn decode(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized,
        {
          let (read, ip) = $inner::decode(src)?;
          if read + 2 > src.len() {
            return Err(DecodeVarintError::IncompleteBuffer(IncompleteBuffer::with_information((read + 2) as u64, src.len() as u64)));
          }

          let port = u16::from_be_bytes([src[read], src[read + 1]]);
          Ok((read + 2, Self::new(ip, port $(, $($defaults),* )? )))
        }

        fn decode_length_delimited(src: &[u8]) -> Result<(usize, Self), Self::Error>
        where
          Self: Sized,
        {
          let (read, ip) = $inner::decode_length_delimited(src)?;
          if read + 2 > src.len() {
            return Err(DecodeVarintError::IncompleteBuffer(IncompleteBuffer::with_information((read + 2) as u64, src.len() as u64)));
          }

          let port = u16::from_be_bytes([src[read], src[read + 1]]);
          Ok((read + 2, Self::new(ip, port $(, $($defaults),* )? )))
        }
      }
    )*
  }
}

net_impl!(@ip Ipv4Addr(u32), Ipv6Addr(u128),);
net_impl!(@sock SocketAddrV4(Ipv4Addr), SocketAddrV6(Ipv6Addr, 0, 0));
