// WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]

use crate::bitmasks::*;
use crate::enums::*;
use crate::flags::*;
use crate::handles::*;
use crate::inner::*;
use crate::platform::*;
use crate::structs::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

/// <https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkAllocationFunction.html>
pub type vkAllocationFunction =
    unsafe extern "C" fn(*mut c_void, usize, usize, SystemAllocationScope) -> *mut c_void;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkFreeFunction.html>
pub type vkFreeFunction = unsafe extern "C" fn(*mut c_void, *mut c_void);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalAllocationNotification.html>
pub type vkInternalAllocationNotification =
    unsafe extern "C" fn(*mut c_void, usize, InternalAllocationType, SystemAllocationScope);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalFreeNotification.html>
pub type vkInternalFreeNotification =
    unsafe extern "C" fn(*mut c_void, usize, InternalAllocationType, SystemAllocationScope);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkReallocationFunction.html>
pub type vkReallocationFunction = unsafe extern "C" fn(
    *mut c_void,
    *mut c_void,
    usize,
    usize,
    SystemAllocationScope,
) -> *mut c_void;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkVoidFunction.html>
pub type vkVoidFunction = unsafe extern "C" fn();

/// <https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDebugReportCallbackEXT.html>
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

/// <https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDebugUtilsMessengerCallbackEXT.html>
pub type vkDebugUtilsMessengerCallbackEXT = unsafe extern "C" fn(
    DebugUtilsMessageSeverityFlagsEXT,
    DebugUtilsMessageTypeFlagsEXT,
    *const DebugUtilsMessengerCallbackDataEXT,
    *mut c_void,
) -> Bool32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDeviceMemoryReportCallbackEXT.html>
pub type vkDeviceMemoryReportCallbackEXT =
    unsafe extern "C" fn(*const DeviceMemoryReportCallbackDataEXT, *mut c_void);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetInstanceProcAddrLUNARG.html>
pub type vkGetInstanceProcAddrLUNARG =
    unsafe extern "C" fn(Instance, *const c_char) -> vkVoidFunction;
