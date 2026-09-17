from __future__ import annotations

from dataclasses import dataclass
from typing import override


# Parses C type declarations to their Rust equivalent
class CTypeParser:
    mappings: dict[str, str]

    def __init__(self):
        self.mappings = {
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
            "void": "c_void",
            # special
            # "VkResult": "ResultCode",
            # "VkInstance": "InstanceHandle",
            # "VkPhysicalDevice": "PhysicalDeviceHandle",
            # "VkDevice": "DeviceHandle",
            # "VkQueue": "QueueHandle",
            # "VkCommandBuffer": "CommandBufferHandle",
            # "VkExternalComputeQueueNV": "ExternalComputeQueueNVHandle",
        }

    def add_mapping(self, key: str, value: str):
        self.mappings[key] = value

    def parse(self, type: str) -> RustType:
        if type.endswith(" const *"):
            type = type.removesuffix(" const *")
            return RustPointer(True, self.parse(type))

        if type.endswith(" const*"):
            type = type.removesuffix(" const*")
            return RustPointer(True, self.parse(type))

        if type.endswith("*"):
            type = type.removesuffix("*")
            const = False

            if type.startswith("const "):
                type = type.removeprefix("const ")
                const = True

            return RustPointer(const, self.parse(type))

        # atom - map it
        atom = self.mappings.get(type)
        if atom is None:
            atom = (
                type.removeprefix("struct ")
                .removeprefix("Vk")
                .removeprefix("StdVideo")
                .removeprefix("PFN_")
                .replace("FlagBits", "Flags")
            )

        return RustAtom(atom)


# A rust type.
class RustType:
    def array(self, size: str) -> RustArray:
        return RustArray(self, size)


# A rust type that cannot be broken down further.
@dataclass
class RustAtom(RustType):
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
        return f"{'*const' if self.const else '*mut'} {self.pointee}"


# A rust array.
@dataclass
class RustArray(RustType):
    inner: RustType
    size: str

    @override
    def __str__(self) -> str:
        return f"[{self.inner}; {self.size} as usize]"
