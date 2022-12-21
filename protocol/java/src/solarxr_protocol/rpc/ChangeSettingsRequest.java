// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class ChangeSettingsRequest extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static ChangeSettingsRequest getRootAsChangeSettingsRequest(ByteBuffer _bb) { return getRootAsChangeSettingsRequest(_bb, new ChangeSettingsRequest()); }
  public static ChangeSettingsRequest getRootAsChangeSettingsRequest(ByteBuffer _bb, ChangeSettingsRequest obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public ChangeSettingsRequest __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public solarxr_protocol.rpc.SteamVRTrackersSetting steamVrTrackers() { return steamVrTrackers(new solarxr_protocol.rpc.SteamVRTrackersSetting()); }
  public solarxr_protocol.rpc.SteamVRTrackersSetting steamVrTrackers(solarxr_protocol.rpc.SteamVRTrackersSetting obj) { int o = __offset(4); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.FilteringSettings filtering() { return filtering(new solarxr_protocol.rpc.FilteringSettings()); }
  public solarxr_protocol.rpc.FilteringSettings filtering(solarxr_protocol.rpc.FilteringSettings obj) { int o = __offset(6); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.DriftCompensation driftCompensation() { return driftCompensation(new solarxr_protocol.rpc.DriftCompensation()); }
  public solarxr_protocol.rpc.DriftCompensation driftCompensation(solarxr_protocol.rpc.DriftCompensation obj) { int o = __offset(8); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.OSCRouterSettings oscRouter() { return oscRouter(new solarxr_protocol.rpc.OSCRouterSettings()); }
  public solarxr_protocol.rpc.OSCRouterSettings oscRouter(solarxr_protocol.rpc.OSCRouterSettings obj) { int o = __offset(10); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.VRCOSCSettings vrcOsc() { return vrcOsc(new solarxr_protocol.rpc.VRCOSCSettings()); }
  public solarxr_protocol.rpc.VRCOSCSettings vrcOsc(solarxr_protocol.rpc.VRCOSCSettings obj) { int o = __offset(12); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.settings.ModelSettings modelSettings() { return modelSettings(new solarxr_protocol.rpc.settings.ModelSettings()); }
  public solarxr_protocol.rpc.settings.ModelSettings modelSettings(solarxr_protocol.rpc.settings.ModelSettings obj) { int o = __offset(14); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public solarxr_protocol.rpc.TapDetectionSettings tapDetectionSettings() { return tapDetectionSettings(new solarxr_protocol.rpc.TapDetectionSettings()); }
  public solarxr_protocol.rpc.TapDetectionSettings tapDetectionSettings(solarxr_protocol.rpc.TapDetectionSettings obj) { int o = __offset(16); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }

  public static int createChangeSettingsRequest(FlatBufferBuilder builder,
      int steamVrTrackersOffset,
      int filteringOffset,
      int driftCompensationOffset,
      int oscRouterOffset,
      int vrcOscOffset,
      int modelSettingsOffset,
      int tapDetectionSettingsOffset) {
    builder.startTable(7);
    ChangeSettingsRequest.addTapDetectionSettings(builder, tapDetectionSettingsOffset);
    ChangeSettingsRequest.addModelSettings(builder, modelSettingsOffset);
    ChangeSettingsRequest.addVrcOsc(builder, vrcOscOffset);
    ChangeSettingsRequest.addOscRouter(builder, oscRouterOffset);
    ChangeSettingsRequest.addDriftCompensation(builder, driftCompensationOffset);
    ChangeSettingsRequest.addFiltering(builder, filteringOffset);
    ChangeSettingsRequest.addSteamVrTrackers(builder, steamVrTrackersOffset);
    return ChangeSettingsRequest.endChangeSettingsRequest(builder);
  }

  public static void startChangeSettingsRequest(FlatBufferBuilder builder) { builder.startTable(7); }
  public static void addSteamVrTrackers(FlatBufferBuilder builder, int steamVrTrackersOffset) { builder.addOffset(0, steamVrTrackersOffset, 0); }
  public static void addFiltering(FlatBufferBuilder builder, int filteringOffset) { builder.addOffset(1, filteringOffset, 0); }
  public static void addDriftCompensation(FlatBufferBuilder builder, int driftCompensationOffset) { builder.addOffset(2, driftCompensationOffset, 0); }
  public static void addOscRouter(FlatBufferBuilder builder, int oscRouterOffset) { builder.addOffset(3, oscRouterOffset, 0); }
  public static void addVrcOsc(FlatBufferBuilder builder, int vrcOscOffset) { builder.addOffset(4, vrcOscOffset, 0); }
  public static void addModelSettings(FlatBufferBuilder builder, int modelSettingsOffset) { builder.addOffset(5, modelSettingsOffset, 0); }
  public static void addTapDetectionSettings(FlatBufferBuilder builder, int tapDetectionSettingsOffset) { builder.addOffset(6, tapDetectionSettingsOffset, 0); }
  public static int endChangeSettingsRequest(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public ChangeSettingsRequest get(int j) { return get(new ChangeSettingsRequest(), j); }
    public ChangeSettingsRequest get(ChangeSettingsRequest obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public ChangeSettingsRequestT unpack() {
    ChangeSettingsRequestT _o = new ChangeSettingsRequestT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(ChangeSettingsRequestT _o) {
    if (steamVrTrackers() != null) _o.setSteamVrTrackers(steamVrTrackers().unpack());
    else _o.setSteamVrTrackers(null);
    if (filtering() != null) _o.setFiltering(filtering().unpack());
    else _o.setFiltering(null);
    if (driftCompensation() != null) _o.setDriftCompensation(driftCompensation().unpack());
    else _o.setDriftCompensation(null);
    if (oscRouter() != null) _o.setOscRouter(oscRouter().unpack());
    else _o.setOscRouter(null);
    if (vrcOsc() != null) _o.setVrcOsc(vrcOsc().unpack());
    else _o.setVrcOsc(null);
    if (modelSettings() != null) _o.setModelSettings(modelSettings().unpack());
    else _o.setModelSettings(null);
    if (tapDetectionSettings() != null) _o.setTapDetectionSettings(tapDetectionSettings().unpack());
    else _o.setTapDetectionSettings(null);
  }
  public static int pack(FlatBufferBuilder builder, ChangeSettingsRequestT _o) {
    if (_o == null) return 0;
    int _steamVrTrackers = _o.getSteamVrTrackers() == null ? 0 : solarxr_protocol.rpc.SteamVRTrackersSetting.pack(builder, _o.getSteamVrTrackers());
    int _filtering = _o.getFiltering() == null ? 0 : solarxr_protocol.rpc.FilteringSettings.pack(builder, _o.getFiltering());
    int _driftCompensation = _o.getDriftCompensation() == null ? 0 : solarxr_protocol.rpc.DriftCompensation.pack(builder, _o.getDriftCompensation());
    int _oscRouter = _o.getOscRouter() == null ? 0 : solarxr_protocol.rpc.OSCRouterSettings.pack(builder, _o.getOscRouter());
    int _vrcOsc = _o.getVrcOsc() == null ? 0 : solarxr_protocol.rpc.VRCOSCSettings.pack(builder, _o.getVrcOsc());
    int _modelSettings = _o.getModelSettings() == null ? 0 : solarxr_protocol.rpc.settings.ModelSettings.pack(builder, _o.getModelSettings());
    int _tapDetectionSettings = _o.getTapDetectionSettings() == null ? 0 : solarxr_protocol.rpc.TapDetectionSettings.pack(builder, _o.getTapDetectionSettings());
    return createChangeSettingsRequest(
      builder,
      _steamVrTrackers,
      _filtering,
      _driftCompensation,
      _oscRouter,
      _vrcOsc,
      _modelSettings,
      _tapDetectionSettings);
  }
}

