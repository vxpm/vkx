import textcase


def struct(name: str) -> str:
    return name.removeprefix("Vk").removeprefix("StdVideo")


def struct_field(name: str) -> str:
    name = textcase.snake(name)
    if name == "type":
        name = "type_"

    return name


def handle(name: str) -> str:
    return name.removeprefix("Vk")


def enum(name: str) -> str:
    return name.removeprefix("Vk").removeprefix("StdVideo")


def enum_variant_prefix(name: str) -> str:
    result = textcase.snake(enum(name)).upper()
    if name.startswith("StdVideo"):
        result = result.replace("H_264", "H264")
        result = result.replace("H_265", "H265")
        result = result.replace("AV_1", "AV1")
        result = result.replace("VP_9", "VP9")

    return result


def enum_variant(
    prefix: str,
    name: str,
) -> str:
    result = (
        name.removeprefix("VK_").removeprefix("STD_VIDEO_").removeprefix(f"{prefix}_")
    )

    if result[0].isdigit():
        result = f"_{result}"

    return result


def flag_enum(name: str) -> str:
    return name.removeprefix("Vk").replace("FlagBits", "Flag")


def flag_variant(name: str, prefix: str) -> str:
    result = name.removeprefix("VK_").removeprefix(f"{prefix}_")
    if result[0].isdigit():
        result = f"_{result}"

    if result.endswith("_BIT"):
        result = result.removesuffix("_BIT")
    else:
        result = result.replace("_BIT_", "_")

    return result


def flag_set(name: str) -> str:
    return name.removeprefix("Vk")


def const(name: str) -> str:
    return name.removeprefix("VK_").removeprefix("STD_VIDEO_")


def extension(name: str) -> str:
    name = name.removeprefix("VK_")
    split = name.split("_", maxsplit=1)
    tag, name = split
    return f"{tag}_{textcase.pascal(name)}"


def command(name: str) -> str:
    return textcase.snake(name.removeprefix("vk"))


def command_fn_type(name: str) -> str:
    return f"Fn{name.removeprefix('vk')}"


def fnptr(name: str) -> str:
    return f"Fn{name.removeprefix('PFN_vk')}"


def command_param(name: str) -> str:
    name = textcase.snake(name)
    if name == "type":
        name = "type_"

    return name
