// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

/**
 * Clears the legtweaks temprorary state back to what the config has.
 * Setting a field to `true` will reset that field.
 */
@Suppress("unused")
class LegTweaksTmpClear : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : LegTweaksTmpClear {
        __init(_i, _bb)
        return this
    }
    val floorClip : Boolean
        get() {
            val o = __offset(4)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    val skatingCorrection : Boolean
        get() {
            val o = __offset(6)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    val toeSnap : Boolean
        get() {
            val o = __offset(8)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    val footPlant : Boolean
        get() {
            val o = __offset(10)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsLegTweaksTmpClear(_bb: ByteBuffer): LegTweaksTmpClear = getRootAsLegTweaksTmpClear(_bb, LegTweaksTmpClear())
        @JvmStatic
        fun getRootAsLegTweaksTmpClear(_bb: ByteBuffer, obj: LegTweaksTmpClear): LegTweaksTmpClear {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun createLegTweaksTmpClear(builder: FlatBufferBuilder, floorClip: Boolean, skatingCorrection: Boolean, toeSnap: Boolean, footPlant: Boolean) : Int {
            builder.startTable(4)
            addFootPlant(builder, footPlant)
            addToeSnap(builder, toeSnap)
            addSkatingCorrection(builder, skatingCorrection)
            addFloorClip(builder, floorClip)
            return endLegTweaksTmpClear(builder)
        }
        @JvmStatic
        fun startLegTweaksTmpClear(builder: FlatBufferBuilder) = builder.startTable(4)
        @JvmStatic
        fun addFloorClip(builder: FlatBufferBuilder, floorClip: Boolean) = builder.addBoolean(0, floorClip, false)
        @JvmStatic
        fun addSkatingCorrection(builder: FlatBufferBuilder, skatingCorrection: Boolean) = builder.addBoolean(1, skatingCorrection, false)
        @JvmStatic
        fun addToeSnap(builder: FlatBufferBuilder, toeSnap: Boolean) = builder.addBoolean(2, toeSnap, false)
        @JvmStatic
        fun addFootPlant(builder: FlatBufferBuilder, footPlant: Boolean) = builder.addBoolean(3, footPlant, false)
        @JvmStatic
        fun endLegTweaksTmpClear(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
