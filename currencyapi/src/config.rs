use extism_pdk::*;

use crate::host::*;

pub fn get_url() -> Result<String, Error> {
    Ok(config::get("url")?.unwrap_or("https://api.freecurrencyapi.com/v1".to_string()))
}

pub fn get_api_key() -> Result<String, Error> {
    config::get("api_key")?.ok_or(Error::msg("Api key not found in configuration."))
}

#[derive(serde::Serialize)]
pub struct Config {
    url: ConfigVar,
    api_key: ConfigVar
}

const CFG: Json<Config> = Json(Config {
    url: ConfigVar {
        name: "Api Url",
        html_type: HtmlType::Url,
        required: false,
        placeholder: "",
    },
    api_key: ConfigVar {
        name: "Api Key",
        html_type: HtmlType::Password,
        required: true,
        placeholder: "",
    },
});

/// Returns the desired format of the configuration
#[plugin_fn]
pub fn config() -> FnResult<Json<Config>> {
    Ok(CFG)
}
