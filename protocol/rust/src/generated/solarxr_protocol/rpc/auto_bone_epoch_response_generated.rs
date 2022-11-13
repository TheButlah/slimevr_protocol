// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum AutoBoneEpochResponseOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct AutoBoneEpochResponse<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for AutoBoneEpochResponse<'a> {
  type Inner = AutoBoneEpochResponse<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> AutoBoneEpochResponse<'a> {
  pub const VT_CURRENT_EPOCH: flatbuffers::VOffsetT = 4;
  pub const VT_TOTAL_EPOCHS: flatbuffers::VOffsetT = 6;
  pub const VT_EPOCH_ERROR: flatbuffers::VOffsetT = 8;
  pub const VT_ADJUSTED_SKELETON_PARTS: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    AutoBoneEpochResponse { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args AutoBoneEpochResponseArgs<'args>
  ) -> flatbuffers::WIPOffset<AutoBoneEpochResponse<'bldr>> {
    let mut builder = AutoBoneEpochResponseBuilder::new(_fbb);
    if let Some(x) = args.adjusted_skeleton_parts { builder.add_adjusted_skeleton_parts(x); }
    builder.add_epoch_error(args.epoch_error);
    builder.add_total_epochs(args.total_epochs);
    builder.add_current_epoch(args.current_epoch);
    builder.finish()
  }


  #[inline]
  pub fn current_epoch(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(AutoBoneEpochResponse::VT_CURRENT_EPOCH, Some(0)).unwrap()}
  }
  #[inline]
  pub fn total_epochs(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(AutoBoneEpochResponse::VT_TOTAL_EPOCHS, Some(0)).unwrap()}
  }
  #[inline]
  pub fn epoch_error(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(AutoBoneEpochResponse::VT_EPOCH_ERROR, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn adjusted_skeleton_parts(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SkeletonPart<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SkeletonPart>>>>(AutoBoneEpochResponse::VT_ADJUSTED_SKELETON_PARTS, None)}
  }
}

impl flatbuffers::Verifiable for AutoBoneEpochResponse<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("current_epoch", Self::VT_CURRENT_EPOCH, false)?
     .visit_field::<u32>("total_epochs", Self::VT_TOTAL_EPOCHS, false)?
     .visit_field::<f32>("epoch_error", Self::VT_EPOCH_ERROR, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<SkeletonPart>>>>("adjusted_skeleton_parts", Self::VT_ADJUSTED_SKELETON_PARTS, false)?
     .finish();
    Ok(())
  }
}
pub struct AutoBoneEpochResponseArgs<'a> {
    pub current_epoch: u32,
    pub total_epochs: u32,
    pub epoch_error: f32,
    pub adjusted_skeleton_parts: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<SkeletonPart<'a>>>>>,
}
impl<'a> Default for AutoBoneEpochResponseArgs<'a> {
  #[inline]
  fn default() -> Self {
    AutoBoneEpochResponseArgs {
      current_epoch: 0,
      total_epochs: 0,
      epoch_error: 0.0,
      adjusted_skeleton_parts: None,
    }
  }
}

pub struct AutoBoneEpochResponseBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> AutoBoneEpochResponseBuilder<'a, 'b> {
  #[inline]
  pub fn add_current_epoch(&mut self, current_epoch: u32) {
    self.fbb_.push_slot::<u32>(AutoBoneEpochResponse::VT_CURRENT_EPOCH, current_epoch, 0);
  }
  #[inline]
  pub fn add_total_epochs(&mut self, total_epochs: u32) {
    self.fbb_.push_slot::<u32>(AutoBoneEpochResponse::VT_TOTAL_EPOCHS, total_epochs, 0);
  }
  #[inline]
  pub fn add_epoch_error(&mut self, epoch_error: f32) {
    self.fbb_.push_slot::<f32>(AutoBoneEpochResponse::VT_EPOCH_ERROR, epoch_error, 0.0);
  }
  #[inline]
  pub fn add_adjusted_skeleton_parts(&mut self, adjusted_skeleton_parts: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<SkeletonPart<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AutoBoneEpochResponse::VT_ADJUSTED_SKELETON_PARTS, adjusted_skeleton_parts);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AutoBoneEpochResponseBuilder<'a, 'b> {
    let start = _fbb.start_table();
    AutoBoneEpochResponseBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<AutoBoneEpochResponse<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for AutoBoneEpochResponse<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("AutoBoneEpochResponse");
      ds.field("current_epoch", &self.current_epoch());
      ds.field("total_epochs", &self.total_epochs());
      ds.field("epoch_error", &self.epoch_error());
      ds.field("adjusted_skeleton_parts", &self.adjusted_skeleton_parts());
      ds.finish()
  }
}
