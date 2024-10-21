// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { DeviceId, DeviceIdT } from '../../solarxr-protocol/datatypes/device-id.js';


export class TrackerId implements flatbuffers.IUnpackableObject<TrackerIdT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):TrackerId {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsTrackerId(bb:flatbuffers.ByteBuffer, obj?:TrackerId):TrackerId {
  return (obj || new TrackerId()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsTrackerId(bb:flatbuffers.ByteBuffer, obj?:TrackerId):TrackerId {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new TrackerId()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

/**
 * The device the tracker is associated with. If there is no hardware device it is
 * associated with, this should be `null`.
 */
deviceId(obj?:DeviceId):DeviceId|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new DeviceId()).__init(this.bb_pos + offset, this.bb!) : null;
}

/**
 * There are possibly multiple trackers per device. This identifies which one.
 */
trackerNum():number {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : 0;
}

static startTrackerId(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addDeviceId(builder:flatbuffers.Builder, deviceIdOffset:flatbuffers.Offset) {
  builder.addFieldStruct(0, deviceIdOffset, 0);
}

static addTrackerNum(builder:flatbuffers.Builder, trackerNum:number) {
  builder.addFieldInt8(1, trackerNum, 0);
}

static endTrackerId(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createTrackerId(builder:flatbuffers.Builder, deviceIdOffset:flatbuffers.Offset, trackerNum:number):flatbuffers.Offset {
  TrackerId.startTrackerId(builder);
  TrackerId.addDeviceId(builder, deviceIdOffset);
  TrackerId.addTrackerNum(builder, trackerNum);
  return TrackerId.endTrackerId(builder);
}

unpack(): TrackerIdT {
  return new TrackerIdT(
    (this.deviceId() !== null ? this.deviceId()!.unpack() : null),
    this.trackerNum()
  );
}


unpackTo(_o: TrackerIdT): void {
  _o.deviceId = (this.deviceId() !== null ? this.deviceId()!.unpack() : null);
  _o.trackerNum = this.trackerNum();
}
}

export class TrackerIdT implements flatbuffers.IGeneratedObject {
constructor(
  public deviceId: DeviceIdT|null = null,
  public trackerNum: number = 0
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return TrackerId.createTrackerId(builder,
    (this.deviceId !== null ? this.deviceId!.pack(builder) : 0),
    this.trackerNum
  );
}
}
