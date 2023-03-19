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
pub enum LegTweaksSettingsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct LegTweaksSettings<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for LegTweaksSettings<'a> {
  type Inner = LegTweaksSettings<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> LegTweaksSettings<'a> {
  pub const VT_CORRECTION_STRENGTH: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    LegTweaksSettings { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args LegTweaksSettingsArgs
  ) -> flatbuffers::WIPOffset<LegTweaksSettings<'bldr>> {
    let mut builder = LegTweaksSettingsBuilder::new(_fbb);
    if let Some(x) = args.correction_strength { builder.add_correction_strength(x); }
    builder.finish()
  }


  #[inline]
  pub fn correction_strength(&self) -> Option<f32> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(LegTweaksSettings::VT_CORRECTION_STRENGTH, None)}
  }
}

impl flatbuffers::Verifiable for LegTweaksSettings<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<f32>("correction_strength", Self::VT_CORRECTION_STRENGTH, false)?
     .finish();
    Ok(())
  }
}
pub struct LegTweaksSettingsArgs {
    pub correction_strength: Option<f32>,
}
impl<'a> Default for LegTweaksSettingsArgs {
  #[inline]
  fn default() -> Self {
    LegTweaksSettingsArgs {
      correction_strength: None,
    }
  }
}

pub struct LegTweaksSettingsBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> LegTweaksSettingsBuilder<'a, 'b> {
  #[inline]
  pub fn add_correction_strength(&mut self, correction_strength: f32) {
    self.fbb_.push_slot_always::<f32>(LegTweaksSettings::VT_CORRECTION_STRENGTH, correction_strength);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> LegTweaksSettingsBuilder<'a, 'b> {
    let start = _fbb.start_table();
    LegTweaksSettingsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<LegTweaksSettings<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for LegTweaksSettings<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("LegTweaksSettings");
      ds.field("correction_strength", &self.correction_strength());
      ds.finish()
  }
}
