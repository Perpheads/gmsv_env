#![feature(c_unwind)]
use std::fs;
use dotenv_parser::parse_dotenv;
use std::env;

#[macro_use]
extern crate gmod;

fn read_env_file_var(lookup_var: &str) -> Option<String> {
    let contents = fs::read_to_string("./garrysmod/.env");
    if contents.is_err() {
        return None;
    }
    let parsed_env = parse_dotenv(&contents.unwrap());

    if parsed_env.is_err() {
        return None;
    }

    parsed_env.unwrap().get(lookup_var).map(|s| s.to_string())
}

unsafe extern "C-unwind" fn read_env(lua: gmod::lua::State) -> i32 {
    let lookup_var = lua.check_string(1).into_owned();
    let env_value = read_env_file_var(&lookup_var).or_else(|| {
        env::var(lookup_var).ok()
    });

    if env_value.is_some() {
        lua.push_string(env_value.unwrap().as_str());
    } else {
        lua.push_nil();
    }

    return 1;
}

#[gmod13_open]
pub unsafe extern "C-unwind" fn gmod13_open(lua: gmod::lua::State) -> i32 {
    lua.push_function(read_env);
    lua.set_global(lua_string!("env"));

    lua.pop();

    return 0;
}

#[gmod13_close]
fn gmod13_close(_lua: gmod::lua::State) -> i32 {
    return 0;
}
