// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

/**
 * Tracker requires full reset
 */
@SuppressWarnings("unused")
public final class StatusTrackerReset extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static StatusTrackerReset getRootAsStatusTrackerReset(ByteBuffer _bb) { return getRootAsStatusTrackerReset(_bb, new StatusTrackerReset()); }
  public static StatusTrackerReset getRootAsStatusTrackerReset(ByteBuffer _bb, StatusTrackerReset obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public StatusTrackerReset __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public solarxr_protocol.datatypes.TrackerId trackerId() { return trackerId(new solarxr_protocol.datatypes.TrackerId()); }
  public solarxr_protocol.datatypes.TrackerId trackerId(solarxr_protocol.datatypes.TrackerId obj) { int o = __offset(4); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }

  public static int createStatusTrackerReset(FlatBufferBuilder builder,
      int trackerIdOffset) {
    builder.startTable(1);
    StatusTrackerReset.addTrackerId(builder, trackerIdOffset);
    return StatusTrackerReset.endStatusTrackerReset(builder);
  }

  public static void startStatusTrackerReset(FlatBufferBuilder builder) { builder.startTable(1); }
  public static void addTrackerId(FlatBufferBuilder builder, int trackerIdOffset) { builder.addOffset(0, trackerIdOffset, 0); }
  public static int endStatusTrackerReset(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public StatusTrackerReset get(int j) { return get(new StatusTrackerReset(), j); }
    public StatusTrackerReset get(StatusTrackerReset obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public StatusTrackerResetT unpack() {
    StatusTrackerResetT _o = new StatusTrackerResetT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(StatusTrackerResetT _o) {
    if (trackerId() != null) _o.setTrackerId(trackerId().unpack());
    else _o.setTrackerId(null);
  }
  public static int pack(FlatBufferBuilder builder, StatusTrackerResetT _o) {
    if (_o == null) return 0;
    int _trackerId = _o.getTrackerId() == null ? 0 : solarxr_protocol.datatypes.TrackerId.pack(builder, _o.getTrackerId());
    return createStatusTrackerReset(
      builder,
      _trackerId);
  }
}

