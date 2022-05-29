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
pub enum DeviceDataMaskOffset {}
#[derive(Copy, Clone, PartialEq)]

/// A mask of values to be reported in subsequent DeviceStatus. Values set to `false`
/// or `null` will not reported. By default, all fields are false/null.
///
/// If you set a value to `true`, it is not guaranteed that the sender actually has
/// such a value to send. In this case, they will probably send `null`, and the receiver
/// has the choice to disconnect due to missing data.
pub struct DeviceDataMask<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for DeviceDataMask<'a> {
  type Inner = DeviceDataMask<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> DeviceDataMask<'a> {
  pub const VT_TRACKER_DATA: flatbuffers::VOffsetT = 4;
  pub const VT_DEVICE_DATA: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    DeviceDataMask { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args DeviceDataMaskArgs<'args>
  ) -> flatbuffers::WIPOffset<DeviceDataMask<'bldr>> {
    let mut builder = DeviceDataMaskBuilder::new(_fbb);
    if let Some(x) = args.tracker_data { builder.add_tracker_data(x); }
    builder.add_device_data(args.device_data);
    builder.finish()
  }


  /// Which tracker data should be sent in this data feed
  #[inline]
  pub fn tracker_data(&self) -> Option<super::tracker::TrackerDataMask<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<super::tracker::TrackerDataMask>>(DeviceDataMask::VT_TRACKER_DATA, None)
  }
  /// true if device data should be sent in this data feed
  #[inline]
  pub fn device_data(&self) -> bool {
    self._tab.get::<bool>(DeviceDataMask::VT_DEVICE_DATA, Some(false)).unwrap()
  }
}

impl flatbuffers::Verifiable for DeviceDataMask<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<super::tracker::TrackerDataMask>>("tracker_data", Self::VT_TRACKER_DATA, false)?
     .visit_field::<bool>("device_data", Self::VT_DEVICE_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct DeviceDataMaskArgs<'a> {
    pub tracker_data: Option<flatbuffers::WIPOffset<super::tracker::TrackerDataMask<'a>>>,
    pub device_data: bool,
}
impl<'a> Default for DeviceDataMaskArgs<'a> {
  #[inline]
  fn default() -> Self {
    DeviceDataMaskArgs {
      tracker_data: None,
      device_data: false,
    }
  }
}

pub struct DeviceDataMaskBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> DeviceDataMaskBuilder<'a, 'b> {
  #[inline]
  pub fn add_tracker_data(&mut self, tracker_data: flatbuffers::WIPOffset<super::tracker::TrackerDataMask<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::tracker::TrackerDataMask>>(DeviceDataMask::VT_TRACKER_DATA, tracker_data);
  }
  #[inline]
  pub fn add_device_data(&mut self, device_data: bool) {
    self.fbb_.push_slot::<bool>(DeviceDataMask::VT_DEVICE_DATA, device_data, false);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> DeviceDataMaskBuilder<'a, 'b> {
    let start = _fbb.start_table();
    DeviceDataMaskBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<DeviceDataMask<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for DeviceDataMask<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("DeviceDataMask");
      ds.field("tracker_data", &self.tracker_data());
      ds.field("device_data", &self.device_data());
      ds.finish()
  }
}
