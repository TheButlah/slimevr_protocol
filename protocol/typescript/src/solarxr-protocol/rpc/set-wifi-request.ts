// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class SetWifiRequest implements flatbuffers.IUnpackableObject<SetWifiRequestT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):SetWifiRequest {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsSetWifiRequest(bb:flatbuffers.ByteBuffer, obj?:SetWifiRequest):SetWifiRequest {
  return (obj || new SetWifiRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsSetWifiRequest(bb:flatbuffers.ByteBuffer, obj?:SetWifiRequest):SetWifiRequest {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new SetWifiRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

ssid():string|null
ssid(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
ssid(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

password():string|null
password(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
password(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

static startSetWifiRequest(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addSsid(builder:flatbuffers.Builder, ssidOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, ssidOffset, 0);
}

static addPassword(builder:flatbuffers.Builder, passwordOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, passwordOffset, 0);
}

static endSetWifiRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createSetWifiRequest(builder:flatbuffers.Builder, ssidOffset:flatbuffers.Offset, passwordOffset:flatbuffers.Offset):flatbuffers.Offset {
  SetWifiRequest.startSetWifiRequest(builder);
  SetWifiRequest.addSsid(builder, ssidOffset);
  SetWifiRequest.addPassword(builder, passwordOffset);
  return SetWifiRequest.endSetWifiRequest(builder);
}

unpack(): SetWifiRequestT {
  return new SetWifiRequestT(
    this.ssid(),
    this.password()
  );
}


unpackTo(_o: SetWifiRequestT): void {
  _o.ssid = this.ssid();
  _o.password = this.password();
}
}

export class SetWifiRequestT implements flatbuffers.IGeneratedObject {
constructor(
  public ssid: string|Uint8Array|null = null,
  public password: string|Uint8Array|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const ssid = (this.ssid !== null ? builder.createString(this.ssid!) : 0);
  const password = (this.password !== null ? builder.createString(this.password!) : 0);

  return SetWifiRequest.createSetWifiRequest(builder,
    ssid,
    password
  );
}
}
