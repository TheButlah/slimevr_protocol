// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.datatypes

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

@Suppress("unused")
class Bytes : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : Bytes {
        __init(_i, _bb)
        return this
    }
    fun b(j: Int) : UByte {
        val o = __offset(4)
        return if (o != 0) {
            bb.get(__vector(o) + j * 1).toUByte()
        } else {
            0u
        }
    }
    val bLength : Int
        get() {
            val o = __offset(4); return if (o != 0) __vector_len(o) else 0
        }
    val bAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(4, 1)
    fun bInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 4, 1)
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsBytes(_bb: ByteBuffer): Bytes = getRootAsBytes(_bb, Bytes())
        @JvmStatic
        fun getRootAsBytes(_bb: ByteBuffer, obj: Bytes): Bytes {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun createBytes(builder: FlatBufferBuilder, bOffset: Int) : Int {
            builder.startTable(1)
            addB(builder, bOffset)
            return endBytes(builder)
        }
        @JvmStatic
        fun startBytes(builder: FlatBufferBuilder) = builder.startTable(1)
        @JvmStatic
        fun addB(builder: FlatBufferBuilder, b: Int) = builder.addOffset(0, b, 0)
        @JvmStatic
        fun createBVector(builder: FlatBufferBuilder, data: UByteArray) : Int {
            builder.startVector(1, data.size, 1)
            for (i in data.size - 1 downTo 0) {
                builder.addByte(data[i].toByte())
            }
            return builder.endVector()
        }
        @JvmStatic
        fun startBVector(builder: FlatBufferBuilder, numElems: Int) = builder.startVector(1, numElems, 1)
        @JvmStatic
        fun endBytes(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
