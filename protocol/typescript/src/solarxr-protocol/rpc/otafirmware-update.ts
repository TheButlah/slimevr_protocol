// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { DeviceId, DeviceIdT } from '../../solarxr-protocol/datatypes/device-id.js';
import { FirmwarePart, FirmwarePartT } from '../../solarxr-protocol/rpc/firmware-part.js';


export class OTAFirmwareUpdate implements flatbuffers.IUnpackableObject<OTAFirmwareUpdateT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):OTAFirmwareUpdate {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsOTAFirmwareUpdate(bb:flatbuffers.ByteBuffer, obj?:OTAFirmwareUpdate):OTAFirmwareUpdate {
  return (obj || new OTAFirmwareUpdate()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsOTAFirmwareUpdate(bb:flatbuffers.ByteBuffer, obj?:OTAFirmwareUpdate):OTAFirmwareUpdate {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new OTAFirmwareUpdate()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

/**
 * id of the device, this refer to the actual DeviceId from the protocol
 */
deviceId(obj?:DeviceId):DeviceId|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new DeviceId()).__init(this.bb_pos + offset, this.bb!) : null;
}

/**
 * A table containing the url and offset of the firmware bin file
 */
firmwarePart(obj?:FirmwarePart):FirmwarePart|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? (obj || new FirmwarePart()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

static startOTAFirmwareUpdate(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addDeviceId(builder:flatbuffers.Builder, deviceIdOffset:flatbuffers.Offset) {
  builder.addFieldStruct(0, deviceIdOffset, 0);
}

static addFirmwarePart(builder:flatbuffers.Builder, firmwarePartOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, firmwarePartOffset, 0);
}

static endOTAFirmwareUpdate(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}


unpack(): OTAFirmwareUpdateT {
  return new OTAFirmwareUpdateT(
    (this.deviceId() !== null ? this.deviceId()!.unpack() : null),
    (this.firmwarePart() !== null ? this.firmwarePart()!.unpack() : null)
  );
}


unpackTo(_o: OTAFirmwareUpdateT): void {
  _o.deviceId = (this.deviceId() !== null ? this.deviceId()!.unpack() : null);
  _o.firmwarePart = (this.firmwarePart() !== null ? this.firmwarePart()!.unpack() : null);
}
}

export class OTAFirmwareUpdateT implements flatbuffers.IGeneratedObject {
constructor(
  public deviceId: DeviceIdT|null = null,
  public firmwarePart: FirmwarePartT|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const firmwarePart = (this.firmwarePart !== null ? this.firmwarePart!.pack(builder) : 0);

  OTAFirmwareUpdate.startOTAFirmwareUpdate(builder);
  OTAFirmwareUpdate.addDeviceId(builder, (this.deviceId !== null ? this.deviceId!.pack(builder) : 0));
  OTAFirmwareUpdate.addFirmwarePart(builder, firmwarePart);

  return OTAFirmwareUpdate.endOTAFirmwareUpdate(builder);
}
}
