import os
from pathlib import Path
from pprint import pprint
import sys

import textcase
from vulkan_object import get_vulkan_object, vulkan_object as vkobj

from .rust_types import RustType

MODULE_PREFIX: str = """ // WARNING: AUTO GENERATED MODULE
#![allow(nonstandard_style)]
#![allow(unused_imports)]

use std::ffi::{c_void, c_int, c_uint, c_char};
use crate::inner::*;
use crate::platform::*;
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


class Context:
    root: Path
    vk: vkobj.VulkanObject = get_vulkan_object(video=True)
    generated_structs: set[str] = set()

    def __init__(self, root: Path):
        self.root = root

    # Generates a struct (or union) definition from a vulkan struct.
    def generate_struct(self, s: vkobj.Struct) -> str:
        assert self.vk.videoStd is not None
        out = CodeWriter()

        type_name = s.name.removeprefix("Vk")

        # definition
        out.writeln(
            f"/// <https://docs.vulkan.org/refpages/latest/refpages/source/{s.name}.html>"
        )

        if s.union:
            out.writeln("#[derive(Clone, Copy)]")
        else:
            out.writeln("#[derive(Debug, Clone, Copy)]")

        out.writeln("#[repr(C)]")
        out.writeln(f"pub struct {type_name} {'{'}")
        out.indent()

        for member in s.members:
            field_name = textcase.snake(member.name)
            if field_name == "type":
                field_name = "type_"

            type = RustType.parse(member.fullType)
            for size in member.fixedSizeArray:
                constant = self.vk.constants.get(
                    size
                ) or self.vk.videoStd.constants.get(size)

                if constant is None:
                    type = type.array(int(size))
                else:
                    type = type.array(int(constant.value))

            out.writeln(f"pub {field_name}: {type},")
        out.deindent()
        out.writeln("}")

        # debug impl
        if s.union:
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

        # aliases
        for alias in s.aliases:
            alias = alias.removeprefix("Vk")
            out.writeln(f"pub type {alias} = {type_name};")

        return out.content

    def generate_handle(self, h: vkobj.Handle) -> str:
        out = CodeWriter()

        out.writeln(
            f"/// <https://docs.vulkan.org/refpages/latest/refpages/source/{h.name}.html>"
        )
        out.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
        out.writeln("#[repr(transparent)]")

        type_name = h.name.removeprefix("Vk")
        if h.dispatchable:
            out.writeln(f"pub struct {type_name}(usize);")
        else:
            out.writeln(f"pub struct {type_name}(u64);")

        # aliases
        for alias in h.aliases:
            alias = alias.removeprefix("Vk")
            out.writeln(f"pub type {alias} = {type_name};")

        return out.content

    def generate_enum(self, e: vkobj.Enum) -> str:
        out = CodeWriter()

        type_name = e.name.removeprefix("Vk")
        type_name_snake = textcase.snake(type_name).upper()
        repr_type = "i32" if e.bitWidth == 32 else "i64"

        # hacks for video variants
        if type_name_snake.startswith("STD_VIDEO"):
            type_name_snake = type_name_snake.replace("_H_264", "_H264")
            type_name_snake = type_name_snake.replace("_H_265", "_H265")
            type_name_snake = type_name_snake.replace("_AV_1", "_AV1")
            type_name_snake = type_name_snake.replace("_VP_9", "_VP9")

        if type_name == "Result":
            type_name = "ResultCode"

        out.writeln(
            f"/// <https://docs.vulkan.org/refpages/latest/refpages/source/{e.name}.html>"
        )
        out.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
        out.writeln(f"#[repr({repr_type})]")
        out.writeln(f"pub enum {type_name} {{")
        out.indent()

        for field in e.fields:
            name = field.name.removeprefix("VK_").removeprefix(f"{type_name_snake}_")
            if name[0].isdigit():
                name = f"_{name}"

            out.writeln(f"{name} = {field.value},")

        out.deindent()
        out.writeln("}")

        # aliases
        for alias in e.aliases:
            alias = alias.removeprefix("Vk")
            out.writeln(f"pub type {alias} = {type_name};")

        return out.content

    def remove_vendor_tag(self, name: str) -> str:
        last = name[-3:]
        if last in self.vk.vendorTags:
            return name[:-3]
        else:
            return name

    def generate_bitmasks(self, m: vkobj.Bitmask) -> str:
        out = CodeWriter()

        type_name = m.name.removeprefix("Vk").replace("FlagBits", "Flags")
        type_name_snake = textcase.snake(
            self.remove_vendor_tag(m.name.removeprefix("Vk").replace("FlagBits", ""))
        ).upper()
        repr_type = "u32" if m.bitWidth == 32 else "u64"

        out.writeln("bitflags::bitflags! {")
        out.indent()
        out.writeln(
            f"/// <https://docs.vulkan.org/refpages/latest/refpages/source/{m.name}.html>"
        )
        out.writeln("#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
        out.writeln("#[repr(transparent)]")
        out.writeln(f"pub struct {type_name}: {repr_type} {{")
        out.indent()

        for flag in m.flags:
            name = flag.name.removeprefix("VK_").removeprefix(f"{type_name_snake}_")
            if name[0].isdigit():
                name = f"_{name}"

            out.writeln(f"const {name} = {flag.value};")

        out.deindent()
        out.writeln("}")
        out.deindent()
        out.writeln("}")

        # aliases
        for alias in m.aliases:
            alias = alias.removeprefix("Vk").replace("FlagBits", "Flags")
            out.writeln(f"pub type {alias} = {type_name};")

        return out.content

    def generate_flags(self, f: vkobj.Flags) -> str:
        out = CodeWriter()

        type_name = f.name.removeprefix("Vk")
        repr_type = "u32" if f.bitWidth == 32 else "u64"

        out.writeln(
            f"/// <https://docs.vulkan.org/refpages/latest/refpages/source/{f.name}.html>"
        )
        out.writeln(f"pub type {type_name} = {repr_type};")

        # aliases
        for alias in f.aliases:
            alias = alias.removeprefix("Vk")
            out.writeln(f"pub type {alias} = {type_name};")

        return out.content

    def generate_fnptr(self, f: vkobj.FuncPointer) -> str:
        out = CodeWriter()

        type_name = f.name.removeprefix("PFN_")

        out.writeln(
            f"/// <https://docs.vulkan.org/refpages/latest/refpages/source/{f.name}.html>"
        )

        params: list[str] = []
        for param in f.params:
            params.append(str(RustType.parse(param.fullType)))

        return_ty = (
            "" if f.returnType == "void" else f"-> {RustType.parse(f.returnType)}"
        )
        signature = f'unsafe extern "C" fn({",".join(params)}) {return_ty}'
        out.writeln(f"pub type {type_name} = {signature};")

        return out.content

    def write_module(self, path: str, content: str):
        path = f"{self.root}/src/{path}"
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w+") as f:
            _ = f.truncate(0)
            _ = f.write(MODULE_PREFIX)
            _ = f.write(content)

    def generate(self):
        assert self.vk.videoStd is not None
        print(f"Generating crate at {self.root}...")

        print("01. Generating structs...")
        structs: list[str] = []

        for struct in self.vk.structs.values():
            structs.append(self.generate_struct(struct))

        for struct in self.vk.videoStd.structs.values():
            structs.append(self.generate_struct(struct))

        base = """use crate::handles::*;
        use crate::enums::*;
        use crate::bitmasks::*;
        use crate::flags::*;
        use crate::fn_pointers::*;

        """
        self.write_module("structs.rs", base + "\n".join(structs))

        print("02. Generating handles...")
        handles: list[str] = []
        for handle in self.vk.handles.values():
            handles.append(self.generate_handle(handle))
        self.write_module("handles.rs", "\n".join(handles))

        print("03. Generating enums...")
        enums: list[str] = []

        for enum in self.vk.enums.values():
            enums.append(self.generate_enum(enum))

        for enum in self.vk.videoStd.enums.values():
            enums.append(self.generate_enum(enum))

        self.write_module("enums.rs", "\n".join(enums))

        print("04. Generating bitmasks...")
        bitmasks: list[str] = []
        for bitmask in self.vk.bitmasks.values():
            bitmasks.append(self.generate_bitmasks(bitmask))
        self.write_module("bitmasks.rs", "\n".join(bitmasks))

        print("05. Generating flags...")
        flags: list[str] = []
        for flags_ty in self.vk.flags.values():
            if flags_ty.bitmaskName is None:
                flags.append(self.generate_flags(flags_ty))
        self.write_module("flags.rs", "\n".join(flags))

        print("06. Generating function pointers...")
        fnptrs: list[str] = []
        for fnptr in self.vk.funcPointers.values():
            fnptrs.append(self.generate_fnptr(fnptr))

        base = """use crate::handles::*;
        use crate::enums::*;
        use crate::bitmasks::*;
        use crate::flags::*;
        use crate::structs::*;

        """
        self.write_module("fn_pointers.rs", base + "\n".join(fnptrs))

        # print("07. Generating video std headers...")
        # video_headers: list[str] = []
        # for fnptr in self.vk.videoStd.values():
        #     fnptrs.append(self.generate_fnptr(fnptr))

        # done
        print("Done!")


def main() -> None:
    root = Path(sys.argv[1])
    ctx = Context(root)
    ctx.generate()
