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
pub enum OverlayDisplayModeChangeRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Changes the state of the overlay's display mode.
pub struct OverlayDisplayModeChangeRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for OverlayDisplayModeChangeRequest<'a> {
  type Inner = OverlayDisplayModeChangeRequest<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> OverlayDisplayModeChangeRequest<'a> {
  pub const VT_IS_VISIBLE: flatbuffers::VOffsetT = 4;
  pub const VT_IS_MIRRORED: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    OverlayDisplayModeChangeRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args OverlayDisplayModeChangeRequestArgs
  ) -> flatbuffers::WIPOffset<OverlayDisplayModeChangeRequest<'bldr>> {
    let mut builder = OverlayDisplayModeChangeRequestBuilder::new(_fbb);
    if let Some(x) = args.is_mirrored { builder.add_is_mirrored(x); }
    if let Some(x) = args.is_visible { builder.add_is_visible(x); }
    builder.finish()
  }


  #[inline]
  pub fn is_visible(&self) -> Option<bool> {
    self._tab.get::<bool>(OverlayDisplayModeChangeRequest::VT_IS_VISIBLE, None)
  }
  #[inline]
  pub fn is_mirrored(&self) -> Option<bool> {
    self._tab.get::<bool>(OverlayDisplayModeChangeRequest::VT_IS_MIRRORED, None)
  }
}

impl flatbuffers::Verifiable for OverlayDisplayModeChangeRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<bool>("is_visible", Self::VT_IS_VISIBLE, false)?
     .visit_field::<bool>("is_mirrored", Self::VT_IS_MIRRORED, false)?
     .finish();
    Ok(())
  }
}
pub struct OverlayDisplayModeChangeRequestArgs {
    pub is_visible: Option<bool>,
    pub is_mirrored: Option<bool>,
}
impl<'a> Default for OverlayDisplayModeChangeRequestArgs {
  #[inline]
  fn default() -> Self {
    OverlayDisplayModeChangeRequestArgs {
      is_visible: None,
      is_mirrored: None,
    }
  }
}

pub struct OverlayDisplayModeChangeRequestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> OverlayDisplayModeChangeRequestBuilder<'a, 'b> {
  #[inline]
  pub fn add_is_visible(&mut self, is_visible: bool) {
    self.fbb_.push_slot_always::<bool>(OverlayDisplayModeChangeRequest::VT_IS_VISIBLE, is_visible);
  }
  #[inline]
  pub fn add_is_mirrored(&mut self, is_mirrored: bool) {
    self.fbb_.push_slot_always::<bool>(OverlayDisplayModeChangeRequest::VT_IS_MIRRORED, is_mirrored);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> OverlayDisplayModeChangeRequestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    OverlayDisplayModeChangeRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<OverlayDisplayModeChangeRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for OverlayDisplayModeChangeRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("OverlayDisplayModeChangeRequest");
      ds.field("is_visible", &self.is_visible());
      ds.field("is_mirrored", &self.is_mirrored());
      ds.finish()
  }
}
