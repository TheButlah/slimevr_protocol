// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class SerialFirmwareUpdateT {
  private solarxr_protocol.rpc.SerialDevicePortT deviceId;
  private boolean needManualReboot;
  private String ssid;
  private String password;
  private solarxr_protocol.rpc.FirmwarePartT[] firmwarePart;

  public solarxr_protocol.rpc.SerialDevicePortT getDeviceId() { return deviceId; }

  public void setDeviceId(solarxr_protocol.rpc.SerialDevicePortT deviceId) { this.deviceId = deviceId; }

  public boolean getNeedManualReboot() { return needManualReboot; }

  public void setNeedManualReboot(boolean needManualReboot) { this.needManualReboot = needManualReboot; }

  public String getSsid() { return ssid; }

  public void setSsid(String ssid) { this.ssid = ssid; }

  public String getPassword() { return password; }

  public void setPassword(String password) { this.password = password; }

  public solarxr_protocol.rpc.FirmwarePartT[] getFirmwarePart() { return firmwarePart; }

  public void setFirmwarePart(solarxr_protocol.rpc.FirmwarePartT[] firmwarePart) { this.firmwarePart = firmwarePart; }


  public SerialFirmwareUpdateT() {
    this.deviceId = null;
    this.needManualReboot = false;
    this.ssid = null;
    this.password = null;
    this.firmwarePart = null;
  }
}

