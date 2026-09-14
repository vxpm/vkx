import os
from pathlib import Path
import sys

import textcase
from vulkan_object import get_vulkan_object, vulkan_object as vkobj

from .rust_types import RustType

MODULE_PREFIX: str = """ // WARNING: AUTO GENERATED MODULE
use std::ffi::{c_void, c_int, c_uint, c_char};
use crate::inner::*;

"""

class CodeWriter:
    content: str = ""
    newline: bool = True
    level: int = 0

    def indent(self):
        self.level += 1

    def deindent(self):
        assert(self.level != 0)
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
        out = CodeWriter()

        type_name = s.name.removeprefix("Vk")

        # definition
        out.writeln(f"/// <https://docs.vulkan.org/refpages/latest/refpages/source/{s.name}.html>")

        if s.union:
            out.writeln(f"#[derive(Clone, Copy)]")
        else:
            out.writeln(f"#[derive(Debug, Clone, Copy)]")

        out.writeln(f"#[repr(C)]")
        out.writeln(f"pub struct {type_name} {"{"}")
        out.indent()

        for member in s.members:
            field_name = textcase.snake(member.name)
            if field_name == "type":
                field_name = "type_"

            type = RustType.parse(member.fullType)
            for size in member.fixedSizeArray:
                constant = self.vk.constants.get(size)
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
            out.writeln("fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")
            out.indent()
            out.writeln(f"write!(f, \"{type_name} {{{{ .. }}}}\")")
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

        out.writeln(f"/// <https://docs.vulkan.org/refpages/latest/refpages/source/{h.name}.html>")
        out.writeln(f"#[derive(Debug, Clone, Copy, PartialEq, Eq)]")
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

        out.writeln(f"/// <https://docs.vulkan.org/refpages/latest/refpages/source/{e.name}.html>")
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

    def generate(self):
        print(f"Generating crate at {self.root}...")

        # 01. generate all structs
        print("01. Generating structs...")
        structs: list[str] = []
        for struct in self.vk.structs.values():
            structs.append(self.generate_struct(struct))

        structs_path = f"{self.root}/src/structs.rs"
        os.makedirs(os.path.dirname(structs_path), exist_ok=True)
        with open(structs_path, 'w+') as f:
            _ = f.truncate(0)
            _ = f.write(MODULE_PREFIX)
            _ = f.write("use crate::handles::*;\n")
            _ = f.write("use crate::enums::*;\n\n")
            for struct in structs:
                _ = f.write(struct)

        # 02. generate all handles
        print("02. Generating handles...")
        handles: list[str] = []
        for handle in self.vk.handles.values():
            handles.append(self.generate_handle(handle))

        handles_path = f"{self.root}/src/handles.rs"
        os.makedirs(os.path.dirname(handles_path), exist_ok=True)
        with open(handles_path, 'w+') as f:
            _ = f.truncate(0)
            _ = f.write(MODULE_PREFIX)
            for handle in handles:
                _ = f.write(handle)

        # 03. generate all enums
        print("03. Generating enums...")
        enums: list[str] = []
        for enum in self.vk.enums.values():
            enums.append(self.generate_enum(enum))

        enums_path = f"{self.root}/src/enums.rs"
        os.makedirs(os.path.dirname(enums_path), exist_ok=True)
        with open(enums_path, 'w+') as f:
            _ = f.truncate(0)
            _ = f.write(MODULE_PREFIX)
            for enum in enums:
                _ = f.write(enum)

        # done
        print("Done!")

def main() -> None:
    root = Path(sys.argv[1])
    ctx = Context(root)
    ctx.generate()
