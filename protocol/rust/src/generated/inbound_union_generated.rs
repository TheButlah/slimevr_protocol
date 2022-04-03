// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_INBOUND_UNION: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_INBOUND_UNION: u8 = 10;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_INBOUND_UNION: [InboundUnion; 11] = [
  InboundUnion::NONE,
  InboundUnion::slimevr_protocol_rpc_HeartbeatRequest,
  InboundUnion::slimevr_protocol_rpc_ResetRequest,
  InboundUnion::slimevr_protocol_rpc_AssignTrackerRequest,
  InboundUnion::slimevr_protocol_rpc_SettingsRequest,
  InboundUnion::slimevr_protocol_rpc_ChangeSettingsRequest,
  InboundUnion::slimevr_protocol_data_feed_PollDeviceStatus,
  InboundUnion::slimevr_protocol_data_feed_DataFeedRequest,
  InboundUnion::slimevr_protocol_data_feed_DataFeedNotify,
  InboundUnion::slimevr_protocol_data_feed_DeviceStatus,
  InboundUnion::slimevr_protocol_data_feed_DataFeed,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct InboundUnion(pub u8);
#[allow(non_upper_case_globals)]
impl InboundUnion {
  pub const NONE: Self = Self(0);
  pub const slimevr_protocol_rpc_HeartbeatRequest: Self = Self(1);
  pub const slimevr_protocol_rpc_ResetRequest: Self = Self(2);
  pub const slimevr_protocol_rpc_AssignTrackerRequest: Self = Self(3);
  pub const slimevr_protocol_rpc_SettingsRequest: Self = Self(4);
  pub const slimevr_protocol_rpc_ChangeSettingsRequest: Self = Self(5);
  pub const slimevr_protocol_data_feed_PollDeviceStatus: Self = Self(6);
  pub const slimevr_protocol_data_feed_DataFeedRequest: Self = Self(7);
  pub const slimevr_protocol_data_feed_DataFeedNotify: Self = Self(8);
  pub const slimevr_protocol_data_feed_DeviceStatus: Self = Self(9);
  pub const slimevr_protocol_data_feed_DataFeed: Self = Self(10);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 10;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::slimevr_protocol_rpc_HeartbeatRequest,
    Self::slimevr_protocol_rpc_ResetRequest,
    Self::slimevr_protocol_rpc_AssignTrackerRequest,
    Self::slimevr_protocol_rpc_SettingsRequest,
    Self::slimevr_protocol_rpc_ChangeSettingsRequest,
    Self::slimevr_protocol_data_feed_PollDeviceStatus,
    Self::slimevr_protocol_data_feed_DataFeedRequest,
    Self::slimevr_protocol_data_feed_DataFeedNotify,
    Self::slimevr_protocol_data_feed_DeviceStatus,
    Self::slimevr_protocol_data_feed_DataFeed,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::slimevr_protocol_rpc_HeartbeatRequest => Some("slimevr_protocol_rpc_HeartbeatRequest"),
      Self::slimevr_protocol_rpc_ResetRequest => Some("slimevr_protocol_rpc_ResetRequest"),
      Self::slimevr_protocol_rpc_AssignTrackerRequest => Some("slimevr_protocol_rpc_AssignTrackerRequest"),
      Self::slimevr_protocol_rpc_SettingsRequest => Some("slimevr_protocol_rpc_SettingsRequest"),
      Self::slimevr_protocol_rpc_ChangeSettingsRequest => Some("slimevr_protocol_rpc_ChangeSettingsRequest"),
      Self::slimevr_protocol_data_feed_PollDeviceStatus => Some("slimevr_protocol_data_feed_PollDeviceStatus"),
      Self::slimevr_protocol_data_feed_DataFeedRequest => Some("slimevr_protocol_data_feed_DataFeedRequest"),
      Self::slimevr_protocol_data_feed_DataFeedNotify => Some("slimevr_protocol_data_feed_DataFeedNotify"),
      Self::slimevr_protocol_data_feed_DeviceStatus => Some("slimevr_protocol_data_feed_DeviceStatus"),
      Self::slimevr_protocol_data_feed_DataFeed => Some("slimevr_protocol_data_feed_DataFeed"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for InboundUnion {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for InboundUnion {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<u8>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for InboundUnion {
    type Output = InboundUnion;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<u8>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for InboundUnion {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = u8::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = u8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for InboundUnion {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for InboundUnion {}
pub struct InboundUnionUnionTableOffset {}

