// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::manual::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};
/// Enum containing all extensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Extensions {
    #[doc(alias = "VK_KHR_surface")]
    KHR_Surface,
    #[doc(alias = "VK_KHR_swapchain")]
    KHR_Swapchain,
    #[doc(alias = "VK_KHR_display")]
    KHR_Display,
    #[doc(alias = "VK_KHR_display_swapchain")]
    KHR_DisplaySwapchain,
    #[doc(alias = "VK_KHR_xlib_surface")]
    KHR_XlibSurface,
    #[doc(alias = "VK_KHR_xcb_surface")]
    KHR_XcbSurface,
    #[doc(alias = "VK_KHR_wayland_surface")]
    KHR_WaylandSurface,
    #[doc(alias = "VK_KHR_android_surface")]
    KHR_AndroidSurface,
    #[doc(alias = "VK_KHR_win32_surface")]
    KHR_Win32Surface,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_sampler_mirror_clamp_to_edge")]
    KHR_SamplerMirrorClampToEdge,
    #[doc(alias = "VK_KHR_video_queue")]
    KHR_VideoQueue,
    #[doc(alias = "VK_KHR_video_decode_queue")]
    KHR_VideoDecodeQueue,
    #[doc(alias = "VK_KHR_video_encode_h264")]
    KHR_VideoEncodeH264,
    #[doc(alias = "VK_KHR_video_encode_h265")]
    KHR_VideoEncodeH265,
    #[doc(alias = "VK_KHR_video_decode_h264")]
    KHR_VideoDecodeH264,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_KHR_dynamic_rendering")]
    KHR_DynamicRendering,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_multiview")]
    KHR_Multiview,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_get_physical_device_properties2")]
    KHR_GetPhysicalDeviceProperties2,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_device_group")]
    KHR_DeviceGroup,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_shader_draw_parameters")]
    KHR_ShaderDrawParameters,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_maintenance1")]
    KHR_Maintenance1,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_device_group_creation")]
    KHR_DeviceGroupCreation,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_external_memory_capabilities")]
    KHR_ExternalMemoryCapabilities,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_external_memory")]
    KHR_ExternalMemory,
    #[doc(alias = "VK_KHR_external_memory_win32")]
    KHR_ExternalMemoryWin32,
    #[doc(alias = "VK_KHR_external_memory_fd")]
    KHR_ExternalMemoryFd,
    #[doc(alias = "VK_KHR_win32_keyed_mutex")]
    KHR_Win32KeyedMutex,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_external_semaphore_capabilities")]
    KHR_ExternalSemaphoreCapabilities,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_external_semaphore")]
    KHR_ExternalSemaphore,
    #[doc(alias = "VK_KHR_external_semaphore_win32")]
    KHR_ExternalSemaphoreWin32,
    #[doc(alias = "VK_KHR_external_semaphore_fd")]
    KHR_ExternalSemaphoreFd,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_push_descriptor")]
    KHR_PushDescriptor,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_shader_float16_int8")]
    KHR_ShaderFloat16Int8,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_16bit_storage")]
    KHR_16BitStorage,
    #[doc(alias = "VK_KHR_incremental_present")]
    KHR_IncrementalPresent,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_descriptor_update_template")]
    KHR_DescriptorUpdateTemplate,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_imageless_framebuffer")]
    KHR_ImagelessFramebuffer,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_create_renderpass2")]
    KHR_CreateRenderpass2,
    #[doc(alias = "VK_KHR_shared_presentable_image")]
    KHR_SharedPresentableImage,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_external_fence_capabilities")]
    KHR_ExternalFenceCapabilities,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_external_fence")]
    KHR_ExternalFence,
    #[doc(alias = "VK_KHR_external_fence_win32")]
    KHR_ExternalFenceWin32,
    #[doc(alias = "VK_KHR_external_fence_fd")]
    KHR_ExternalFenceFd,
    /// Intended for development tooling.
    #[doc(alias = "VK_KHR_performance_query")]
    KHR_PerformanceQuery,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_maintenance2")]
    KHR_Maintenance2,
    #[doc(alias = "VK_KHR_get_surface_capabilities2")]
    KHR_GetSurfaceCapabilities2,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_variable_pointers")]
    KHR_VariablePointers,
    #[doc(alias = "VK_KHR_get_display_properties2")]
    KHR_GetDisplayProperties2,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_dedicated_allocation")]
    KHR_DedicatedAllocation,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_storage_buffer_storage_class")]
    KHR_StorageBufferStorageClass,
    #[doc(alias = "VK_KHR_shader_bfloat16")]
    KHR_ShaderBfloat16,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_relaxed_block_layout")]
    KHR_RelaxedBlockLayout,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_get_memory_requirements2")]
    KHR_GetMemoryRequirements2,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_image_format_list")]
    KHR_ImageFormatList,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_sampler_ycbcr_conversion")]
    KHR_SamplerYcbcrConversion,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_bind_memory2")]
    KHR_BindMemory2,
    #[doc(alias = "VK_KHR_portability_subset")]
    KHR_PortabilitySubset,
    /// Promoted to core in version 1.1.
    #[doc(alias = "VK_KHR_maintenance3")]
    KHR_Maintenance3,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_draw_indirect_count")]
    KHR_DrawIndirectCount,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_shader_subgroup_extended_types")]
    KHR_ShaderSubgroupExtendedTypes,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_8bit_storage")]
    KHR_8BitStorage,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_shader_atomic_int64")]
    KHR_ShaderAtomicInt64,
    #[doc(alias = "VK_KHR_shader_clock")]
    KHR_ShaderClock,
    #[doc(alias = "VK_KHR_video_decode_h265")]
    KHR_VideoDecodeH265,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_global_priority")]
    KHR_GlobalPriority,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_driver_properties")]
    KHR_DriverProperties,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_shader_float_controls")]
    KHR_ShaderFloatControls,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_depth_stencil_resolve")]
    KHR_DepthStencilResolve,
    #[doc(alias = "VK_KHR_swapchain_mutable_format")]
    KHR_SwapchainMutableFormat,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_timeline_semaphore")]
    KHR_TimelineSemaphore,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_vulkan_memory_model")]
    KHR_VulkanMemoryModel,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_KHR_shader_terminate_invocation")]
    KHR_ShaderTerminateInvocation,
    #[doc(alias = "VK_KHR_fragment_shading_rate")]
    KHR_FragmentShadingRate,
    #[doc(alias = "VK_KHR_shader_constant_data")]
    KHR_ShaderConstantData,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_dynamic_rendering_local_read")]
    KHR_DynamicRenderingLocalRead,
    #[doc(alias = "VK_KHR_shader_abort")]
    KHR_ShaderAbort,
    #[doc(alias = "VK_KHR_shader_quad_control")]
    KHR_ShaderQuadControl,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_spirv_1_4")]
    KHR_Spirv14,
    #[doc(alias = "VK_KHR_surface_protected_capabilities")]
    KHR_SurfaceProtectedCapabilities,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_separate_depth_stencil_layouts")]
    KHR_SeparateDepthStencilLayouts,
    #[doc(alias = "VK_KHR_present_wait")]
    KHR_PresentWait,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_uniform_buffer_standard_layout")]
    KHR_UniformBufferStandardLayout,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_KHR_buffer_device_address")]
    KHR_BufferDeviceAddress,
    #[doc(alias = "VK_KHR_deferred_host_operations")]
    KHR_DeferredHostOperations,
    /// Intended for development tooling.
    #[doc(alias = "VK_KHR_pipeline_executable_properties")]
    KHR_PipelineExecutableProperties,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_map_memory2")]
    KHR_MapMemory2,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_KHR_shader_integer_dot_product")]
    KHR_ShaderIntegerDotProduct,
    #[doc(alias = "VK_KHR_pipeline_library")]
    KHR_PipelineLibrary,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_KHR_shader_non_semantic_info")]
    KHR_ShaderNonSemanticInfo,
    #[doc(alias = "VK_KHR_present_id")]
    KHR_PresentId,
    #[doc(alias = "VK_KHR_video_encode_queue")]
    KHR_VideoEncodeQueue,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_KHR_synchronization2")]
    KHR_Synchronization2,
    #[doc(alias = "VK_KHR_device_address_commands")]
    KHR_DeviceAddressCommands,
    #[doc(alias = "VK_KHR_fragment_shader_barycentric")]
    KHR_FragmentShaderBarycentric,
    #[doc(alias = "VK_KHR_shader_subgroup_uniform_control_flow")]
    KHR_ShaderSubgroupUniformControlFlow,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_KHR_zero_initialize_workgroup_memory")]
    KHR_ZeroInitializeWorkgroupMemory,
    #[doc(alias = "VK_KHR_workgroup_memory_explicit_layout")]
    KHR_WorkgroupMemoryExplicitLayout,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_KHR_copy_commands2")]
    KHR_CopyCommands2,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_KHR_format_feature_flags2")]
    KHR_FormatFeatureFlags2,
    #[doc(alias = "VK_KHR_ray_tracing_maintenance1")]
    KHR_RayTracingMaintenance1,
    #[doc(alias = "VK_KHR_shader_untyped_pointers")]
    KHR_ShaderUntypedPointers,
    #[doc(alias = "VK_KHR_portability_enumeration")]
    KHR_PortabilityEnumeration,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_KHR_maintenance4")]
    KHR_Maintenance4,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_shader_subgroup_rotate")]
    KHR_ShaderSubgroupRotate,
    #[doc(alias = "VK_KHR_shader_maximal_reconvergence")]
    KHR_ShaderMaximalReconvergence,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_maintenance5")]
    KHR_Maintenance5,
    #[doc(alias = "VK_KHR_present_id2")]
    KHR_PresentId2,
    #[doc(alias = "VK_KHR_present_wait2")]
    KHR_PresentWait2,
    #[doc(alias = "VK_KHR_ray_tracing_position_fetch")]
    KHR_RayTracingPositionFetch,
    #[doc(alias = "VK_KHR_pipeline_binary")]
    KHR_PipelineBinary,
    #[doc(alias = "VK_KHR_surface_maintenance1")]
    KHR_SurfaceMaintenance1,
    #[doc(alias = "VK_KHR_swapchain_maintenance1")]
    KHR_SwapchainMaintenance1,
    #[doc(alias = "VK_KHR_internally_synchronized_queues")]
    KHR_InternallySynchronizedQueues,
    #[doc(alias = "VK_KHR_cooperative_matrix")]
    KHR_CooperativeMatrix,
    #[doc(alias = "VK_KHR_compute_shader_derivatives")]
    KHR_ComputeShaderDerivatives,
    #[doc(alias = "VK_KHR_video_decode_av1")]
    KHR_VideoDecodeAv1,
    #[doc(alias = "VK_KHR_video_encode_av1")]
    KHR_VideoEncodeAv1,
    #[doc(alias = "VK_KHR_video_decode_vp9")]
    KHR_VideoDecodeVp9,
    #[doc(alias = "VK_KHR_video_maintenance1")]
    KHR_VideoMaintenance1,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_vertex_attribute_divisor")]
    KHR_VertexAttributeDivisor,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_load_store_op_none")]
    KHR_LoadStoreOpNone,
    #[doc(alias = "VK_KHR_unified_image_layouts")]
    KHR_UnifiedImageLayouts,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_shader_float_controls2")]
    KHR_ShaderFloatControls2,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_index_type_uint8")]
    KHR_IndexTypeUint8,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_line_rasterization")]
    KHR_LineRasterization,
    #[doc(alias = "VK_KHR_calibrated_timestamps")]
    KHR_CalibratedTimestamps,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_shader_expect_assume")]
    KHR_ShaderExpectAssume,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_KHR_maintenance6")]
    KHR_Maintenance6,
    #[doc(alias = "VK_KHR_copy_memory_indirect")]
    KHR_CopyMemoryIndirect,
    #[doc(alias = "VK_KHR_video_encode_intra_refresh")]
    KHR_VideoEncodeIntraRefresh,
    #[doc(alias = "VK_KHR_video_encode_quantization_map")]
    KHR_VideoEncodeQuantizationMap,
    #[doc(alias = "VK_KHR_shader_relaxed_extended_instruction")]
    KHR_ShaderRelaxedExtendedInstruction,
    #[doc(alias = "VK_KHR_maintenance7")]
    KHR_Maintenance7,
    #[doc(alias = "VK_KHR_device_fault")]
    KHR_DeviceFault,
    #[doc(alias = "VK_KHR_maintenance8")]
    KHR_Maintenance8,
    #[doc(alias = "VK_KHR_shader_fma")]
    KHR_ShaderFma,
    #[doc(alias = "VK_KHR_maintenance9")]
    KHR_Maintenance9,
    #[doc(alias = "VK_KHR_video_maintenance2")]
    KHR_VideoMaintenance2,
    #[doc(alias = "VK_KHR_video_encode_feedback2")]
    KHR_VideoEncodeFeedback2,
    #[doc(alias = "VK_KHR_depth_clamp_zero_one")]
    KHR_DepthClampZeroOne,
    #[doc(alias = "VK_KHR_robustness2")]
    KHR_Robustness2,
    #[doc(alias = "VK_KHR_present_mode_fifo_latest_ready")]
    KHR_PresentModeFifoLatestReady,
    #[doc(alias = "VK_KHR_opacity_micromap")]
    KHR_OpacityMicromap,
    #[doc(alias = "VK_KHR_maintenance10")]
    KHR_Maintenance10,
    #[doc(alias = "VK_KHR_pipeline_library_group_handles")]
    KHR_PipelineLibraryGroupHandles,
    #[doc(alias = "VK_KHR_maintenance11")]
    KHR_Maintenance11,
    #[doc(alias = "VK_KHR_extended_flags")]
    KHR_ExtendedFlags,
    /// Deprecated by [`Self::EXT_DebugUtils`].
    /// Intended for debugging.
    #[doc(alias = "VK_EXT_debug_report")]
    EXT_DebugReport,
    #[doc(alias = "VK_NV_glsl_shader")]
    NV_GlslShader,
    #[doc(alias = "VK_EXT_depth_range_unrestricted")]
    EXT_DepthRangeUnrestricted,
    #[doc(alias = "VK_IMG_filter_cubic")]
    IMG_FilterCubic,
    #[doc(alias = "VK_AMD_rasterization_order")]
    AMD_RasterizationOrder,
    #[doc(alias = "VK_AMD_shader_trinary_minmax")]
    AMD_ShaderTrinaryMinmax,
    #[doc(alias = "VK_AMD_shader_explicit_vertex_parameter")]
    AMD_ShaderExplicitVertexParameter,
    /// Promoted to [`Self::EXT_DebugUtils`].
    /// Intended for debugging.
    #[doc(alias = "VK_EXT_debug_marker")]
    EXT_DebugMarker,
    #[doc(alias = "VK_AMD_gcn_shader")]
    AMD_GcnShader,
    /// Deprecated by [`Self::KHR_DedicatedAllocation`].
    #[doc(alias = "VK_NV_dedicated_allocation")]
    NV_DedicatedAllocation,
    /// Intended for OpenGL emulation, Direct3D emulation, development tooling.
    #[doc(alias = "VK_EXT_transform_feedback")]
    EXT_TransformFeedback,
    #[doc(alias = "VK_NVX_binary_import")]
    NVX_BinaryImport,
    #[doc(alias = "VK_NVX_image_view_handle")]
    NVX_ImageViewHandle,
    /// Promoted to [`Self::KHR_DrawIndirectCount`].
    #[doc(alias = "VK_AMD_draw_indirect_count")]
    AMD_DrawIndirectCount,
    #[doc(alias = "VK_AMD_negative_viewport_height")]
    AMD_NegativeViewportHeight,
    /// Deprecated by [`Self::KHR_ShaderFloat16Int8`].
    #[doc(alias = "VK_AMD_gpu_shader_half_float")]
    AMD_GpuShaderHalfFloat,
    #[doc(alias = "VK_AMD_shader_ballot")]
    AMD_ShaderBallot,
    #[doc(alias = "VK_AMD_texture_gather_bias_lod")]
    AMD_TextureGatherBiasLod,
    /// Intended for development tooling.
    #[doc(alias = "VK_AMD_shader_info")]
    AMD_ShaderInfo,
    #[doc(alias = "VK_AMD_shader_image_load_store_lod")]
    AMD_ShaderImageLoadStoreLod,
    #[doc(alias = "VK_GGP_stream_descriptor_surface")]
    GGP_StreamDescriptorSurface,
    #[doc(alias = "VK_NV_corner_sampled_image")]
    NV_CornerSampledImage,
    #[doc(alias = "VK_IMG_format_pvrtc")]
    IMG_FormatPvrtc,
    /// Deprecated by [`Self::KHR_ExternalMemoryCapabilities`].
    #[doc(alias = "VK_NV_external_memory_capabilities")]
    NV_ExternalMemoryCapabilities,
    /// Deprecated by [`Self::KHR_ExternalMemory`].
    #[doc(alias = "VK_NV_external_memory")]
    NV_ExternalMemory,
    /// Deprecated by [`Self::KHR_ExternalMemoryWin32`].
    #[doc(alias = "VK_NV_external_memory_win32")]
    NV_ExternalMemoryWin32,
    /// Promoted to [`Self::KHR_Win32KeyedMutex`].
    #[doc(alias = "VK_NV_win32_keyed_mutex")]
    NV_Win32KeyedMutex,
    /// Deprecated by [`Self::EXT_LayerSettings`].
    /// Intended for debugging.
    #[doc(alias = "VK_EXT_validation_flags")]
    EXT_ValidationFlags,
    #[doc(alias = "VK_NN_vi_surface")]
    NN_ViSurface,
    /// Deprecated by 1.2.
    #[doc(alias = "VK_EXT_shader_subgroup_ballot")]
    EXT_ShaderSubgroupBallot,
    /// Deprecated by 1.1.
    #[doc(alias = "VK_EXT_shader_subgroup_vote")]
    EXT_ShaderSubgroupVote,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_texture_compression_astc_hdr")]
    EXT_TextureCompressionAstcHdr,
    #[doc(alias = "VK_EXT_astc_decode_mode")]
    EXT_AstcDecodeMode,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_EXT_pipeline_robustness")]
    EXT_PipelineRobustness,
    #[doc(alias = "VK_EXT_conditional_rendering")]
    EXT_ConditionalRendering,
    #[doc(alias = "VK_NV_clip_space_w_scaling")]
    NV_ClipSpaceWScaling,
    #[doc(alias = "VK_EXT_direct_mode_display")]
    EXT_DirectModeDisplay,
    #[doc(alias = "VK_EXT_acquire_xlib_display")]
    EXT_AcquireXlibDisplay,
    #[doc(alias = "VK_EXT_display_surface_counter")]
    EXT_DisplaySurfaceCounter,
    #[doc(alias = "VK_EXT_display_control")]
    EXT_DisplayControl,
    #[doc(alias = "VK_GOOGLE_display_timing")]
    GOOGLE_DisplayTiming,
    #[doc(alias = "VK_NV_sample_mask_override_coverage")]
    NV_SampleMaskOverrideCoverage,
    #[doc(alias = "VK_NV_geometry_shader_passthrough")]
    NV_GeometryShaderPassthrough,
    #[doc(alias = "VK_NV_viewport_array2")]
    NV_ViewportArray2,
    #[doc(alias = "VK_NVX_multiview_per_view_attributes")]
    NVX_MultiviewPerViewAttributes,
    #[doc(alias = "VK_NV_viewport_swizzle")]
    NV_ViewportSwizzle,
    #[doc(alias = "VK_EXT_discard_rectangles")]
    EXT_DiscardRectangles,
    #[doc(alias = "VK_EXT_conservative_rasterization")]
    EXT_ConservativeRasterization,
    /// Intended for Direct3D emulation.
    #[doc(alias = "VK_EXT_depth_clip_enable")]
    EXT_DepthClipEnable,
    #[doc(alias = "VK_EXT_swapchain_colorspace")]
    EXT_SwapchainColorspace,
    #[doc(alias = "VK_EXT_hdr_metadata")]
    EXT_HdrMetadata,
    /// Intended for OpenGL emulation.
    #[doc(alias = "VK_IMG_relaxed_line_rasterization")]
    IMG_RelaxedLineRasterization,
    /// Deprecated by [`Self::EXT_MetalSurface`].
    #[doc(alias = "VK_MVK_ios_surface")]
    MVK_IosSurface,
    /// Deprecated by [`Self::EXT_MetalSurface`].
    #[doc(alias = "VK_MVK_macos_surface")]
    MVK_MacosSurface,
    #[doc(alias = "VK_EXT_external_memory_dma_buf")]
    EXT_ExternalMemoryDmaBuf,
    #[doc(alias = "VK_EXT_queue_family_foreign")]
    EXT_QueueFamilyForeign,
    /// Intended for debugging.
    #[doc(alias = "VK_EXT_debug_utils")]
    EXT_DebugUtils,
    #[doc(alias = "VK_ANDROID_external_memory_android_hardware_buffer")]
    ANDROID_ExternalMemoryAndroidHardwareBuffer,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_EXT_sampler_filter_minmax")]
    EXT_SamplerFilterMinmax,
    /// Deprecated by [`Self::KHR_ShaderFloat16Int8`].
    #[doc(alias = "VK_AMD_gpu_shader_int16")]
    AMD_GpuShaderInt16,
    #[doc(alias = "VK_AMD_gpa_interface")]
    AMD_GpaInterface,
    #[doc(alias = "VK_AMDX_shader_enqueue")]
    AMDX_ShaderEnqueue,
    #[doc(alias = "VK_EXT_descriptor_heap")]
    EXT_DescriptorHeap,
    #[doc(alias = "VK_AMD_mixed_attachment_samples")]
    AMD_MixedAttachmentSamples,
    #[doc(alias = "VK_AMD_shader_fragment_mask")]
    AMD_ShaderFragmentMask,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_inline_uniform_block")]
    EXT_InlineUniformBlock,
    #[doc(alias = "VK_EXT_shader_stencil_export")]
    EXT_ShaderStencilExport,
    #[doc(alias = "VK_EXT_sample_locations")]
    EXT_SampleLocations,
    #[doc(alias = "VK_EXT_blend_operation_advanced")]
    EXT_BlendOperationAdvanced,
    #[doc(alias = "VK_NV_fragment_coverage_to_color")]
    NV_FragmentCoverageToColor,
    #[doc(alias = "VK_NV_framebuffer_mixed_samples")]
    NV_FramebufferMixedSamples,
    #[doc(alias = "VK_NV_fill_rectangle")]
    NV_FillRectangle,
    #[doc(alias = "VK_NV_shader_sm_builtins")]
    NV_ShaderSmBuiltins,
    #[doc(alias = "VK_EXT_post_depth_coverage")]
    EXT_PostDepthCoverage,
    #[doc(alias = "VK_EXT_image_drm_format_modifier")]
    EXT_ImageDrmFormatModifier,
    #[doc(alias = "VK_EXT_validation_cache")]
    EXT_ValidationCache,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_EXT_descriptor_indexing")]
    EXT_DescriptorIndexing,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_EXT_shader_viewport_index_layer")]
    EXT_ShaderViewportIndexLayer,
    #[doc(alias = "VK_NV_shading_rate_image")]
    NV_ShadingRateImage,
    /// Deprecated by [`Self::KHR_RayTracingPipeline`].
    #[doc(alias = "VK_NV_ray_tracing")]
    NV_RayTracing,
    #[doc(alias = "VK_NV_representative_fragment_test")]
    NV_RepresentativeFragmentTest,
    #[doc(alias = "VK_EXT_filter_cubic")]
    EXT_FilterCubic,
    /// Promoted to [`Self::EXT_CustomResolve`].
    #[doc(alias = "VK_QCOM_render_pass_shader_resolve")]
    QCOM_RenderPassShaderResolve,
    #[doc(alias = "VK_QCOM_cooperative_matrix_conversion")]
    QCOM_CooperativeMatrixConversion,
    #[doc(alias = "VK_QCOM_elapsed_timer_query")]
    QCOM_ElapsedTimerQuery,
    /// Promoted to [`Self::KHR_GlobalPriority`].
    #[doc(alias = "VK_EXT_global_priority")]
    EXT_GlobalPriority,
    #[doc(alias = "VK_EXT_external_memory_host")]
    EXT_ExternalMemoryHost,
    /// Intended for development tooling.
    #[doc(alias = "VK_AMD_buffer_marker")]
    AMD_BufferMarker,
    #[doc(alias = "VK_AMD_pipeline_compiler_control")]
    AMD_PipelineCompilerControl,
    /// Promoted to [`Self::KHR_CalibratedTimestamps`].
    #[doc(alias = "VK_EXT_calibrated_timestamps")]
    EXT_CalibratedTimestamps,
    #[doc(alias = "VK_AMD_shader_core_properties")]
    AMD_ShaderCoreProperties,
    #[doc(alias = "VK_AMD_memory_overallocation_behavior")]
    AMD_MemoryOverallocationBehavior,
    /// Promoted to [`Self::KHR_VertexAttributeDivisor`].
    #[doc(alias = "VK_EXT_vertex_attribute_divisor")]
    EXT_VertexAttributeDivisor,
    #[doc(alias = "VK_GGP_frame_token")]
    GGP_FrameToken,
    /// Promoted to core in version 1.3.
    /// Intended for development tooling.
    #[doc(alias = "VK_EXT_pipeline_creation_feedback")]
    EXT_PipelineCreationFeedback,
    /// Promoted to [`Self::EXT_ShaderSubgroupPartitioned`].
    #[doc(alias = "VK_NV_shader_subgroup_partitioned")]
    NV_ShaderSubgroupPartitioned,
    /// Promoted to [`Self::KHR_ComputeShaderDerivatives`].
    #[doc(alias = "VK_NV_compute_shader_derivatives")]
    NV_ComputeShaderDerivatives,
    #[doc(alias = "VK_NV_mesh_shader")]
    NV_MeshShader,
    /// Promoted to [`Self::KHR_FragmentShaderBarycentric`].
    #[doc(alias = "VK_NV_fragment_shader_barycentric")]
    NV_FragmentShaderBarycentric,
    #[doc(alias = "VK_NV_shader_image_footprint")]
    NV_ShaderImageFootprint,
    #[doc(alias = "VK_NV_scissor_exclusive")]
    NV_ScissorExclusive,
    #[doc(alias = "VK_NV_device_diagnostic_checkpoints")]
    NV_DeviceDiagnosticCheckpoints,
    #[doc(alias = "VK_EXT_present_timing")]
    EXT_PresentTiming,
    #[doc(alias = "VK_INTEL_shader_integer_functions2")]
    INTEL_ShaderIntegerFunctions2,
    /// Intended for development tooling.
    #[doc(alias = "VK_INTEL_performance_query")]
    INTEL_PerformanceQuery,
    #[doc(alias = "VK_EXT_pci_bus_info")]
    EXT_PciBusInfo,
    #[doc(alias = "VK_AMD_display_native_hdr")]
    AMD_DisplayNativeHdr,
    #[doc(alias = "VK_FUCHSIA_imagepipe_surface")]
    FUCHSIA_ImagepipeSurface,
    #[doc(alias = "VK_EXT_metal_surface")]
    EXT_MetalSurface,
    #[doc(alias = "VK_EXT_fragment_density_map")]
    EXT_FragmentDensityMap,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_EXT_scalar_block_layout")]
    EXT_ScalarBlockLayout,
    #[doc(alias = "VK_GOOGLE_hlsl_functionality1")]
    GOOGLE_HlslFunctionality1,
    #[doc(alias = "VK_GOOGLE_decorate_string")]
    GOOGLE_DecorateString,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_subgroup_size_control")]
    EXT_SubgroupSizeControl,
    #[doc(alias = "VK_AMD_shader_core_properties2")]
    AMD_ShaderCoreProperties2,
    #[doc(alias = "VK_AMD_device_coherent_memory")]
    AMD_DeviceCoherentMemory,
    #[doc(alias = "VK_EXT_shader_image_atomic_int64")]
    EXT_ShaderImageAtomicInt64,
    #[doc(alias = "VK_EXT_memory_budget")]
    EXT_MemoryBudget,
    #[doc(alias = "VK_EXT_memory_priority")]
    EXT_MemoryPriority,
    #[doc(alias = "VK_NV_dedicated_allocation_image_aliasing")]
    NV_DedicatedAllocationImageAliasing,
    /// Deprecated by [`Self::KHR_BufferDeviceAddress`].
    #[doc(alias = "VK_EXT_buffer_device_address")]
    EXT_BufferDeviceAddress,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_tooling_info")]
    EXT_ToolingInfo,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_EXT_separate_stencil_usage")]
    EXT_SeparateStencilUsage,
    /// Deprecated by [`Self::EXT_LayerSettings`].
    /// Intended for debugging.
    #[doc(alias = "VK_EXT_validation_features")]
    EXT_ValidationFeatures,
    #[doc(alias = "VK_NV_cooperative_matrix")]
    NV_CooperativeMatrix,
    #[doc(alias = "VK_NV_coverage_reduction_mode")]
    NV_CoverageReductionMode,
    #[doc(alias = "VK_EXT_fragment_shader_interlock")]
    EXT_FragmentShaderInterlock,
    #[doc(alias = "VK_EXT_ycbcr_image_arrays")]
    EXT_YcbcrImageArrays,
    /// Intended for OpenGL emulation.
    #[doc(alias = "VK_EXT_provoking_vertex")]
    EXT_ProvokingVertex,
    #[doc(alias = "VK_EXT_full_screen_exclusive")]
    EXT_FullScreenExclusive,
    #[doc(alias = "VK_EXT_headless_surface")]
    EXT_HeadlessSurface,
    /// Promoted to [`Self::KHR_LineRasterization`].
    /// Intended for cadsupport.
    #[doc(alias = "VK_EXT_line_rasterization")]
    EXT_LineRasterization,
    #[doc(alias = "VK_EXT_shader_atomic_float")]
    EXT_ShaderAtomicFloat,
    /// Promoted to core in version 1.2.
    #[doc(alias = "VK_EXT_host_query_reset")]
    EXT_HostQueryReset,
    /// Promoted to [`Self::KHR_IndexTypeUint8`].
    #[doc(alias = "VK_EXT_index_type_uint8")]
    EXT_IndexTypeUint8,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_extended_dynamic_state")]
    EXT_ExtendedDynamicState,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_EXT_host_image_copy")]
    EXT_HostImageCopy,
    #[doc(alias = "VK_EXT_map_memory_placed")]
    EXT_MapMemoryPlaced,
    #[doc(alias = "VK_EXT_shader_atomic_float2")]
    EXT_ShaderAtomicFloat2,
    /// Promoted to [`Self::KHR_SurfaceMaintenance1`].
    #[doc(alias = "VK_EXT_surface_maintenance1")]
    EXT_SurfaceMaintenance1,
    /// Promoted to [`Self::KHR_SwapchainMaintenance1`].
    #[doc(alias = "VK_EXT_swapchain_maintenance1")]
    EXT_SwapchainMaintenance1,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_shader_demote_to_helper_invocation")]
    EXT_ShaderDemoteToHelperInvocation,
    #[doc(alias = "VK_NV_device_generated_commands")]
    NV_DeviceGeneratedCommands,
    #[doc(alias = "VK_NV_inherited_viewport_scissor")]
    NV_InheritedViewportScissor,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_texel_buffer_alignment")]
    EXT_TexelBufferAlignment,
    #[doc(alias = "VK_QCOM_render_pass_transform")]
    QCOM_RenderPassTransform,
    /// Intended for Direct3D emulation.
    #[doc(alias = "VK_EXT_depth_bias_control")]
    EXT_DepthBiasControl,
    /// Intended for development tooling.
    #[doc(alias = "VK_EXT_device_memory_report")]
    EXT_DeviceMemoryReport,
    #[doc(alias = "VK_EXT_acquire_drm_display")]
    EXT_AcquireDrmDisplay,
    /// Promoted to [`Self::KHR_Robustness2`].
    #[doc(alias = "VK_EXT_robustness2")]
    EXT_Robustness2,
    /// Intended for OpenGL emulation, Direct3D emulation.
    #[doc(alias = "VK_EXT_custom_border_color")]
    EXT_CustomBorderColor,
    #[doc(alias = "VK_EXT_texture_compression_astc_3d")]
    EXT_TextureCompressionAstc3D,
    #[doc(alias = "VK_GOOGLE_user_type")]
    GOOGLE_UserType,
    #[doc(alias = "VK_NV_present_barrier")]
    NV_PresentBarrier,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_private_data")]
    EXT_PrivateData,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_pipeline_creation_cache_control")]
    EXT_PipelineCreationCacheControl,
    #[doc(alias = "VK_NV_device_diagnostics_config")]
    NV_DeviceDiagnosticsConfig,
    #[doc(alias = "VK_QCOM_render_pass_store_ops")]
    QCOM_RenderPassStoreOps,
    #[doc(alias = "VK_QCOM_queue_perf_hint")]
    QCOM_QueuePerfHint,
    #[doc(alias = "VK_QCOM_image_processing3")]
    QCOM_ImageProcessing3,
    #[doc(alias = "VK_QCOM_shader_multiple_wait_queues")]
    QCOM_ShaderMultipleWaitQueues,
    #[doc(alias = "VK_EXT_shader_split_barrier")]
    EXT_ShaderSplitBarrier,
    #[doc(alias = "VK_NV_cuda_kernel_launch")]
    NV_CudaKernelLaunch,
    #[doc(alias = "VK_QCOM_tile_shading")]
    QCOM_TileShading,
    /// Deprecated by [`Self::NV_LowLatency2`].
    #[doc(alias = "VK_NV_low_latency")]
    NV_LowLatency,
    #[doc(alias = "VK_EXT_metal_objects")]
    EXT_MetalObjects,
    /// Deprecated by [`Self::EXT_DescriptorHeap`].
    #[doc(alias = "VK_EXT_descriptor_buffer")]
    EXT_DescriptorBuffer,
    #[doc(alias = "VK_EXT_graphics_pipeline_library")]
    EXT_GraphicsPipelineLibrary,
    #[doc(alias = "VK_AMD_shader_early_and_late_fragment_tests")]
    AMD_ShaderEarlyAndLateFragmentTests,
    #[doc(alias = "VK_NV_fragment_shading_rate_enums")]
    NV_FragmentShadingRateEnums,
    #[doc(alias = "VK_NV_ray_tracing_motion_blur")]
    NV_RayTracingMotionBlur,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_ycbcr_2plane_444_formats")]
    EXT_Ycbcr2Plane444Formats,
    #[doc(alias = "VK_EXT_fragment_density_map2")]
    EXT_FragmentDensityMap2,
    #[doc(alias = "VK_QCOM_rotated_copy_commands")]
    QCOM_RotatedCopyCommands,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_image_robustness")]
    EXT_ImageRobustness,
    #[doc(alias = "VK_EXT_image_compression_control")]
    EXT_ImageCompressionControl,
    /// Intended for OpenGL emulation, Direct3D emulation.
    #[doc(alias = "VK_EXT_attachment_feedback_loop_layout")]
    EXT_AttachmentFeedbackLoopLayout,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_4444_formats")]
    EXT_4444Formats,
    /// Promoted to [`Self::KHR_DeviceFault`].
    #[doc(alias = "VK_EXT_device_fault")]
    EXT_DeviceFault,
    /// Promoted to [`Self::EXT_RasterizationOrderAttachmentAccess`].
    #[doc(alias = "VK_ARM_rasterization_order_attachment_access")]
    ARM_RasterizationOrderAttachmentAccess,
    #[doc(alias = "VK_EXT_rgba10x6_formats")]
    EXT_Rgba10X6Formats,
    #[doc(alias = "VK_NV_acquire_winrt_display")]
    NV_AcquireWinrtDisplay,
    #[doc(alias = "VK_EXT_directfb_surface")]
    EXT_DirectfbSurface,
    /// Promoted to [`Self::EXT_MutableDescriptorType`].
    /// Intended for Direct3D emulation.
    #[doc(alias = "VK_VALVE_mutable_descriptor_type")]
    VALVE_MutableDescriptorType,
    #[doc(alias = "VK_EXT_vertex_input_dynamic_state")]
    EXT_VertexInputDynamicState,
    #[doc(alias = "VK_EXT_physical_device_drm")]
    EXT_PhysicalDeviceDrm,
    /// Intended for debugging, development tooling.
    #[doc(alias = "VK_EXT_device_address_binding_report")]
    EXT_DeviceAddressBindingReport,
    /// Intended for OpenGL emulation.
    #[doc(alias = "VK_EXT_depth_clip_control")]
    EXT_DepthClipControl,
    /// Intended for OpenGL emulation.
    #[doc(alias = "VK_EXT_primitive_topology_list_restart")]
    EXT_PrimitiveTopologyListRestart,
    /// Promoted to [`Self::KHR_PresentModeFifoLatestReady`].
    #[doc(alias = "VK_EXT_present_mode_fifo_latest_ready")]
    EXT_PresentModeFifoLatestReady,
    #[doc(alias = "VK_FUCHSIA_external_memory")]
    FUCHSIA_ExternalMemory,
    #[doc(alias = "VK_FUCHSIA_external_semaphore")]
    FUCHSIA_ExternalSemaphore,
    #[doc(alias = "VK_FUCHSIA_buffer_collection")]
    FUCHSIA_BufferCollection,
    #[doc(alias = "VK_HUAWEI_subpass_shading")]
    HUAWEI_SubpassShading,
    #[doc(alias = "VK_HUAWEI_invocation_mask")]
    HUAWEI_InvocationMask,
    #[doc(alias = "VK_NV_external_memory_rdma")]
    NV_ExternalMemoryRdma,
    #[doc(alias = "VK_EXT_pipeline_properties")]
    EXT_PipelineProperties,
    #[doc(alias = "VK_EXT_frame_boundary")]
    EXT_FrameBoundary,
    #[doc(alias = "VK_EXT_multisampled_render_to_single_sampled")]
    EXT_MultisampledRenderToSingleSampled,
    /// Promoted to core in version 1.3.
    #[doc(alias = "VK_EXT_extended_dynamic_state2")]
    EXT_ExtendedDynamicState2,
    #[doc(alias = "VK_QNX_screen_surface")]
    QNX_ScreenSurface,
    #[doc(alias = "VK_EXT_color_write_enable")]
    EXT_ColorWriteEnable,
    /// Intended for OpenGL emulation.
    #[doc(alias = "VK_EXT_primitives_generated_query")]
    EXT_PrimitivesGeneratedQuery,
    /// Promoted to [`Self::KHR_GlobalPriority`].
    #[doc(alias = "VK_EXT_global_priority_query")]
    EXT_GlobalPriorityQuery,
    #[doc(alias = "VK_VALVE_video_encode_rgb_conversion")]
    VALVE_VideoEncodeRgbConversion,
    #[doc(alias = "VK_EXT_image_view_min_lod")]
    EXT_ImageViewMinLod,
    #[doc(alias = "VK_EXT_multi_draw")]
    EXT_MultiDraw,
    /// Intended for OpenGL emulation.
    #[doc(alias = "VK_EXT_image_2d_view_of_3d")]
    EXT_Image2DViewOf3D,
    #[doc(alias = "VK_EXT_shader_tile_image")]
    EXT_ShaderTileImage,
    /// Promoted to [`Self::KHR_OpacityMicromap`].
    #[doc(alias = "VK_EXT_opacity_micromap")]
    EXT_OpacityMicromap,
    /// Deprecated by [`Self::NV_ClusterAccelerationStructure`].
    #[doc(alias = "VK_NV_displacement_micromap")]
    NV_DisplacementMicromap,
    /// Promoted to [`Self::KHR_LoadStoreOpNone`].
    #[doc(alias = "VK_EXT_load_store_op_none")]
    EXT_LoadStoreOpNone,
    #[doc(alias = "VK_HUAWEI_cluster_culling_shader")]
    HUAWEI_ClusterCullingShader,
    /// Intended for OpenGL emulation, Direct3D emulation.
    #[doc(alias = "VK_EXT_border_color_swizzle")]
    EXT_BorderColorSwizzle,
    #[doc(alias = "VK_EXT_pageable_device_local_memory")]
    EXT_PageableDeviceLocalMemory,
    #[doc(alias = "VK_ARM_shader_core_properties")]
    ARM_ShaderCoreProperties,
    #[doc(alias = "VK_ARM_scheduling_controls")]
    ARM_SchedulingControls,
    /// Intended for Direct3D emulation.
    #[doc(alias = "VK_EXT_image_sliced_view_of_3d")]
    EXT_ImageSlicedViewOf3D,
    /// Intended for Direct3D emulation.
    #[doc(alias = "VK_VALVE_descriptor_set_host_mapping")]
    VALVE_DescriptorSetHostMapping,
    /// Promoted to [`Self::KHR_DepthClampZeroOne`].
    #[doc(alias = "VK_EXT_depth_clamp_zero_one")]
    EXT_DepthClampZeroOne,
    /// Intended for Direct3D emulation, OpenGL emulation.
    #[doc(alias = "VK_EXT_non_seamless_cube_map")]
    EXT_NonSeamlessCubeMap,
    #[doc(alias = "VK_ARM_render_pass_striped")]
    ARM_RenderPassStriped,
    /// Promoted to [`Self::EXT_FragmentDensityMapOffset`].
    #[doc(alias = "VK_QCOM_fragment_density_map_offset")]
    QCOM_FragmentDensityMapOffset,
    /// Promoted to [`Self::KHR_CopyMemoryIndirect`].
    #[doc(alias = "VK_NV_copy_memory_indirect")]
    NV_CopyMemoryIndirect,
    /// Promoted to [`Self::EXT_MemoryDecompression`].
    #[doc(alias = "VK_NV_memory_decompression")]
    NV_MemoryDecompression,
    #[doc(alias = "VK_NV_device_generated_commands_compute")]
    NV_DeviceGeneratedCommandsCompute,
    #[doc(alias = "VK_NV_ray_tracing_linear_swept_spheres")]
    NV_RayTracingLinearSweptSpheres,
    #[doc(alias = "VK_NV_linear_color_attachment")]
    NV_LinearColorAttachment,
    /// Intended for OpenGL emulation.
    #[doc(alias = "VK_GOOGLE_surfaceless_query")]
    GOOGLE_SurfacelessQuery,
    #[doc(alias = "VK_EXT_image_compression_control_swapchain")]
    EXT_ImageCompressionControlSwapchain,
    #[doc(alias = "VK_QCOM_image_processing")]
    QCOM_ImageProcessing,
    #[doc(alias = "VK_EXT_nested_command_buffer")]
    EXT_NestedCommandBuffer,
    #[doc(alias = "VK_OHOS_external_memory")]
    OHOS_ExternalMemory,
    #[doc(alias = "VK_EXT_external_memory_acquire_unmodified")]
    EXT_ExternalMemoryAcquireUnmodified,
    #[doc(alias = "VK_EXT_extended_dynamic_state3")]
    EXT_ExtendedDynamicState3,
    #[doc(alias = "VK_EXT_subpass_merge_feedback")]
    EXT_SubpassMergeFeedback,
    #[doc(alias = "VK_LUNARG_direct_driver_loading")]
    LUNARG_DirectDriverLoading,
    #[doc(alias = "VK_ARM_tensors")]
    ARM_Tensors,
    #[doc(alias = "VK_EXT_shader_module_identifier")]
    EXT_ShaderModuleIdentifier,
    #[doc(alias = "VK_EXT_rasterization_order_attachment_access")]
    EXT_RasterizationOrderAttachmentAccess,
    #[doc(alias = "VK_NV_optical_flow")]
    NV_OpticalFlow,
    /// Intended for OpenGL emulation.
    #[doc(alias = "VK_EXT_legacy_dithering")]
    EXT_LegacyDithering,
    /// Promoted to core in version 1.4.
    #[doc(alias = "VK_EXT_pipeline_protected_access")]
    EXT_PipelineProtectedAccess,
    /// Intended for OpenGL emulation.
    #[doc(alias = "VK_ANDROID_external_format_resolve")]
    ANDROID_ExternalFormatResolve,
    #[doc(alias = "VK_AMD_anti_lag")]
    AMD_AntiLag,
    #[doc(alias = "VK_AMDX_dense_geometry_format")]
    AMDX_DenseGeometryFormat,
    #[doc(alias = "VK_EXT_shader_object")]
    EXT_ShaderObject,
    #[doc(alias = "VK_QCOM_tile_properties")]
    QCOM_TileProperties,
    #[doc(alias = "VK_SEC_amigo_profiling")]
    SEC_AmigoProfiling,
    #[doc(alias = "VK_QCOM_multiview_per_view_viewports")]
    QCOM_MultiviewPerViewViewports,
    /// Promoted to [`Self::EXT_RayTracingInvocationReorder`].
    #[doc(alias = "VK_NV_ray_tracing_invocation_reorder")]
    NV_RayTracingInvocationReorder,
    #[doc(alias = "VK_NV_cooperative_vector")]
    NV_CooperativeVector,
    #[doc(alias = "VK_NV_extended_sparse_address_space")]
    NV_ExtendedSparseAddressSpace,
    /// Intended for Direct3D emulation.
    #[doc(alias = "VK_EXT_mutable_descriptor_type")]
    EXT_MutableDescriptorType,
    /// Intended for OpenGL emulation.
    #[doc(alias = "VK_EXT_legacy_vertex_attributes")]
    EXT_LegacyVertexAttributes,
    #[doc(alias = "VK_EXT_layer_settings")]
    EXT_LayerSettings,
    #[doc(alias = "VK_ARM_shader_core_builtins")]
    ARM_ShaderCoreBuiltins,
    /// Promoted to [`Self::KHR_PipelineLibraryGroupHandles`].
    #[doc(alias = "VK_EXT_pipeline_library_group_handles")]
    EXT_PipelineLibraryGroupHandles,
    #[doc(alias = "VK_EXT_dynamic_rendering_unused_attachments")]
    EXT_DynamicRenderingUnusedAttachments,
    #[doc(alias = "VK_NV_low_latency2")]
    NV_LowLatency2,
    #[doc(alias = "VK_ARM_data_graph")]
    ARM_DataGraph,
    #[doc(alias = "VK_ARM_data_graph_instruction_set_tosa")]
    ARM_DataGraphInstructionSetTosa,
    #[doc(alias = "VK_QCOM_multiview_per_view_render_areas")]
    QCOM_MultiviewPerViewRenderAreas,
    /// Deprecated by [`Self::EXT_DescriptorHeap`].
    #[doc(alias = "VK_NV_per_stage_descriptor_set")]
    NV_PerStageDescriptorSet,
    #[doc(alias = "VK_QCOM_image_processing2")]
    QCOM_ImageProcessing2,
    #[doc(alias = "VK_QCOM_filter_cubic_weights")]
    QCOM_FilterCubicWeights,
    #[doc(alias = "VK_QCOM_ycbcr_degamma")]
    QCOM_YcbcrDegamma,
    #[doc(alias = "VK_QCOM_filter_cubic_clamp")]
    QCOM_FilterCubicClamp,
    /// Intended for OpenGL emulation, Direct3D emulation.
    #[doc(alias = "VK_EXT_attachment_feedback_loop_dynamic_state")]
    EXT_AttachmentFeedbackLoopDynamicState,
    #[doc(alias = "VK_QNX_external_memory_screen_buffer")]
    QNX_ExternalMemoryScreenBuffer,
    #[doc(alias = "VK_MSFT_layered_driver")]
    MSFT_LayeredDriver,
    #[doc(alias = "VK_NV_descriptor_pool_overallocation")]
    NV_DescriptorPoolOverallocation,
    #[doc(alias = "VK_QCOM_tile_memory_heap")]
    QCOM_TileMemoryHeap,
    #[doc(alias = "VK_EXT_memory_decompression")]
    EXT_MemoryDecompression,
    #[doc(alias = "VK_NV_display_stereo")]
    NV_DisplayStereo,
    #[doc(alias = "VK_NV_raw_access_chains")]
    NV_RawAccessChains,
    #[doc(alias = "VK_NV_external_compute_queue")]
    NV_ExternalComputeQueue,
    #[doc(alias = "VK_NV_command_buffer_inheritance")]
    NV_CommandBufferInheritance,
    #[doc(alias = "VK_NV_shader_atomic_float16_vector")]
    NV_ShaderAtomicFloat16Vector,
    #[doc(alias = "VK_EXT_shader_replicated_composites")]
    EXT_ShaderReplicatedComposites,
    #[doc(alias = "VK_ARM_tensor_controls")]
    ARM_TensorControls,
    #[doc(alias = "VK_EXT_shader_float8")]
    EXT_ShaderFloat8,
    #[doc(alias = "VK_NV_ray_tracing_validation")]
    NV_RayTracingValidation,
    #[doc(alias = "VK_NV_cluster_acceleration_structure")]
    NV_ClusterAccelerationStructure,
    #[doc(alias = "VK_NV_partitioned_acceleration_structure")]
    NV_PartitionedAccelerationStructure,
    #[doc(alias = "VK_EXT_device_generated_commands")]
    EXT_DeviceGeneratedCommands,
    /// Intended for Direct3D emulation.
    #[doc(alias = "VK_MESA_image_alignment_control")]
    MESA_ImageAlignmentControl,
    #[doc(alias = "VK_NV_push_constant_bank")]
    NV_PushConstantBank,
    #[doc(alias = "VK_EXT_ray_tracing_invocation_reorder")]
    EXT_RayTracingInvocationReorder,
    #[doc(alias = "VK_EXT_depth_clamp_control")]
    EXT_DepthClampControl,
    #[doc(alias = "VK_OHOS_surface")]
    OHOS_Surface,
    #[doc(alias = "VK_HUAWEI_hdr_vivid")]
    HUAWEI_HdrVivid,
    #[doc(alias = "VK_NV_cooperative_matrix2")]
    NV_CooperativeMatrix2,
    #[doc(alias = "VK_ARM_pipeline_opacity_micromap")]
    ARM_PipelineOpacityMicromap,
    #[doc(alias = "VK_IMG_filter_linear_2d")]
    IMG_FilterLinear2D,
    #[doc(alias = "VK_EXT_external_memory_metal")]
    EXT_ExternalMemoryMetal,
    /// Intended for development tooling.
    #[doc(alias = "VK_ARM_performance_counters_by_region")]
    ARM_PerformanceCountersByRegion,
    /// Intended for development tooling.
    #[doc(alias = "VK_ARM_shader_instrumentation")]
    ARM_ShaderInstrumentation,
    /// Promoted to [`Self::KHR_Maintenance9`].
    #[doc(alias = "VK_EXT_vertex_attribute_robustness")]
    EXT_VertexAttributeRobustness,
    #[doc(alias = "VK_ARM_format_pack")]
    ARM_FormatPack,
    #[doc(alias = "VK_VALVE_fragment_density_map_layered")]
    VALVE_FragmentDensityMapLayered,
    #[doc(alias = "VK_NV_present_metering")]
    NV_PresentMetering,
    #[doc(alias = "VK_EXT_multisampled_render_to_swapchain")]
    EXT_MultisampledRenderToSwapchain,
    #[doc(alias = "VK_EXT_fragment_density_map_offset")]
    EXT_FragmentDensityMapOffset,
    #[doc(alias = "VK_EXT_zero_initialize_device_memory")]
    EXT_ZeroInitializeDeviceMemory,
    #[doc(alias = "VK_EXT_shader_64bit_indexing")]
    EXT_Shader64BitIndexing,
    #[doc(alias = "VK_EXT_custom_resolve")]
    EXT_CustomResolve,
    #[doc(alias = "VK_QCOM_data_graph_model")]
    QCOM_DataGraphModel,
    #[doc(alias = "VK_ARM_data_graph_optical_flow")]
    ARM_DataGraphOpticalFlow,
    #[doc(alias = "VK_EXT_shader_long_vector")]
    EXT_ShaderLongVector,
    #[doc(alias = "VK_SEC_pipeline_cache_incremental_mode")]
    SEC_PipelineCacheIncrementalMode,
    #[doc(alias = "VK_EXT_shader_uniform_buffer_unsized_array")]
    EXT_ShaderUniformBufferUnsizedArray,
    #[doc(alias = "VK_NV_compute_occupancy_priority")]
    NV_ComputeOccupancyPriority,
    #[doc(alias = "VK_EXT_cooperative_matrix_maintenance1")]
    EXT_CooperativeMatrixMaintenance1,
    #[doc(alias = "VK_EXT_shader_subgroup_partitioned")]
    EXT_ShaderSubgroupPartitioned,
    #[doc(alias = "VK_SEC_ubm_surface")]
    SEC_UbmSurface,
    #[doc(alias = "VK_EXT_shader_ocp_microscaling_types")]
    EXT_ShaderOcpMicroscalingTypes,
    #[doc(alias = "VK_VALVE_shader_mixed_float_dot_product")]
    VALVE_ShaderMixedFloatDotProduct,
    #[doc(alias = "VK_SEC_throttle_hint")]
    SEC_ThrottleHint,
    #[doc(alias = "VK_ARM_data_graph_neural_accelerator_statistics")]
    ARM_DataGraphNeuralAcceleratorStatistics,
    /// Intended for OpenGL emulation.
    #[doc(alias = "VK_EXT_primitive_restart_index")]
    EXT_PrimitiveRestartIndex,
    #[doc(alias = "VK_EXT_image_tiling_control")]
    EXT_ImageTilingControl,
    #[doc(alias = "VK_NV_cooperative_matrix_decode_vector")]
    NV_CooperativeMatrixDecodeVector,
    #[doc(alias = "VK_NV_private_data_base_handle")]
    NV_PrivateDataBaseHandle,
    /// Intended for Direct3D emulation.
    #[doc(alias = "VK_VALVE_buffer_device_address_allocation_alignment")]
    VALVE_BufferDeviceAddressAllocationAlignment,
    #[doc(alias = "VK_KHR_acceleration_structure")]
    KHR_AccelerationStructure,
    #[doc(alias = "VK_KHR_ray_tracing_pipeline")]
    KHR_RayTracingPipeline,
    #[doc(alias = "VK_KHR_ray_query")]
    KHR_RayQuery,
    #[doc(alias = "VK_EXT_mesh_shader")]
    EXT_MeshShader,
}
impl Extensions {
    pub fn name(self) -> &'static std::ffi::CStr {
        match self {
            Self::KHR_Surface => c"VK_KHR_surface",
            Self::KHR_Swapchain => c"VK_KHR_swapchain",
            Self::KHR_Display => c"VK_KHR_display",
            Self::KHR_DisplaySwapchain => c"VK_KHR_display_swapchain",
            Self::KHR_XlibSurface => c"VK_KHR_xlib_surface",
            Self::KHR_XcbSurface => c"VK_KHR_xcb_surface",
            Self::KHR_WaylandSurface => c"VK_KHR_wayland_surface",
            Self::KHR_AndroidSurface => c"VK_KHR_android_surface",
            Self::KHR_Win32Surface => c"VK_KHR_win32_surface",
            Self::KHR_SamplerMirrorClampToEdge => c"VK_KHR_sampler_mirror_clamp_to_edge",
            Self::KHR_VideoQueue => c"VK_KHR_video_queue",
            Self::KHR_VideoDecodeQueue => c"VK_KHR_video_decode_queue",
            Self::KHR_VideoEncodeH264 => c"VK_KHR_video_encode_h264",
            Self::KHR_VideoEncodeH265 => c"VK_KHR_video_encode_h265",
            Self::KHR_VideoDecodeH264 => c"VK_KHR_video_decode_h264",
            Self::KHR_DynamicRendering => c"VK_KHR_dynamic_rendering",
            Self::KHR_Multiview => c"VK_KHR_multiview",
            Self::KHR_GetPhysicalDeviceProperties2 => c"VK_KHR_get_physical_device_properties2",
            Self::KHR_DeviceGroup => c"VK_KHR_device_group",
            Self::KHR_ShaderDrawParameters => c"VK_KHR_shader_draw_parameters",
            Self::KHR_Maintenance1 => c"VK_KHR_maintenance1",
            Self::KHR_DeviceGroupCreation => c"VK_KHR_device_group_creation",
            Self::KHR_ExternalMemoryCapabilities => c"VK_KHR_external_memory_capabilities",
            Self::KHR_ExternalMemory => c"VK_KHR_external_memory",
            Self::KHR_ExternalMemoryWin32 => c"VK_KHR_external_memory_win32",
            Self::KHR_ExternalMemoryFd => c"VK_KHR_external_memory_fd",
            Self::KHR_Win32KeyedMutex => c"VK_KHR_win32_keyed_mutex",
            Self::KHR_ExternalSemaphoreCapabilities => c"VK_KHR_external_semaphore_capabilities",
            Self::KHR_ExternalSemaphore => c"VK_KHR_external_semaphore",
            Self::KHR_ExternalSemaphoreWin32 => c"VK_KHR_external_semaphore_win32",
            Self::KHR_ExternalSemaphoreFd => c"VK_KHR_external_semaphore_fd",
            Self::KHR_PushDescriptor => c"VK_KHR_push_descriptor",
            Self::KHR_ShaderFloat16Int8 => c"VK_KHR_shader_float16_int8",
            Self::KHR_16BitStorage => c"VK_KHR_16bit_storage",
            Self::KHR_IncrementalPresent => c"VK_KHR_incremental_present",
            Self::KHR_DescriptorUpdateTemplate => c"VK_KHR_descriptor_update_template",
            Self::KHR_ImagelessFramebuffer => c"VK_KHR_imageless_framebuffer",
            Self::KHR_CreateRenderpass2 => c"VK_KHR_create_renderpass2",
            Self::KHR_SharedPresentableImage => c"VK_KHR_shared_presentable_image",
            Self::KHR_ExternalFenceCapabilities => c"VK_KHR_external_fence_capabilities",
            Self::KHR_ExternalFence => c"VK_KHR_external_fence",
            Self::KHR_ExternalFenceWin32 => c"VK_KHR_external_fence_win32",
            Self::KHR_ExternalFenceFd => c"VK_KHR_external_fence_fd",
            Self::KHR_PerformanceQuery => c"VK_KHR_performance_query",
            Self::KHR_Maintenance2 => c"VK_KHR_maintenance2",
            Self::KHR_GetSurfaceCapabilities2 => c"VK_KHR_get_surface_capabilities2",
            Self::KHR_VariablePointers => c"VK_KHR_variable_pointers",
            Self::KHR_GetDisplayProperties2 => c"VK_KHR_get_display_properties2",
            Self::KHR_DedicatedAllocation => c"VK_KHR_dedicated_allocation",
            Self::KHR_StorageBufferStorageClass => c"VK_KHR_storage_buffer_storage_class",
            Self::KHR_ShaderBfloat16 => c"VK_KHR_shader_bfloat16",
            Self::KHR_RelaxedBlockLayout => c"VK_KHR_relaxed_block_layout",
            Self::KHR_GetMemoryRequirements2 => c"VK_KHR_get_memory_requirements2",
            Self::KHR_ImageFormatList => c"VK_KHR_image_format_list",
            Self::KHR_SamplerYcbcrConversion => c"VK_KHR_sampler_ycbcr_conversion",
            Self::KHR_BindMemory2 => c"VK_KHR_bind_memory2",
            Self::KHR_PortabilitySubset => c"VK_KHR_portability_subset",
            Self::KHR_Maintenance3 => c"VK_KHR_maintenance3",
            Self::KHR_DrawIndirectCount => c"VK_KHR_draw_indirect_count",
            Self::KHR_ShaderSubgroupExtendedTypes => c"VK_KHR_shader_subgroup_extended_types",
            Self::KHR_8BitStorage => c"VK_KHR_8bit_storage",
            Self::KHR_ShaderAtomicInt64 => c"VK_KHR_shader_atomic_int64",
            Self::KHR_ShaderClock => c"VK_KHR_shader_clock",
            Self::KHR_VideoDecodeH265 => c"VK_KHR_video_decode_h265",
            Self::KHR_GlobalPriority => c"VK_KHR_global_priority",
            Self::KHR_DriverProperties => c"VK_KHR_driver_properties",
            Self::KHR_ShaderFloatControls => c"VK_KHR_shader_float_controls",
            Self::KHR_DepthStencilResolve => c"VK_KHR_depth_stencil_resolve",
            Self::KHR_SwapchainMutableFormat => c"VK_KHR_swapchain_mutable_format",
            Self::KHR_TimelineSemaphore => c"VK_KHR_timeline_semaphore",
            Self::KHR_VulkanMemoryModel => c"VK_KHR_vulkan_memory_model",
            Self::KHR_ShaderTerminateInvocation => c"VK_KHR_shader_terminate_invocation",
            Self::KHR_FragmentShadingRate => c"VK_KHR_fragment_shading_rate",
            Self::KHR_ShaderConstantData => c"VK_KHR_shader_constant_data",
            Self::KHR_DynamicRenderingLocalRead => c"VK_KHR_dynamic_rendering_local_read",
            Self::KHR_ShaderAbort => c"VK_KHR_shader_abort",
            Self::KHR_ShaderQuadControl => c"VK_KHR_shader_quad_control",
            Self::KHR_Spirv14 => c"VK_KHR_spirv_1_4",
            Self::KHR_SurfaceProtectedCapabilities => c"VK_KHR_surface_protected_capabilities",
            Self::KHR_SeparateDepthStencilLayouts => c"VK_KHR_separate_depth_stencil_layouts",
            Self::KHR_PresentWait => c"VK_KHR_present_wait",
            Self::KHR_UniformBufferStandardLayout => c"VK_KHR_uniform_buffer_standard_layout",
            Self::KHR_BufferDeviceAddress => c"VK_KHR_buffer_device_address",
            Self::KHR_DeferredHostOperations => c"VK_KHR_deferred_host_operations",
            Self::KHR_PipelineExecutableProperties => c"VK_KHR_pipeline_executable_properties",
            Self::KHR_MapMemory2 => c"VK_KHR_map_memory2",
            Self::KHR_ShaderIntegerDotProduct => c"VK_KHR_shader_integer_dot_product",
            Self::KHR_PipelineLibrary => c"VK_KHR_pipeline_library",
            Self::KHR_ShaderNonSemanticInfo => c"VK_KHR_shader_non_semantic_info",
            Self::KHR_PresentId => c"VK_KHR_present_id",
            Self::KHR_VideoEncodeQueue => c"VK_KHR_video_encode_queue",
            Self::KHR_Synchronization2 => c"VK_KHR_synchronization2",
            Self::KHR_DeviceAddressCommands => c"VK_KHR_device_address_commands",
            Self::KHR_FragmentShaderBarycentric => c"VK_KHR_fragment_shader_barycentric",
            Self::KHR_ShaderSubgroupUniformControlFlow => {
                c"VK_KHR_shader_subgroup_uniform_control_flow"
            }
            Self::KHR_ZeroInitializeWorkgroupMemory => c"VK_KHR_zero_initialize_workgroup_memory",
            Self::KHR_WorkgroupMemoryExplicitLayout => c"VK_KHR_workgroup_memory_explicit_layout",
            Self::KHR_CopyCommands2 => c"VK_KHR_copy_commands2",
            Self::KHR_FormatFeatureFlags2 => c"VK_KHR_format_feature_flags2",
            Self::KHR_RayTracingMaintenance1 => c"VK_KHR_ray_tracing_maintenance1",
            Self::KHR_ShaderUntypedPointers => c"VK_KHR_shader_untyped_pointers",
            Self::KHR_PortabilityEnumeration => c"VK_KHR_portability_enumeration",
            Self::KHR_Maintenance4 => c"VK_KHR_maintenance4",
            Self::KHR_ShaderSubgroupRotate => c"VK_KHR_shader_subgroup_rotate",
            Self::KHR_ShaderMaximalReconvergence => c"VK_KHR_shader_maximal_reconvergence",
            Self::KHR_Maintenance5 => c"VK_KHR_maintenance5",
            Self::KHR_PresentId2 => c"VK_KHR_present_id2",
            Self::KHR_PresentWait2 => c"VK_KHR_present_wait2",
            Self::KHR_RayTracingPositionFetch => c"VK_KHR_ray_tracing_position_fetch",
            Self::KHR_PipelineBinary => c"VK_KHR_pipeline_binary",
            Self::KHR_SurfaceMaintenance1 => c"VK_KHR_surface_maintenance1",
            Self::KHR_SwapchainMaintenance1 => c"VK_KHR_swapchain_maintenance1",
            Self::KHR_InternallySynchronizedQueues => c"VK_KHR_internally_synchronized_queues",
            Self::KHR_CooperativeMatrix => c"VK_KHR_cooperative_matrix",
            Self::KHR_ComputeShaderDerivatives => c"VK_KHR_compute_shader_derivatives",
            Self::KHR_VideoDecodeAv1 => c"VK_KHR_video_decode_av1",
            Self::KHR_VideoEncodeAv1 => c"VK_KHR_video_encode_av1",
            Self::KHR_VideoDecodeVp9 => c"VK_KHR_video_decode_vp9",
            Self::KHR_VideoMaintenance1 => c"VK_KHR_video_maintenance1",
            Self::KHR_VertexAttributeDivisor => c"VK_KHR_vertex_attribute_divisor",
            Self::KHR_LoadStoreOpNone => c"VK_KHR_load_store_op_none",
            Self::KHR_UnifiedImageLayouts => c"VK_KHR_unified_image_layouts",
            Self::KHR_ShaderFloatControls2 => c"VK_KHR_shader_float_controls2",
            Self::KHR_IndexTypeUint8 => c"VK_KHR_index_type_uint8",
            Self::KHR_LineRasterization => c"VK_KHR_line_rasterization",
            Self::KHR_CalibratedTimestamps => c"VK_KHR_calibrated_timestamps",
            Self::KHR_ShaderExpectAssume => c"VK_KHR_shader_expect_assume",
            Self::KHR_Maintenance6 => c"VK_KHR_maintenance6",
            Self::KHR_CopyMemoryIndirect => c"VK_KHR_copy_memory_indirect",
            Self::KHR_VideoEncodeIntraRefresh => c"VK_KHR_video_encode_intra_refresh",
            Self::KHR_VideoEncodeQuantizationMap => c"VK_KHR_video_encode_quantization_map",
            Self::KHR_ShaderRelaxedExtendedInstruction => {
                c"VK_KHR_shader_relaxed_extended_instruction"
            }
            Self::KHR_Maintenance7 => c"VK_KHR_maintenance7",
            Self::KHR_DeviceFault => c"VK_KHR_device_fault",
            Self::KHR_Maintenance8 => c"VK_KHR_maintenance8",
            Self::KHR_ShaderFma => c"VK_KHR_shader_fma",
            Self::KHR_Maintenance9 => c"VK_KHR_maintenance9",
            Self::KHR_VideoMaintenance2 => c"VK_KHR_video_maintenance2",
            Self::KHR_VideoEncodeFeedback2 => c"VK_KHR_video_encode_feedback2",
            Self::KHR_DepthClampZeroOne => c"VK_KHR_depth_clamp_zero_one",
            Self::KHR_Robustness2 => c"VK_KHR_robustness2",
            Self::KHR_PresentModeFifoLatestReady => c"VK_KHR_present_mode_fifo_latest_ready",
            Self::KHR_OpacityMicromap => c"VK_KHR_opacity_micromap",
            Self::KHR_Maintenance10 => c"VK_KHR_maintenance10",
            Self::KHR_PipelineLibraryGroupHandles => c"VK_KHR_pipeline_library_group_handles",
            Self::KHR_Maintenance11 => c"VK_KHR_maintenance11",
            Self::KHR_ExtendedFlags => c"VK_KHR_extended_flags",
            Self::EXT_DebugReport => c"VK_EXT_debug_report",
            Self::NV_GlslShader => c"VK_NV_glsl_shader",
            Self::EXT_DepthRangeUnrestricted => c"VK_EXT_depth_range_unrestricted",
            Self::IMG_FilterCubic => c"VK_IMG_filter_cubic",
            Self::AMD_RasterizationOrder => c"VK_AMD_rasterization_order",
            Self::AMD_ShaderTrinaryMinmax => c"VK_AMD_shader_trinary_minmax",
            Self::AMD_ShaderExplicitVertexParameter => c"VK_AMD_shader_explicit_vertex_parameter",
            Self::EXT_DebugMarker => c"VK_EXT_debug_marker",
            Self::AMD_GcnShader => c"VK_AMD_gcn_shader",
            Self::NV_DedicatedAllocation => c"VK_NV_dedicated_allocation",
            Self::EXT_TransformFeedback => c"VK_EXT_transform_feedback",
            Self::NVX_BinaryImport => c"VK_NVX_binary_import",
            Self::NVX_ImageViewHandle => c"VK_NVX_image_view_handle",
            Self::AMD_DrawIndirectCount => c"VK_AMD_draw_indirect_count",
            Self::AMD_NegativeViewportHeight => c"VK_AMD_negative_viewport_height",
            Self::AMD_GpuShaderHalfFloat => c"VK_AMD_gpu_shader_half_float",
            Self::AMD_ShaderBallot => c"VK_AMD_shader_ballot",
            Self::AMD_TextureGatherBiasLod => c"VK_AMD_texture_gather_bias_lod",
            Self::AMD_ShaderInfo => c"VK_AMD_shader_info",
            Self::AMD_ShaderImageLoadStoreLod => c"VK_AMD_shader_image_load_store_lod",
            Self::GGP_StreamDescriptorSurface => c"VK_GGP_stream_descriptor_surface",
            Self::NV_CornerSampledImage => c"VK_NV_corner_sampled_image",
            Self::IMG_FormatPvrtc => c"VK_IMG_format_pvrtc",
            Self::NV_ExternalMemoryCapabilities => c"VK_NV_external_memory_capabilities",
            Self::NV_ExternalMemory => c"VK_NV_external_memory",
            Self::NV_ExternalMemoryWin32 => c"VK_NV_external_memory_win32",
            Self::NV_Win32KeyedMutex => c"VK_NV_win32_keyed_mutex",
            Self::EXT_ValidationFlags => c"VK_EXT_validation_flags",
            Self::NN_ViSurface => c"VK_NN_vi_surface",
            Self::EXT_ShaderSubgroupBallot => c"VK_EXT_shader_subgroup_ballot",
            Self::EXT_ShaderSubgroupVote => c"VK_EXT_shader_subgroup_vote",
            Self::EXT_TextureCompressionAstcHdr => c"VK_EXT_texture_compression_astc_hdr",
            Self::EXT_AstcDecodeMode => c"VK_EXT_astc_decode_mode",
            Self::EXT_PipelineRobustness => c"VK_EXT_pipeline_robustness",
            Self::EXT_ConditionalRendering => c"VK_EXT_conditional_rendering",
            Self::NV_ClipSpaceWScaling => c"VK_NV_clip_space_w_scaling",
            Self::EXT_DirectModeDisplay => c"VK_EXT_direct_mode_display",
            Self::EXT_AcquireXlibDisplay => c"VK_EXT_acquire_xlib_display",
            Self::EXT_DisplaySurfaceCounter => c"VK_EXT_display_surface_counter",
            Self::EXT_DisplayControl => c"VK_EXT_display_control",
            Self::GOOGLE_DisplayTiming => c"VK_GOOGLE_display_timing",
            Self::NV_SampleMaskOverrideCoverage => c"VK_NV_sample_mask_override_coverage",
            Self::NV_GeometryShaderPassthrough => c"VK_NV_geometry_shader_passthrough",
            Self::NV_ViewportArray2 => c"VK_NV_viewport_array2",
            Self::NVX_MultiviewPerViewAttributes => c"VK_NVX_multiview_per_view_attributes",
            Self::NV_ViewportSwizzle => c"VK_NV_viewport_swizzle",
            Self::EXT_DiscardRectangles => c"VK_EXT_discard_rectangles",
            Self::EXT_ConservativeRasterization => c"VK_EXT_conservative_rasterization",
            Self::EXT_DepthClipEnable => c"VK_EXT_depth_clip_enable",
            Self::EXT_SwapchainColorspace => c"VK_EXT_swapchain_colorspace",
            Self::EXT_HdrMetadata => c"VK_EXT_hdr_metadata",
            Self::IMG_RelaxedLineRasterization => c"VK_IMG_relaxed_line_rasterization",
            Self::MVK_IosSurface => c"VK_MVK_ios_surface",
            Self::MVK_MacosSurface => c"VK_MVK_macos_surface",
            Self::EXT_ExternalMemoryDmaBuf => c"VK_EXT_external_memory_dma_buf",
            Self::EXT_QueueFamilyForeign => c"VK_EXT_queue_family_foreign",
            Self::EXT_DebugUtils => c"VK_EXT_debug_utils",
            Self::ANDROID_ExternalMemoryAndroidHardwareBuffer => {
                c"VK_ANDROID_external_memory_android_hardware_buffer"
            }
            Self::EXT_SamplerFilterMinmax => c"VK_EXT_sampler_filter_minmax",
            Self::AMD_GpuShaderInt16 => c"VK_AMD_gpu_shader_int16",
            Self::AMD_GpaInterface => c"VK_AMD_gpa_interface",
            Self::AMDX_ShaderEnqueue => c"VK_AMDX_shader_enqueue",
            Self::EXT_DescriptorHeap => c"VK_EXT_descriptor_heap",
            Self::AMD_MixedAttachmentSamples => c"VK_AMD_mixed_attachment_samples",
            Self::AMD_ShaderFragmentMask => c"VK_AMD_shader_fragment_mask",
            Self::EXT_InlineUniformBlock => c"VK_EXT_inline_uniform_block",
            Self::EXT_ShaderStencilExport => c"VK_EXT_shader_stencil_export",
            Self::EXT_SampleLocations => c"VK_EXT_sample_locations",
            Self::EXT_BlendOperationAdvanced => c"VK_EXT_blend_operation_advanced",
            Self::NV_FragmentCoverageToColor => c"VK_NV_fragment_coverage_to_color",
            Self::NV_FramebufferMixedSamples => c"VK_NV_framebuffer_mixed_samples",
            Self::NV_FillRectangle => c"VK_NV_fill_rectangle",
            Self::NV_ShaderSmBuiltins => c"VK_NV_shader_sm_builtins",
            Self::EXT_PostDepthCoverage => c"VK_EXT_post_depth_coverage",
            Self::EXT_ImageDrmFormatModifier => c"VK_EXT_image_drm_format_modifier",
            Self::EXT_ValidationCache => c"VK_EXT_validation_cache",
            Self::EXT_DescriptorIndexing => c"VK_EXT_descriptor_indexing",
            Self::EXT_ShaderViewportIndexLayer => c"VK_EXT_shader_viewport_index_layer",
            Self::NV_ShadingRateImage => c"VK_NV_shading_rate_image",
            Self::NV_RayTracing => c"VK_NV_ray_tracing",
            Self::NV_RepresentativeFragmentTest => c"VK_NV_representative_fragment_test",
            Self::EXT_FilterCubic => c"VK_EXT_filter_cubic",
            Self::QCOM_RenderPassShaderResolve => c"VK_QCOM_render_pass_shader_resolve",
            Self::QCOM_CooperativeMatrixConversion => c"VK_QCOM_cooperative_matrix_conversion",
            Self::QCOM_ElapsedTimerQuery => c"VK_QCOM_elapsed_timer_query",
            Self::EXT_GlobalPriority => c"VK_EXT_global_priority",
            Self::EXT_ExternalMemoryHost => c"VK_EXT_external_memory_host",
            Self::AMD_BufferMarker => c"VK_AMD_buffer_marker",
            Self::AMD_PipelineCompilerControl => c"VK_AMD_pipeline_compiler_control",
            Self::EXT_CalibratedTimestamps => c"VK_EXT_calibrated_timestamps",
            Self::AMD_ShaderCoreProperties => c"VK_AMD_shader_core_properties",
            Self::AMD_MemoryOverallocationBehavior => c"VK_AMD_memory_overallocation_behavior",
            Self::EXT_VertexAttributeDivisor => c"VK_EXT_vertex_attribute_divisor",
            Self::GGP_FrameToken => c"VK_GGP_frame_token",
            Self::EXT_PipelineCreationFeedback => c"VK_EXT_pipeline_creation_feedback",
            Self::NV_ShaderSubgroupPartitioned => c"VK_NV_shader_subgroup_partitioned",
            Self::NV_ComputeShaderDerivatives => c"VK_NV_compute_shader_derivatives",
            Self::NV_MeshShader => c"VK_NV_mesh_shader",
            Self::NV_FragmentShaderBarycentric => c"VK_NV_fragment_shader_barycentric",
            Self::NV_ShaderImageFootprint => c"VK_NV_shader_image_footprint",
            Self::NV_ScissorExclusive => c"VK_NV_scissor_exclusive",
            Self::NV_DeviceDiagnosticCheckpoints => c"VK_NV_device_diagnostic_checkpoints",
            Self::EXT_PresentTiming => c"VK_EXT_present_timing",
            Self::INTEL_ShaderIntegerFunctions2 => c"VK_INTEL_shader_integer_functions2",
            Self::INTEL_PerformanceQuery => c"VK_INTEL_performance_query",
            Self::EXT_PciBusInfo => c"VK_EXT_pci_bus_info",
            Self::AMD_DisplayNativeHdr => c"VK_AMD_display_native_hdr",
            Self::FUCHSIA_ImagepipeSurface => c"VK_FUCHSIA_imagepipe_surface",
            Self::EXT_MetalSurface => c"VK_EXT_metal_surface",
            Self::EXT_FragmentDensityMap => c"VK_EXT_fragment_density_map",
            Self::EXT_ScalarBlockLayout => c"VK_EXT_scalar_block_layout",
            Self::GOOGLE_HlslFunctionality1 => c"VK_GOOGLE_hlsl_functionality1",
            Self::GOOGLE_DecorateString => c"VK_GOOGLE_decorate_string",
            Self::EXT_SubgroupSizeControl => c"VK_EXT_subgroup_size_control",
            Self::AMD_ShaderCoreProperties2 => c"VK_AMD_shader_core_properties2",
            Self::AMD_DeviceCoherentMemory => c"VK_AMD_device_coherent_memory",
            Self::EXT_ShaderImageAtomicInt64 => c"VK_EXT_shader_image_atomic_int64",
            Self::EXT_MemoryBudget => c"VK_EXT_memory_budget",
            Self::EXT_MemoryPriority => c"VK_EXT_memory_priority",
            Self::NV_DedicatedAllocationImageAliasing => {
                c"VK_NV_dedicated_allocation_image_aliasing"
            }
            Self::EXT_BufferDeviceAddress => c"VK_EXT_buffer_device_address",
            Self::EXT_ToolingInfo => c"VK_EXT_tooling_info",
            Self::EXT_SeparateStencilUsage => c"VK_EXT_separate_stencil_usage",
            Self::EXT_ValidationFeatures => c"VK_EXT_validation_features",
            Self::NV_CooperativeMatrix => c"VK_NV_cooperative_matrix",
            Self::NV_CoverageReductionMode => c"VK_NV_coverage_reduction_mode",
            Self::EXT_FragmentShaderInterlock => c"VK_EXT_fragment_shader_interlock",
            Self::EXT_YcbcrImageArrays => c"VK_EXT_ycbcr_image_arrays",
            Self::EXT_ProvokingVertex => c"VK_EXT_provoking_vertex",
            Self::EXT_FullScreenExclusive => c"VK_EXT_full_screen_exclusive",
            Self::EXT_HeadlessSurface => c"VK_EXT_headless_surface",
            Self::EXT_LineRasterization => c"VK_EXT_line_rasterization",
            Self::EXT_ShaderAtomicFloat => c"VK_EXT_shader_atomic_float",
            Self::EXT_HostQueryReset => c"VK_EXT_host_query_reset",
            Self::EXT_IndexTypeUint8 => c"VK_EXT_index_type_uint8",
            Self::EXT_ExtendedDynamicState => c"VK_EXT_extended_dynamic_state",
            Self::EXT_HostImageCopy => c"VK_EXT_host_image_copy",
            Self::EXT_MapMemoryPlaced => c"VK_EXT_map_memory_placed",
            Self::EXT_ShaderAtomicFloat2 => c"VK_EXT_shader_atomic_float2",
            Self::EXT_SurfaceMaintenance1 => c"VK_EXT_surface_maintenance1",
            Self::EXT_SwapchainMaintenance1 => c"VK_EXT_swapchain_maintenance1",
            Self::EXT_ShaderDemoteToHelperInvocation => {
                c"VK_EXT_shader_demote_to_helper_invocation"
            }
            Self::NV_DeviceGeneratedCommands => c"VK_NV_device_generated_commands",
            Self::NV_InheritedViewportScissor => c"VK_NV_inherited_viewport_scissor",
            Self::EXT_TexelBufferAlignment => c"VK_EXT_texel_buffer_alignment",
            Self::QCOM_RenderPassTransform => c"VK_QCOM_render_pass_transform",
            Self::EXT_DepthBiasControl => c"VK_EXT_depth_bias_control",
            Self::EXT_DeviceMemoryReport => c"VK_EXT_device_memory_report",
            Self::EXT_AcquireDrmDisplay => c"VK_EXT_acquire_drm_display",
            Self::EXT_Robustness2 => c"VK_EXT_robustness2",
            Self::EXT_CustomBorderColor => c"VK_EXT_custom_border_color",
            Self::EXT_TextureCompressionAstc3D => c"VK_EXT_texture_compression_astc_3d",
            Self::GOOGLE_UserType => c"VK_GOOGLE_user_type",
            Self::NV_PresentBarrier => c"VK_NV_present_barrier",
            Self::EXT_PrivateData => c"VK_EXT_private_data",
            Self::EXT_PipelineCreationCacheControl => c"VK_EXT_pipeline_creation_cache_control",
            Self::NV_DeviceDiagnosticsConfig => c"VK_NV_device_diagnostics_config",
            Self::QCOM_RenderPassStoreOps => c"VK_QCOM_render_pass_store_ops",
            Self::QCOM_QueuePerfHint => c"VK_QCOM_queue_perf_hint",
            Self::QCOM_ImageProcessing3 => c"VK_QCOM_image_processing3",
            Self::QCOM_ShaderMultipleWaitQueues => c"VK_QCOM_shader_multiple_wait_queues",
            Self::EXT_ShaderSplitBarrier => c"VK_EXT_shader_split_barrier",
            Self::NV_CudaKernelLaunch => c"VK_NV_cuda_kernel_launch",
            Self::QCOM_TileShading => c"VK_QCOM_tile_shading",
            Self::NV_LowLatency => c"VK_NV_low_latency",
            Self::EXT_MetalObjects => c"VK_EXT_metal_objects",
            Self::EXT_DescriptorBuffer => c"VK_EXT_descriptor_buffer",
            Self::EXT_GraphicsPipelineLibrary => c"VK_EXT_graphics_pipeline_library",
            Self::AMD_ShaderEarlyAndLateFragmentTests => {
                c"VK_AMD_shader_early_and_late_fragment_tests"
            }
            Self::NV_FragmentShadingRateEnums => c"VK_NV_fragment_shading_rate_enums",
            Self::NV_RayTracingMotionBlur => c"VK_NV_ray_tracing_motion_blur",
            Self::EXT_Ycbcr2Plane444Formats => c"VK_EXT_ycbcr_2plane_444_formats",
            Self::EXT_FragmentDensityMap2 => c"VK_EXT_fragment_density_map2",
            Self::QCOM_RotatedCopyCommands => c"VK_QCOM_rotated_copy_commands",
            Self::EXT_ImageRobustness => c"VK_EXT_image_robustness",
            Self::EXT_ImageCompressionControl => c"VK_EXT_image_compression_control",
            Self::EXT_AttachmentFeedbackLoopLayout => c"VK_EXT_attachment_feedback_loop_layout",
            Self::EXT_4444Formats => c"VK_EXT_4444_formats",
            Self::EXT_DeviceFault => c"VK_EXT_device_fault",
            Self::ARM_RasterizationOrderAttachmentAccess => {
                c"VK_ARM_rasterization_order_attachment_access"
            }
            Self::EXT_Rgba10X6Formats => c"VK_EXT_rgba10x6_formats",
            Self::NV_AcquireWinrtDisplay => c"VK_NV_acquire_winrt_display",
            Self::EXT_DirectfbSurface => c"VK_EXT_directfb_surface",
            Self::VALVE_MutableDescriptorType => c"VK_VALVE_mutable_descriptor_type",
            Self::EXT_VertexInputDynamicState => c"VK_EXT_vertex_input_dynamic_state",
            Self::EXT_PhysicalDeviceDrm => c"VK_EXT_physical_device_drm",
            Self::EXT_DeviceAddressBindingReport => c"VK_EXT_device_address_binding_report",
            Self::EXT_DepthClipControl => c"VK_EXT_depth_clip_control",
            Self::EXT_PrimitiveTopologyListRestart => c"VK_EXT_primitive_topology_list_restart",
            Self::EXT_PresentModeFifoLatestReady => c"VK_EXT_present_mode_fifo_latest_ready",
            Self::FUCHSIA_ExternalMemory => c"VK_FUCHSIA_external_memory",
            Self::FUCHSIA_ExternalSemaphore => c"VK_FUCHSIA_external_semaphore",
            Self::FUCHSIA_BufferCollection => c"VK_FUCHSIA_buffer_collection",
            Self::HUAWEI_SubpassShading => c"VK_HUAWEI_subpass_shading",
            Self::HUAWEI_InvocationMask => c"VK_HUAWEI_invocation_mask",
            Self::NV_ExternalMemoryRdma => c"VK_NV_external_memory_rdma",
            Self::EXT_PipelineProperties => c"VK_EXT_pipeline_properties",
            Self::EXT_FrameBoundary => c"VK_EXT_frame_boundary",
            Self::EXT_MultisampledRenderToSingleSampled => {
                c"VK_EXT_multisampled_render_to_single_sampled"
            }
            Self::EXT_ExtendedDynamicState2 => c"VK_EXT_extended_dynamic_state2",
            Self::QNX_ScreenSurface => c"VK_QNX_screen_surface",
            Self::EXT_ColorWriteEnable => c"VK_EXT_color_write_enable",
            Self::EXT_PrimitivesGeneratedQuery => c"VK_EXT_primitives_generated_query",
            Self::EXT_GlobalPriorityQuery => c"VK_EXT_global_priority_query",
            Self::VALVE_VideoEncodeRgbConversion => c"VK_VALVE_video_encode_rgb_conversion",
            Self::EXT_ImageViewMinLod => c"VK_EXT_image_view_min_lod",
            Self::EXT_MultiDraw => c"VK_EXT_multi_draw",
            Self::EXT_Image2DViewOf3D => c"VK_EXT_image_2d_view_of_3d",
            Self::EXT_ShaderTileImage => c"VK_EXT_shader_tile_image",
            Self::EXT_OpacityMicromap => c"VK_EXT_opacity_micromap",
            Self::NV_DisplacementMicromap => c"VK_NV_displacement_micromap",
            Self::EXT_LoadStoreOpNone => c"VK_EXT_load_store_op_none",
            Self::HUAWEI_ClusterCullingShader => c"VK_HUAWEI_cluster_culling_shader",
            Self::EXT_BorderColorSwizzle => c"VK_EXT_border_color_swizzle",
            Self::EXT_PageableDeviceLocalMemory => c"VK_EXT_pageable_device_local_memory",
            Self::ARM_ShaderCoreProperties => c"VK_ARM_shader_core_properties",
            Self::ARM_SchedulingControls => c"VK_ARM_scheduling_controls",
            Self::EXT_ImageSlicedViewOf3D => c"VK_EXT_image_sliced_view_of_3d",
            Self::VALVE_DescriptorSetHostMapping => c"VK_VALVE_descriptor_set_host_mapping",
            Self::EXT_DepthClampZeroOne => c"VK_EXT_depth_clamp_zero_one",
            Self::EXT_NonSeamlessCubeMap => c"VK_EXT_non_seamless_cube_map",
            Self::ARM_RenderPassStriped => c"VK_ARM_render_pass_striped",
            Self::QCOM_FragmentDensityMapOffset => c"VK_QCOM_fragment_density_map_offset",
            Self::NV_CopyMemoryIndirect => c"VK_NV_copy_memory_indirect",
            Self::NV_MemoryDecompression => c"VK_NV_memory_decompression",
            Self::NV_DeviceGeneratedCommandsCompute => c"VK_NV_device_generated_commands_compute",
            Self::NV_RayTracingLinearSweptSpheres => c"VK_NV_ray_tracing_linear_swept_spheres",
            Self::NV_LinearColorAttachment => c"VK_NV_linear_color_attachment",
            Self::GOOGLE_SurfacelessQuery => c"VK_GOOGLE_surfaceless_query",
            Self::EXT_ImageCompressionControlSwapchain => {
                c"VK_EXT_image_compression_control_swapchain"
            }
            Self::QCOM_ImageProcessing => c"VK_QCOM_image_processing",
            Self::EXT_NestedCommandBuffer => c"VK_EXT_nested_command_buffer",
            Self::OHOS_ExternalMemory => c"VK_OHOS_external_memory",
            Self::EXT_ExternalMemoryAcquireUnmodified => {
                c"VK_EXT_external_memory_acquire_unmodified"
            }
            Self::EXT_ExtendedDynamicState3 => c"VK_EXT_extended_dynamic_state3",
            Self::EXT_SubpassMergeFeedback => c"VK_EXT_subpass_merge_feedback",
            Self::LUNARG_DirectDriverLoading => c"VK_LUNARG_direct_driver_loading",
            Self::ARM_Tensors => c"VK_ARM_tensors",
            Self::EXT_ShaderModuleIdentifier => c"VK_EXT_shader_module_identifier",
            Self::EXT_RasterizationOrderAttachmentAccess => {
                c"VK_EXT_rasterization_order_attachment_access"
            }
            Self::NV_OpticalFlow => c"VK_NV_optical_flow",
            Self::EXT_LegacyDithering => c"VK_EXT_legacy_dithering",
            Self::EXT_PipelineProtectedAccess => c"VK_EXT_pipeline_protected_access",
            Self::ANDROID_ExternalFormatResolve => c"VK_ANDROID_external_format_resolve",
            Self::AMD_AntiLag => c"VK_AMD_anti_lag",
            Self::AMDX_DenseGeometryFormat => c"VK_AMDX_dense_geometry_format",
            Self::EXT_ShaderObject => c"VK_EXT_shader_object",
            Self::QCOM_TileProperties => c"VK_QCOM_tile_properties",
            Self::SEC_AmigoProfiling => c"VK_SEC_amigo_profiling",
            Self::QCOM_MultiviewPerViewViewports => c"VK_QCOM_multiview_per_view_viewports",
            Self::NV_RayTracingInvocationReorder => c"VK_NV_ray_tracing_invocation_reorder",
            Self::NV_CooperativeVector => c"VK_NV_cooperative_vector",
            Self::NV_ExtendedSparseAddressSpace => c"VK_NV_extended_sparse_address_space",
            Self::EXT_MutableDescriptorType => c"VK_EXT_mutable_descriptor_type",
            Self::EXT_LegacyVertexAttributes => c"VK_EXT_legacy_vertex_attributes",
            Self::EXT_LayerSettings => c"VK_EXT_layer_settings",
            Self::ARM_ShaderCoreBuiltins => c"VK_ARM_shader_core_builtins",
            Self::EXT_PipelineLibraryGroupHandles => c"VK_EXT_pipeline_library_group_handles",
            Self::EXT_DynamicRenderingUnusedAttachments => {
                c"VK_EXT_dynamic_rendering_unused_attachments"
            }
            Self::NV_LowLatency2 => c"VK_NV_low_latency2",
            Self::ARM_DataGraph => c"VK_ARM_data_graph",
            Self::ARM_DataGraphInstructionSetTosa => c"VK_ARM_data_graph_instruction_set_tosa",
            Self::QCOM_MultiviewPerViewRenderAreas => c"VK_QCOM_multiview_per_view_render_areas",
            Self::NV_PerStageDescriptorSet => c"VK_NV_per_stage_descriptor_set",
            Self::QCOM_ImageProcessing2 => c"VK_QCOM_image_processing2",
            Self::QCOM_FilterCubicWeights => c"VK_QCOM_filter_cubic_weights",
            Self::QCOM_YcbcrDegamma => c"VK_QCOM_ycbcr_degamma",
            Self::QCOM_FilterCubicClamp => c"VK_QCOM_filter_cubic_clamp",
            Self::EXT_AttachmentFeedbackLoopDynamicState => {
                c"VK_EXT_attachment_feedback_loop_dynamic_state"
            }
            Self::QNX_ExternalMemoryScreenBuffer => c"VK_QNX_external_memory_screen_buffer",
            Self::MSFT_LayeredDriver => c"VK_MSFT_layered_driver",
            Self::NV_DescriptorPoolOverallocation => c"VK_NV_descriptor_pool_overallocation",
            Self::QCOM_TileMemoryHeap => c"VK_QCOM_tile_memory_heap",
            Self::EXT_MemoryDecompression => c"VK_EXT_memory_decompression",
            Self::NV_DisplayStereo => c"VK_NV_display_stereo",
            Self::NV_RawAccessChains => c"VK_NV_raw_access_chains",
            Self::NV_ExternalComputeQueue => c"VK_NV_external_compute_queue",
            Self::NV_CommandBufferInheritance => c"VK_NV_command_buffer_inheritance",
            Self::NV_ShaderAtomicFloat16Vector => c"VK_NV_shader_atomic_float16_vector",
            Self::EXT_ShaderReplicatedComposites => c"VK_EXT_shader_replicated_composites",
            Self::ARM_TensorControls => c"VK_ARM_tensor_controls",
            Self::EXT_ShaderFloat8 => c"VK_EXT_shader_float8",
            Self::NV_RayTracingValidation => c"VK_NV_ray_tracing_validation",
            Self::NV_ClusterAccelerationStructure => c"VK_NV_cluster_acceleration_structure",
            Self::NV_PartitionedAccelerationStructure => {
                c"VK_NV_partitioned_acceleration_structure"
            }
            Self::EXT_DeviceGeneratedCommands => c"VK_EXT_device_generated_commands",
            Self::MESA_ImageAlignmentControl => c"VK_MESA_image_alignment_control",
            Self::NV_PushConstantBank => c"VK_NV_push_constant_bank",
            Self::EXT_RayTracingInvocationReorder => c"VK_EXT_ray_tracing_invocation_reorder",
            Self::EXT_DepthClampControl => c"VK_EXT_depth_clamp_control",
            Self::OHOS_Surface => c"VK_OHOS_surface",
            Self::HUAWEI_HdrVivid => c"VK_HUAWEI_hdr_vivid",
            Self::NV_CooperativeMatrix2 => c"VK_NV_cooperative_matrix2",
            Self::ARM_PipelineOpacityMicromap => c"VK_ARM_pipeline_opacity_micromap",
            Self::IMG_FilterLinear2D => c"VK_IMG_filter_linear_2d",
            Self::EXT_ExternalMemoryMetal => c"VK_EXT_external_memory_metal",
            Self::ARM_PerformanceCountersByRegion => c"VK_ARM_performance_counters_by_region",
            Self::ARM_ShaderInstrumentation => c"VK_ARM_shader_instrumentation",
            Self::EXT_VertexAttributeRobustness => c"VK_EXT_vertex_attribute_robustness",
            Self::ARM_FormatPack => c"VK_ARM_format_pack",
            Self::VALVE_FragmentDensityMapLayered => c"VK_VALVE_fragment_density_map_layered",
            Self::NV_PresentMetering => c"VK_NV_present_metering",
            Self::EXT_MultisampledRenderToSwapchain => c"VK_EXT_multisampled_render_to_swapchain",
            Self::EXT_FragmentDensityMapOffset => c"VK_EXT_fragment_density_map_offset",
            Self::EXT_ZeroInitializeDeviceMemory => c"VK_EXT_zero_initialize_device_memory",
            Self::EXT_Shader64BitIndexing => c"VK_EXT_shader_64bit_indexing",
            Self::EXT_CustomResolve => c"VK_EXT_custom_resolve",
            Self::QCOM_DataGraphModel => c"VK_QCOM_data_graph_model",
            Self::ARM_DataGraphOpticalFlow => c"VK_ARM_data_graph_optical_flow",
            Self::EXT_ShaderLongVector => c"VK_EXT_shader_long_vector",
            Self::SEC_PipelineCacheIncrementalMode => c"VK_SEC_pipeline_cache_incremental_mode",
            Self::EXT_ShaderUniformBufferUnsizedArray => {
                c"VK_EXT_shader_uniform_buffer_unsized_array"
            }
            Self::NV_ComputeOccupancyPriority => c"VK_NV_compute_occupancy_priority",
            Self::EXT_CooperativeMatrixMaintenance1 => c"VK_EXT_cooperative_matrix_maintenance1",
            Self::EXT_ShaderSubgroupPartitioned => c"VK_EXT_shader_subgroup_partitioned",
            Self::SEC_UbmSurface => c"VK_SEC_ubm_surface",
            Self::EXT_ShaderOcpMicroscalingTypes => c"VK_EXT_shader_ocp_microscaling_types",
            Self::VALVE_ShaderMixedFloatDotProduct => c"VK_VALVE_shader_mixed_float_dot_product",
            Self::SEC_ThrottleHint => c"VK_SEC_throttle_hint",
            Self::ARM_DataGraphNeuralAcceleratorStatistics => {
                c"VK_ARM_data_graph_neural_accelerator_statistics"
            }
            Self::EXT_PrimitiveRestartIndex => c"VK_EXT_primitive_restart_index",
            Self::EXT_ImageTilingControl => c"VK_EXT_image_tiling_control",
            Self::NV_CooperativeMatrixDecodeVector => c"VK_NV_cooperative_matrix_decode_vector",
            Self::NV_PrivateDataBaseHandle => c"VK_NV_private_data_base_handle",
            Self::VALVE_BufferDeviceAddressAllocationAlignment => {
                c"VK_VALVE_buffer_device_address_allocation_alignment"
            }
            Self::KHR_AccelerationStructure => c"VK_KHR_acceleration_structure",
            Self::KHR_RayTracingPipeline => c"VK_KHR_ray_tracing_pipeline",
            Self::KHR_RayQuery => c"VK_KHR_ray_query",
            Self::EXT_MeshShader => c"VK_EXT_mesh_shader",
        }
    }
}
