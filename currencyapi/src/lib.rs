use extism_pdk::*;
use std::collections::BTreeMap;

mod config;
mod host;

pub use config::*;

// Taken from entity::currency at endovelicus
struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: Option<String>,
    pub rate: f64,
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

/// Returns message
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
