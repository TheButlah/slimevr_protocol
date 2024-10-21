// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

@Suppress("unused")
class ResetResponse : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : ResetResponse {
        __init(_i, _bb)
        return this
    }
    val resetType : UByte
        get() {
            val o = __offset(4)
            return if(o != 0) bb.get(o + bb_pos).toUByte() else 0u
        }
    val status : UByte
        get() {
            val o = __offset(6)
            return if(o != 0) bb.get(o + bb_pos).toUByte() else 0u
        }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsResetResponse(_bb: ByteBuffer): ResetResponse = getRootAsResetResponse(_bb, ResetResponse())
        @JvmStatic
        fun getRootAsResetResponse(_bb: ByteBuffer, obj: ResetResponse): ResetResponse {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun createResetResponse(builder: FlatBufferBuilder, resetType: UByte, status: UByte) : Int {
            builder.startTable(2)
            addStatus(builder, status)
            addResetType(builder, resetType)
            return endResetResponse(builder)
        }
        @JvmStatic
        fun startResetResponse(builder: FlatBufferBuilder) = builder.startTable(2)
        @JvmStatic
        fun addResetType(builder: FlatBufferBuilder, resetType: UByte) = builder.addByte(0, resetType.toByte(), 0)
        @JvmStatic
        fun addStatus(builder: FlatBufferBuilder, status: UByte) = builder.addByte(1, status.toByte(), 0)
        @JvmStatic
        fun endResetResponse(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
