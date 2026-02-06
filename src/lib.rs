use mlua::prelude::*;

fn load(lua: &Lua, value: LuaValue) -> LuaResult<LuaValue> {
    let toml = if let LuaValue::String(toml) = value {
        toml
    } else {
        return Err(format!("invalid type: {}, expected string", value.type_name()).into_lua_err());
    };

    let toml_value = toml::from_str::<toml::Value>(&toml.to_string_lossy())
        .map_err(|e| LuaError::external(e.to_string()))?;

    lua.to_value(&toml_value)
}

fn dump(lua: &Lua, value: LuaValue) -> LuaResult<LuaValue> {
    Ok(LuaValue::String(lua.create_string(
        &toml::to_string(&value).map_err(|e| LuaError::external(e.to_string()))?,
    )?))
}

#[mlua::lua_module]
fn toml(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("load", lua.create_function(load)?)?;
    exports.set("dump", lua.create_function(dump)?)?;
    Ok(exports)
}
