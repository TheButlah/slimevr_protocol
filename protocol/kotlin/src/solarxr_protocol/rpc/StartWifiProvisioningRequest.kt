// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

@Suppress("unused")
class StartWifiProvisioningRequest : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : StartWifiProvisioningRequest {
        __init(_i, _bb)
        return this
    }
    val ssid : String?
        get() {
            val o = __offset(4)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val ssidAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(4, 1)
    fun ssidInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 4, 1)
    val password : String?
        get() {
            val o = __offset(6)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val passwordAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(6, 1)
    fun passwordInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 6, 1)
    val port : String?
        get() {
            val o = __offset(8)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val portAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(8, 1)
    fun portInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 8, 1)
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsStartWifiProvisioningRequest(_bb: ByteBuffer): StartWifiProvisioningRequest = getRootAsStartWifiProvisioningRequest(_bb, StartWifiProvisioningRequest())
        @JvmStatic
        fun getRootAsStartWifiProvisioningRequest(_bb: ByteBuffer, obj: StartWifiProvisioningRequest): StartWifiProvisioningRequest {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun createStartWifiProvisioningRequest(builder: FlatBufferBuilder, ssidOffset: Int, passwordOffset: Int, portOffset: Int) : Int {
            builder.startTable(3)
            addPort(builder, portOffset)
            addPassword(builder, passwordOffset)
            addSsid(builder, ssidOffset)
            return endStartWifiProvisioningRequest(builder)
        }
        @JvmStatic
        fun startStartWifiProvisioningRequest(builder: FlatBufferBuilder) = builder.startTable(3)
        @JvmStatic
        fun addSsid(builder: FlatBufferBuilder, ssid: Int) = builder.addOffset(0, ssid, 0)
        @JvmStatic
        fun addPassword(builder: FlatBufferBuilder, password: Int) = builder.addOffset(1, password, 0)
        @JvmStatic
        fun addPort(builder: FlatBufferBuilder, port: Int) = builder.addOffset(2, port, 0)
        @JvmStatic
        fun endStartWifiProvisioningRequest(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
