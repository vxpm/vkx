from __future__ import annotations
from dataclasses import dataclass
from typing import override

# A rust type.
class RustType:
    @staticmethod
    def parse(type: str) -> RustType:
        if type.endswith(" const *"):
            type = type.removesuffix(" const *")
            return RustPointer(True, RustType.parse(type))

        if type.endswith(" const*"):
            type = type.removesuffix(" const*")
            return RustPointer(True, RustType.parse(type))

        if type.endswith("*"):
            type = type.removesuffix("*")
            const = False

            if type.startswith("const "):
                type = type.removeprefix("const ")
                const = True

            return RustPointer(const, RustType.parse(type))

        # a raw name - map it
        mapping = {
            # primitives
            "uint8_t": "u8",
            "uint16_t": "u16",
            "uint32_t": "u32",
            "uint64_t": "u64",
            "int8_t": "i8",
            "int16_t": "i16",
            "int32_t": "i32",
            "int64_t": "i64",
            "size_t": "usize",

            "int": "c_int",
            "unsigned int": "c_uint",

            "float": "f32",
            "double": "f64",

            "char": "c_char",

            # special
            "void": "c_void",
            "VkResult": "VkResult",
        }

        rust = mapping.get(type)
        if rust is None:
            rust = type.removeprefix("struct ").removeprefix("Vk")

        return RustBasic(rust)

    def array(self, size: int) -> RustArray:
        return RustArray(self, size)

# A rust type that cannot be broken down further.
@dataclass
class RustBasic(RustType):
    value: str

    @override
    def __str__(self) -> str:
        return self.value

# A rust pointer.
@dataclass
class RustPointer(RustType):
    const: bool
    pointee: RustType

    @override
    def __str__(self) -> str:
        return f"{"*const" if self.const else "*mut"} {self.pointee}"

# A rust array.
@dataclass
class RustArray(RustType):
    inner: RustType
    size: int

    @override
    def __str__(self) -> str:
        return f"[{self.inner}; {self.size}]"
