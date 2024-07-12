use extism_pdk::*;

#[plugin_fn]
pub fn get() -> FnResult<String>{
    Ok("Hello, world!".to_owned())
}
