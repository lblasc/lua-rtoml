package = "lua-rtoml"
version = "0.2-0"

source = {
    url = "git+https://github.com/lblasc/lua-rtoml.git"
}

description = {
    summary = "Lua bindings for the Rust toml crate.",
    homepage = "https://github.com/lblasc/lua-rtoml",
    license = "MIT"
}

dependencies = {
    "lua >= 5.1",
    "luarocks-build-rust-mlua >= 0.2.7"
}

build = {
    type = "rust-mlua",
    modules = { "toml" }
}
