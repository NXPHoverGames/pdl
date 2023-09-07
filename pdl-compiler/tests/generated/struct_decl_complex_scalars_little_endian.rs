#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use pdl_runtime::{Error, Packet, Private};
type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u32,
    pub e: u16,
    pub f: u8,
}
impl Foo {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 7
    }
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let chunk = bytes.get_mut().get_u16_le();
        let a = (chunk & 0x7) as u8;
        let b = (chunk >> 3) as u8;
        let c = ((chunk >> 11) & 0x1f) as u8;
        if bytes.get().remaining() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 3,
                got: bytes.get().remaining(),
            });
        }
        let d = bytes.get_mut().get_uint_le(3) as u32;
        if bytes.get().remaining() < 2 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 2,
                got: bytes.get().remaining(),
            });
        }
        let chunk = bytes.get_mut().get_u16_le();
        let e = (chunk & 0xfff);
        let f = ((chunk >> 12) & 0xf) as u8;
        Ok(Self { a, b, c, d, e, f })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        if self.a > 0x7 {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "a", self.a, 0x7);
        }
        if self.c > 0x1f {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "c", self.c, 0x1f);
        }
        let value = (self.a as u16) | ((self.b as u16) << 3) | ((self.c as u16) << 11);
        buffer.put_u16_le(value);
        if self.d > 0xff_ffff {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "d", self.d, 0xff_ffff);
        }
        buffer.put_uint_le(self.d as u64, 3);
        if self.e > 0xfff {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "e", self.e, 0xfff);
        }
        if self.f > 0xf {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "f", self.f, 0xf);
        }
        let value = self.e | ((self.f as u16) << 12);
        buffer.put_u16_le(value);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        7
    }
}
