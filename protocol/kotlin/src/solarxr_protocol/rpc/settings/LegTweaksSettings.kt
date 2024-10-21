// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc.settings

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

@Suppress("unused")
class LegTweaksSettings : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : LegTweaksSettings {
        __init(_i, _bb)
        return this
    }
    val correctionStrength : Float?
        get() {
            val o = __offset(4)
            return if(o != 0) bb.getFloat(o + bb_pos) else null
        }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsLegTweaksSettings(_bb: ByteBuffer): LegTweaksSettings = getRootAsLegTweaksSettings(_bb, LegTweaksSettings())
        @JvmStatic
        fun getRootAsLegTweaksSettings(_bb: ByteBuffer, obj: LegTweaksSettings): LegTweaksSettings {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun createLegTweaksSettings(builder: FlatBufferBuilder, correctionStrength: Float?) : Int {
            builder.startTable(1)
            correctionStrength?.run { addCorrectionStrength(builder, correctionStrength) }
            return endLegTweaksSettings(builder)
        }
        @JvmStatic
        fun startLegTweaksSettings(builder: FlatBufferBuilder) = builder.startTable(1)
        @JvmStatic
        fun addCorrectionStrength(builder: FlatBufferBuilder, correctionStrength: Float) = builder.addFloat(0, correctionStrength, 0.0)
        @JvmStatic
        fun endLegTweaksSettings(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
