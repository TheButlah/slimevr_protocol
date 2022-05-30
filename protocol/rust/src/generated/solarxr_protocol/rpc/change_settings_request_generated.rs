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
pub enum ChangeSettingsRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ChangeSettingsRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ChangeSettingsRequest<'a> {
  type Inner = ChangeSettingsRequest<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> ChangeSettingsRequest<'a> {
  pub const VT_STEAM_VR_TRACKERS: flatbuffers::VOffsetT = 4;
  pub const VT_FILTERING: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ChangeSettingsRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ChangeSettingsRequestArgs<'args>
  ) -> flatbuffers::WIPOffset<ChangeSettingsRequest<'bldr>> {
    let mut builder = ChangeSettingsRequestBuilder::new(_fbb);
    if let Some(x) = args.filtering { builder.add_filtering(x); }
    if let Some(x) = args.steam_vr_trackers { builder.add_steam_vr_trackers(x); }
    builder.finish()
  }


  #[inline]
  pub fn steam_vr_trackers(&self) -> Option<SteamVRTrackersSetting<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<SteamVRTrackersSetting>>(ChangeSettingsRequest::VT_STEAM_VR_TRACKERS, None)
  }
  #[inline]
  pub fn filtering(&self) -> Option<FilteringSettings<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<FilteringSettings>>(ChangeSettingsRequest::VT_FILTERING, None)
  }
}

impl flatbuffers::Verifiable for ChangeSettingsRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<SteamVRTrackersSetting>>("steam_vr_trackers", Self::VT_STEAM_VR_TRACKERS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<FilteringSettings>>("filtering", Self::VT_FILTERING, false)?
     .finish();
    Ok(())
  }
}
pub struct ChangeSettingsRequestArgs<'a> {
    pub steam_vr_trackers: Option<flatbuffers::WIPOffset<SteamVRTrackersSetting<'a>>>,
    pub filtering: Option<flatbuffers::WIPOffset<FilteringSettings<'a>>>,
}
impl<'a> Default for ChangeSettingsRequestArgs<'a> {
  #[inline]
  fn default() -> Self {
    ChangeSettingsRequestArgs {
      steam_vr_trackers: None,
      filtering: None,
    }
  }
}

pub struct ChangeSettingsRequestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ChangeSettingsRequestBuilder<'a, 'b> {
  #[inline]
  pub fn add_steam_vr_trackers(&mut self, steam_vr_trackers: flatbuffers::WIPOffset<SteamVRTrackersSetting<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<SteamVRTrackersSetting>>(ChangeSettingsRequest::VT_STEAM_VR_TRACKERS, steam_vr_trackers);
  }
  #[inline]
  pub fn add_filtering(&mut self, filtering: flatbuffers::WIPOffset<FilteringSettings<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<FilteringSettings>>(ChangeSettingsRequest::VT_FILTERING, filtering);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ChangeSettingsRequestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ChangeSettingsRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ChangeSettingsRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ChangeSettingsRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ChangeSettingsRequest");
      ds.field("steam_vr_trackers", &self.steam_vr_trackers());
      ds.field("filtering", &self.filtering());
      ds.finish()
  }
}
