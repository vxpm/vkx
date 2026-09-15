import os
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import TypeVar

import textcase
from vulkan_object import get_vulkan_object
from vulkan_object import vulkan_object as vkobj

from .rust_types import RustPointer, RustType

MODULE_PREFIX: str = """ // WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::{c_void, c_int, c_uint, c_char};
use crate::manual::*;
use crate::platform::*;
"""

FN_PTRS_MODULE_PREFIX: str = """
use crate::bitmasks::*;
use crate::consts_inner::*;
use crate::enums::*;
use crate::extensions::*;
use crate::flags::*;
use crate::handles::*;
use crate::structs::*;
"""

STRUCTS_MODULE_PREFIX: str = """
use crate::bitmasks::*;
use crate::consts_inner::*;
use crate::enums::*;
use crate::extensions::*;
use crate::flags::*;
use crate::fn_ptrs::*;
use crate::handles::*;
"""

CONSTS_MODULE_PREFIX: str = """
use crate::consts_inner::*;
"""

COMMANDS_MODULE_PREFIX: str = """
use crate::bitmasks::*;
use crate::consts_inner::*;
use crate::enums::*;
use crate::extensions::*;
use crate::flags::*;
use crate::fn_ptrs::*;
use crate::handles::*;
use crate::structs::*;
"""


class CodeWriter:
    content: str = ""
    newline: bool = True
    level: int = 0

    def indent(self):
        self.level += 1

    def deindent(self):
        assert self.level != 0
        self.level -= 1

    def write_indent(self):
        if self.newline:
            self.content += "    " * self.level
            self.newline = False

    def write(self, value: str):
        self.write_indent()
        self.content += value

    def writeln(self, value: str):
        self.write(value + "\n")
        self.newline = True


T = TypeVar("T")


@dataclass
class Generated[T]:
    name: str
    definition: str
    original: T


@dataclass
class Registry:
    bitmasks: list[Generated[vkobj.Bitmask]] = field(default_factory=list)
    constants: list[Generated[vkobj.Constant]] = field(default_factory=list)
    constants_inner: list[Generated[vkobj.Constant]] = field(default_factory=list)
    enums: list[Generated[vkobj.Enum]] = field(default_factory=list)
    flags: list[Generated[vkobj.Flags]] = field(default_factory=list)
    fnptrs: list[Generated[vkobj.FuncPointer]] = field(default_factory=list)
    handles: list[Generated[vkobj.Handle]] = field(default_factory=list)
    structs: list[Generated[vkobj.Struct]] = field(default_factory=list)
    commands: list[Generated[vkobj.Command]] = field(default_factory=list)
    extensions: str = field(default_factory=lambda: "")


def struct_name(name: str) -> str:
    return name.removeprefix("Vk").removeprefix("StdVideo")


def bitmask_flag_name(name: str, parent: str) -> str:
    name = name.removeprefix("VK_").removeprefix(f"{parent}_")
    if name[0].isdigit():
        name = f"_{name}"

    if name.endswith("_BIT"):
        name = name.removesuffix("_BIT")
    else:
        name = name.replace("_BIT_", "")

    return name


def extension_name(name: str) -> str:
    name = name.removeprefix("VK_")
    tag, name = name.split("_", maxsplit=1)
    return f"{tag}_{textcase.pascal(name)}"


def version_number(ver: str) -> str:
    return ver.removeprefix("VK_VERSION_").replace("_", ".")


def vulkan_doc_header(out: CodeWriter, name: str):
    out.writeln(f"/// `{name}`")
    out.writeln("///")
    out.writeln("/// # Vulkan documentation")
    out.writeln(
        f"/// <https://docs.vulkan.org/refpages/latest/refpages/source/{name}.html>"
    )


def command_doc_header(out: CodeWriter, command: vkobj.Command):
    if len(command.tasks) > 0:
        out.writeln("/// ## Performed tasks")
        for task in command.tasks:
            out.writeln(f"/// - `{task}`")

        out.writeln("///")

    if command.primary or command.secondary:
        out.writeln("/// ## Allowed command buffers")

        if command.primary:
            out.writeln("/// - Primary")

        if command.secondary:
            out.writeln("/// - Secondary")

        out.writeln("///")

    if len(command.queues) > 0:
        out.writeln("/// ## Allowed queues")
        for queue in command.queues:
            queue = bitmask_flag_name(queue, "QUEUE")
            out.writeln(f"/// - [`{queue}`](QueueFlags::{queue})")

        out.writeln("///")

    if len(command.successCodes) > 0 or len(command.errorCodes) > 0:
        assert len(command.successCodes) > 0
        assert len(command.errorCodes) > 0
        out.writeln("/// # Result codes")

        out.writeln("/// ## Success")
        for success in command.successCodes:
            success = success.removeprefix("VK_")
            out.writeln(f"/// - [`{success}`](ResultCode::{success})")

        out.writeln("/// ## Error")
        for error in command.errorCodes:
            if error.startswith("VK_ERROR_"):
                error = error.removeprefix("VK_ERROR_")
                variant = f"ERROR_{error}"
            else:
                error = error.removeprefix("VK_")
                variant = error

            out.writeln(f"/// - [`{error}`](ResultCode::{variant})")


class Context:
    root: Path
    vk: vkobj.VulkanObject = get_vulkan_object(video=True)
    reg: Registry = Registry()

    def __init__(self, root: Path):
        self.root = root

    def remove_vendor_tag(self, name: str) -> str:
        last = name[-3:]
        if last in self.vk.vendorTags:
            return name[:-3]
        else:
            return name

    # Generates a struct (or union) definition from a vulkan struct.
    def generate_struct(self, x: vkobj.Struct) -> Generated[vkobj.Struct]:
        assert self.vk.videoStd is not None
        out = CodeWriter()

        type_name = struct_name(x.name)

        # docs
        vulkan_doc_header(out, x.name)

        out.writeln("///")
        if len(x.extensions) > 0 or x.version is not None:
            out.writeln("/// # Enabling")
            out.writeln("/// This type requires _at least_ one of the following:")

            if x.version is not None:
                # TODO: list features
                version = version_number(x.version.name)
                out.writeln(f"/// - Version {version} with appropriate features")

            for ext in x.extensions:
                ext = extension_name(ext)
                out.writeln(f"/// - Extension [`{ext}`](Extensions::{ext})")

            out.writeln("///")
            out.writeln(
                "/// Note this is not an exhaustive requirement list. For more information check vulkan documentation."
            )

        if not x.union and len(x.extendedBy) > 0:
            children = (f"[`{struct_name(child)}`]" for child in x.extendedBy)
            out.writeln("/// # Extended by")
            for child in children:
                out.writeln(f"/// - {child}")

        out.writeln(f'#[doc(alias = "{x.name}")]')

        # definition
        out.writeln("#[repr(C)]")
        if x.union:
            out.writeln("#[derive(Clone, Copy)]")
            out.writeln(f"pub union {type_name} {'{'}")
        else:
            out.writeln("#[derive(Debug, Clone, Copy)]")
            out.writeln(f"pub struct {type_name} {'{'}")

        out.indent()

        p_next_const = True
        for member in x.members:
            field_name = textcase.snake(member.name)
            if field_name == "type":
                field_name = "type_"

            if field_name == "p_next":
                p_next_const = member.const

            type = RustType.parse(member.fullType)
            for size in member.fixedSizeArray:
                type = type.array(size)

            if member.optional:
                out.writeln("/// Optional")

            if member.nullTerminated:
                out.writeln("/// Null terminated")

            out.writeln(f"pub {field_name}: {type},")
        out.deindent()
        out.writeln("}")

        # debug impl
        if x.union:
            out.writeln(f"impl std::fmt::Debug for {type_name} {{")
            out.indent()
            out.writeln(
                "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {"
            )
            out.indent()
            out.writeln(f'write!(f, "{type_name} {{{{ .. }}}}")')
            out.deindent()
            out.writeln("}")
            out.deindent()
            out.writeln("}")

        # default impl
        if x.union:
            field_name = textcase.snake(x.members[0].name)
            if field_name == "type":
                field_name = "type_"

            out.writeln(f"impl Default for {type_name} {{")
            out.indent()
            out.writeln("fn default() -> Self {")
            out.indent()
            out.writeln(f"Self {{ {field_name}: Default::default() }}")
            out.deindent()
            out.writeln("}")
            out.deindent()
            out.writeln("}")
        else:
            out.writeln(f"impl Default for {type_name} {{")
            out.indent()
            out.writeln("#[inline(always)]")
            out.writeln("fn default() -> Self {")
            out.indent()

            out.writeln("Self {")
            out.indent()
            for member in x.members:
                field_name = textcase.snake(member.name)
                if field_name == "type":
                    field_name = "type_"

                if field_name == "s_type" and x.sType is not None:
                    s_type = x.sType.removeprefix("VK_STRUCTURE_TYPE_")
                    out.writeln(f"{field_name}: StructureType::{s_type},")
                    continue

                if member.pointer:
                    out.writeln(
                        f"{field_name}: unsafe {{ std::mem::transmute(std::ptr::null::<()>()) }},"
                    )
                    continue

                default = "Default::default()"
                for _ in range(len(member.fixedSizeArray)):
                    default = f"[{default}; _]"

                out.writeln(f"{field_name}: {default},")

            out.deindent()
            out.writeln("}")
            out.deindent()
            out.writeln("}")
            out.deindent()
            out.writeln("}")

        # extends
        for parent in x.extends:
            assert not x.union
            parent_type_name = struct_name(parent)
            out.writeln(f"impl Extends<{parent_type_name}> for {type_name} {{}}")

        if x.allowDuplicate:
            out.writeln(f"impl Extends<{type_name}> for {type_name} {{}}")

        if len(x.extends) > 0 or x.allowDuplicate:
            out.writeln(f"impl {type_name} {{")
            out.indent()
            out.writeln("#[inline(always)]")
            out.writeln(
                f"pub fn with_next<T: Extends<Self>>(self, next: *{'const' if p_next_const else 'mut'} T) -> Self {{"
            )
            out.indent()
            out.writeln("Self { p_next: next.cast(), ..self }")
            out.deindent()
            out.writeln("}")
            out.deindent()
            out.writeln("}")

        # aliases
        for alias in x.aliases:
            alias = alias.removeprefix("Vk")
            out.writeln(f"pub type {alias} = {type_name};")

        return Generated(type_name, out.content, x)

    def generate_handle(self, x: vkobj.Handle) -> Generated[vkobj.Handle]:
        out = CodeWriter()

        type_name = x.name.removeprefix("Vk")

        vulkan_doc_header(out, x.name)
        out.writeln("///")
        out.writeln("/// # Handle type")
        if x.dispatchable:
            out.writeln("/// Dispatchable")
        else:
            out.writeln("/// Non-dispatchable")

        out.writeln(f'#[doc(alias = "{x.name}")]')
        out.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]")
        out.writeln("#[repr(transparent)]")

        if x.dispatchable:
            out.writeln(f"pub struct {type_name}(usize);")
        else:
            out.writeln(f"pub struct {type_name}(u64);")

        # aliases
        for alias in x.aliases:
            alias = alias.removeprefix("Vk")
            out.writeln(f"pub type {alias} = {type_name};")

        return Generated(type_name, out.content, x)

    def generate_enum(self, x: vkobj.Enum) -> Generated[vkobj.Enum]:
        out = CodeWriter()

        type_name = x.name.removeprefix("Vk")
        type_name_snake = textcase.snake(type_name).upper()
        repr_type = "i32" if x.bitWidth == 32 else "i64"

        # hacks for video variants
        if type_name_snake.startswith("STD_VIDEO"):
            type_name = type_name.removeprefix("StdVideo")
            type_name_snake = type_name_snake.replace("_H_264", "_H264")
            type_name_snake = type_name_snake.replace("_H_265", "_H265")
            type_name_snake = type_name_snake.replace("_AV_1", "_AV1")
            type_name_snake = type_name_snake.replace("_VP_9", "_VP9")

        if type_name == "Result":
            type_name = "ResultCode"

        vulkan_doc_header(out, x.name)
        out.writeln(f'#[doc(alias = "{x.name}")]')
        out.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]")
        out.writeln("#[non_exhaustive]")
        out.writeln(f"#[repr({repr_type})]")
        out.writeln(f"pub enum {type_name} {{")
        out.indent()

        first_field = True
        field_aliases: list[tuple[str, str]] = []
        for field in x.fields:
            name = field.name.removeprefix("VK_").removeprefix(f"{type_name_snake}_")
            if name[0].isdigit():
                name = f"_{name}"

            for alias in field.aliases:
                field_aliases.append((name, alias))

            if first_field:
                out.writeln("#[default]")
                first_field = False

            out.writeln(f'#[doc(alias = "{field.name}")]')
            out.writeln(f"{name} = {field.value},")

        out.deindent()
        out.writeln("}")

        # aliases
        for alias in x.aliases:
            alias = alias.removeprefix("Vk")
            out.writeln(f"pub type {alias} = {type_name};")

        if len(field_aliases) > 0:
            out.writeln(f"impl {type_name} {{")
            out.indent()
            for variant, alias in field_aliases:
                name = alias.removeprefix("VK_").removeprefix(f"{type_name_snake}_")
                if name[0].isdigit():
                    name = f"_{name}"

                out.writeln(f'#[doc(alias = "{alias}")]')
                out.writeln(f"pub const {name}: Self = Self::{variant};")
            out.deindent()
            out.writeln("}")

        return Generated(type_name, out.content, x)

    def generate_bitmasks(self, x: vkobj.Bitmask) -> Generated[vkobj.Bitmask]:
        out = CodeWriter()

        type_name = x.name.removeprefix("Vk").replace("FlagBits", "Flags")
        type_name_snake = textcase.snake(
            self.remove_vendor_tag(x.name.removeprefix("Vk").replace("FlagBits", ""))
        ).upper()
        repr_type = "u32" if x.bitWidth == 32 else "u64"

        out.writeln("bitflags::bitflags! {")
        out.indent()
        vulkan_doc_header(out, x.name)
        out.writeln(f'#[doc(alias = "{x.name}")]')
        out.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]")
        out.writeln("#[repr(transparent)]")
        out.writeln(f"pub struct {type_name}: {repr_type} {{")
        out.indent()

        flag_aliases: list[tuple[str, str]] = []
        for flag in x.flags:
            name = bitmask_flag_name(flag.name, type_name_snake)
            for alias in flag.aliases:
                flag_aliases.append((name, alias))

            out.writeln(f'#[doc(alias = "{flag.name}")]')
            out.writeln(f"const {name} = {flag.value};")

        out.deindent()
        out.writeln("}")
        out.deindent()
        out.writeln("}")

        # aliases
        for alias in x.aliases:
            alias = alias.removeprefix("Vk").replace("FlagBits", "Flags")
            out.writeln(f"pub type {alias} = {type_name};")

        if len(flag_aliases) > 0:
            out.writeln(f"impl {type_name} {{")
            out.indent()
            for variant, alias in flag_aliases:
                name = alias.removeprefix("VK_").removeprefix(f"{type_name_snake}_")
                if name[0].isdigit():
                    name = f"_{name}"

                out.writeln(f'#[doc(alias = "{alias}")]')
                out.writeln(f"pub const {name}: Self = Self::{variant};")
            out.deindent()
            out.writeln("}")

        return Generated(type_name, out.content, x)

    def generate_flags(self, x: vkobj.Flags) -> Generated[vkobj.Flags]:
        out = CodeWriter()

        type_name = x.name.removeprefix("Vk")
        repr_type = "u32" if x.bitWidth == 32 else "u64"

        vulkan_doc_header(out, x.name)
        out.writeln(f'#[doc(alias = "{x.name}")]')
        out.writeln(f"pub type {type_name} = {repr_type};")

        # aliases
        for alias in x.aliases:
            alias = alias.removeprefix("Vk")
            out.writeln(f"pub type {alias} = {type_name};")

        return Generated(type_name, out.content, x)

    def generate_fnptr(self, x: vkobj.FuncPointer) -> Generated[vkobj.FuncPointer]:
        out = CodeWriter()

        type_name = x.name.removeprefix("PFN_")

        vulkan_doc_header(out, x.name)
        out.writeln(f'#[doc(alias = "{x.name}")]')

        params: list[str] = []
        for param in x.params:
            params.append(str(RustType.parse(param.fullType)))

        return_ty = (
            "" if x.returnType == "void" else f"-> {RustType.parse(x.returnType)}"
        )
        signature = f'unsafe extern "C" fn({", ".join(params)}) {return_ty}'
        out.writeln(f"pub type {type_name} = {signature};")

        return Generated(type_name, out.content, x)

    def generate_const_inner(self, x: vkobj.Constant) -> Generated[vkobj.Constant]:
        out = CodeWriter()

        const_name = x.name
        const_ty = RustType.parse(x.type)
        const_value = x.value

        vulkan_doc_header(out, x.name)
        out.writeln(f"pub const {const_name}: {const_ty} = {const_value};")

        return Generated(const_name, out.content, x)

    def generate_const(self, x: vkobj.Constant) -> Generated[vkobj.Constant]:
        out = CodeWriter()

        const_name = x.name.removeprefix("VK_").removeprefix("STD_VIDEO_")
        const_ty = RustType.parse(x.type)

        vulkan_doc_header(out, x.name)
        out.writeln(f'#[doc(alias = "{x.name}")]')
        out.writeln(f"pub const {const_name}: {const_ty} = {x.name};")

        return Generated(const_name, out.content, x)

    def generate_command(self, x: vkobj.Command) -> Generated[vkobj.Command]:
        out = CodeWriter()

        method_name = textcase.snake(x.name.removeprefix("vk"))

        receiver = None
        allowed_receivers = {
            "Instance",
            "PhysicalDevice",
            "Device",
            "Queue",
            "CommandBuffer",
        }

        params: list[str] = []
        for param in x.params:
            name = textcase.snake(param.name).removeprefix("pp_").removeprefix("p_")

            if name == "type":
                name = "type_"

            if len(param.fixedSizeArray) > 0:
                # it's actually a pointer. amazing
                is_const = param.fullType.startswith("const ")
                ty = param.fullType.removeprefix("const ")
                ty = RustType.parse(ty)

                for size in param.fixedSizeArray:
                    ty = ty.array(size)

                # make it a pointer
                ty = RustPointer(is_const, ty)
            else:
                ty = RustType.parse(param.fullType)

            if receiver is None and str(ty) in allowed_receivers:
                receiver = ty
                continue

            params.append(f"{name}: {ty}")

        return_ty = (
            "" if x.returnType == "void" else f"-> {RustType.parse(x.returnType)}"
        )

        if receiver is not None:
            signature = (
                f"pub unsafe fn {method_name}(self, {', '.join(params)}) {return_ty}"
            )

            out.writeln(f"impl {receiver} {{")
            out.indent()

            vulkan_doc_header(out, x.name)
            out.writeln("///")
            command_doc_header(out, x)
            out.writeln(f'#[doc(alias = "{x.name}")]')
            out.writeln(f"{signature} {{")
            out.indent()
            out.writeln("todo!()")
            out.deindent()
            out.writeln("}")

            out.deindent()
            out.writeln("}")
        else:
            signature = f"pub unsafe fn {method_name}({', '.join(params)}) {return_ty}"

            vulkan_doc_header(out, x.name)
            out.writeln("///")
            command_doc_header(out, x)
            out.writeln(f'#[doc(alias = "{x.name}")]')
            out.writeln(f"{signature} {{")
            out.indent()
            out.writeln("todo!()")
            out.deindent()
            out.writeln("}")

        return Generated(method_name, out.content, x)

    def generate_extensions(self) -> str:
        out = CodeWriter()

        out.writeln("/// Enum containing all extensions.")
        out.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
        out.writeln("#[non_exhaustive]")
        out.writeln("pub enum Extensions {")
        out.indent()

        for ext in self.vk.extensions.values():
            assert ext.vendorTag is not None
            name = extension_name(ext.name)

            if ext.promotedTo is not None and ext.promotedTo != "":
                to = ext.promotedTo
                if to.startswith("VK_VERSION_"):
                    to = f"core in version {version_number(to)}"
                else:
                    to = f"[`Self::{extension_name(to)}`]"

                out.writeln(f"/// Promoted to {to}.")

            if ext.deprecatedBy is not None and ext.deprecatedBy != "":
                by = ext.deprecatedBy
                if by.startswith("VK_VERSION_"):
                    by = version_number(by)
                else:
                    by = f"[`Self::{extension_name(by)}`]"

                out.writeln(f"/// Deprecated by {by}.")

            if len(ext.specialUse) > 0:
                known_use_cases = {
                    "glemulation": "OpenGL emulation",
                    "d3demulation": "Direct3D emulation",
                    "devtools": "development tooling",
                }
                usecases = [known_use_cases.get(i, i) for i in ext.specialUse]
                out.writeln(f"/// Intended for {', '.join(usecases)}.")

            out.writeln(f'#[doc(alias = "{ext.name}")]')
            out.writeln(f"{name},")

        out.deindent()
        out.writeln("}")

        out.writeln("impl Extensions {")
        out.indent()
        out.writeln("pub fn name(self) -> &'static std::ffi::CStr {")
        out.indent()
        out.writeln("match self {")
        out.indent()

        for ext in self.vk.extensions.values():
            assert ext.vendorTag is not None
            name = ext.name.removeprefix("VK_").removeprefix(f"{ext.vendorTag}_")
            name = textcase.pascal(name)
            out.writeln(f'Self::{ext.vendorTag}_{name} => c"{ext.name}",')

        out.deindent()
        out.writeln("}")
        out.deindent()
        out.writeln("}")
        out.deindent()
        out.writeln("}")

        return out.content

    def fill_registry(self):
        assert self.vk.videoStd is not None

        # structs
        for struct in self.vk.structs.values():
            struct = self.generate_struct(struct)
            self.reg.structs.append(struct)

        for struct in self.vk.videoStd.structs.values():
            struct = self.generate_struct(struct)
            self.reg.structs.append(struct)

        # handles
        for handle in self.vk.handles.values():
            handle = self.generate_handle(handle)
            self.reg.handles.append(handle)

        # enums
        for enum in self.vk.enums.values():
            enum = self.generate_enum(enum)
            self.reg.enums.append(enum)

        for enum in self.vk.videoStd.enums.values():
            enum = self.generate_enum(enum)
            self.reg.enums.append(enum)

        # bitmasks
        for bitmask in self.vk.bitmasks.values():
            bitmask = self.generate_bitmasks(bitmask)
            self.reg.bitmasks.append(bitmask)

        # flags
        for flags in self.vk.flags.values():
            if flags.bitmaskName is not None:
                continue

            flags = self.generate_flags(flags)
            self.reg.flags.append(flags)

        # function pointers
        for fnptr in self.vk.funcPointers.values():
            fnptr = self.generate_fnptr(fnptr)
            self.reg.fnptrs.append(fnptr)

        # constants
        for const in self.vk.constants.values():
            const_inner = self.generate_const_inner(const)
            const = self.generate_const(const)
            self.reg.constants_inner.append(const_inner)
            self.reg.constants.append(const)

        for const in self.vk.videoStd.constants.values():
            const_inner = self.generate_const_inner(const)
            const = self.generate_const(const)
            self.reg.constants_inner.append(const_inner)
            self.reg.constants.append(const)

        # commands
        for command in self.vk.commands.values():
            command = self.generate_command(command)
            self.reg.commands.append(command)

        # extensions
        self.reg.extensions = self.generate_extensions()

    def write_module(self, path: str, content: str):
        path = f"{self.root}/src/{path}"
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w+") as f:
            _ = f.truncate(0)
            _ = f.write(MODULE_PREFIX)
            _ = f.write(content)

    def write_generated_to_module(
        self, path: str, prefix: str, generated: list[Generated[T]]
    ):
        content = prefix
        for elem in generated:
            content += f"{elem.definition}\n"

        self.write_module(path, content)

    def generate(self):
        print("Filling registry...")
        self.fill_registry()
        print("Done!")
        print(f"- Bitmasks: {len(self.reg.bitmasks)}")
        print(f"- Commands: {len(self.reg.commands)}")
        print(f"- Constants: {len(self.reg.constants_inner)}")
        print(f"- Enums: {len(self.reg.enums)}")
        print(f"- Flags: {len(self.reg.flags)}")
        print(f"- Function Pointers: {len(self.reg.fnptrs)}")
        print(f"- Handles: {len(self.reg.handles)}")
        print(f"- Structs: {len(self.reg.structs)}")
        print("Generating source files...")
        self.write_generated_to_module("bitmasks.rs", "", self.reg.bitmasks)
        self.write_generated_to_module(
            "commands.rs", COMMANDS_MODULE_PREFIX, self.reg.commands
        )
        self.write_generated_to_module("consts_inner.rs", "", self.reg.constants_inner)
        self.write_generated_to_module(
            "consts.rs", CONSTS_MODULE_PREFIX, self.reg.constants
        )
        self.write_generated_to_module("enums.rs", "", self.reg.enums)
        self.write_generated_to_module("flags.rs", "", self.reg.flags)
        self.write_generated_to_module(
            "fn_ptrs.rs", FN_PTRS_MODULE_PREFIX, self.reg.fnptrs
        )
        self.write_generated_to_module("handles.rs", "", self.reg.handles)
        self.write_generated_to_module(
            "structs.rs", STRUCTS_MODULE_PREFIX, self.reg.structs
        )
        self.write_module("extensions.rs", self.reg.extensions)

        print("Done!")


def main() -> None:
    root = Path(sys.argv[1])
    ctx = Context(root)
    ctx.generate()
