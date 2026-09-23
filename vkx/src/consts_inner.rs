// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(clippy::all)]

use crate::loader::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

/// [`VK_MAX_PHYSICAL_DEVICE_NAME_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PHYSICAL_DEVICE_NAME_SIZE.html)
///
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;

/// [`VK_UUID_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_UUID_SIZE.html)
///
pub const VK_UUID_SIZE: u32 = 16;

/// [`VK_LUID_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LUID_SIZE.html)
///
pub const VK_LUID_SIZE: u32 = 8;

/// [`VK_MAX_EXTENSION_NAME_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_EXTENSION_NAME_SIZE.html)
///
pub const VK_MAX_EXTENSION_NAME_SIZE: u32 = 256;

/// [`VK_MAX_DESCRIPTION_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DESCRIPTION_SIZE.html)
///
pub const VK_MAX_DESCRIPTION_SIZE: u32 = 256;

/// [`VK_MAX_MEMORY_TYPES`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_MEMORY_TYPES.html)
///
pub const VK_MAX_MEMORY_TYPES: u32 = 32;

/// [`VK_MAX_MEMORY_HEAPS`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_MEMORY_HEAPS.html)
///
pub const VK_MAX_MEMORY_HEAPS: u32 = 16;

/// [`VK_LOD_CLAMP_NONE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LOD_CLAMP_NONE.html)
///
pub const VK_LOD_CLAMP_NONE: f32 = 1000.0;

/// [`VK_REMAINING_MIP_LEVELS`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_MIP_LEVELS.html)
///
pub const VK_REMAINING_MIP_LEVELS: u32 = 4294967295;

/// [`VK_REMAINING_ARRAY_LAYERS`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_ARRAY_LAYERS.html)
///
pub const VK_REMAINING_ARRAY_LAYERS: u32 = 4294967295;

/// [`VK_REMAINING_3D_SLICES_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_3D_SLICES_EXT.html)
///
pub const VK_REMAINING_3D_SLICES_EXT: u32 = 4294967295;

/// [`VK_WHOLE_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_WHOLE_SIZE.html)
///
pub const VK_WHOLE_SIZE: u64 = 18446744073709551615;

/// [`VK_ATTACHMENT_UNUSED`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ATTACHMENT_UNUSED.html)
///
pub const VK_ATTACHMENT_UNUSED: u32 = 4294967295;

/// [`VK_TRUE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_TRUE.html)
///
pub const VK_TRUE: u32 = 1;

/// [`VK_FALSE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FALSE.html)
///
pub const VK_FALSE: u32 = 0;

/// [`VK_QUEUE_FAMILY_IGNORED`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_IGNORED.html)
///
pub const VK_QUEUE_FAMILY_IGNORED: u32 = 4294967295;

/// [`VK_QUEUE_FAMILY_EXTERNAL`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_EXTERNAL.html)
///
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = 4294967294;

/// [`VK_QUEUE_FAMILY_FOREIGN_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_FOREIGN_EXT.html)
///
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: u32 = 4294967293;

/// [`VK_SUBPASS_EXTERNAL`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBPASS_EXTERNAL.html)
///
pub const VK_SUBPASS_EXTERNAL: u32 = 4294967295;

/// [`VK_MAX_DEVICE_GROUP_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DEVICE_GROUP_SIZE.html)
///
pub const VK_MAX_DEVICE_GROUP_SIZE: u32 = 32;

/// [`VK_MAX_DRIVER_NAME_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DRIVER_NAME_SIZE.html)
///
pub const VK_MAX_DRIVER_NAME_SIZE: u32 = 256;

/// [`VK_MAX_DRIVER_INFO_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DRIVER_INFO_SIZE.html)
///
pub const VK_MAX_DRIVER_INFO_SIZE: u32 = 256;

/// [`VK_SHADER_UNUSED_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_UNUSED_KHR.html)
///
pub const VK_SHADER_UNUSED_KHR: u32 = 4294967295;

/// [`VK_MAX_GLOBAL_PRIORITY_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_GLOBAL_PRIORITY_SIZE.html)
///
pub const VK_MAX_GLOBAL_PRIORITY_SIZE: u32 = 16;

/// [`VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT.html)
///
pub const VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;

/// [`VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR.html)
///
pub const VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;

/// [`VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR.html)
///
pub const VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: u32 = 7;

/// [`VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR.html)
///
pub const VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = 3;

/// [`VK_SHADER_INDEX_UNUSED_AMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_INDEX_UNUSED_AMDX.html)
///
pub const VK_SHADER_INDEX_UNUSED_AMDX: u32 = 4294967295;

/// [`VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV.html)
///
pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 = 4294967295;

/// [`VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX.html)
///
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 = 128;

/// [`VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX.html)
///
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 = 128;

/// [`VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM.html)
///
pub const VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: u32 = 128;

/// [`VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM.html)
///
pub const VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: u32 = 3;

/// [`VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV.html)
///
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = 0.25;

/// [`VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV.html)
///
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = 0.5;

/// [`VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV.html)
///
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = 0.75;

/// [`VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM.html)
///
pub const VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM: u32 = 128;

/// [`VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM.html)
///
pub const VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM: u32 = 4;

/// [`STD_VIDEO_H264_CPB_CNT_LIST_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_CPB_CNT_LIST_SIZE.html)
///
pub const STD_VIDEO_H264_CPB_CNT_LIST_SIZE: u32 = 32;

/// [`STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS.html)
///
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;

/// [`STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS.html)
///
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;

/// [`STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS.html)
///
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;

/// [`STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS.html)
///
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;

/// [`STD_VIDEO_H264_MAX_NUM_LIST_REF`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_MAX_NUM_LIST_REF.html)
///
pub const STD_VIDEO_H264_MAX_NUM_LIST_REF: u32 = 32;

/// [`STD_VIDEO_H264_MAX_CHROMA_PLANES`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_MAX_CHROMA_PLANES.html)
///
pub const STD_VIDEO_H264_MAX_CHROMA_PLANES: u32 = 2;

/// [`STD_VIDEO_H264_NO_REFERENCE_PICTURE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_NO_REFERENCE_PICTURE.html)
///
pub const STD_VIDEO_H264_NO_REFERENCE_PICTURE: u8 = 255;

/// [`STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE.html)
///
pub const STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE: u32 = 2;

/// [`STD_VIDEO_H265_CPB_CNT_LIST_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_CPB_CNT_LIST_SIZE.html)
///
pub const STD_VIDEO_H265_CPB_CNT_LIST_SIZE: u32 = 32;

/// [`STD_VIDEO_H265_SUBLAYERS_LIST_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SUBLAYERS_LIST_SIZE.html)
///
pub const STD_VIDEO_H265_SUBLAYERS_LIST_SIZE: u32 = 7;

/// [`STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS.html)
///
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;

/// [`STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS.html)
///
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;

/// [`STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS.html)
///
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;

/// [`STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS.html)
///
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;

/// [`STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS.html)
///
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS: u32 = 6;

/// [`STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS.html)
///
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS: u32 = 64;

/// [`STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS.html)
///
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS: u32 = 2;

/// [`STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS.html)
///
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS: u32 = 64;

/// [`STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE.html)
///
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE: u32 = 6;

/// [`STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE.html)
///
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE: u32 = 19;

/// [`STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE.html)
///
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE: u32 = 21;

/// [`STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE.html)
///
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE: u32 = 3;

/// [`STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE.html)
///
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE: u32 = 128;

/// [`STD_VIDEO_H265_MAX_NUM_LIST_REF`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_NUM_LIST_REF.html)
///
pub const STD_VIDEO_H265_MAX_NUM_LIST_REF: u32 = 15;

/// [`STD_VIDEO_H265_MAX_CHROMA_PLANES`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_CHROMA_PLANES.html)
///
pub const STD_VIDEO_H265_MAX_CHROMA_PLANES: u32 = 2;

/// [`STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS.html)
///
pub const STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS: u32 = 64;

/// [`STD_VIDEO_H265_MAX_DPB_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_DPB_SIZE.html)
///
pub const STD_VIDEO_H265_MAX_DPB_SIZE: u32 = 16;

/// [`STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS.html)
///
pub const STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS: u32 = 32;

/// [`STD_VIDEO_H265_MAX_LONG_TERM_PICS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_LONG_TERM_PICS.html)
///
pub const STD_VIDEO_H265_MAX_LONG_TERM_PICS: u32 = 16;

/// [`STD_VIDEO_H265_MAX_DELTA_POC`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_DELTA_POC.html)
///
pub const STD_VIDEO_H265_MAX_DELTA_POC: u32 = 48;

/// [`STD_VIDEO_H265_NO_REFERENCE_PICTURE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_NO_REFERENCE_PICTURE.html)
///
pub const STD_VIDEO_H265_NO_REFERENCE_PICTURE: u8 = 255;

/// [`STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE.html)
///
pub const STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE: u32 = 8;

/// [`STD_VIDEO_AV1_NUM_REF_FRAMES`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_NUM_REF_FRAMES.html)
///
pub const STD_VIDEO_AV1_NUM_REF_FRAMES: u32 = 8;

/// [`STD_VIDEO_AV1_REFS_PER_FRAME`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_REFS_PER_FRAME.html)
///
pub const STD_VIDEO_AV1_REFS_PER_FRAME: u32 = 7;

/// [`STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME.html)
///
pub const STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME: u32 = 8;

/// [`STD_VIDEO_AV1_MAX_TILE_COLS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_TILE_COLS.html)
///
pub const STD_VIDEO_AV1_MAX_TILE_COLS: u32 = 64;

/// [`STD_VIDEO_AV1_MAX_TILE_ROWS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_TILE_ROWS.html)
///
pub const STD_VIDEO_AV1_MAX_TILE_ROWS: u32 = 64;

/// [`STD_VIDEO_AV1_MAX_SEGMENTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_SEGMENTS.html)
///
pub const STD_VIDEO_AV1_MAX_SEGMENTS: u32 = 8;

/// [`STD_VIDEO_AV1_SEG_LVL_MAX`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_SEG_LVL_MAX.html)
///
pub const STD_VIDEO_AV1_SEG_LVL_MAX: u32 = 8;

/// [`STD_VIDEO_AV1_PRIMARY_REF_NONE`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_PRIMARY_REF_NONE.html)
///
pub const STD_VIDEO_AV1_PRIMARY_REF_NONE: u8 = 7;

/// [`STD_VIDEO_AV1_SELECT_INTEGER_MV`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_SELECT_INTEGER_MV.html)
///
pub const STD_VIDEO_AV1_SELECT_INTEGER_MV: u8 = 2;

/// [`STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS.html)
///
pub const STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS: u32 = 2;

/// [`STD_VIDEO_AV1_SKIP_MODE_FRAMES`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_SKIP_MODE_FRAMES.html)
///
pub const STD_VIDEO_AV1_SKIP_MODE_FRAMES: u32 = 2;

/// [`STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS.html)
///
pub const STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS: u32 = 4;

/// [`STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS.html)
///
pub const STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS: u32 = 2;

/// [`STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS.html)
///
pub const STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS: u32 = 8;

/// [`STD_VIDEO_AV1_MAX_NUM_PLANES`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_PLANES.html)
///
pub const STD_VIDEO_AV1_MAX_NUM_PLANES: u32 = 3;

/// [`STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS.html)
///
pub const STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS: u32 = 6;

/// [`STD_VIDEO_AV1_MAX_NUM_Y_POINTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_Y_POINTS.html)
///
pub const STD_VIDEO_AV1_MAX_NUM_Y_POINTS: u32 = 14;

/// [`STD_VIDEO_AV1_MAX_NUM_CB_POINTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_CB_POINTS.html)
///
pub const STD_VIDEO_AV1_MAX_NUM_CB_POINTS: u32 = 10;

/// [`STD_VIDEO_AV1_MAX_NUM_CR_POINTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_CR_POINTS.html)
///
pub const STD_VIDEO_AV1_MAX_NUM_CR_POINTS: u32 = 10;

/// [`STD_VIDEO_AV1_MAX_NUM_POS_LUMA`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_POS_LUMA.html)
///
pub const STD_VIDEO_AV1_MAX_NUM_POS_LUMA: u32 = 24;

/// [`STD_VIDEO_AV1_MAX_NUM_POS_CHROMA`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_POS_CHROMA.html)
///
pub const STD_VIDEO_AV1_MAX_NUM_POS_CHROMA: u32 = 25;

/// [`STD_VIDEO_VP9_NUM_REF_FRAMES`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_NUM_REF_FRAMES.html)
///
pub const STD_VIDEO_VP9_NUM_REF_FRAMES: u32 = 8;

/// [`STD_VIDEO_VP9_REFS_PER_FRAME`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_REFS_PER_FRAME.html)
///
pub const STD_VIDEO_VP9_REFS_PER_FRAME: u32 = 3;

/// [`STD_VIDEO_VP9_MAX_REF_FRAMES`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_MAX_REF_FRAMES.html)
///
pub const STD_VIDEO_VP9_MAX_REF_FRAMES: u32 = 4;

/// [`STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS.html)
///
pub const STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS: u32 = 2;

/// [`STD_VIDEO_VP9_MAX_SEGMENTS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_MAX_SEGMENTS.html)
///
pub const STD_VIDEO_VP9_MAX_SEGMENTS: u32 = 8;

/// [`STD_VIDEO_VP9_SEG_LVL_MAX`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_SEG_LVL_MAX.html)
///
pub const STD_VIDEO_VP9_SEG_LVL_MAX: u32 = 4;

/// [`STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS.html)
///
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS: u32 = 7;

/// [`STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB`](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB.html)
///
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB: u32 = 3;
