use mlua::prelude::*;

fn load<'lua>(lua: &'lua Lua, value: LuaValue<'lua>) -> LuaResult<LuaValue<'lua>> {
    let toml = if let LuaValue::String(toml) = value {
        toml
    } else {
        return Err(format!("invalid type: {}, expected string", value.type_name()).to_lua_err());
    };

    let toml_value = toml::from_slice::<toml::Value>(toml.as_bytes())
        .map_err(|e| LuaError::external(e.to_string()))?;

    lua.to_value(&toml_value)
}

fn dump<'lua>(lua: &'lua Lua, value: LuaValue<'lua>) -> LuaResult<LuaValue<'lua>> {
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
