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
pub enum RpcMessageHeaderOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RpcMessageHeader<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RpcMessageHeader<'a> {
  type Inner = RpcMessageHeader<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> RpcMessageHeader<'a> {
  pub const VT_TX_ID: flatbuffers::VOffsetT = 4;
  pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 6;
  pub const VT_MESSAGE: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    RpcMessageHeader { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args RpcMessageHeaderArgs<'args>
  ) -> flatbuffers::WIPOffset<RpcMessageHeader<'bldr>> {
    let mut builder = RpcMessageHeaderBuilder::new(_fbb);
    if let Some(x) = args.message { builder.add_message(x); }
    if let Some(x) = args.tx_id { builder.add_tx_id(x); }
    builder.add_message_type(args.message_type);
    builder.finish()
  }


  /// For a request, this identifies the request. For a response, this corresponds
  /// to the request that it is responding to.
  #[inline]
  pub fn tx_id(&self) -> Option<&'a super::datatypes::TransactionId> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::datatypes::TransactionId>(RpcMessageHeader::VT_TX_ID, None)}
  }
  #[inline]
  pub fn message_type(&self) -> RpcMessage {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<RpcMessage>(RpcMessageHeader::VT_MESSAGE_TYPE, Some(RpcMessage::NONE)).unwrap()}
  }
  #[inline]
  pub fn message(&self) -> Option<flatbuffers::Table<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(RpcMessageHeader::VT_MESSAGE, None)}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_heartbeat_request(&self) -> Option<HeartbeatRequest<'a>> {
    if self.message_type() == RpcMessage::HeartbeatRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { HeartbeatRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_heartbeat_response(&self) -> Option<HeartbeatResponse<'a>> {
    if self.message_type() == RpcMessage::HeartbeatResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { HeartbeatResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_reset_request(&self) -> Option<ResetRequest<'a>> {
    if self.message_type() == RpcMessage::ResetRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { ResetRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_assign_tracker_request(&self) -> Option<AssignTrackerRequest<'a>> {
    if self.message_type() == RpcMessage::AssignTrackerRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { AssignTrackerRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_settings_request(&self) -> Option<SettingsRequest<'a>> {
    if self.message_type() == RpcMessage::SettingsRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SettingsRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_settings_response(&self) -> Option<SettingsResponse<'a>> {
    if self.message_type() == RpcMessage::SettingsResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SettingsResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_change_settings_request(&self) -> Option<ChangeSettingsRequest<'a>> {
    if self.message_type() == RpcMessage::ChangeSettingsRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { ChangeSettingsRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_clear_drift_compensation_request(&self) -> Option<ClearDriftCompensationRequest<'a>> {
    if self.message_type() == RpcMessage::ClearDriftCompensationRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { ClearDriftCompensationRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_record_bvhrequest(&self) -> Option<RecordBVHRequest<'a>> {
    if self.message_type() == RpcMessage::RecordBVHRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { RecordBVHRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_record_bvhstatus(&self) -> Option<RecordBVHStatus<'a>> {
    if self.message_type() == RpcMessage::RecordBVHStatus {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { RecordBVHStatus::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_skeleton_config_request(&self) -> Option<SkeletonConfigRequest<'a>> {
    if self.message_type() == RpcMessage::SkeletonConfigRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SkeletonConfigRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_change_skeleton_config_request(&self) -> Option<ChangeSkeletonConfigRequest<'a>> {
    if self.message_type() == RpcMessage::ChangeSkeletonConfigRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { ChangeSkeletonConfigRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_skeleton_reset_all_request(&self) -> Option<SkeletonResetAllRequest<'a>> {
    if self.message_type() == RpcMessage::SkeletonResetAllRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SkeletonResetAllRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_skeleton_config_response(&self) -> Option<SkeletonConfigResponse<'a>> {
    if self.message_type() == RpcMessage::SkeletonConfigResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SkeletonConfigResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_open_serial_request(&self) -> Option<OpenSerialRequest<'a>> {
    if self.message_type() == RpcMessage::OpenSerialRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { OpenSerialRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_close_serial_request(&self) -> Option<CloseSerialRequest<'a>> {
    if self.message_type() == RpcMessage::CloseSerialRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { CloseSerialRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_set_wifi_request(&self) -> Option<SetWifiRequest<'a>> {
    if self.message_type() == RpcMessage::SetWifiRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SetWifiRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_serial_update_response(&self) -> Option<SerialUpdateResponse<'a>> {
    if self.message_type() == RpcMessage::SerialUpdateResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SerialUpdateResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_auto_bone_process_request(&self) -> Option<AutoBoneProcessRequest<'a>> {
    if self.message_type() == RpcMessage::AutoBoneProcessRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { AutoBoneProcessRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_auto_bone_process_status_response(&self) -> Option<AutoBoneProcessStatusResponse<'a>> {
    if self.message_type() == RpcMessage::AutoBoneProcessStatusResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { AutoBoneProcessStatusResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_auto_bone_epoch_response(&self) -> Option<AutoBoneEpochResponse<'a>> {
    if self.message_type() == RpcMessage::AutoBoneEpochResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { AutoBoneEpochResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_overlay_display_mode_request(&self) -> Option<OverlayDisplayModeRequest<'a>> {
    if self.message_type() == RpcMessage::OverlayDisplayModeRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { OverlayDisplayModeRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_overlay_display_mode_change_request(&self) -> Option<OverlayDisplayModeChangeRequest<'a>> {
    if self.message_type() == RpcMessage::OverlayDisplayModeChangeRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { OverlayDisplayModeChangeRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_overlay_display_mode_response(&self) -> Option<OverlayDisplayModeResponse<'a>> {
    if self.message_type() == RpcMessage::OverlayDisplayModeResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { OverlayDisplayModeResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_serial_tracker_reboot_request(&self) -> Option<SerialTrackerRebootRequest<'a>> {
    if self.message_type() == RpcMessage::SerialTrackerRebootRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SerialTrackerRebootRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_serial_tracker_get_info_request(&self) -> Option<SerialTrackerGetInfoRequest<'a>> {
    if self.message_type() == RpcMessage::SerialTrackerGetInfoRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SerialTrackerGetInfoRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_serial_tracker_factory_reset_request(&self) -> Option<SerialTrackerFactoryResetRequest<'a>> {
    if self.message_type() == RpcMessage::SerialTrackerFactoryResetRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SerialTrackerFactoryResetRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_serial_devices_request(&self) -> Option<SerialDevicesRequest<'a>> {
    if self.message_type() == RpcMessage::SerialDevicesRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SerialDevicesRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_serial_devices_response(&self) -> Option<SerialDevicesResponse<'a>> {
    if self.message_type() == RpcMessage::SerialDevicesResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { SerialDevicesResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_new_serial_device_response(&self) -> Option<NewSerialDeviceResponse<'a>> {
    if self.message_type() == RpcMessage::NewSerialDeviceResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { NewSerialDeviceResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_start_wifi_provisioning_request(&self) -> Option<StartWifiProvisioningRequest<'a>> {
    if self.message_type() == RpcMessage::StartWifiProvisioningRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { StartWifiProvisioningRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_stop_wifi_provisioning_request(&self) -> Option<StopWifiProvisioningRequest<'a>> {
    if self.message_type() == RpcMessage::StopWifiProvisioningRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { StopWifiProvisioningRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_wifi_provisioning_status_response(&self) -> Option<WifiProvisioningStatusResponse<'a>> {
    if self.message_type() == RpcMessage::WifiProvisioningStatusResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { WifiProvisioningStatusResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_server_infos_request(&self) -> Option<ServerInfosRequest<'a>> {
    if self.message_type() == RpcMessage::ServerInfosRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { ServerInfosRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_server_infos_response(&self) -> Option<ServerInfosResponse<'a>> {
    if self.message_type() == RpcMessage::ServerInfosResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { ServerInfosResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_guiinfos_request(&self) -> Option<GUIInfosRequest<'a>> {
    if self.message_type() == RpcMessage::GUIInfosRequest {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { GUIInfosRequest::init_from_table(t) }
     })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_guiinfos_response(&self) -> Option<GUIInfosResponse<'a>> {
    if self.message_type() == RpcMessage::GUIInfosResponse {
      self.message().map(|t| {
       // Safety:
       // Created from a valid Table for this object
       // Which contains a valid union in this slot
       unsafe { GUIInfosResponse::init_from_table(t) }
     })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for RpcMessageHeader<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<super::datatypes::TransactionId>("tx_id", Self::VT_TX_ID, false)?
     .visit_union::<RpcMessage, _>("message_type", Self::VT_MESSAGE_TYPE, "message", Self::VT_MESSAGE, false, |key, v, pos| {
        match key {
          RpcMessage::HeartbeatRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<HeartbeatRequest>>("RpcMessage::HeartbeatRequest", pos),
          RpcMessage::HeartbeatResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<HeartbeatResponse>>("RpcMessage::HeartbeatResponse", pos),
          RpcMessage::ResetRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<ResetRequest>>("RpcMessage::ResetRequest", pos),
          RpcMessage::AssignTrackerRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<AssignTrackerRequest>>("RpcMessage::AssignTrackerRequest", pos),
          RpcMessage::SettingsRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SettingsRequest>>("RpcMessage::SettingsRequest", pos),
          RpcMessage::SettingsResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SettingsResponse>>("RpcMessage::SettingsResponse", pos),
          RpcMessage::ChangeSettingsRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<ChangeSettingsRequest>>("RpcMessage::ChangeSettingsRequest", pos),
          RpcMessage::ClearDriftCompensationRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<ClearDriftCompensationRequest>>("RpcMessage::ClearDriftCompensationRequest", pos),
          RpcMessage::RecordBVHRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<RecordBVHRequest>>("RpcMessage::RecordBVHRequest", pos),
          RpcMessage::RecordBVHStatus => v.verify_union_variant::<flatbuffers::ForwardsUOffset<RecordBVHStatus>>("RpcMessage::RecordBVHStatus", pos),
          RpcMessage::SkeletonConfigRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SkeletonConfigRequest>>("RpcMessage::SkeletonConfigRequest", pos),
          RpcMessage::ChangeSkeletonConfigRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<ChangeSkeletonConfigRequest>>("RpcMessage::ChangeSkeletonConfigRequest", pos),
          RpcMessage::SkeletonResetAllRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SkeletonResetAllRequest>>("RpcMessage::SkeletonResetAllRequest", pos),
          RpcMessage::SkeletonConfigResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SkeletonConfigResponse>>("RpcMessage::SkeletonConfigResponse", pos),
          RpcMessage::OpenSerialRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<OpenSerialRequest>>("RpcMessage::OpenSerialRequest", pos),
          RpcMessage::CloseSerialRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<CloseSerialRequest>>("RpcMessage::CloseSerialRequest", pos),
          RpcMessage::SetWifiRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SetWifiRequest>>("RpcMessage::SetWifiRequest", pos),
          RpcMessage::SerialUpdateResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SerialUpdateResponse>>("RpcMessage::SerialUpdateResponse", pos),
          RpcMessage::AutoBoneProcessRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<AutoBoneProcessRequest>>("RpcMessage::AutoBoneProcessRequest", pos),
          RpcMessage::AutoBoneProcessStatusResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<AutoBoneProcessStatusResponse>>("RpcMessage::AutoBoneProcessStatusResponse", pos),
          RpcMessage::AutoBoneEpochResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<AutoBoneEpochResponse>>("RpcMessage::AutoBoneEpochResponse", pos),
          RpcMessage::OverlayDisplayModeRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<OverlayDisplayModeRequest>>("RpcMessage::OverlayDisplayModeRequest", pos),
          RpcMessage::OverlayDisplayModeChangeRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<OverlayDisplayModeChangeRequest>>("RpcMessage::OverlayDisplayModeChangeRequest", pos),
          RpcMessage::OverlayDisplayModeResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<OverlayDisplayModeResponse>>("RpcMessage::OverlayDisplayModeResponse", pos),
          RpcMessage::SerialTrackerRebootRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SerialTrackerRebootRequest>>("RpcMessage::SerialTrackerRebootRequest", pos),
          RpcMessage::SerialTrackerGetInfoRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SerialTrackerGetInfoRequest>>("RpcMessage::SerialTrackerGetInfoRequest", pos),
          RpcMessage::SerialTrackerFactoryResetRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SerialTrackerFactoryResetRequest>>("RpcMessage::SerialTrackerFactoryResetRequest", pos),
          RpcMessage::SerialDevicesRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SerialDevicesRequest>>("RpcMessage::SerialDevicesRequest", pos),
          RpcMessage::SerialDevicesResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<SerialDevicesResponse>>("RpcMessage::SerialDevicesResponse", pos),
          RpcMessage::NewSerialDeviceResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<NewSerialDeviceResponse>>("RpcMessage::NewSerialDeviceResponse", pos),
          RpcMessage::StartWifiProvisioningRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<StartWifiProvisioningRequest>>("RpcMessage::StartWifiProvisioningRequest", pos),
          RpcMessage::StopWifiProvisioningRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<StopWifiProvisioningRequest>>("RpcMessage::StopWifiProvisioningRequest", pos),
          RpcMessage::WifiProvisioningStatusResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<WifiProvisioningStatusResponse>>("RpcMessage::WifiProvisioningStatusResponse", pos),
          RpcMessage::ServerInfosRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<ServerInfosRequest>>("RpcMessage::ServerInfosRequest", pos),
          RpcMessage::ServerInfosResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<ServerInfosResponse>>("RpcMessage::ServerInfosResponse", pos),
          RpcMessage::GUIInfosRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<GUIInfosRequest>>("RpcMessage::GUIInfosRequest", pos),
          RpcMessage::GUIInfosResponse => v.verify_union_variant::<flatbuffers::ForwardsUOffset<GUIInfosResponse>>("RpcMessage::GUIInfosResponse", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct RpcMessageHeaderArgs<'a> {
    pub tx_id: Option<&'a super::datatypes::TransactionId>,
    pub message_type: RpcMessage,
    pub message: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for RpcMessageHeaderArgs<'a> {
  #[inline]
  fn default() -> Self {
    RpcMessageHeaderArgs {
      tx_id: None,
      message_type: RpcMessage::NONE,
      message: None,
    }
  }
}

pub struct RpcMessageHeaderBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RpcMessageHeaderBuilder<'a, 'b> {
  #[inline]
  pub fn add_tx_id(&mut self, tx_id: &super::datatypes::TransactionId) {
    self.fbb_.push_slot_always::<&super::datatypes::TransactionId>(RpcMessageHeader::VT_TX_ID, tx_id);
  }
  #[inline]
  pub fn add_message_type(&mut self, message_type: RpcMessage) {
    self.fbb_.push_slot::<RpcMessage>(RpcMessageHeader::VT_MESSAGE_TYPE, message_type, RpcMessage::NONE);
  }
  #[inline]
  pub fn add_message(&mut self, message: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RpcMessageHeader::VT_MESSAGE, message);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RpcMessageHeaderBuilder<'a, 'b> {
    let start = _fbb.start_table();
    RpcMessageHeaderBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<RpcMessageHeader<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for RpcMessageHeader<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("RpcMessageHeader");
      ds.field("tx_id", &self.tx_id());
      ds.field("message_type", &self.message_type());
      match self.message_type() {
        RpcMessage::HeartbeatRequest => {
          if let Some(x) = self.message_as_heartbeat_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::HeartbeatResponse => {
          if let Some(x) = self.message_as_heartbeat_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::ResetRequest => {
          if let Some(x) = self.message_as_reset_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::AssignTrackerRequest => {
          if let Some(x) = self.message_as_assign_tracker_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SettingsRequest => {
          if let Some(x) = self.message_as_settings_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SettingsResponse => {
          if let Some(x) = self.message_as_settings_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::ChangeSettingsRequest => {
          if let Some(x) = self.message_as_change_settings_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::ClearDriftCompensationRequest => {
          if let Some(x) = self.message_as_clear_drift_compensation_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::RecordBVHRequest => {
          if let Some(x) = self.message_as_record_bvhrequest() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::RecordBVHStatus => {
          if let Some(x) = self.message_as_record_bvhstatus() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SkeletonConfigRequest => {
          if let Some(x) = self.message_as_skeleton_config_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::ChangeSkeletonConfigRequest => {
          if let Some(x) = self.message_as_change_skeleton_config_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SkeletonResetAllRequest => {
          if let Some(x) = self.message_as_skeleton_reset_all_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SkeletonConfigResponse => {
          if let Some(x) = self.message_as_skeleton_config_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::OpenSerialRequest => {
          if let Some(x) = self.message_as_open_serial_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::CloseSerialRequest => {
          if let Some(x) = self.message_as_close_serial_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SetWifiRequest => {
          if let Some(x) = self.message_as_set_wifi_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SerialUpdateResponse => {
          if let Some(x) = self.message_as_serial_update_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::AutoBoneProcessRequest => {
          if let Some(x) = self.message_as_auto_bone_process_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::AutoBoneProcessStatusResponse => {
          if let Some(x) = self.message_as_auto_bone_process_status_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::AutoBoneEpochResponse => {
          if let Some(x) = self.message_as_auto_bone_epoch_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::OverlayDisplayModeRequest => {
          if let Some(x) = self.message_as_overlay_display_mode_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::OverlayDisplayModeChangeRequest => {
          if let Some(x) = self.message_as_overlay_display_mode_change_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::OverlayDisplayModeResponse => {
          if let Some(x) = self.message_as_overlay_display_mode_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SerialTrackerRebootRequest => {
          if let Some(x) = self.message_as_serial_tracker_reboot_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SerialTrackerGetInfoRequest => {
          if let Some(x) = self.message_as_serial_tracker_get_info_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SerialTrackerFactoryResetRequest => {
          if let Some(x) = self.message_as_serial_tracker_factory_reset_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SerialDevicesRequest => {
          if let Some(x) = self.message_as_serial_devices_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::SerialDevicesResponse => {
          if let Some(x) = self.message_as_serial_devices_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::NewSerialDeviceResponse => {
          if let Some(x) = self.message_as_new_serial_device_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::StartWifiProvisioningRequest => {
          if let Some(x) = self.message_as_start_wifi_provisioning_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::StopWifiProvisioningRequest => {
          if let Some(x) = self.message_as_stop_wifi_provisioning_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::WifiProvisioningStatusResponse => {
          if let Some(x) = self.message_as_wifi_provisioning_status_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::ServerInfosRequest => {
          if let Some(x) = self.message_as_server_infos_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::ServerInfosResponse => {
          if let Some(x) = self.message_as_server_infos_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::GUIInfosRequest => {
          if let Some(x) = self.message_as_guiinfos_request() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RpcMessage::GUIInfosResponse => {
          if let Some(x) = self.message_as_guiinfos_response() {
            ds.field("message", &x)
          } else {
            ds.field("message", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("message", &x)
        },
      };
      ds.finish()
  }
}
