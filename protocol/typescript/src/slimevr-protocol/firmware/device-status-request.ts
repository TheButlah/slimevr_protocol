// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

export class DeviceStatusRequest {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
__init(i:number, bb:flatbuffers.ByteBuffer):DeviceStatusRequest {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsDeviceStatusRequest(bb:flatbuffers.ByteBuffer, obj?:DeviceStatusRequest):DeviceStatusRequest {
  return (obj || new DeviceStatusRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsDeviceStatusRequest(bb:flatbuffers.ByteBuffer, obj?:DeviceStatusRequest):DeviceStatusRequest {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new DeviceStatusRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static startDeviceStatusRequest(builder:flatbuffers.Builder) {
  builder.startObject(0);
}

static endDeviceStatusRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createDeviceStatusRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  DeviceStatusRequest.startDeviceStatusRequest(builder);
  return DeviceStatusRequest.endDeviceStatusRequest(builder);
}
}
