use extism_pdk::*;

// Host functions from endovelicus
#[host_fn]
extern "ExtismHost" {
    // fn a_func(input: String) -> String;
    // ...
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum HtmlType {
    Checkbox,
    Color,
    Date,
    #[serde(rename(serialize = "datetime-local"))]
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Tel,
    Text,
    Time,
    Url,
    Week,
}

// Configuration
#[derive(serde::Serialize)]
pub struct ConfigVar {
   pub name: &'static str,
   pub html_type: HtmlType,
   pub placeholder: &'static str,
   pub required: bool
}

