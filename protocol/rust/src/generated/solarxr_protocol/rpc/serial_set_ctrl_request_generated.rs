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
pub enum SerialSetCtrlRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

/// SerialSetCtrlRequest
/// Changing the Serial RTS and DTR signal from the Serial Line.
/// This signals are common used on Dev-Boards (used in DIY Slimes) for the Flash- or Reset-Pin
/// Some Drivers or Boards use this Pin not the same way. So if you open the WiFi Window,
/// the Reset Pin or the Flash pin is pressed all the time and does not allow a normal Boot.
/// This Control is to give the User control over this function.
pub struct SerialSetCtrlRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SerialSetCtrlRequest<'a> {
  type Inner = SerialSetCtrlRequest<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> SerialSetCtrlRequest<'a> {
  pub const VT_RTS: flatbuffers::VOffsetT = 4;
  pub const VT_DTR: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    SerialSetCtrlRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args SerialSetCtrlRequestArgs
  ) -> flatbuffers::WIPOffset<SerialSetCtrlRequest<'bldr>> {
    let mut builder = SerialSetCtrlRequestBuilder::new(_fbb);
    builder.add_dtr(args.dtr);
    builder.add_rts(args.rts);
    builder.finish()
  }


  #[inline]
  pub fn rts(&self) -> bool {
    self._tab.get::<bool>(SerialSetCtrlRequest::VT_RTS, Some(false)).unwrap()
  }
  #[inline]
  pub fn dtr(&self) -> bool {
    self._tab.get::<bool>(SerialSetCtrlRequest::VT_DTR, Some(false)).unwrap()
  }
}

impl flatbuffers::Verifiable for SerialSetCtrlRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<bool>("rts", Self::VT_RTS, false)?
     .visit_field::<bool>("dtr", Self::VT_DTR, false)?
     .finish();
    Ok(())
  }
}
pub struct SerialSetCtrlRequestArgs {
    pub rts: bool,
    pub dtr: bool,
}
impl<'a> Default for SerialSetCtrlRequestArgs {
  #[inline]
  fn default() -> Self {
    SerialSetCtrlRequestArgs {
      rts: false,
      dtr: false,
    }
  }
}

pub struct SerialSetCtrlRequestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SerialSetCtrlRequestBuilder<'a, 'b> {
  #[inline]
  pub fn add_rts(&mut self, rts: bool) {
    self.fbb_.push_slot::<bool>(SerialSetCtrlRequest::VT_RTS, rts, false);
  }
  #[inline]
  pub fn add_dtr(&mut self, dtr: bool) {
    self.fbb_.push_slot::<bool>(SerialSetCtrlRequest::VT_DTR, dtr, false);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SerialSetCtrlRequestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    SerialSetCtrlRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SerialSetCtrlRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for SerialSetCtrlRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("SerialSetCtrlRequest");
      ds.field("rts", &self.rts());
      ds.field("dtr", &self.dtr());
      ds.finish()
  }
}
