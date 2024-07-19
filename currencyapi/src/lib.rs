use std::collections::BTreeMap;

use extism_pdk::*;

// Taken from entity::currency at endovelicus
struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: Option<String>,
    pub rate: f64,
}

#[host_fn]
extern "ExtismHost" {
    fn a_python_func(input: String) -> String;
}

// Truncated version of the currency object returned by the currency api
struct ApiCurrency {
    symbol: String,
    name: String,
    symbol_native: String,
    code: String,
    name_plural: String,
    r#type: String,
}

struct ApiCurrencies {
    data: BTreeMap<String, ApiCurrency>,
}

fn get_url() -> Result<String, Error> {
    Ok(config::get("url")?.unwrap_or("https://api.freecurrencyapi.com/v1".to_string()))
}

fn get_api_key() -> Result<String, Error> {
    config::get("api_key")?.ok_or(Error::msg("Api key not found in configuration."))
}

#[plugin_fn]
pub fn test() -> FnResult<&'static str> {
    let url = get_url()?;
    let api_key = get_api_key()?;

    let status_url = format!("{url}/status?apikey={api_key}");
    match http::request::<()>(
        &HttpRequest {
            url: status_url,
            headers: BTreeMap::new(),
            method: None,
        },
        None,
    )?
    .status_code()
    {
        200..=299 => Ok("Plugin validation succeeded."),
        0..=199 | 300.. => Err(Error::msg("Api key not found in configuration.").into()),
    }
}
