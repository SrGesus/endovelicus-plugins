use std::marker::PhantomData;

use extism_pdk::*;

use crate::host::*;

fn get_url() -> Result<String, Error> {
    Ok(config::get("url")?.unwrap_or("https://api.freecurrencyapi.com/v1".to_string()))
}

fn get_api_key() -> Result<String, Error> {
    config::get("api_key")?.ok_or(Error::msg("Api key not found in configuration."))
}

// Config

#[derive(serde::Serialize)]
pub struct Config {
    #[serde(skip)]
    _phantom: std::marker::PhantomData<&'static str>,
    url: ConfigVar,
}

const CFG: Json<Config> = Json(Config {
    _phantom: PhantomData,
    url: ConfigVar {
        name: "Api Url",
        html_type: HtmlType::Url,
        required: false,
        placeholder: "",
    },
});

/// Returns the desired format of the configuration
#[plugin_fn]
pub fn config() -> FnResult<Json<Config>> {
    Ok(CFG)
}
