// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class VMCOSCSettingsT {
  private solarxr_protocol.rpc.OSCSettingsT oscSettings;
  private boolean anchorHip;
  private String vrmAddress;

  public solarxr_protocol.rpc.OSCSettingsT getOscSettings() { return oscSettings; }

  public void setOscSettings(solarxr_protocol.rpc.OSCSettingsT oscSettings) { this.oscSettings = oscSettings; }

  public boolean getAnchorHip() { return anchorHip; }

  public void setAnchorHip(boolean anchorHip) { this.anchorHip = anchorHip; }

  public String getVrmAddress() { return vrmAddress; }

  public void setVrmAddress(String vrmAddress) { this.vrmAddress = vrmAddress; }


  public VMCOSCSettingsT() {
    this.oscSettings = null;
    this.anchorHip = false;
    this.vrmAddress = null;
  }
}

