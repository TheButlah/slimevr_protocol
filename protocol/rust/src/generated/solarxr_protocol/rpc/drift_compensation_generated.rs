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
pub enum DriftCompensationOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Settings related to IMU yaw drift compensation
pub struct DriftCompensation<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for DriftCompensation<'a> {
  type Inner = DriftCompensation<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> DriftCompensation<'a> {
  pub const VT_ENABLED: flatbuffers::VOffsetT = 4;
  pub const VT_AMOUNT: flatbuffers::VOffsetT = 6;
  pub const VT_MAX_RESETS: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    DriftCompensation { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args DriftCompensationArgs
  ) -> flatbuffers::WIPOffset<DriftCompensation<'bldr>> {
    let mut builder = DriftCompensationBuilder::new(_fbb);
    builder.add_amount(args.amount);
    builder.add_max_resets(args.max_resets);
    builder.add_enabled(args.enabled);
    builder.finish()
  }


  #[inline]
  pub fn enabled(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(DriftCompensation::VT_ENABLED, Some(false)).unwrap()}
  }
  /// 0 to 1. A higher value results in more yaw drift compensation
  #[inline]
  pub fn amount(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(DriftCompensation::VT_AMOUNT, Some(0.0)).unwrap()}
  }
  /// Number of previous resets to take into account when calculating yaw drift
  #[inline]
  pub fn max_resets(&self) -> u16 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u16>(DriftCompensation::VT_MAX_RESETS, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for DriftCompensation<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<bool>("enabled", Self::VT_ENABLED, false)?
     .visit_field::<f32>("amount", Self::VT_AMOUNT, false)?
     .visit_field::<u16>("max_resets", Self::VT_MAX_RESETS, false)?
     .finish();
    Ok(())
  }
}
pub struct DriftCompensationArgs {
    pub enabled: bool,
    pub amount: f32,
    pub max_resets: u16,
}
impl<'a> Default for DriftCompensationArgs {
  #[inline]
  fn default() -> Self {
    DriftCompensationArgs {
      enabled: false,
      amount: 0.0,
      max_resets: 0,
    }
  }
}

pub struct DriftCompensationBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> DriftCompensationBuilder<'a, 'b> {
  #[inline]
  pub fn add_enabled(&mut self, enabled: bool) {
    self.fbb_.push_slot::<bool>(DriftCompensation::VT_ENABLED, enabled, false);
  }
  #[inline]
  pub fn add_amount(&mut self, amount: f32) {
    self.fbb_.push_slot::<f32>(DriftCompensation::VT_AMOUNT, amount, 0.0);
  }
  #[inline]
  pub fn add_max_resets(&mut self, max_resets: u16) {
    self.fbb_.push_slot::<u16>(DriftCompensation::VT_MAX_RESETS, max_resets, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> DriftCompensationBuilder<'a, 'b> {
    let start = _fbb.start_table();
    DriftCompensationBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<DriftCompensation<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for DriftCompensation<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("DriftCompensation");
      ds.field("enabled", &self.enabled());
      ds.field("amount", &self.amount());
      ds.field("max_resets", &self.max_resets());
      ds.finish()
  }
}
