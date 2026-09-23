// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(clippy::all)]

use crate::loader::*;
use crate::platform::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

use crate::consts_inner::*;
use crate::enums::*;
use crate::flags::*;
use crate::handles::*;
use crate::internal::*;
use crate::structs::*;

/// [`PFN_vkAllocationFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkAllocationFunction.html)
///
#[doc(alias = "PFN_vkAllocationFunction")]
pub type FnAllocationFunction =
    unsafe extern "C" fn(*mut c_void, usize, usize, SystemAllocationScope) -> *mut c_void;

/// [`PFN_vkFreeFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkFreeFunction.html)
///
#[doc(alias = "PFN_vkFreeFunction")]
pub type FnFreeFunction = unsafe extern "C" fn(*mut c_void, *mut c_void);

/// [`PFN_vkInternalAllocationNotification`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalAllocationNotification.html)
///
#[doc(alias = "PFN_vkInternalAllocationNotification")]
pub type FnInternalAllocationNotification =
    unsafe extern "C" fn(*mut c_void, usize, InternalAllocationType, SystemAllocationScope);

/// [`PFN_vkInternalFreeNotification`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalFreeNotification.html)
///
#[doc(alias = "PFN_vkInternalFreeNotification")]
pub type FnInternalFreeNotification =
    unsafe extern "C" fn(*mut c_void, usize, InternalAllocationType, SystemAllocationScope);

/// [`PFN_vkReallocationFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkReallocationFunction.html)
///
#[doc(alias = "PFN_vkReallocationFunction")]
pub type FnReallocationFunction = unsafe extern "C" fn(
    *mut c_void,
    *mut c_void,
    usize,
    usize,
    SystemAllocationScope,
) -> *mut c_void;

/// [`PFN_vkVoidFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkVoidFunction.html)
///
#[doc(alias = "PFN_vkVoidFunction")]
pub type FnVoidFunction = unsafe extern "C" fn();

/// [`PFN_vkDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDebugReportCallbackEXT.html)
///
#[doc(alias = "PFN_vkDebugReportCallbackEXT")]
pub type FnDebugReportCallbackEXT = unsafe extern "C" fn(
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
pub type FnDebugUtilsMessengerCallbackEXT = unsafe extern "C" fn(
    DebugUtilsMessageSeverityFlagsEXT,
    DebugUtilsMessageTypeFlagsEXT,
    *const DebugUtilsMessengerCallbackDataEXT,
    *mut c_void,
) -> Bool32;

/// [`PFN_vkDeviceMemoryReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDeviceMemoryReportCallbackEXT.html)
///
#[doc(alias = "PFN_vkDeviceMemoryReportCallbackEXT")]
pub type FnDeviceMemoryReportCallbackEXT =
    unsafe extern "C" fn(*const DeviceMemoryReportCallbackDataEXT, *mut c_void);

/// [`PFN_vkGetInstanceProcAddrLUNARG`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetInstanceProcAddrLUNARG.html)
///
#[doc(alias = "PFN_vkGetInstanceProcAddrLUNARG")]
pub type FnGetInstanceProcAddrLUNARG =
    unsafe extern "C" fn(InstanceHandle, *const c_char) -> FnVoidFunction;
