// automatically generated by the FlatBuffers compiler, do not modify
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
/// A MAC address or a bluetooth address, or some other uniquely identifying address
/// associated with the endpoint that we are communicating with. If it doesn't take
/// up the full set of bytes, it is aligned towards the least significant bits.
// struct HardwareAddress, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct HardwareAddress(pub [u8; 8]);
impl Default for HardwareAddress { 
  fn default() -> Self { 
    Self([0; 8])
  }
}
impl core::fmt::Debug for HardwareAddress {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("HardwareAddress")
      .field("addr", &self.addr())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for HardwareAddress {}
impl flatbuffers::SafeSliceAccess for HardwareAddress {}
impl<'a> flatbuffers::Follow<'a> for HardwareAddress {
  type Inner = &'a HardwareAddress;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a HardwareAddress>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a HardwareAddress {
  type Inner = &'a HardwareAddress;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<HardwareAddress>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for HardwareAddress {
    type Output = HardwareAddress;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::core::slice::from_raw_parts(self as *const HardwareAddress as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b HardwareAddress {
    type Output = HardwareAddress;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::core::slice::from_raw_parts(*self as *const HardwareAddress as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for HardwareAddress {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> HardwareAddress {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    addr: u64,
  ) -> Self {
    let mut s = Self([0; 8]);
    s.set_addr(addr);
    s
  }

  pub fn addr(&self) -> u64 {
    let mut mem = core::mem::MaybeUninit::<u64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<u64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_addr(&mut self, x: u64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const u64 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<u64>(),
      );
    }
  }

}

