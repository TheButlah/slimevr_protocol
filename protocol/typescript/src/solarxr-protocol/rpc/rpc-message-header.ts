// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { TransactionId, TransactionIdT } from '../../solarxr-protocol/datatypes/transaction-id';
import { AssignTrackerRequest, AssignTrackerRequestT } from '../../solarxr-protocol/rpc/assign-tracker-request';
import { AutoBoneEpochResponse, AutoBoneEpochResponseT } from '../../solarxr-protocol/rpc/auto-bone-epoch-response';
import { AutoBoneProcessRequest, AutoBoneProcessRequestT } from '../../solarxr-protocol/rpc/auto-bone-process-request';
import { AutoBoneProcessStatusResponse, AutoBoneProcessStatusResponseT } from '../../solarxr-protocol/rpc/auto-bone-process-status-response';
import { ChangeSettingsRequest, ChangeSettingsRequestT } from '../../solarxr-protocol/rpc/change-settings-request';
import { ChangeSkeletonConfigRequest, ChangeSkeletonConfigRequestT } from '../../solarxr-protocol/rpc/change-skeleton-config-request';
import { CloseSerialRequest, CloseSerialRequestT } from '../../solarxr-protocol/rpc/close-serial-request';
import { HeartbeatRequest, HeartbeatRequestT } from '../../solarxr-protocol/rpc/heartbeat-request';
import { HeartbeatResponse, HeartbeatResponseT } from '../../solarxr-protocol/rpc/heartbeat-response';
import { OpenSerialRequest, OpenSerialRequestT } from '../../solarxr-protocol/rpc/open-serial-request';
import { RecordBVHRequest, RecordBVHRequestT } from '../../solarxr-protocol/rpc/record-bvhrequest';
import { RecordBVHStatus, RecordBVHStatusT } from '../../solarxr-protocol/rpc/record-bvhstatus';
import { ResetRequest, ResetRequestT } from '../../solarxr-protocol/rpc/reset-request';
import { RpcMessage, unionToRpcMessage, unionListToRpcMessage } from '../../solarxr-protocol/rpc/rpc-message';
import { SerialRestartTrackerRequest, SerialRestartTrackerRequestT } from '../../solarxr-protocol/rpc/serial-restart-tracker-request';
import { SerialSetCtrlRequest, SerialSetCtrlRequestT } from '../../solarxr-protocol/rpc/serial-set-ctrl-request';
import { SerialUpdateResponse, SerialUpdateResponseT } from '../../solarxr-protocol/rpc/serial-update-response';
import { SetWifiRequest, SetWifiRequestT } from '../../solarxr-protocol/rpc/set-wifi-request';
import { SettingsRequest, SettingsRequestT } from '../../solarxr-protocol/rpc/settings-request';
import { SettingsResponse, SettingsResponseT } from '../../solarxr-protocol/rpc/settings-response';
import { SkeletonConfigRequest, SkeletonConfigRequestT } from '../../solarxr-protocol/rpc/skeleton-config-request';
import { SkeletonConfigResponse, SkeletonConfigResponseT } from '../../solarxr-protocol/rpc/skeleton-config-response';
import { SkeletonResetAllRequest, SkeletonResetAllRequestT } from '../../solarxr-protocol/rpc/skeleton-reset-all-request';


export class RpcMessageHeader {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
__init(i:number, bb:flatbuffers.ByteBuffer):RpcMessageHeader {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsRpcMessageHeader(bb:flatbuffers.ByteBuffer, obj?:RpcMessageHeader):RpcMessageHeader {
  return (obj || new RpcMessageHeader()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsRpcMessageHeader(bb:flatbuffers.ByteBuffer, obj?:RpcMessageHeader):RpcMessageHeader {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new RpcMessageHeader()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

/**
 * For a request, this identifies the request. For a response, this corresponds
 * to the request that it is responding to.
 */
txId(obj?:TransactionId):TransactionId|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new TransactionId()).__init(this.bb_pos + offset, this.bb!) : null;
}

messageType():RpcMessage {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : RpcMessage.NONE;
}

message<T extends flatbuffers.Table>(obj:any):any|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.__union(obj, this.bb_pos + offset) : null;
}

static startRpcMessageHeader(builder:flatbuffers.Builder) {
  builder.startObject(3);
}

static addTxId(builder:flatbuffers.Builder, txIdOffset:flatbuffers.Offset) {
  builder.addFieldStruct(0, txIdOffset, 0);
}

static addMessageType(builder:flatbuffers.Builder, messageType:RpcMessage) {
  builder.addFieldInt8(1, messageType, RpcMessage.NONE);
}

static addMessage(builder:flatbuffers.Builder, messageOffset:flatbuffers.Offset) {
  builder.addFieldOffset(2, messageOffset, 0);
}

static endRpcMessageHeader(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createRpcMessageHeader(builder:flatbuffers.Builder, txIdOffset:flatbuffers.Offset, messageType:RpcMessage, messageOffset:flatbuffers.Offset):flatbuffers.Offset {
  RpcMessageHeader.startRpcMessageHeader(builder);
  RpcMessageHeader.addTxId(builder, txIdOffset);
  RpcMessageHeader.addMessageType(builder, messageType);
  RpcMessageHeader.addMessage(builder, messageOffset);
  return RpcMessageHeader.endRpcMessageHeader(builder);
}

unpack(): RpcMessageHeaderT {
  return new RpcMessageHeaderT(
    (this.txId() !== null ? this.txId()!.unpack() : null),
    this.messageType(),
    (() => {
      let temp = unionToRpcMessage(this.messageType(), this.message.bind(this));
      if(temp === null) { return null; }
      return temp.unpack()
  })()
  );
}


unpackTo(_o: RpcMessageHeaderT): void {
  _o.txId = (this.txId() !== null ? this.txId()!.unpack() : null);
  _o.messageType = this.messageType();
  _o.message = (() => {
      let temp = unionToRpcMessage(this.messageType(), this.message.bind(this));
      if(temp === null) { return null; }
      return temp.unpack()
  })();
}
}

export class RpcMessageHeaderT {
constructor(
  public txId: TransactionIdT|null = null,
  public messageType: RpcMessage = RpcMessage.NONE,
  public message: AssignTrackerRequestT|AutoBoneEpochResponseT|AutoBoneProcessRequestT|AutoBoneProcessStatusResponseT|ChangeSettingsRequestT|ChangeSkeletonConfigRequestT|CloseSerialRequestT|HeartbeatRequestT|HeartbeatResponseT|OpenSerialRequestT|RecordBVHRequestT|RecordBVHStatusT|ResetRequestT|SerialRestartTrackerRequestT|SerialSetCtrlRequestT|SerialUpdateResponseT|SetWifiRequestT|SettingsRequestT|SettingsResponseT|SkeletonConfigRequestT|SkeletonConfigResponseT|SkeletonResetAllRequestT|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const message = builder.createObjectOffset(this.message);

  return RpcMessageHeader.createRpcMessageHeader(builder,
    (this.txId !== null ? this.txId!.pack(builder) : 0),
    this.messageType,
    message
  );
}
}
