generate:
    uv run --directory generator src/main.py ../vkx
    cd vkx && cargo fmt
