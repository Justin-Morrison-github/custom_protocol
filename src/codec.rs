#![allow(dead_code)]
use std::ops::{BitOrAssign, ShlAssign};

use crc::{CRC_32_ISO_HDLC, Crc};
const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
use bitvec::{self, order::Msb0, slice::BitSlice, vec::BitVec, view::BitView};

use crate::bcp::{CRC_BITS, ID_BITS, LEN_BITS};

pub fn calc_crc(bytes: &[u8]) -> u32 {
    CASTAGNOLI.checksum(bytes)
}

pub enum DecodeError {
    TooManyBits,
    UnexpectedEnd,
    InvalidId,
    InvalidLen,
}

#[derive(Debug)]
pub struct Bits {
    bits: BitVec<u8, Msb0>,
}

pub struct BitsReader<'a> {
    bits: &'a BitSlice<u8, Msb0>,
    pos: usize,
}

pub struct BitsWriter<'a> {
    bits: &'a BitSlice<u8, Msb0>,
    pos: usize,
}

impl Bits {
    pub fn reader(&self) -> BitsReader<'_> {
        BitsReader {
            bits: &self.bits,
            pos: 0,
        }
    }

    pub fn writer(&self) -> BitsReader<'_> {
        BitsReader {
            bits: &self.bits,
            pos: 0,
        }
    }

    pub fn new() -> Self {
        Bits {
            bits: BitVec::new(),
        }
    }

    pub fn push_header(&mut self, id: u16, len: u8) {
        self.push_u16_n_bits(id, ID_BITS);
        self.push_u8_n_bits(len, LEN_BITS);
    }

    pub fn push_u8_n_bits(&mut self, value: u8, num_bits: usize) {
        self.bits
            .extend_from_bitslice(&value.view_bits::<Msb0>()[(8 - num_bits)..]);
    }

    pub fn push_u16_n_bits(&mut self, value: u16, num_bits: usize) {
        self.bits
            .extend_from_bitslice(&value.view_bits::<Msb0>()[(16 - num_bits)..]);
    }

    pub fn push_u32_n_bits(&mut self, value: u32, num_bits: usize) {
        self.bits
            .extend_from_bitslice(&value.view_bits::<Msb0>()[(32 - num_bits)..]);
    }

    pub fn push_u8(&mut self, value: u8) {
        self.bits.extend_from_bitslice(&value.view_bits::<Msb0>());
    }

    pub fn push_u16(&mut self, value: u16) {
        self.bits.extend_from_bitslice(&value.view_bits::<Msb0>());
    }

    pub fn push_u32(&mut self, value: u32) {
        self.bits.extend_from_bitslice(&value.view_bits::<Msb0>());
    }

    pub fn push_bool(&mut self, value: bool) {
        self.push_u8_n_bits(u8::from(value), 1);
    }

    pub fn push_i8(&mut self, value: i8) {
        self.push_u8(value as u8);
    }

    pub fn push_i16(&mut self, value: i16) {
        self.push_u16(value as u16);
    }

    pub fn push_i32(&mut self, value: i32) {
        self.push_u32(value as u32);
    }

    pub fn push_i8_n_bits(&mut self, value: i8, num_bits: usize) {
        self.push_u8_n_bits(value as u8, num_bits);
    }

    pub fn push_i16_n_bits(&mut self, value: i16, num_bits: usize) {
        self.push_u16_n_bits(value as u16, num_bits);
    }

    pub fn push_i32_n_bits(&mut self, value: i32, num_bits: usize) {
        self.push_u32_n_bits(value as u32, num_bits);
    }

    pub fn append_crc(&mut self) {
        let crc = calc_crc(self.bits.as_raw_slice());
        self.bits.extend_from_bitslice(crc.view_bits::<Msb0>());
    }
}

impl<'a> BitsReader<'a> {
    pub fn read_id(&mut self) -> Result<u16, DecodeError> {
        self.read_u16_n_bits(ID_BITS)
    }

    pub fn read_len(&mut self) -> Result<u8, DecodeError> {
        self.read_u8_n_bits(LEN_BITS)
    }

    pub fn read_crc(&mut self) -> Result<u8, DecodeError> {
        self.read_u8_n_bits(CRC_BITS)
    }

    pub fn read_u8(&mut self) -> Result<u8, DecodeError> {
        self.read_u8_n_bits(8)
    }

    pub fn read_u16(&mut self) -> Result<u16, DecodeError> {
        self.read_u16_n_bits(16)
    }

    pub fn read_u32(&mut self) -> Result<u32, DecodeError> {
        self.read_u32_n_bits(32)
    }

    pub fn read_u8_n_bits(&mut self, n: usize) -> Result<u8, DecodeError> {
        self.read_n_bits::<u8>(n)
    }

    pub fn read_u16_n_bits(&mut self, n: usize) -> Result<u16, DecodeError> {
        self.read_n_bits::<u16>(n)
    }

    pub fn read_u32_n_bits(&mut self, n: usize) -> Result<u32, DecodeError> {
        self.read_n_bits::<u32>(n)
    }

    pub fn read_i8(&mut self) -> Result<i8, DecodeError> {
        Ok(self.read_n_bits::<u8>(8)? as i8)
    }

    pub fn read_i16(&mut self) -> Result<i16, DecodeError> {
        Ok(self.read_n_bits::<u16>(16)? as i16)
    }

    pub fn read_i32(&mut self) -> Result<i32, DecodeError> {
        Ok(self.read_n_bits::<u32>(32)? as i32)
    }

    pub fn read_i8_n_bits(&mut self, n: usize) -> Result<i8, DecodeError> {
        Ok(self.read_n_bits::<u8>(n)? as i8)
    }

    pub fn read_i16_n_bits(&mut self, n: usize) -> Result<i16, DecodeError> {
        Ok(self.read_n_bits::<u16>(n)? as i16)
    }

    pub fn read_i32_n_bits(&mut self, n: usize) -> Result<i32, DecodeError> {
        Ok(self.read_n_bits::<u32>(n)? as i32)
    }

    pub fn read_bool(&mut self) -> Result<bool, DecodeError> {
        Ok(self.read_u8_n_bits(1)? != 0)
    }

    pub fn read_n_bits<T>(&mut self, num_bits: usize) -> Result<T, DecodeError>
    where
        T: Default + From<bool> + ShlAssign<usize> + BitOrAssign + Copy,
    {
        if num_bits > 8 {
            return Err(DecodeError::TooManyBits);
        }

        if self.pos + num_bits > self.bits.len() {
            return Err(DecodeError::UnexpectedEnd);
        }

        let mut value = T::default();

        for bit in &self.bits[self.pos..self.pos + num_bits] {
            value <<= 1;
            value |= T::from(*bit);
        }

        self.pos += num_bits;
        Ok(value)
    }
}
