from __future__ import annotations

import ast
import os
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, override

import textcase
import tomlkit
from vulkan_object import get_vulkan_object
from vulkan_object import vulkan_object as vkobj

import names
from rust_types import CTypeParser, RustPointer, RustType

MODULE_PREFIX: str = """ // WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::ffi::{c_void, c_int, c_uint, c_char};
use crate::loader::*;
use crate::platform::*;
"""

FN_PTRS_MODULE_PREFIX: str = """
use crate::consts_inner::*;
use crate::enums::*;
use crate::flags::*;
use crate::handles::*;
use crate::internal::*;
use crate::structs::*;
"""

STRUCTS_MODULE_PREFIX: str = """
use crate::consts_inner::*;
use crate::enums::*;
use crate::flags::*;
use crate::fn_ptrs::*;
use crate::handles::*;
use crate::internal::*;
"""

CONSTS_MODULE_PREFIX: str = """
use crate::consts_inner::*;
"""

COMMANDS_MODULE_PREFIX: str = """
use crate::consts_inner::*;
use crate::enums::*;
use crate::internal::*;
use crate::flags::*;
use crate::fn_ptrs::*;
use crate::handles::*;
use crate::structs::*;
"""

FLAGS_MODULE_PREFIX: str = """
use crate::enums::*;
"""

HANDLES_MODULE_PREFIX: str = """
use crate::internal::*;
use crate::enums::*;
"""


@dataclass
class RequirementVersion:
    version_number: str

    @override
    def __str__(self):
        return f"Version {self.version_number} with appropriate features"


@dataclass
class RequirementExtension:
    ext: str

    @override
    def __str__(self):
        return f"Extension [`{self.ext}`](Extension::{self.ext})"


@dataclass
class RequirementOp:
    op: str
    values: list[RequirementVersion | RequirementExtension | RequirementOp]

    @staticmethod
    def parse(expr: str) -> RequirementVersion | RequirementExtension | RequirementOp:
        # hack: use python AST to parse it
        def inner(
            node: Any,
        ) -> RequirementVersion | RequirementExtension | RequirementOp:
            if isinstance(node, ast.Name):
                if node.id.startswith("VK_VERSION_"):
                    return RequirementVersion(version_number(node.id))
                else:
                    return RequirementExtension(names.extension(node.id))

            if isinstance(node, ast.BoolOp):
                if isinstance(node.op, ast.And):
                    op = "and"
                else:
                    assert isinstance(node.op, ast.Or)
                    op = "or"

                values = list(map(inner, node.values))
                requirement = RequirementOp(op, values)
                requirement.flatten()
                return requirement

            print("Failed to parse vulkan requirements")
            sys.exit(-1)

        expr = expr.replace(",", " or ").replace("+", " and ")
        node = ast.parse(expr, mode="eval").body
        return inner(node)

    def flatten_inner(self):
        new_children: list[
            RequirementVersion | RequirementExtension | RequirementOp
        ] = []

        for child in self.values:
            if isinstance(child, RequirementOp) and child.op == self.op:
                new_children.extend(child.values)
            else:
                new_children.append(child)

        self.values = new_children

    def flatten(self):
        current = self.values
        while True:
            self.flatten_inner()
            if self.values == current:
                return

            current = self.values

    @override
    def __str__(self):
        inner = f" {self.op.upper()} ".join(map(str, self.values))
        return f"({inner})"


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


@dataclass
class Registry:
    constants: list[str] = field(default_factory=list)
    constants_inner: list[str] = field(default_factory=list)
    enums: list[str] = field(default_factory=list)
    flags: list[str] = field(default_factory=list)
    fnptrs: list[str] = field(default_factory=list)
    handles: list[str] = field(default_factory=list)
    structs: list[str] = field(default_factory=list)
    commands: list[str] = field(default_factory=list)


def version_number(ver: str) -> str:
    return ver.removeprefix("VK_VERSION_").replace("_", ".")


class Context:
    root: Path
    vk: vkobj.VulkanObject = get_vulkan_object(video=True)
    reg: Registry = Registry()
    ty_parser: CTypeParser = CTypeParser()
    result_variants: list[vkobj.EnumField]
    dispatchable_handles: dict[str, str]
    global_commands: list[str]
    instance_commands: list[str]

    def __init__(self, root: Path):
        self.ty_parser.add_mapping("VkResult", "ResultCode")
        self.root = root
        self.result_variants = []
        self.dispatchable_handles = {}
        self.global_commands = []
        self.instance_commands = []

    def remove_vendor_tag(self, name: str) -> str:
        split = name.rsplit("_", maxsplit=1)

        if len(split) < 2:
            return name

        prefix, suffix = split
        if suffix in self.vk.vendorTags:
            return prefix
        else:
            return name

    def vulkan_doc_link(self, name: str, video: bool = False) -> str:
        if video:
            return "https://docs.vulkan.org/spec/latest/chapters/videocoding.html"
        else:
            return (
                f"https://docs.vulkan.org/refpages/latest/refpages/source/{name}.html"
            )

    def vulkan_doc_header(self, out: CodeWriter, name: str, video: bool = False):
        if video:
            out.writeln(
                f"/// [`{name}`]({self.vulkan_doc_link(name, True)}) (Vulkan Video)"
            )
        else:
            out.writeln(f"/// [`{name}`]({self.vulkan_doc_link(name, False)})")

        out.writeln("///")

    def requirements_doc_header(
        self, out: CodeWriter, version: vkobj.Version | None, extensions: list[str]
    ):
        if len(extensions) > 0 or version is not None:
            out.writeln("/// # Requirements")
            out.writeln("/// This requires _at least_ one of the following:")

            if version is not None:
                minimum_version = list(
                    map(int, version_number(version.name).split("."))
                )
            else:
                minimum_version = None

            # preprocess all extensions
            # TODO: ordered set instead of list
            all_extensions: list[str] = []
            for ext_name in extensions:
                if ext_name not in all_extensions:
                    all_extensions.append(ext_name)
                ext = self.vk.extensions[ext_name]

                if (
                    ext.deprecatedBy is not None
                    and ext.deprecatedBy != ""
                    and ext.deprecatedBy not in all_extensions
                ):
                    all_extensions.append(ext.deprecatedBy)

                if (
                    ext.obsoletedBy is not None
                    and ext.obsoletedBy != ""
                    and ext.obsoletedBy not in all_extensions
                ):
                    all_extensions.append(ext.obsoletedBy)

                if ext.promotedTo is not None and ext.promotedTo != "":
                    if ext.promotedTo.startswith("VK_VERSION_"):
                        promoted_version = list(
                            map(int, version_number(ext.promotedTo).split("."))
                        )

                        if minimum_version is not None:
                            minimum_version = min(minimum_version, promoted_version)
                        else:
                            minimum_version = promoted_version
                    elif ext.promotedTo not in all_extensions:
                        all_extensions.append(ext.promotedTo)

            # doc
            if minimum_version is not None:
                # TODO: list features?
                assert version is not None or len(all_extensions) != 0
                version_str = ".".join(map(str, minimum_version))
                out.writeln(f"/// - Version {version_str} with appropriate features")

            for ext in all_extensions:
                ext_name = names.extension(ext)
                out.writeln(f"/// - Extension [`{ext_name}`](Extension::{ext_name})")

            out.writeln("///")
            out.writeln(
                "/// Note this list might not be exhaustive. For more information check vulkan documentation."
            )
            out.writeln("///")

    def command_doc_header(self, out: CodeWriter, command: vkobj.Command):
        optional_params = [param for param in command.params if param.optional]

        legacy_high_prio = (
            command.legacy is not None and command.legacy.version is not None
        )
        legacy_low_prio = (
            command.legacy is not None and len(command.legacy.extensions) > 0
        )

        if legacy_high_prio:
            assert command.legacy is not None
            assert command.legacy.version is not None
            out.writeln(f"/// # Legacy API (`{command.legacy.link}`)")

            version_num = version_number(command.legacy.version.name)
            out.writeln(f"/// This command is legacy since version {version_num}.")

            if len(command.legacy.extensions) > 0:
                out.writeln(
                    "/// This command is also legacy when any of the following extensions are enabled:"
                )

                for ext in command.legacy.extensions:
                    ext_name = names.extension(ext)
                    out.writeln(
                        f"/// - Extension [`{ext_name}`](Extension::{ext_name})"
                    )

                out.writeln("///")

            if command.legacy.supersededBy is not None:
                out.writeln(
                    f"/// It has been superseded by [`{command.legacy.supersededBy}`]({self.vulkan_doc_link(command.legacy.supersededBy)})."
                )

            out.writeln("///")

        if len(optional_params) > 0:
            out.writeln("/// # Optional parameters")
            for param in optional_params:
                name = names.command_param(param.name)
                out.writeln(f"/// - {name}")
            out.writeln("///")

        if len(command.tasks) > 0:
            out.writeln("/// # Performed tasks")
            for task in command.tasks:
                out.writeln(f"/// - `{task}`")

            out.writeln("///")

        if command.primary or command.secondary:
            out.writeln("/// # Allowed command buffers")

            if command.primary:
                out.writeln("/// - Primary")

            if command.secondary:
                out.writeln("/// - Secondary")

            out.writeln("///")

        if len(command.queues) > 0:
            out.writeln("/// # Allowed queues")
            for queue in command.queues:
                name = names.flag_variant(queue, "QUEUE")
                out.writeln(f"/// - [`{name}`](QueueFlag::{name})")

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

            out.writeln("///")

        if not legacy_high_prio and legacy_low_prio:
            assert command.legacy is not None
            out.writeln(f"/// # Legacy API (`{command.legacy.link}`)")
            out.writeln(
                "/// This command is legacy when any of the following extensions are enabled:"
            )

            for ext in command.legacy.extensions:
                ext_name = names.extension(ext)
                out.writeln(f"/// - Extension [`{ext_name}`](Extension::{ext_name})")

            if command.legacy.supersededBy is not None:
                out.writeln("///")
                out.writeln(
                    f"/// It has been superseded by [`{command.legacy.supersededBy}`]({self.vulkan_doc_link(command.legacy.supersededBy)})."
                )

            out.writeln("///")

    def returned_only_doc_header(self, out: CodeWriter, returned_only: bool):
        if returned_only:
            out.writeln("/// # Returned only")
            out.writeln(
                "/// This type is only returned by Vulkan, never constructed by the API user."
            )

    # Generates a struct (or union) definition from a vulkan struct.
    def generate_struct(self, x: vkobj.Struct) -> str:
        assert self.vk.videoStd is not None
        out = CodeWriter()

        type_name = names.struct(x.name)

        # docs
        self.vulkan_doc_header(out, x.name, x.videoStdHeader is not None)
        self.requirements_doc_header(out, x.version, x.extensions)
        self.returned_only_doc_header(out, x.returnedOnly)

        if not x.union and len(x.extends) > 0:
            parents = (f"[`{names.struct(parent)}`]" for parent in x.extends)
            out.writeln("/// # Extends")
            for parent in parents:
                out.writeln(f"/// - {parent}")

        if not x.union and len(x.extendedBy) > 0:
            children = (f"[`{names.struct(child)}`]" for child in x.extendedBy)
            out.writeln("/// # Extended by")
            for child in children:
                out.writeln(f"/// - {child}")

        # definition
        out.writeln(f'#[doc(alias = "{x.name}")]')
        out.writeln("#[repr(C)]")
        if x.union:
            out.writeln("#[derive(Clone, Copy)]")
            out.writeln(f"pub union {type_name} {'{'}")
        else:
            out.writeln("#[derive(Debug, Clone, Copy)]")
            out.writeln(f"pub struct {type_name} {'{'}")

        out.indent()

        has_p_next = False
        for member in x.members:
            field_name = names.struct_field(member.name)
            if field_name == "p_next":
                has_p_next = True

            type = self.ty_parser.parse(member.fullType)
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
            field_name = names.struct_field(x.members[0].name)
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
                field_name = names.struct_field(member.name)
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
        if has_p_next:
            out.writeln(f"unsafe impl Extendable for {type_name} {{")
            out.indent()
            out.writeln("#[inline(always)]")
            out.writeln("fn with_next<T: Extends<Self>>(self, next: *mut T) -> Self {")
            out.indent()
            out.writeln("unsafe {")
            out.indent()

            out.writeln("let base_next: *mut crate::BaseOutStructure = next.cast();")
            out.writeln(
                "let old = std::ptr::replace(&raw mut (*base_next).p_next, self.p_next as _);"
            )
            out.writeln(
                'assert!(old.is_null(), "pushed a structure in a chain into another chain");'
            )
            out.writeln(" Self { p_next: next as _, ..self }")

            out.deindent()
            out.writeln("}")
            out.deindent()
            out.writeln("}")
            out.deindent()
            out.writeln("}")

        for parent in x.extends:
            assert has_p_next
            assert not x.union
            parent_type_name = names.struct(parent)
            out.writeln(f"unsafe impl Extends<{parent_type_name}> for {type_name} {{}}")

        if x.allowDuplicate:
            assert has_p_next
            out.writeln(f"unsafe impl Extends<{type_name}> for {type_name} {{}}")

        # aliases
        for alias in x.aliases:
            alias_name = names.struct(alias)
            self.vulkan_doc_header(out, alias)
            out.writeln(f'#[doc(alias = "{alias}")]')
            out.writeln(f"pub type {alias_name} = {type_name};")

        return out.content

    def generate_handle(self, x: vkobj.Handle) -> str:
        out = CodeWriter()

        handle_name = names.handle(x.name)

        # docs
        self.vulkan_doc_header(out, x.name)
        self.requirements_doc_header(out, None, x.extensions)

        out.writeln("/// # About")
        if x.dispatchable:
            out.writeln("/// Dispatchable handle.")
        else:
            out.writeln("/// Non-dispatchable handle.")

        if x.parent is not None:
            parent = names.handle(x.parent.name)
            out.writeln(f"/// Child of [`{parent}`].")

        # definition
        out.writeln(f'#[doc(alias = "{x.name}")]')
        out.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]")
        out.writeln("#[repr(transparent)]")

        if x.dispatchable and len(x.extensions) == 0:
            raw_handle_name = f"{handle_name}Handle"
            inner_handle_type = "usize"

            self.dispatchable_handles[raw_handle_name] = handle_name
            self.ty_parser.add_mapping(x.name, raw_handle_name)
        else:
            raw_handle_name = handle_name
            inner_handle_type = "u64"

        out.writeln(f"pub struct {raw_handle_name}({inner_handle_type});")
        out.writeln(f"impl {raw_handle_name} {{")
        out.indent()
        out.writeln(
            "/// Returns a null handle. This is an alias for [`Self::default`]."
        )
        out.writeln("#[inline(always)]")
        out.writeln("pub fn null() -> Self {")
        out.indent()
        out.writeln("Self::default()")
        out.deindent()
        out.writeln("}")
        out.deindent()
        out.writeln("}")

        # aliases
        for alias in x.aliases:
            alias_name = names.handle(alias)
            self.vulkan_doc_header(out, alias)
            out.writeln(f'#[doc(alias = "{alias}")]')
            out.writeln(f"pub type {alias_name} = {raw_handle_name};")

        return out.content

    def generate_enum(self, x: vkobj.Enum) -> str:
        out = CodeWriter()

        enum_name = names.enum(x.name)
        variant_prefix = names.enum_variant_prefix(x.name)
        repr_type = "i32" if x.bitWidth == 32 else "i64"

        # special case result
        if enum_name == "Result":
            enum_name = "ResultCode"
            self.result_variants = x.fields

        # docs
        self.vulkan_doc_header(out, x.name, x.videoStdHeader is not None)
        self.requirements_doc_header(out, None, x.extensions)
        self.returned_only_doc_header(out, x.returnedOnly)

        # definition
        out.writeln(f'#[doc(alias = "{x.name}")]')
        out.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]")
        out.writeln("#[non_exhaustive]")
        out.writeln(f"#[repr({repr_type})]")
        out.writeln(f"pub enum {enum_name} {{")
        out.indent()

        first_variant = True
        variant_aliases: list[tuple[str, str]] = []
        for variant in x.fields:
            name = names.enum_variant(variant_prefix, variant.name)

            for alias in variant.aliases:
                variant_aliases.append((name, alias))

            if first_variant:
                out.writeln("#[default]")
                first_variant = False

            self.requirements_doc_header(out, None, variant.extensions)
            out.writeln(f'#[doc(alias = "{variant.name}")]')
            out.writeln(f"{name} = {variant.value},")

        out.deindent()
        out.writeln("}")

        # aliases
        for alias in x.aliases:
            alias_name = names.enum(alias)
            self.vulkan_doc_header(out, alias)
            out.writeln(f'#[doc(alias = "{alias}")]')
            out.writeln(f"pub type {alias_name} = {enum_name};")

        if len(variant_aliases) > 0:
            out.writeln(f"impl {enum_name} {{")
            out.indent()

            for variant, alias in variant_aliases:
                alias_name = names.enum_variant(variant_prefix, alias)
                self.vulkan_doc_header(out, alias)
                out.writeln(f'#[doc(alias = "{alias}")]')
                out.writeln(f"pub const {alias_name}: Self = Self::{variant};")

            out.deindent()
            out.writeln("}")

        return out.content

    def generate_flags(self, x: vkobj.Bitmask) -> str:
        out = CodeWriter()

        flag_set_name = names.flag_set(x.flagName)
        flag_enum_name = names.flag_enum(x.name)
        flag_prefix = self.remove_vendor_tag(
            textcase.snake(x.name.removeprefix("Vk").replace("FlagBits", "")).upper()
        )
        repr_type = "u32" if x.bitWidth == 32 else "u64"

        out.writeln("flagset::flags! {")
        out.indent()

        # docs
        self.vulkan_doc_header(out, x.name)
        self.requirements_doc_header(out, None, x.extensions)
        self.returned_only_doc_header(out, x.returnedOnly)

        # definition
        out.writeln(f'#[doc(alias = "{x.name}")]')
        out.writeln("#[non_exhaustive]")
        out.writeln(f"pub enum {flag_enum_name}: {repr_type} {{")
        out.indent()

        flag_aliases: list[tuple[str, str]] = []
        for flag in x.flags:
            name = names.flag_variant(flag.name, flag_prefix)
            for alias in flag.aliases:
                flag_aliases.append((name, alias))

            self.requirements_doc_header(out, None, flag.extensions)
            out.writeln(f'#[doc(alias = "{flag.name}")]')
            out.writeln(f"{name} = {flag.value},")

        out.deindent()
        out.writeln("}")
        out.deindent()
        out.writeln("}")

        # aliases
        for alias in x.aliases:
            alias_name = names.flag_enum(alias)
            self.vulkan_doc_header(out, alias)
            out.writeln(f'#[doc(alias = "{alias}")]')
            out.writeln(f"pub type {alias_name} = {flag_enum_name};")

        if len(flag_aliases) > 0:
            out.writeln(f"impl {flag_enum_name} {{")
            out.indent()

            generated_aliases: list[str] = []
            for flag, alias in flag_aliases:
                alias_name = names.flag_variant(alias, flag_prefix)
                if alias_name == flag or alias_name in generated_aliases:
                    continue

                generated_aliases.append(alias_name)
                self.vulkan_doc_header(out, alias)
                out.writeln(f'#[doc(alias = "{alias}")]')
                out.writeln(f"pub const {alias_name}: Self = Self::{flag};")
            out.deindent()
            out.writeln("}")

        # flag set
        flag_set = self.vk.flags[x.flagName]
        self.vulkan_doc_header(out, flag_set.name)
        self.requirements_doc_header(out, None, flag_set.extensions)
        self.returned_only_doc_header(out, flag_set.returnedOnly)
        out.writeln(f'#[doc(alias = "{x.flagName}")]')
        out.writeln(f"pub type {flag_set_name} = flagset::FlagSet<{flag_enum_name}>;")

        # flag set aliases
        for alias in flag_set.aliases:
            alias_name = names.flag_set(alias)
            self.vulkan_doc_header(out, alias)
            out.writeln(f'#[doc(alias = "{alias}")]')
            out.writeln(f"pub type {alias_name} = {flag_set_name};")

        return out.content

    def generate_anon_flag_set(self, x: vkobj.Flags) -> str:
        out = CodeWriter()

        flag_set_name = names.flag_set(x.name)
        repr_type = "u32" if x.bitWidth == 32 else "u64"

        # docs
        self.vulkan_doc_header(out, x.name)
        self.requirements_doc_header(out, None, x.extensions)
        self.returned_only_doc_header(out, x.returnedOnly)

        # definition
        out.writeln(f'#[doc(alias = "{x.name}")]')
        out.writeln(f"pub type {flag_set_name} = {repr_type};")

        # aliases
        for alias in x.aliases:
            alias_name = names.flag_set(alias)
            self.vulkan_doc_header(out, alias)
            out.writeln(f'#[doc(alias = "{alias}")]')
            out.writeln(f"pub type {alias_name} = {flag_set_name};")

        return out.content

    def generate_fnptr(self, x: vkobj.FuncPointer) -> str:
        out = CodeWriter()

        type_name = x.name.removeprefix("PFN_")

        # docs
        self.vulkan_doc_header(out, x.name)

        # definition
        out.writeln(f'#[doc(alias = "{x.name}")]')
        params: list[RustType] = []
        for param in x.params:
            params.append(self.ty_parser.parse(param.fullType))

        return_ty = (
            "" if x.returnType == "void" else f"-> {self.ty_parser.parse(x.returnType)}"
        )
        signature = f'unsafe extern "C" fn({", ".join(map(str, params))}) {return_ty}'
        out.writeln(f"pub type {type_name} = {signature};")

        return out.content

    def generate_const_inner(self, x: vkobj.Constant) -> str:
        out = CodeWriter()

        const_name = x.name
        const_ty = self.ty_parser.parse(x.type)
        const_value = x.value

        # docs
        self.vulkan_doc_header(out, x.name)

        # definition
        out.writeln(f"pub const {const_name}: {const_ty} = {const_value};")

        return out.content

    def generate_const(self, x: vkobj.Constant) -> str:
        out = CodeWriter()

        const_name = names.const(x.name)
        const_ty = self.ty_parser.parse(x.type)

        # docs
        self.vulkan_doc_header(out, x.name, x.videoStdHeader is not None)

        # definition
        out.writeln(f'#[doc(alias = "{x.name}")]')
        out.writeln(f"pub const {const_name}: {const_ty} = {x.name};")

        return out.content

    def generate_command(self, x: vkobj.Command) -> str:
        out = CodeWriter()

        command_name = names.command(x.name)
        command_fn_alias_name = names.command_fn_alias(x.name)

        # preprocess parameters
        handle = None
        params: list[tuple[str, RustType]] = []
        for param in x.params:
            param_name = names.command_param(param.name)

            if len(param.fixedSizeArray) > 0:
                # it's actually a pointer. amazing
                is_const = param.fullType.startswith("const ")
                ty = param.fullType.removeprefix("const ")
                ty = self.ty_parser.parse(ty)

                for size in param.fixedSizeArray:
                    ty = ty.array(size)

                # make it a pointer
                ty = RustPointer(is_const, ty)
            else:
                ty = self.ty_parser.parse(param.fullType)

            handle_with_dispatch = self.dispatchable_handles.get(str(ty), None)
            if handle is None and handle_with_dispatch is not None:
                handle = handle_with_dispatch
                continue

            params.append((param_name, ty))

        return_ty = (
            "" if x.returnType == "void" else f"-> {self.ty_parser.parse(x.returnType)}"
        )

        if handle is not None:
            # these commands are prefixed with raw to not clash with their smart handle implementation
            raw_prefixed_commands = {
                "enumerate_physical_devices",
                "create_device",
                "get_device_queue",
                "get_device_queue_2",
                "allocate_command_buffers",
            }

            if command_name in raw_prefixed_commands:
                command_name = f"raw_{command_name}"

            # method on a handle
            self.instance_commands.append(x.name)
            self.vulkan_doc_header(out, x.name)
            out.writeln(
                f'pub type FN_{command_fn_alias_name} = unsafe extern "C" fn({handle}Handle, {", ".join(str(x[1]) for x in params)}) {return_ty};'
            )

            if str(handle) != "Device":
                handle_snake = textcase.snake(str(handle))
                old_method_name = command_name
                command_name = command_name.replace(f"{handle_snake}_", "", count=1)
                if command_name == old_method_name:
                    command_name = command_name.replace(f"_{handle_snake}", "", count=1)

            signature = f"pub unsafe fn {command_name}(&self, {', '.join(f'{x[0]}: {x[1]}' for x in params)}) {return_ty}"

            out.writeln(f"impl {handle} {{")
            out.indent()

            # docs
            self.vulkan_doc_header(out, x.name)
            self.requirements_doc_header(out, x.version, x.extensions)
            self.command_doc_header(out, x)

            # definition
            out.writeln(f'#[doc(alias = "{x.name}")]')
            out.writeln("#[inline(always)]")
            out.writeln(f"{signature} {{")
            out.indent()

            out.writeln(
                f"let command = unsafe {{ std::mem::transmute::<vkVoidFunction, FN_{command_fn_alias_name}>(vtable_get(self.vtable(), InstanceCommand::{x.name} as usize)) }};"
            )
            out.writeln(
                f"unsafe {{ (command)(self.handle, {', '.join(x[0] for x in params)}) }}"
            )

            out.deindent()
            out.writeln("}")

            out.deindent()
            out.writeln("}")
        else:
            # free function
            self.global_commands.append(x.name)
            self.vulkan_doc_header(out, x.name)
            out.writeln(
                f'pub type FN_{command_fn_alias_name} = unsafe extern "C" fn({", ".join(str(x[1]) for x in params)}) {return_ty};'
            )

            signature = f"pub unsafe fn {command_name}({', '.join(f'{x[0]}: {x[1]}' for x in params)}) {return_ty}"

            # docs
            self.vulkan_doc_header(out, x.name)
            self.requirements_doc_header(out, x.version, x.extensions)
            self.command_doc_header(out, x)

            # definition
            out.writeln(f'#[doc(alias = "{x.name}")]')
            out.writeln("#[inline(always)]")
            out.writeln(f"{signature} {{")
            out.indent()
            out.writeln(
                'let commands = GLOBAL.get().expect("vkx setup should have been run").commands;'
            )
            out.writeln(
                f"let command = unsafe {{ std::mem::transmute::<vkVoidFunction, FN_{command_fn_alias_name}>(vtable_get(&commands, GlobalCommand::{x.name} as usize)) }};"
            )
            out.writeln(f"unsafe {{ (command)({', '.join(x[0] for x in params)}) }}")
            out.deindent()
            out.writeln("}")

        return out.content

    def generate_success_and_error_enums(self) -> tuple[str, str, str]:
        success = CodeWriter()
        error = CodeWriter()
        split = CodeWriter()

        # definition
        success.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
        success.writeln("#[non_exhaustive]")
        success.writeln("/// Enum with just the success codes of [`ResultCode`].")
        success.writeln("pub enum SuccessCode {")
        success.indent()

        error.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
        error.writeln("#[non_exhaustive]")
        error.writeln("/// Enum with just the error codes of [`ResultCode`].")
        error.writeln("pub enum ErrorCode {")
        error.indent()

        split.writeln("impl ResultCode {")
        split.indent()
        split.writeln(
            "/// Splits this result code into a [`Result`] containing either a [`SuccessCode`] or an [`ErrorCode`]."
        )
        split.writeln("pub fn split(self) -> Result<SuccessCode, ErrorCode> {")
        split.indent()
        split.writeln("match self {")
        split.indent()

        variant_prefix = names.enum_variant_prefix("VkResult")
        for variant in self.result_variants:
            name = names.enum_variant(variant_prefix, variant.name)

            result_code_name = name
            if name.startswith("ERROR_"):
                name = name.removeprefix("ERROR_")
                value = f"Err(ErrorCode::{name})"
                out = error
            else:
                value = f"Ok(SuccessCode::{name})"
                out = success

            self.requirements_doc_header(out, None, variant.extensions)
            out.writeln(f'#[doc(alias = "{variant.name}")]')
            out.writeln(f"{name} = {variant.value},")
            split.writeln(f"Self::{result_code_name} => {value},")

        success.deindent()
        success.writeln("}")

        error.deindent()
        error.writeln("}")

        split.deindent()
        split.writeln("}")
        split.deindent()
        split.writeln("}")
        split.deindent()
        split.writeln("}")

        return (success.content, error.content, split.content)

    def generate_extension_enum(self) -> str:
        out = CodeWriter()

        out.writeln("/// Enum containing all extensions.")
        out.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")
        out.writeln("#[non_exhaustive]")
        out.writeln("pub enum Extension {")
        out.indent()

        for ext in self.vk.extensions.values():
            assert ext.vendorTag is not None
            name = names.extension(ext.name)

            out.writeln(
                f"/// <https://docs.vulkan.org/refpages/latest/refpages/source/{ext.name}.html>"
            )
            out.writeln("///")

            out.writeln("/// # About")
            if ext.instance:
                out.writeln("/// Instance level.")
            else:
                out.writeln("/// Device level.")

            if ext.promotedTo is not None and ext.promotedTo != "":
                to = ext.promotedTo
                if to.startswith("VK_VERSION_"):
                    to = f"core in version {version_number(to)}"
                else:
                    to = f"[`Self::{names.extension(to)}`]"

                out.writeln(f"/// Promoted to {to}.")

            if ext.deprecatedBy is not None and ext.deprecatedBy != "":
                by = ext.deprecatedBy
                if by.startswith("VK_VERSION_"):
                    by = f"version {version_number(by)}"
                else:
                    by = f"[`Self::{names.extension(by)}`]"

                out.writeln(f"/// Deprecated by {by}.")

            if len(ext.specialUse) > 0:
                known_use_cases = {
                    "glemulation": "OpenGL emulation",
                    "d3demulation": "Direct3D emulation",
                    "devtools": "development tooling",
                    "cadsupport": "CAD tooling",
                }
                usecases = [known_use_cases.get(i, i) for i in ext.specialUse]
                out.writeln(f"/// Intended for {', '.join(usecases)}.")

            if ext.depends is not None and ext.depends != "":
                out.writeln("///")
                out.writeln("/// # Requirements")
                requirements = RequirementOp.parse(ext.depends)

                if isinstance(requirements, RequirementVersion):
                    out.writeln(
                        f"/// This extension requires at least version {requirements.version_number}."
                    )
                elif isinstance(requirements, RequirementExtension):
                    out.writeln(
                        f"/// This extension requires extension [`{requirements.ext}`](Extension::{requirements.ext})."
                    )
                else:
                    if requirements.op == "or":
                        out.writeln(
                            "/// This extensions requires at least one of the following: "
                        )
                    else:
                        out.writeln(
                            "/// This extensions requires all of the following: "
                        )

                    for req in requirements.values:
                        req_str = str(req)

                        # HACK: omit parentheses if top-level and simple binary requirement
                        if isinstance(req, RequirementOp) and len(req.values) == 2:
                            req_str = req_str[1:][:-1]

                        out.writeln(f"/// - {req_str}")

            out.writeln(f'#[doc(alias = "{ext.name}")]')
            out.writeln(f"{name},")

        out.deindent()
        out.writeln("}")

        out.writeln("impl Extension {")
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

        out.writeln("pub fn from_name(name: &std::ffi::CStr) -> Option<Self> {")
        out.indent()
        out.writeln("match name {")
        out.indent()

        for ext in self.vk.extensions.values():
            assert ext.vendorTag is not None
            name = ext.name.removeprefix("VK_").removeprefix(f"{ext.vendorTag}_")
            name = textcase.pascal(name)
            out.writeln(
                f'x if x == c"{ext.name}" => Some(Self::{ext.vendorTag}_{name}),'
            )

        out.writeln("_ => None")

        out.deindent()
        out.writeln("}")
        out.deindent()
        out.writeln("}")

        out.deindent()
        out.writeln("}")

        return out.content

    def generate_commands_enum(self, name: str, commands: list[str]) -> str:
        out = CodeWriter()

        out.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
        out.writeln(f"pub(crate) enum {name} {{")
        out.indent()

        for cmd in commands:
            out.writeln(f"{cmd},")

        out.deindent()
        out.writeln("}")

        out.writeln(f"impl {name} {{")
        out.indent()
        out.writeln(
            f"pub const VARIANTS: &[Self; {len(commands)}] = &[{', '.join(f'Self::{cmd}' for cmd in commands)}];"
        )
        out.writeln("pub fn name(self) -> &'static std::ffi::CStr {")
        out.indent()
        out.writeln("match self {")
        out.indent()

        for cmd in commands:
            out.writeln(f'Self::{cmd} => c"{cmd}",')

        out.deindent()
        out.writeln("}")
        out.deindent()
        out.writeln("}")
        out.deindent()
        out.writeln("}")

        return out.content

    def fill_registry(self):
        assert self.vk.videoStd is not None

        # handles
        for handle in self.vk.handles.values():
            handle = self.generate_handle(handle)
            self.reg.handles.append(handle)

        # structs
        for struct in self.vk.structs.values():
            struct = self.generate_struct(struct)
            self.reg.structs.append(struct)

        for struct in self.vk.videoStd.structs.values():
            struct = self.generate_struct(struct)
            self.reg.structs.append(struct)

        # enums
        for enum in self.vk.enums.values():
            enum = self.generate_enum(enum)
            self.reg.enums.append(enum)

        for enum in self.vk.videoStd.enums.values():
            enum = self.generate_enum(enum)
            self.reg.enums.append(enum)

        # flags
        for flags in self.vk.bitmasks.values():
            flags = self.generate_flags(flags)
            self.reg.flags.append(flags)

        # anon flags
        for flags in self.vk.flags.values():
            if flags.bitmaskName is not None:
                continue

            flags = self.generate_anon_flag_set(flags)
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

    def generate_custom_enums(self) -> str:
        success_enum, error_enum, split_impl = self.generate_success_and_error_enums()
        extensions_enum = self.generate_extension_enum()
        global_commands_enum = self.generate_commands_enum(
            "GlobalCommand", self.global_commands
        )
        instance_commands_enum = self.generate_commands_enum(
            "InstanceCommand", self.instance_commands
        )
        return f"{success_enum}\n{error_enum}\n{split_impl}\n{extensions_enum}\n{global_commands_enum}\n{instance_commands_enum}"

    def write_file(self, path: str, content: str):
        path = f"{self.root}/{path}"
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w+") as f:
            _ = f.truncate(0)
            _ = f.write(content)

    def write_module(self, path: str, content: str):
        path = f"src/{path}"
        self.write_file(path, f"{MODULE_PREFIX}\n{content}")

    def write_generated_to_module(self, path: str, base: str, generated: list[str]):
        content = f"{base}\n"
        for elem in generated:
            content += f"{elem}\n"

        self.write_module(path, content)

    def generate(self):
        print("Filling registry...")

        self.fill_registry()
        custom_enums = self.generate_custom_enums()

        print("Done!")
        print(f"- Commands: {len(self.reg.commands)}")
        print(f"- Constants: {len(self.reg.constants_inner)}")
        print(f"- Enums: {len(self.reg.enums)}")
        print(f"- Flags: {len(self.reg.flags)}")
        print(f"- Function Pointers: {len(self.reg.fnptrs)}")
        print(f"- Handles: {len(self.reg.handles)}")
        print(f"- Structs: {len(self.reg.structs)}")

        print("Generating version information and updating Cargo.toml...")
        with open(f"{self.root}/Cargo.toml", "r", encoding="utf-8") as f:
            manifest = tomlkit.parse(f.read())

        version = manifest["package"]["version"]
        assert isinstance(version, str)

        crate_version, vk_version = version.split("+")
        if vk_version.split(".") > self.vk.headerVersionComplete.split("."):
            print("ERROR: crate's vulkan version is > this header's version!")
            sys.exit(-1)

        # increment crate version if not the same vulkan version
        if vk_version.split(".") != self.vk.headerVersionComplete.split("."):
            crate_version = list(map(int, crate_version.split(".")))
            crate_version[-1] += 1
            crate_version = ".".join(map(str, crate_version))
            manifest["package"]["version"] = (
                f"{crate_version}+{self.vk.headerVersionComplete}"
            )

        with open(f"{self.root}/Cargo.toml", "w", encoding="utf-8") as f:
            _ = f.write(tomlkit.dumps(manifest))

        self.write_file("version.md", self.vk.headerVersionComplete)

        print("Generating source files...")
        self.write_generated_to_module(
            "commands.rs", COMMANDS_MODULE_PREFIX, self.reg.commands
        )
        self.write_generated_to_module("consts_inner.rs", "", self.reg.constants_inner)
        self.write_generated_to_module(
            "consts.rs", CONSTS_MODULE_PREFIX, self.reg.constants
        )
        self.write_generated_to_module("enums.rs", custom_enums, self.reg.enums)
        self.write_generated_to_module("flags.rs", FLAGS_MODULE_PREFIX, self.reg.flags)
        self.write_generated_to_module(
            "fn_ptrs.rs", FN_PTRS_MODULE_PREFIX, self.reg.fnptrs
        )
        self.write_generated_to_module(
            "handles.rs", HANDLES_MODULE_PREFIX, self.reg.handles
        )
        self.write_generated_to_module(
            "structs.rs", STRUCTS_MODULE_PREFIX, self.reg.structs
        )
        print("Done!")


root = Path(sys.argv[1])
ctx = Context(root)
ctx.generate()
