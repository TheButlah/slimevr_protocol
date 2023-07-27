// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class SettingsResponseT {
  private solarxr_protocol.rpc.SteamVRTrackersSettingT steamVrTrackers;
  private solarxr_protocol.rpc.FilteringSettingsT filtering;
  private solarxr_protocol.rpc.DriftCompensationSettingsT driftCompensation;
  private solarxr_protocol.rpc.OSCRouterSettingsT oscRouter;
  private solarxr_protocol.rpc.VRCOSCSettingsT vrcOsc;
  private solarxr_protocol.rpc.VMCOSCSettingsT vmcOsc;
  private solarxr_protocol.rpc.settings.ModelSettingsT modelSettings;
  private solarxr_protocol.rpc.TapDetectionSettingsT tapDetectionSettings;
  private solarxr_protocol.rpc.AutoBoneSettingsT autoBoneSettings;
  private int armsResetMode;

  public solarxr_protocol.rpc.SteamVRTrackersSettingT getSteamVrTrackers() { return steamVrTrackers; }

  public void setSteamVrTrackers(solarxr_protocol.rpc.SteamVRTrackersSettingT steamVrTrackers) { this.steamVrTrackers = steamVrTrackers; }

  public solarxr_protocol.rpc.FilteringSettingsT getFiltering() { return filtering; }

  public void setFiltering(solarxr_protocol.rpc.FilteringSettingsT filtering) { this.filtering = filtering; }

  public solarxr_protocol.rpc.DriftCompensationSettingsT getDriftCompensation() { return driftCompensation; }

  public void setDriftCompensation(solarxr_protocol.rpc.DriftCompensationSettingsT driftCompensation) { this.driftCompensation = driftCompensation; }

  public solarxr_protocol.rpc.OSCRouterSettingsT getOscRouter() { return oscRouter; }

  public void setOscRouter(solarxr_protocol.rpc.OSCRouterSettingsT oscRouter) { this.oscRouter = oscRouter; }

  public solarxr_protocol.rpc.VRCOSCSettingsT getVrcOsc() { return vrcOsc; }

  public void setVrcOsc(solarxr_protocol.rpc.VRCOSCSettingsT vrcOsc) { this.vrcOsc = vrcOsc; }

  public solarxr_protocol.rpc.VMCOSCSettingsT getVmcOsc() { return vmcOsc; }

  public void setVmcOsc(solarxr_protocol.rpc.VMCOSCSettingsT vmcOsc) { this.vmcOsc = vmcOsc; }

  public solarxr_protocol.rpc.settings.ModelSettingsT getModelSettings() { return modelSettings; }

  public void setModelSettings(solarxr_protocol.rpc.settings.ModelSettingsT modelSettings) { this.modelSettings = modelSettings; }

  public solarxr_protocol.rpc.TapDetectionSettingsT getTapDetectionSettings() { return tapDetectionSettings; }

  public void setTapDetectionSettings(solarxr_protocol.rpc.TapDetectionSettingsT tapDetectionSettings) { this.tapDetectionSettings = tapDetectionSettings; }

  public solarxr_protocol.rpc.AutoBoneSettingsT getAutoBoneSettings() { return autoBoneSettings; }

  public void setAutoBoneSettings(solarxr_protocol.rpc.AutoBoneSettingsT autoBoneSettings) { this.autoBoneSettings = autoBoneSettings; }

  public int getArmsResetMode() { return armsResetMode; }

  public void setArmsResetMode(int armsResetMode) { this.armsResetMode = armsResetMode; }


  public SettingsResponseT() {
    this.steamVrTrackers = null;
    this.filtering = null;
    this.driftCompensation = null;
    this.oscRouter = null;
    this.vrcOsc = null;
    this.vmcOsc = null;
    this.modelSettings = null;
    this.tapDetectionSettings = null;
    this.autoBoneSettings = null;
    this.armsResetMode = 0;
  }
}

