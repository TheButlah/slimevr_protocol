// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.datatypes.hardware_info;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class HardwareStatusT {
  private Integer errorStatus;
  private Integer ping;
  private Short rssi;
  private Float mcuTemp;
  private Float batteryVoltage;
  private Integer batteryPctEstimate;
  private solarxr_protocol.datatypes.LogDataT logData;

  public Integer getErrorStatus() { return errorStatus; }

  public void setErrorStatus(Integer errorStatus) { this.errorStatus = errorStatus; }

  public Integer getPing() { return ping; }

  public void setPing(Integer ping) { this.ping = ping; }

  public Short getRssi() { return rssi; }

  public void setRssi(Short rssi) { this.rssi = rssi; }

  public Float getMcuTemp() { return mcuTemp; }

  public void setMcuTemp(Float mcuTemp) { this.mcuTemp = mcuTemp; }

  public Float getBatteryVoltage() { return batteryVoltage; }

  public void setBatteryVoltage(Float batteryVoltage) { this.batteryVoltage = batteryVoltage; }

  public Integer getBatteryPctEstimate() { return batteryPctEstimate; }

  public void setBatteryPctEstimate(Integer batteryPctEstimate) { this.batteryPctEstimate = batteryPctEstimate; }

  public solarxr_protocol.datatypes.LogDataT getLogData() { return logData; }

  public void setLogData(solarxr_protocol.datatypes.LogDataT logData) { this.logData = logData; }


  public HardwareStatusT() {
    this.errorStatus = null;
    this.ping = null;
    this.rssi = null;
    this.mcuTemp = null;
    this.batteryVoltage = null;
    this.batteryPctEstimate = null;
    this.logData = null;
  }
}

