// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::loader::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::bitmasks::*;
use crate::consts_inner::*;
use crate::enums::*;
use crate::flags::*;
use crate::handles::*;
use crate::internal::*;
use crate::structs::*;

/// [`PFN_vkAllocationFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkAllocationFunction.html)
///
#[doc(alias = "PFN_vkAllocationFunction")]
pub type vkAllocationFunction =
    unsafe extern "C" fn(*mut c_void, usize, usize, SystemAllocationScope) -> *mut c_void;

/// [`PFN_vkFreeFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkFreeFunction.html)
///
#[doc(alias = "PFN_vkFreeFunction")]
pub type vkFreeFunction = unsafe extern "C" fn(*mut c_void, *mut c_void);

/// [`PFN_vkInternalAllocationNotification`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalAllocationNotification.html)
///
#[doc(alias = "PFN_vkInternalAllocationNotification")]
pub type vkInternalAllocationNotification =
    unsafe extern "C" fn(*mut c_void, usize, InternalAllocationType, SystemAllocationScope);

/// [`PFN_vkInternalFreeNotification`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalFreeNotification.html)
///
#[doc(alias = "PFN_vkInternalFreeNotification")]
pub type vkInternalFreeNotification =
    unsafe extern "C" fn(*mut c_void, usize, InternalAllocationType, SystemAllocationScope);

/// [`PFN_vkReallocationFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkReallocationFunction.html)
///
#[doc(alias = "PFN_vkReallocationFunction")]
pub type vkReallocationFunction = unsafe extern "C" fn(
    *mut c_void,
    *mut c_void,
    usize,
    usize,
    SystemAllocationScope,
) -> *mut c_void;

/// [`PFN_vkVoidFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkVoidFunction.html)
///
#[doc(alias = "PFN_vkVoidFunction")]
pub type vkVoidFunction = unsafe extern "C" fn();

/// [`PFN_vkDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDebugReportCallbackEXT.html)
///
#[doc(alias = "PFN_vkDebugReportCallbackEXT")]
pub type vkDebugReportCallbackEXT = unsafe extern "C" fn(
    DebugReportFlagsEXT,
    DebugReportObjectTypeEXT,
    u64,
    usize,
    i32,
    *const c_char,
    *const c_char,
    *mut c_void,
) -> Bool32;

/// [`PFN_vkDebugUtilsMessengerCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDebugUtilsMessengerCallbackEXT.html)
///
#[doc(alias = "PFN_vkDebugUtilsMessengerCallbackEXT")]
pub type vkDebugUtilsMessengerCallbackEXT = unsafe extern "C" fn(
    DebugUtilsMessageSeverityFlagsEXT,
    DebugUtilsMessageTypeFlagsEXT,
    *const DebugUtilsMessengerCallbackDataEXT,
    *mut c_void,
) -> Bool32;

/// [`PFN_vkDeviceMemoryReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDeviceMemoryReportCallbackEXT.html)
///
#[doc(alias = "PFN_vkDeviceMemoryReportCallbackEXT")]
pub type vkDeviceMemoryReportCallbackEXT =
    unsafe extern "C" fn(*const DeviceMemoryReportCallbackDataEXT, *mut c_void);

/// [`PFN_vkGetInstanceProcAddrLUNARG`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetInstanceProcAddrLUNARG.html)
///
#[doc(alias = "PFN_vkGetInstanceProcAddrLUNARG")]
pub type vkGetInstanceProcAddrLUNARG =
    unsafe extern "C" fn(InstanceHandle, *const c_char) -> vkVoidFunction;
