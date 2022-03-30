// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { OutboundUnion, unionToOutboundUnion, unionListToOutboundUnion } from '../../slimevr-protocol/firmware/outbound-union';


export class OutboundPacket {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
__init(i:number, bb:flatbuffers.ByteBuffer):OutboundPacket {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsOutboundPacket(bb:flatbuffers.ByteBuffer, obj?:OutboundPacket):OutboundPacket {
  return (obj || new OutboundPacket()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsOutboundPacket(bb:flatbuffers.ByteBuffer, obj?:OutboundPacket):OutboundPacket {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new OutboundPacket()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

packetCounter():bigint {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint64(this.bb_pos + offset) : BigInt('0');
}

acknowledgeMe():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

packetType():OutboundUnion {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : OutboundUnion.NONE;
}

packet<T extends flatbuffers.Table>(obj:any):any|null {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? this.bb!.__union(obj, this.bb_pos + offset) : null;
}

static startOutboundPacket(builder:flatbuffers.Builder) {
  builder.startObject(4);
}

static addPacketCounter(builder:flatbuffers.Builder, packetCounter:bigint) {
  builder.addFieldInt64(0, packetCounter, BigInt('0'));
}

static addAcknowledgeMe(builder:flatbuffers.Builder, acknowledgeMe:boolean) {
  builder.addFieldInt8(1, +acknowledgeMe, +false);
}

static addPacketType(builder:flatbuffers.Builder, packetType:OutboundUnion) {
  builder.addFieldInt8(2, packetType, OutboundUnion.NONE);
}

static addPacket(builder:flatbuffers.Builder, packetOffset:flatbuffers.Offset) {
  builder.addFieldOffset(3, packetOffset, 0);
}

static endOutboundPacket(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createOutboundPacket(builder:flatbuffers.Builder, packetCounter:bigint, acknowledgeMe:boolean, packetType:OutboundUnion, packetOffset:flatbuffers.Offset):flatbuffers.Offset {
  OutboundPacket.startOutboundPacket(builder);
  OutboundPacket.addPacketCounter(builder, packetCounter);
  OutboundPacket.addAcknowledgeMe(builder, acknowledgeMe);
  OutboundPacket.addPacketType(builder, packetType);
  OutboundPacket.addPacket(builder, packetOffset);
  return OutboundPacket.endOutboundPacket(builder);
}
}
