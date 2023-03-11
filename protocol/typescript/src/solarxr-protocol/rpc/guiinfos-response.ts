// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



export class GUIInfosResponse implements flatbuffers.IUnpackableObject<GUIInfosResponseT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):GUIInfosResponse {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsGUIInfosResponse(bb:flatbuffers.ByteBuffer, obj?:GUIInfosResponse):GUIInfosResponse {
  return (obj || new GUIInfosResponse()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsGUIInfosResponse(bb:flatbuffers.ByteBuffer, obj?:GUIInfosResponse):GUIInfosResponse {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new GUIInfosResponse()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

inProportions():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

static startGUIInfosResponse(builder:flatbuffers.Builder) {
  builder.startObject(1);
}

static addInProportions(builder:flatbuffers.Builder, inProportions:boolean) {
  builder.addFieldInt8(0, +inProportions, +false);
}

static endGUIInfosResponse(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createGUIInfosResponse(builder:flatbuffers.Builder, inProportions:boolean):flatbuffers.Offset {
  GUIInfosResponse.startGUIInfosResponse(builder);
  GUIInfosResponse.addInProportions(builder, inProportions);
  return GUIInfosResponse.endGUIInfosResponse(builder);
}

unpack(): GUIInfosResponseT {
  return new GUIInfosResponseT(
    this.inProportions()
  );
}


unpackTo(_o: GUIInfosResponseT): void {
  _o.inProportions = this.inProportions();
}
}

export class GUIInfosResponseT implements flatbuffers.IGeneratedObject {
constructor(
  public inProportions: boolean = false
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return GUIInfosResponse.createGUIInfosResponse(builder,
    this.inProportions
  );
}
}
