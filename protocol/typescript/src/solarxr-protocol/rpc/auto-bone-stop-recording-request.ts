// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



/**
 * Stops the current recording, using it as far as it has been recorded
 */
export class AutoBoneStopRecordingRequest implements flatbuffers.IUnpackableObject<AutoBoneStopRecordingRequestT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):AutoBoneStopRecordingRequest {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsAutoBoneStopRecordingRequest(bb:flatbuffers.ByteBuffer, obj?:AutoBoneStopRecordingRequest):AutoBoneStopRecordingRequest {
  return (obj || new AutoBoneStopRecordingRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsAutoBoneStopRecordingRequest(bb:flatbuffers.ByteBuffer, obj?:AutoBoneStopRecordingRequest):AutoBoneStopRecordingRequest {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new AutoBoneStopRecordingRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static startAutoBoneStopRecordingRequest(builder:flatbuffers.Builder) {
  builder.startObject(0);
}

static endAutoBoneStopRecordingRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createAutoBoneStopRecordingRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  AutoBoneStopRecordingRequest.startAutoBoneStopRecordingRequest(builder);
  return AutoBoneStopRecordingRequest.endAutoBoneStopRecordingRequest(builder);
}

unpack(): AutoBoneStopRecordingRequestT {
  return new AutoBoneStopRecordingRequestT();
}


unpackTo(_o: AutoBoneStopRecordingRequestT): void {}
}

export class AutoBoneStopRecordingRequestT implements flatbuffers.IGeneratedObject {
constructor(){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return AutoBoneStopRecordingRequest.createAutoBoneStopRecordingRequest(builder);
}
}
