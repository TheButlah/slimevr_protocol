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
pub enum SetWifiRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct SetWifiRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SetWifiRequest<'a> {
  type Inner = SetWifiRequest<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> SetWifiRequest<'a> {
  pub const VT_SSID: flatbuffers::VOffsetT = 4;
  pub const VT_PASSWORD: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    SetWifiRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args SetWifiRequestArgs<'args>
  ) -> flatbuffers::WIPOffset<SetWifiRequest<'bldr>> {
    let mut builder = SetWifiRequestBuilder::new(_fbb);
    if let Some(x) = args.password { builder.add_password(x); }
    if let Some(x) = args.ssid { builder.add_ssid(x); }
    builder.finish()
  }


  #[inline]
  pub fn ssid(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SetWifiRequest::VT_SSID, None)
  }
  #[inline]
  pub fn password(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(SetWifiRequest::VT_PASSWORD, None)
  }
}

impl flatbuffers::Verifiable for SetWifiRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("ssid", Self::VT_SSID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("password", Self::VT_PASSWORD, false)?
     .finish();
    Ok(())
  }
}
pub struct SetWifiRequestArgs<'a> {
    pub ssid: Option<flatbuffers::WIPOffset<&'a str>>,
    pub password: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for SetWifiRequestArgs<'a> {
  #[inline]
  fn default() -> Self {
    SetWifiRequestArgs {
      ssid: None,
      password: None,
    }
  }
}

pub struct SetWifiRequestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SetWifiRequestBuilder<'a, 'b> {
  #[inline]
  pub fn add_ssid(&mut self, ssid: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SetWifiRequest::VT_SSID, ssid);
  }
  #[inline]
  pub fn add_password(&mut self, password: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(SetWifiRequest::VT_PASSWORD, password);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SetWifiRequestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    SetWifiRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SetWifiRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for SetWifiRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("SetWifiRequest");
      ds.field("ssid", &self.ssid());
      ds.field("password", &self.password());
      ds.finish()
  }
}
