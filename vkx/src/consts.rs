// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::loader::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::consts_inner::*;

/// [`VK_MAX_PHYSICAL_DEVICE_NAME_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PHYSICAL_DEVICE_NAME_SIZE.html)
///
#[doc(alias = "VK_MAX_PHYSICAL_DEVICE_NAME_SIZE")]
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = VK_MAX_PHYSICAL_DEVICE_NAME_SIZE;

/// [`VK_UUID_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_UUID_SIZE.html)
///
#[doc(alias = "VK_UUID_SIZE")]
pub const UUID_SIZE: u32 = VK_UUID_SIZE;

/// [`VK_LUID_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LUID_SIZE.html)
///
#[doc(alias = "VK_LUID_SIZE")]
pub const LUID_SIZE: u32 = VK_LUID_SIZE;

/// [`VK_MAX_EXTENSION_NAME_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_EXTENSION_NAME_SIZE.html)
///
#[doc(alias = "VK_MAX_EXTENSION_NAME_SIZE")]
pub const MAX_EXTENSION_NAME_SIZE: u32 = VK_MAX_EXTENSION_NAME_SIZE;

/// [`VK_MAX_DESCRIPTION_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DESCRIPTION_SIZE.html)
///
#[doc(alias = "VK_MAX_DESCRIPTION_SIZE")]
pub const MAX_DESCRIPTION_SIZE: u32 = VK_MAX_DESCRIPTION_SIZE;

/// [`VK_MAX_MEMORY_TYPES`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_MEMORY_TYPES.html)
///
#[doc(alias = "VK_MAX_MEMORY_TYPES")]
pub const MAX_MEMORY_TYPES: u32 = VK_MAX_MEMORY_TYPES;

/// [`VK_MAX_MEMORY_HEAPS`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_MEMORY_HEAPS.html)
///
#[doc(alias = "VK_MAX_MEMORY_HEAPS")]
pub const MAX_MEMORY_HEAPS: u32 = VK_MAX_MEMORY_HEAPS;

/// [`VK_LOD_CLAMP_NONE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LOD_CLAMP_NONE.html)
///
#[doc(alias = "VK_LOD_CLAMP_NONE")]
pub const LOD_CLAMP_NONE: f32 = VK_LOD_CLAMP_NONE;

/// [`VK_REMAINING_MIP_LEVELS`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_MIP_LEVELS.html)
///
#[doc(alias = "VK_REMAINING_MIP_LEVELS")]
pub const REMAINING_MIP_LEVELS: u32 = VK_REMAINING_MIP_LEVELS;

/// [`VK_REMAINING_ARRAY_LAYERS`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_ARRAY_LAYERS.html)
///
#[doc(alias = "VK_REMAINING_ARRAY_LAYERS")]
pub const REMAINING_ARRAY_LAYERS: u32 = VK_REMAINING_ARRAY_LAYERS;

/// [`VK_REMAINING_3D_SLICES_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_3D_SLICES_EXT.html)
///
#[doc(alias = "VK_REMAINING_3D_SLICES_EXT")]
pub const REMAINING_3D_SLICES_EXT: u32 = VK_REMAINING_3D_SLICES_EXT;

/// [`VK_WHOLE_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_WHOLE_SIZE.html)
///
#[doc(alias = "VK_WHOLE_SIZE")]
pub const WHOLE_SIZE: u64 = VK_WHOLE_SIZE;

/// [`VK_ATTACHMENT_UNUSED`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ATTACHMENT_UNUSED.html)
///
#[doc(alias = "VK_ATTACHMENT_UNUSED")]
pub const ATTACHMENT_UNUSED: u32 = VK_ATTACHMENT_UNUSED;

/// [`VK_TRUE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_TRUE.html)
///
#[doc(alias = "VK_TRUE")]
pub const TRUE: u32 = VK_TRUE;

/// [`VK_FALSE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FALSE.html)
///
#[doc(alias = "VK_FALSE")]
pub const FALSE: u32 = VK_FALSE;

/// [`VK_QUEUE_FAMILY_IGNORED`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_IGNORED.html)
///
#[doc(alias = "VK_QUEUE_FAMILY_IGNORED")]
pub const QUEUE_FAMILY_IGNORED: u32 = VK_QUEUE_FAMILY_IGNORED;

/// [`VK_QUEUE_FAMILY_EXTERNAL`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_EXTERNAL.html)
///
#[doc(alias = "VK_QUEUE_FAMILY_EXTERNAL")]
pub const QUEUE_FAMILY_EXTERNAL: u32 = VK_QUEUE_FAMILY_EXTERNAL;

/// [`VK_QUEUE_FAMILY_FOREIGN_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_FOREIGN_EXT.html)
///
#[doc(alias = "VK_QUEUE_FAMILY_FOREIGN_EXT")]
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = VK_QUEUE_FAMILY_FOREIGN_EXT;

/// [`VK_SUBPASS_EXTERNAL`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBPASS_EXTERNAL.html)
///
#[doc(alias = "VK_SUBPASS_EXTERNAL")]
pub const SUBPASS_EXTERNAL: u32 = VK_SUBPASS_EXTERNAL;

/// [`VK_MAX_DEVICE_GROUP_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DEVICE_GROUP_SIZE.html)
///
#[doc(alias = "VK_MAX_DEVICE_GROUP_SIZE")]
pub const MAX_DEVICE_GROUP_SIZE: u32 = VK_MAX_DEVICE_GROUP_SIZE;

/// [`VK_MAX_DRIVER_NAME_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DRIVER_NAME_SIZE.html)
///
#[doc(alias = "VK_MAX_DRIVER_NAME_SIZE")]
pub const MAX_DRIVER_NAME_SIZE: u32 = VK_MAX_DRIVER_NAME_SIZE;

/// [`VK_MAX_DRIVER_INFO_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DRIVER_INFO_SIZE.html)
///
#[doc(alias = "VK_MAX_DRIVER_INFO_SIZE")]
pub const MAX_DRIVER_INFO_SIZE: u32 = VK_MAX_DRIVER_INFO_SIZE;

/// [`VK_SHADER_UNUSED_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_UNUSED_KHR.html)
///
#[doc(alias = "VK_SHADER_UNUSED_KHR")]
pub const SHADER_UNUSED_KHR: u32 = VK_SHADER_UNUSED_KHR;

/// [`VK_MAX_GLOBAL_PRIORITY_SIZE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_GLOBAL_PRIORITY_SIZE.html)
///
#[doc(alias = "VK_MAX_GLOBAL_PRIORITY_SIZE")]
pub const MAX_GLOBAL_PRIORITY_SIZE: u32 = VK_MAX_GLOBAL_PRIORITY_SIZE;

/// [`VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT.html)
///
#[doc(alias = "VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT")]
pub const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT;

/// [`VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR.html)
///
#[doc(alias = "VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR")]
pub const MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR;

/// [`VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR.html)
///
#[doc(alias = "VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR")]
pub const MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: u32 = VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR;

/// [`VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR.html)
///
#[doc(alias = "VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR")]
pub const MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR;

/// [`VK_SHADER_INDEX_UNUSED_AMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_INDEX_UNUSED_AMDX.html)
///
#[doc(alias = "VK_SHADER_INDEX_UNUSED_AMDX")]
pub const SHADER_INDEX_UNUSED_AMDX: u32 = VK_SHADER_INDEX_UNUSED_AMDX;

/// [`VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV.html)
///
#[doc(alias = "VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV")]
pub const PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 =
    VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV;

/// [`VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX.html)
///
#[doc(alias = "VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX")]
pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 =
    VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX;

/// [`VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX.html)
///
#[doc(alias = "VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX")]
pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 =
    VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX;

/// [`VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM.html)
///
#[doc(alias = "VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM")]
pub const MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: u32 =
    VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM;

/// [`VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM.html)
///
#[doc(alias = "VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM")]
pub const DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: u32 =
    VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM;

/// [`VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV.html)
///
#[doc(alias = "VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV")]
pub const COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV;

/// [`VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV.html)
///
#[doc(alias = "VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV")]
pub const COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV;

/// [`VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV.html)
///
#[doc(alias = "VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV")]
pub const COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV;

/// [`VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM.html)
///
#[doc(alias = "VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM")]
pub const MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM: u32 = VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM;

/// [`VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM.html)
///
#[doc(alias = "VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM")]
pub const MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM: u32 =
    VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM;

/// [`STD_VIDEO_H264_CPB_CNT_LIST_SIZE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H264_CPB_CNT_LIST_SIZE")]
pub const H264_CPB_CNT_LIST_SIZE: u32 = STD_VIDEO_H264_CPB_CNT_LIST_SIZE;

/// [`STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS")]
pub const H264_SCALING_LIST_4X4_NUM_LISTS: u32 = STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS;

/// [`STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS")]
pub const H264_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS;

/// [`STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS")]
pub const H264_SCALING_LIST_8X8_NUM_LISTS: u32 = STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS;

/// [`STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS")]
pub const H264_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS;

/// [`STD_VIDEO_H264_MAX_NUM_LIST_REF`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H264_MAX_NUM_LIST_REF")]
pub const H264_MAX_NUM_LIST_REF: u32 = STD_VIDEO_H264_MAX_NUM_LIST_REF;

/// [`STD_VIDEO_H264_MAX_CHROMA_PLANES`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H264_MAX_CHROMA_PLANES")]
pub const H264_MAX_CHROMA_PLANES: u32 = STD_VIDEO_H264_MAX_CHROMA_PLANES;

/// [`STD_VIDEO_H264_NO_REFERENCE_PICTURE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H264_NO_REFERENCE_PICTURE")]
pub const H264_NO_REFERENCE_PICTURE: u8 = STD_VIDEO_H264_NO_REFERENCE_PICTURE;

/// [`STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE")]
pub const DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE: u32 =
    STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE;

/// [`STD_VIDEO_H265_CPB_CNT_LIST_SIZE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_CPB_CNT_LIST_SIZE")]
pub const H265_CPB_CNT_LIST_SIZE: u32 = STD_VIDEO_H265_CPB_CNT_LIST_SIZE;

/// [`STD_VIDEO_H265_SUBLAYERS_LIST_SIZE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_SUBLAYERS_LIST_SIZE")]
pub const H265_SUBLAYERS_LIST_SIZE: u32 = STD_VIDEO_H265_SUBLAYERS_LIST_SIZE;

/// [`STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS")]
pub const H265_SCALING_LIST_4X4_NUM_LISTS: u32 = STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS;

/// [`STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS")]
pub const H265_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS;

/// [`STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS")]
pub const H265_SCALING_LIST_8X8_NUM_LISTS: u32 = STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS;

/// [`STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS")]
pub const H265_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS;

/// [`STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS")]
pub const H265_SCALING_LIST_16X16_NUM_LISTS: u32 = STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS;

/// [`STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS")]
pub const H265_SCALING_LIST_16X16_NUM_ELEMENTS: u32 =
    STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS;

/// [`STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS")]
pub const H265_SCALING_LIST_32X32_NUM_LISTS: u32 = STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS;

/// [`STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS")]
pub const H265_SCALING_LIST_32X32_NUM_ELEMENTS: u32 =
    STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS;

/// [`STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE")]
pub const H265_CHROMA_QP_OFFSET_LIST_SIZE: u32 = STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE;

/// [`STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE")]
pub const H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE: u32 =
    STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE;

/// [`STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE")]
pub const H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE: u32 =
    STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE;

/// [`STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE")]
pub const H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE: u32 =
    STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE;

/// [`STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE")]
pub const H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE: u32 =
    STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE;

/// [`STD_VIDEO_H265_MAX_NUM_LIST_REF`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_MAX_NUM_LIST_REF")]
pub const H265_MAX_NUM_LIST_REF: u32 = STD_VIDEO_H265_MAX_NUM_LIST_REF;

/// [`STD_VIDEO_H265_MAX_CHROMA_PLANES`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_MAX_CHROMA_PLANES")]
pub const H265_MAX_CHROMA_PLANES: u32 = STD_VIDEO_H265_MAX_CHROMA_PLANES;

/// [`STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS")]
pub const H265_MAX_SHORT_TERM_REF_PIC_SETS: u32 = STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS;

/// [`STD_VIDEO_H265_MAX_DPB_SIZE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_MAX_DPB_SIZE")]
pub const H265_MAX_DPB_SIZE: u32 = STD_VIDEO_H265_MAX_DPB_SIZE;

/// [`STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS")]
pub const H265_MAX_LONG_TERM_REF_PICS_SPS: u32 = STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS;

/// [`STD_VIDEO_H265_MAX_LONG_TERM_PICS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_MAX_LONG_TERM_PICS")]
pub const H265_MAX_LONG_TERM_PICS: u32 = STD_VIDEO_H265_MAX_LONG_TERM_PICS;

/// [`STD_VIDEO_H265_MAX_DELTA_POC`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_MAX_DELTA_POC")]
pub const H265_MAX_DELTA_POC: u32 = STD_VIDEO_H265_MAX_DELTA_POC;

/// [`STD_VIDEO_H265_NO_REFERENCE_PICTURE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_H265_NO_REFERENCE_PICTURE")]
pub const H265_NO_REFERENCE_PICTURE: u8 = STD_VIDEO_H265_NO_REFERENCE_PICTURE;

/// [`STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE")]
pub const DECODE_H265_REF_PIC_SET_LIST_SIZE: u32 = STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE;

/// [`STD_VIDEO_AV1_NUM_REF_FRAMES`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_NUM_REF_FRAMES")]
pub const AV1_NUM_REF_FRAMES: u32 = STD_VIDEO_AV1_NUM_REF_FRAMES;

/// [`STD_VIDEO_AV1_REFS_PER_FRAME`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_REFS_PER_FRAME")]
pub const AV1_REFS_PER_FRAME: u32 = STD_VIDEO_AV1_REFS_PER_FRAME;

/// [`STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME")]
pub const AV1_TOTAL_REFS_PER_FRAME: u32 = STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME;

/// [`STD_VIDEO_AV1_MAX_TILE_COLS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_MAX_TILE_COLS")]
pub const AV1_MAX_TILE_COLS: u32 = STD_VIDEO_AV1_MAX_TILE_COLS;

/// [`STD_VIDEO_AV1_MAX_TILE_ROWS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_MAX_TILE_ROWS")]
pub const AV1_MAX_TILE_ROWS: u32 = STD_VIDEO_AV1_MAX_TILE_ROWS;

/// [`STD_VIDEO_AV1_MAX_SEGMENTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_MAX_SEGMENTS")]
pub const AV1_MAX_SEGMENTS: u32 = STD_VIDEO_AV1_MAX_SEGMENTS;

/// [`STD_VIDEO_AV1_SEG_LVL_MAX`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_SEG_LVL_MAX")]
pub const AV1_SEG_LVL_MAX: u32 = STD_VIDEO_AV1_SEG_LVL_MAX;

/// [`STD_VIDEO_AV1_PRIMARY_REF_NONE`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_PRIMARY_REF_NONE")]
pub const AV1_PRIMARY_REF_NONE: u8 = STD_VIDEO_AV1_PRIMARY_REF_NONE;

/// [`STD_VIDEO_AV1_SELECT_INTEGER_MV`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_SELECT_INTEGER_MV")]
pub const AV1_SELECT_INTEGER_MV: u8 = STD_VIDEO_AV1_SELECT_INTEGER_MV;

/// [`STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS")]
pub const AV1_SELECT_SCREEN_CONTENT_TOOLS: u32 = STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS;

/// [`STD_VIDEO_AV1_SKIP_MODE_FRAMES`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_SKIP_MODE_FRAMES")]
pub const AV1_SKIP_MODE_FRAMES: u32 = STD_VIDEO_AV1_SKIP_MODE_FRAMES;

/// [`STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS")]
pub const AV1_MAX_LOOP_FILTER_STRENGTHS: u32 = STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS;

/// [`STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS")]
pub const AV1_LOOP_FILTER_ADJUSTMENTS: u32 = STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS;

/// [`STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS")]
pub const AV1_MAX_CDEF_FILTER_STRENGTHS: u32 = STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS;

/// [`STD_VIDEO_AV1_MAX_NUM_PLANES`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_MAX_NUM_PLANES")]
pub const AV1_MAX_NUM_PLANES: u32 = STD_VIDEO_AV1_MAX_NUM_PLANES;

/// [`STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS")]
pub const AV1_GLOBAL_MOTION_PARAMS: u32 = STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS;

/// [`STD_VIDEO_AV1_MAX_NUM_Y_POINTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_MAX_NUM_Y_POINTS")]
pub const AV1_MAX_NUM_Y_POINTS: u32 = STD_VIDEO_AV1_MAX_NUM_Y_POINTS;

/// [`STD_VIDEO_AV1_MAX_NUM_CB_POINTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_MAX_NUM_CB_POINTS")]
pub const AV1_MAX_NUM_CB_POINTS: u32 = STD_VIDEO_AV1_MAX_NUM_CB_POINTS;

/// [`STD_VIDEO_AV1_MAX_NUM_CR_POINTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_MAX_NUM_CR_POINTS")]
pub const AV1_MAX_NUM_CR_POINTS: u32 = STD_VIDEO_AV1_MAX_NUM_CR_POINTS;

/// [`STD_VIDEO_AV1_MAX_NUM_POS_LUMA`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_MAX_NUM_POS_LUMA")]
pub const AV1_MAX_NUM_POS_LUMA: u32 = STD_VIDEO_AV1_MAX_NUM_POS_LUMA;

/// [`STD_VIDEO_AV1_MAX_NUM_POS_CHROMA`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_AV1_MAX_NUM_POS_CHROMA")]
pub const AV1_MAX_NUM_POS_CHROMA: u32 = STD_VIDEO_AV1_MAX_NUM_POS_CHROMA;

/// [`STD_VIDEO_VP9_NUM_REF_FRAMES`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_VP9_NUM_REF_FRAMES")]
pub const VP9_NUM_REF_FRAMES: u32 = STD_VIDEO_VP9_NUM_REF_FRAMES;

/// [`STD_VIDEO_VP9_REFS_PER_FRAME`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_VP9_REFS_PER_FRAME")]
pub const VP9_REFS_PER_FRAME: u32 = STD_VIDEO_VP9_REFS_PER_FRAME;

/// [`STD_VIDEO_VP9_MAX_REF_FRAMES`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_VP9_MAX_REF_FRAMES")]
pub const VP9_MAX_REF_FRAMES: u32 = STD_VIDEO_VP9_MAX_REF_FRAMES;

/// [`STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS")]
pub const VP9_LOOP_FILTER_ADJUSTMENTS: u32 = STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS;

/// [`STD_VIDEO_VP9_MAX_SEGMENTS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_VP9_MAX_SEGMENTS")]
pub const VP9_MAX_SEGMENTS: u32 = STD_VIDEO_VP9_MAX_SEGMENTS;

/// [`STD_VIDEO_VP9_SEG_LVL_MAX`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_VP9_SEG_LVL_MAX")]
pub const VP9_SEG_LVL_MAX: u32 = STD_VIDEO_VP9_SEG_LVL_MAX;

/// [`STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS")]
pub const VP9_MAX_SEGMENTATION_TREE_PROBS: u32 = STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS;

/// [`STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB`](https://docs.vulkan.org/spec/latest/chapters/videocoding.html) (Vulkan Video)
///
#[doc(alias = "STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB")]
pub const VP9_MAX_SEGMENTATION_PRED_PROB: u32 = STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB;
