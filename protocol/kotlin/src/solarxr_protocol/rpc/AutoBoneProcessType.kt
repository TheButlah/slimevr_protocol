// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

@Suppress("unused")
class AutoBoneProcessType private constructor() {
    companion object {
        const val NONE: UByte = 0u
        const val RECORD: UByte = 1u
        const val SAVE: UByte = 2u
        const val PROCESS: UByte = 3u
        /**
         * @deprecated
         * Use AutoBoneApplyRequest instead
         */
        const val APPLY: UByte = 4u
        val names : Array<String> = arrayOf("NONE", "RECORD", "SAVE", "PROCESS", "APPLY")
        @JvmStatic
        fun name(e: Int) : String = names[e]
    }
}
