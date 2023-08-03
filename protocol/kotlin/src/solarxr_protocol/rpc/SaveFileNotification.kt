// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

/**
 * Used for the server to save a file and have it prompt in the user side
 */
@Suppress("unused")
class SaveFileNotification : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : SaveFileNotification {
        __init(_i, _bb)
        return this
    }
    /**
     * MIME type of file if one exists, use `file_extension` otherwise
     */
    val mimeType : String?
        get() {
            val o = __offset(4)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val mimeTypeAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(4, 1)
    fun mimeTypeInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 4, 1)
    /**
     * Use MIME type preferably if one exists
     */
    val fileExtension : String?
        get() {
            val o = __offset(6)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val fileExtensionAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(6, 1)
    fun fileExtensionInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 6, 1)
    /**
     * Directory recommended to save the file on
     */
    val expectedDir : UByte?
        get() {
            val o = __offset(8)
            return if(o != 0) bb.get(o + bb_pos).toUByte() else null
        }
    /**
     * Recommended filename
     */
    val expectedFilename : String?
        get() {
            val o = __offset(10)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val expectedFilenameAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(10, 1)
    fun expectedFilenameInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 10, 1)
    /**
     * ID of the SaveFile, needs to be returned by the SaveFileRequest
     */
    val id : UInt
        get() {
            val o = __offset(12)
            return if(o != 0) bb.getInt(o + bb_pos).toUInt() else 0u
        }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsSaveFileNotification(_bb: ByteBuffer): SaveFileNotification = getRootAsSaveFileNotification(_bb, SaveFileNotification())
        @JvmStatic
        fun getRootAsSaveFileNotification(_bb: ByteBuffer, obj: SaveFileNotification): SaveFileNotification {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun createSaveFileNotification(builder: FlatBufferBuilder, mimeTypeOffset: Int, fileExtensionOffset: Int, expectedDir: UByte?, expectedFilenameOffset: Int, id: UInt) : Int {
            builder.startTable(5)
            addId(builder, id)
            addExpectedFilename(builder, expectedFilenameOffset)
            addFileExtension(builder, fileExtensionOffset)
            addMimeType(builder, mimeTypeOffset)
            expectedDir?.run { addExpectedDir(builder, expectedDir) }
            return endSaveFileNotification(builder)
        }
        @JvmStatic
        fun startSaveFileNotification(builder: FlatBufferBuilder) = builder.startTable(5)
        @JvmStatic
        fun addMimeType(builder: FlatBufferBuilder, mimeType: Int) = builder.addOffset(0, mimeType, 0)
        @JvmStatic
        fun addFileExtension(builder: FlatBufferBuilder, fileExtension: Int) = builder.addOffset(1, fileExtension, 0)
        @JvmStatic
        fun addExpectedDir(builder: FlatBufferBuilder, expectedDir: UByte) = builder.addByte(2, expectedDir.toByte(), 0)
        @JvmStatic
        fun addExpectedFilename(builder: FlatBufferBuilder, expectedFilename: Int) = builder.addOffset(3, expectedFilename, 0)
        @JvmStatic
        fun addId(builder: FlatBufferBuilder, id: UInt) = builder.addInt(4, id.toInt(), 0)
        @JvmStatic
        fun endSaveFileNotification(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
