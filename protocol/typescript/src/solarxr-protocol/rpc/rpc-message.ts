// automatically generated by the FlatBuffers compiler, do not modify

import { AssignTrackerRequest, AssignTrackerRequestT } from '../../solarxr-protocol/rpc/assign-tracker-request.js';
import { AutoBoneEpochResponse, AutoBoneEpochResponseT } from '../../solarxr-protocol/rpc/auto-bone-epoch-response.js';
import { AutoBoneProcessRequest, AutoBoneProcessRequestT } from '../../solarxr-protocol/rpc/auto-bone-process-request.js';
import { AutoBoneProcessStatusResponse, AutoBoneProcessStatusResponseT } from '../../solarxr-protocol/rpc/auto-bone-process-status-response.js';
import { ChangeSettingsRequest, ChangeSettingsRequestT } from '../../solarxr-protocol/rpc/change-settings-request.js';
import { ChangeSkeletonConfigRequest, ChangeSkeletonConfigRequestT } from '../../solarxr-protocol/rpc/change-skeleton-config-request.js';
import { ClearDriftCompensationRequest, ClearDriftCompensationRequestT } from '../../solarxr-protocol/rpc/clear-drift-compensation-request.js';
import { CloseSerialRequest, CloseSerialRequestT } from '../../solarxr-protocol/rpc/close-serial-request.js';
import { HeartbeatRequest, HeartbeatRequestT } from '../../solarxr-protocol/rpc/heartbeat-request.js';
import { HeartbeatResponse, HeartbeatResponseT } from '../../solarxr-protocol/rpc/heartbeat-response.js';
import { LegTweaksTmpChange, LegTweaksTmpChangeT } from '../../solarxr-protocol/rpc/leg-tweaks-tmp-change.js';
import { LegTweaksTmpClear, LegTweaksTmpClearT } from '../../solarxr-protocol/rpc/leg-tweaks-tmp-clear.js';
import { NewSerialDeviceResponse, NewSerialDeviceResponseT } from '../../solarxr-protocol/rpc/new-serial-device-response.js';
import { OpenSerialRequest, OpenSerialRequestT } from '../../solarxr-protocol/rpc/open-serial-request.js';
import { OverlayDisplayModeChangeRequest, OverlayDisplayModeChangeRequestT } from '../../solarxr-protocol/rpc/overlay-display-mode-change-request.js';
import { OverlayDisplayModeRequest, OverlayDisplayModeRequestT } from '../../solarxr-protocol/rpc/overlay-display-mode-request.js';
import { OverlayDisplayModeResponse, OverlayDisplayModeResponseT } from '../../solarxr-protocol/rpc/overlay-display-mode-response.js';
import { RecordBVHRequest, RecordBVHRequestT } from '../../solarxr-protocol/rpc/record-bvhrequest.js';
import { RecordBVHStatus, RecordBVHStatusT } from '../../solarxr-protocol/rpc/record-bvhstatus.js';
import { ResetRequest, ResetRequestT } from '../../solarxr-protocol/rpc/reset-request.js';
import { ResetResponse, ResetResponseT } from '../../solarxr-protocol/rpc/reset-response.js';
import { SerialDevicesRequest, SerialDevicesRequestT } from '../../solarxr-protocol/rpc/serial-devices-request.js';
import { SerialDevicesResponse, SerialDevicesResponseT } from '../../solarxr-protocol/rpc/serial-devices-response.js';
import { SerialTrackerFactoryResetRequest, SerialTrackerFactoryResetRequestT } from '../../solarxr-protocol/rpc/serial-tracker-factory-reset-request.js';
import { SerialTrackerGetInfoRequest, SerialTrackerGetInfoRequestT } from '../../solarxr-protocol/rpc/serial-tracker-get-info-request.js';
import { SerialTrackerRebootRequest, SerialTrackerRebootRequestT } from '../../solarxr-protocol/rpc/serial-tracker-reboot-request.js';
import { SerialUpdateResponse, SerialUpdateResponseT } from '../../solarxr-protocol/rpc/serial-update-response.js';
import { ServerInfosRequest, ServerInfosRequestT } from '../../solarxr-protocol/rpc/server-infos-request.js';
import { ServerInfosResponse, ServerInfosResponseT } from '../../solarxr-protocol/rpc/server-infos-response.js';
import { SetWifiRequest, SetWifiRequestT } from '../../solarxr-protocol/rpc/set-wifi-request.js';
import { SettingsRequest, SettingsRequestT } from '../../solarxr-protocol/rpc/settings-request.js';
import { SettingsResponse, SettingsResponseT } from '../../solarxr-protocol/rpc/settings-response.js';
import { SkeletonConfigRequest, SkeletonConfigRequestT } from '../../solarxr-protocol/rpc/skeleton-config-request.js';
import { SkeletonConfigResponse, SkeletonConfigResponseT } from '../../solarxr-protocol/rpc/skeleton-config-response.js';
import { SkeletonResetAllRequest, SkeletonResetAllRequestT } from '../../solarxr-protocol/rpc/skeleton-reset-all-request.js';
import { StartWifiProvisioningRequest, StartWifiProvisioningRequestT } from '../../solarxr-protocol/rpc/start-wifi-provisioning-request.js';
import { StopWifiProvisioningRequest, StopWifiProvisioningRequestT } from '../../solarxr-protocol/rpc/stop-wifi-provisioning-request.js';
import { WifiProvisioningStatusResponse, WifiProvisioningStatusResponseT } from '../../solarxr-protocol/rpc/wifi-provisioning-status-response.js';


export enum RpcMessage {
  NONE = 0,
  HeartbeatRequest = 1,
  HeartbeatResponse = 2,
  ResetRequest = 3,
  ResetResponse = 4,
  AssignTrackerRequest = 5,
  SettingsRequest = 6,
  SettingsResponse = 7,
  ChangeSettingsRequest = 8,
  ClearDriftCompensationRequest = 9,
  RecordBVHRequest = 10,
  RecordBVHStatus = 11,
  SkeletonConfigRequest = 12,
  ChangeSkeletonConfigRequest = 13,
  SkeletonResetAllRequest = 14,
  SkeletonConfigResponse = 15,
  OpenSerialRequest = 16,
  CloseSerialRequest = 17,
  SetWifiRequest = 18,
  SerialUpdateResponse = 19,
  AutoBoneProcessRequest = 20,
  AutoBoneProcessStatusResponse = 21,
  AutoBoneEpochResponse = 22,
  OverlayDisplayModeRequest = 23,
  OverlayDisplayModeChangeRequest = 24,
  OverlayDisplayModeResponse = 25,
  SerialTrackerRebootRequest = 26,
  SerialTrackerGetInfoRequest = 27,
  SerialTrackerFactoryResetRequest = 28,
  SerialDevicesRequest = 29,
  SerialDevicesResponse = 30,
  NewSerialDeviceResponse = 31,
  StartWifiProvisioningRequest = 32,
  StopWifiProvisioningRequest = 33,
  WifiProvisioningStatusResponse = 34,
  ServerInfosRequest = 35,
  ServerInfosResponse = 36,
  LegTweaksTmpChange = 37,
  LegTweaksTmpClear = 38
}

export function unionToRpcMessage(
  type: RpcMessage,
  accessor: (obj:AssignTrackerRequest|AutoBoneEpochResponse|AutoBoneProcessRequest|AutoBoneProcessStatusResponse|ChangeSettingsRequest|ChangeSkeletonConfigRequest|ClearDriftCompensationRequest|CloseSerialRequest|HeartbeatRequest|HeartbeatResponse|LegTweaksTmpChange|LegTweaksTmpClear|NewSerialDeviceResponse|OpenSerialRequest|OverlayDisplayModeChangeRequest|OverlayDisplayModeRequest|OverlayDisplayModeResponse|RecordBVHRequest|RecordBVHStatus|ResetRequest|ResetResponse|SerialDevicesRequest|SerialDevicesResponse|SerialTrackerFactoryResetRequest|SerialTrackerGetInfoRequest|SerialTrackerRebootRequest|SerialUpdateResponse|ServerInfosRequest|ServerInfosResponse|SetWifiRequest|SettingsRequest|SettingsResponse|SkeletonConfigRequest|SkeletonConfigResponse|SkeletonResetAllRequest|StartWifiProvisioningRequest|StopWifiProvisioningRequest|WifiProvisioningStatusResponse) => AssignTrackerRequest|AutoBoneEpochResponse|AutoBoneProcessRequest|AutoBoneProcessStatusResponse|ChangeSettingsRequest|ChangeSkeletonConfigRequest|ClearDriftCompensationRequest|CloseSerialRequest|HeartbeatRequest|HeartbeatResponse|LegTweaksTmpChange|LegTweaksTmpClear|NewSerialDeviceResponse|OpenSerialRequest|OverlayDisplayModeChangeRequest|OverlayDisplayModeRequest|OverlayDisplayModeResponse|RecordBVHRequest|RecordBVHStatus|ResetRequest|ResetResponse|SerialDevicesRequest|SerialDevicesResponse|SerialTrackerFactoryResetRequest|SerialTrackerGetInfoRequest|SerialTrackerRebootRequest|SerialUpdateResponse|ServerInfosRequest|ServerInfosResponse|SetWifiRequest|SettingsRequest|SettingsResponse|SkeletonConfigRequest|SkeletonConfigResponse|SkeletonResetAllRequest|StartWifiProvisioningRequest|StopWifiProvisioningRequest|WifiProvisioningStatusResponse|null
): AssignTrackerRequest|AutoBoneEpochResponse|AutoBoneProcessRequest|AutoBoneProcessStatusResponse|ChangeSettingsRequest|ChangeSkeletonConfigRequest|ClearDriftCompensationRequest|CloseSerialRequest|HeartbeatRequest|HeartbeatResponse|LegTweaksTmpChange|LegTweaksTmpClear|NewSerialDeviceResponse|OpenSerialRequest|OverlayDisplayModeChangeRequest|OverlayDisplayModeRequest|OverlayDisplayModeResponse|RecordBVHRequest|RecordBVHStatus|ResetRequest|ResetResponse|SerialDevicesRequest|SerialDevicesResponse|SerialTrackerFactoryResetRequest|SerialTrackerGetInfoRequest|SerialTrackerRebootRequest|SerialUpdateResponse|ServerInfosRequest|ServerInfosResponse|SetWifiRequest|SettingsRequest|SettingsResponse|SkeletonConfigRequest|SkeletonConfigResponse|SkeletonResetAllRequest|StartWifiProvisioningRequest|StopWifiProvisioningRequest|WifiProvisioningStatusResponse|null {
  switch(RpcMessage[type]) {
    case 'NONE': return null; 
    case 'HeartbeatRequest': return accessor(new HeartbeatRequest())! as HeartbeatRequest;
    case 'HeartbeatResponse': return accessor(new HeartbeatResponse())! as HeartbeatResponse;
    case 'ResetRequest': return accessor(new ResetRequest())! as ResetRequest;
    case 'ResetResponse': return accessor(new ResetResponse())! as ResetResponse;
    case 'AssignTrackerRequest': return accessor(new AssignTrackerRequest())! as AssignTrackerRequest;
    case 'SettingsRequest': return accessor(new SettingsRequest())! as SettingsRequest;
    case 'SettingsResponse': return accessor(new SettingsResponse())! as SettingsResponse;
    case 'ChangeSettingsRequest': return accessor(new ChangeSettingsRequest())! as ChangeSettingsRequest;
    case 'ClearDriftCompensationRequest': return accessor(new ClearDriftCompensationRequest())! as ClearDriftCompensationRequest;
    case 'RecordBVHRequest': return accessor(new RecordBVHRequest())! as RecordBVHRequest;
    case 'RecordBVHStatus': return accessor(new RecordBVHStatus())! as RecordBVHStatus;
    case 'SkeletonConfigRequest': return accessor(new SkeletonConfigRequest())! as SkeletonConfigRequest;
    case 'ChangeSkeletonConfigRequest': return accessor(new ChangeSkeletonConfigRequest())! as ChangeSkeletonConfigRequest;
    case 'SkeletonResetAllRequest': return accessor(new SkeletonResetAllRequest())! as SkeletonResetAllRequest;
    case 'SkeletonConfigResponse': return accessor(new SkeletonConfigResponse())! as SkeletonConfigResponse;
    case 'OpenSerialRequest': return accessor(new OpenSerialRequest())! as OpenSerialRequest;
    case 'CloseSerialRequest': return accessor(new CloseSerialRequest())! as CloseSerialRequest;
    case 'SetWifiRequest': return accessor(new SetWifiRequest())! as SetWifiRequest;
    case 'SerialUpdateResponse': return accessor(new SerialUpdateResponse())! as SerialUpdateResponse;
    case 'AutoBoneProcessRequest': return accessor(new AutoBoneProcessRequest())! as AutoBoneProcessRequest;
    case 'AutoBoneProcessStatusResponse': return accessor(new AutoBoneProcessStatusResponse())! as AutoBoneProcessStatusResponse;
    case 'AutoBoneEpochResponse': return accessor(new AutoBoneEpochResponse())! as AutoBoneEpochResponse;
    case 'OverlayDisplayModeRequest': return accessor(new OverlayDisplayModeRequest())! as OverlayDisplayModeRequest;
    case 'OverlayDisplayModeChangeRequest': return accessor(new OverlayDisplayModeChangeRequest())! as OverlayDisplayModeChangeRequest;
    case 'OverlayDisplayModeResponse': return accessor(new OverlayDisplayModeResponse())! as OverlayDisplayModeResponse;
    case 'SerialTrackerRebootRequest': return accessor(new SerialTrackerRebootRequest())! as SerialTrackerRebootRequest;
    case 'SerialTrackerGetInfoRequest': return accessor(new SerialTrackerGetInfoRequest())! as SerialTrackerGetInfoRequest;
    case 'SerialTrackerFactoryResetRequest': return accessor(new SerialTrackerFactoryResetRequest())! as SerialTrackerFactoryResetRequest;
    case 'SerialDevicesRequest': return accessor(new SerialDevicesRequest())! as SerialDevicesRequest;
    case 'SerialDevicesResponse': return accessor(new SerialDevicesResponse())! as SerialDevicesResponse;
    case 'NewSerialDeviceResponse': return accessor(new NewSerialDeviceResponse())! as NewSerialDeviceResponse;
    case 'StartWifiProvisioningRequest': return accessor(new StartWifiProvisioningRequest())! as StartWifiProvisioningRequest;
    case 'StopWifiProvisioningRequest': return accessor(new StopWifiProvisioningRequest())! as StopWifiProvisioningRequest;
    case 'WifiProvisioningStatusResponse': return accessor(new WifiProvisioningStatusResponse())! as WifiProvisioningStatusResponse;
    case 'ServerInfosRequest': return accessor(new ServerInfosRequest())! as ServerInfosRequest;
    case 'ServerInfosResponse': return accessor(new ServerInfosResponse())! as ServerInfosResponse;
    case 'LegTweaksTmpChange': return accessor(new LegTweaksTmpChange())! as LegTweaksTmpChange;
    case 'LegTweaksTmpClear': return accessor(new LegTweaksTmpClear())! as LegTweaksTmpClear;
    default: return null;
  }
}

export function unionListToRpcMessage(
  type: RpcMessage, 
  accessor: (index: number, obj:AssignTrackerRequest|AutoBoneEpochResponse|AutoBoneProcessRequest|AutoBoneProcessStatusResponse|ChangeSettingsRequest|ChangeSkeletonConfigRequest|ClearDriftCompensationRequest|CloseSerialRequest|HeartbeatRequest|HeartbeatResponse|LegTweaksTmpChange|LegTweaksTmpClear|NewSerialDeviceResponse|OpenSerialRequest|OverlayDisplayModeChangeRequest|OverlayDisplayModeRequest|OverlayDisplayModeResponse|RecordBVHRequest|RecordBVHStatus|ResetRequest|ResetResponse|SerialDevicesRequest|SerialDevicesResponse|SerialTrackerFactoryResetRequest|SerialTrackerGetInfoRequest|SerialTrackerRebootRequest|SerialUpdateResponse|ServerInfosRequest|ServerInfosResponse|SetWifiRequest|SettingsRequest|SettingsResponse|SkeletonConfigRequest|SkeletonConfigResponse|SkeletonResetAllRequest|StartWifiProvisioningRequest|StopWifiProvisioningRequest|WifiProvisioningStatusResponse) => AssignTrackerRequest|AutoBoneEpochResponse|AutoBoneProcessRequest|AutoBoneProcessStatusResponse|ChangeSettingsRequest|ChangeSkeletonConfigRequest|ClearDriftCompensationRequest|CloseSerialRequest|HeartbeatRequest|HeartbeatResponse|LegTweaksTmpChange|LegTweaksTmpClear|NewSerialDeviceResponse|OpenSerialRequest|OverlayDisplayModeChangeRequest|OverlayDisplayModeRequest|OverlayDisplayModeResponse|RecordBVHRequest|RecordBVHStatus|ResetRequest|ResetResponse|SerialDevicesRequest|SerialDevicesResponse|SerialTrackerFactoryResetRequest|SerialTrackerGetInfoRequest|SerialTrackerRebootRequest|SerialUpdateResponse|ServerInfosRequest|ServerInfosResponse|SetWifiRequest|SettingsRequest|SettingsResponse|SkeletonConfigRequest|SkeletonConfigResponse|SkeletonResetAllRequest|StartWifiProvisioningRequest|StopWifiProvisioningRequest|WifiProvisioningStatusResponse|null, 
  index: number
): AssignTrackerRequest|AutoBoneEpochResponse|AutoBoneProcessRequest|AutoBoneProcessStatusResponse|ChangeSettingsRequest|ChangeSkeletonConfigRequest|ClearDriftCompensationRequest|CloseSerialRequest|HeartbeatRequest|HeartbeatResponse|LegTweaksTmpChange|LegTweaksTmpClear|NewSerialDeviceResponse|OpenSerialRequest|OverlayDisplayModeChangeRequest|OverlayDisplayModeRequest|OverlayDisplayModeResponse|RecordBVHRequest|RecordBVHStatus|ResetRequest|ResetResponse|SerialDevicesRequest|SerialDevicesResponse|SerialTrackerFactoryResetRequest|SerialTrackerGetInfoRequest|SerialTrackerRebootRequest|SerialUpdateResponse|ServerInfosRequest|ServerInfosResponse|SetWifiRequest|SettingsRequest|SettingsResponse|SkeletonConfigRequest|SkeletonConfigResponse|SkeletonResetAllRequest|StartWifiProvisioningRequest|StopWifiProvisioningRequest|WifiProvisioningStatusResponse|null {
  switch(RpcMessage[type]) {
    case 'NONE': return null; 
    case 'HeartbeatRequest': return accessor(index, new HeartbeatRequest())! as HeartbeatRequest;
    case 'HeartbeatResponse': return accessor(index, new HeartbeatResponse())! as HeartbeatResponse;
    case 'ResetRequest': return accessor(index, new ResetRequest())! as ResetRequest;
    case 'ResetResponse': return accessor(index, new ResetResponse())! as ResetResponse;
    case 'AssignTrackerRequest': return accessor(index, new AssignTrackerRequest())! as AssignTrackerRequest;
    case 'SettingsRequest': return accessor(index, new SettingsRequest())! as SettingsRequest;
    case 'SettingsResponse': return accessor(index, new SettingsResponse())! as SettingsResponse;
    case 'ChangeSettingsRequest': return accessor(index, new ChangeSettingsRequest())! as ChangeSettingsRequest;
    case 'ClearDriftCompensationRequest': return accessor(index, new ClearDriftCompensationRequest())! as ClearDriftCompensationRequest;
    case 'RecordBVHRequest': return accessor(index, new RecordBVHRequest())! as RecordBVHRequest;
    case 'RecordBVHStatus': return accessor(index, new RecordBVHStatus())! as RecordBVHStatus;
    case 'SkeletonConfigRequest': return accessor(index, new SkeletonConfigRequest())! as SkeletonConfigRequest;
    case 'ChangeSkeletonConfigRequest': return accessor(index, new ChangeSkeletonConfigRequest())! as ChangeSkeletonConfigRequest;
    case 'SkeletonResetAllRequest': return accessor(index, new SkeletonResetAllRequest())! as SkeletonResetAllRequest;
    case 'SkeletonConfigResponse': return accessor(index, new SkeletonConfigResponse())! as SkeletonConfigResponse;
    case 'OpenSerialRequest': return accessor(index, new OpenSerialRequest())! as OpenSerialRequest;
    case 'CloseSerialRequest': return accessor(index, new CloseSerialRequest())! as CloseSerialRequest;
    case 'SetWifiRequest': return accessor(index, new SetWifiRequest())! as SetWifiRequest;
    case 'SerialUpdateResponse': return accessor(index, new SerialUpdateResponse())! as SerialUpdateResponse;
    case 'AutoBoneProcessRequest': return accessor(index, new AutoBoneProcessRequest())! as AutoBoneProcessRequest;
    case 'AutoBoneProcessStatusResponse': return accessor(index, new AutoBoneProcessStatusResponse())! as AutoBoneProcessStatusResponse;
    case 'AutoBoneEpochResponse': return accessor(index, new AutoBoneEpochResponse())! as AutoBoneEpochResponse;
    case 'OverlayDisplayModeRequest': return accessor(index, new OverlayDisplayModeRequest())! as OverlayDisplayModeRequest;
    case 'OverlayDisplayModeChangeRequest': return accessor(index, new OverlayDisplayModeChangeRequest())! as OverlayDisplayModeChangeRequest;
    case 'OverlayDisplayModeResponse': return accessor(index, new OverlayDisplayModeResponse())! as OverlayDisplayModeResponse;
    case 'SerialTrackerRebootRequest': return accessor(index, new SerialTrackerRebootRequest())! as SerialTrackerRebootRequest;
    case 'SerialTrackerGetInfoRequest': return accessor(index, new SerialTrackerGetInfoRequest())! as SerialTrackerGetInfoRequest;
    case 'SerialTrackerFactoryResetRequest': return accessor(index, new SerialTrackerFactoryResetRequest())! as SerialTrackerFactoryResetRequest;
    case 'SerialDevicesRequest': return accessor(index, new SerialDevicesRequest())! as SerialDevicesRequest;
    case 'SerialDevicesResponse': return accessor(index, new SerialDevicesResponse())! as SerialDevicesResponse;
    case 'NewSerialDeviceResponse': return accessor(index, new NewSerialDeviceResponse())! as NewSerialDeviceResponse;
    case 'StartWifiProvisioningRequest': return accessor(index, new StartWifiProvisioningRequest())! as StartWifiProvisioningRequest;
    case 'StopWifiProvisioningRequest': return accessor(index, new StopWifiProvisioningRequest())! as StopWifiProvisioningRequest;
    case 'WifiProvisioningStatusResponse': return accessor(index, new WifiProvisioningStatusResponse())! as WifiProvisioningStatusResponse;
    case 'ServerInfosRequest': return accessor(index, new ServerInfosRequest())! as ServerInfosRequest;
    case 'ServerInfosResponse': return accessor(index, new ServerInfosResponse())! as ServerInfosResponse;
    case 'LegTweaksTmpChange': return accessor(index, new LegTweaksTmpChange())! as LegTweaksTmpChange;
    case 'LegTweaksTmpClear': return accessor(index, new LegTweaksTmpClear())! as LegTweaksTmpClear;
    default: return null;
  }
}
