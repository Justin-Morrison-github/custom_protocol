use crc::{CRC_32_ISO_HDLC, Crc};
const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
use bitvec::{self, order::Msb0, vec::BitVec, view::BitView};

use crate::bcp::{ID_BITS, LEN_BITS};

pub fn calc_crc(bytes: &[u8]) -> u32 {
    CASTAGNOLI.checksum(bytes)
}

#[derive(Debug)]
pub struct Bits {
    bits: BitVec<u8, Msb0>,
}

impl Bits {
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
        let raw = value as u8;
        self.push_u8(raw);
    }

    pub fn push_i16(&mut self, value: i16) {
        let raw = value as u16;
        self.push_u16(raw);
    }

    pub fn push_i32(&mut self, value: i32) {
        let raw = value as u32;
        self.push_u32(raw);
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
