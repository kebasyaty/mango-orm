//! # Global storage
//!
//! `FormCache` - Structure for caching map of widgets, json and html, for mango models.
//! `FORM_STORE` - Storage of settings for mango models.
//! `MONGODB_CLIENT_STORE` - Storage for Clients of MongoDB.
//!

use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::sync::RwLock;

// GLOBAL STORAGE
// #################################################################################################
/// Structure for caching map of widgets, json and html, for mango models
#[derive(Default, Clone, Debug)]
pub struct FormCache {
    pub meta: crate::models::Meta,
    pub map_widgets: std::collections::HashMap<String, crate::forms::Widget>,
    pub form_json: String,
    pub form_html: String,
}

// Store
lazy_static! {
    // Storage of settings for mango models
    // ---------------------------------------------------------------------------------------------
    pub static ref FORM_STORE: RwLock<std::collections::HashMap<String, FormCache>> = {
        RwLock::new(std::collections::HashMap::new())
    };
    // Caching clients MongoDB
    // ---------------------------------------------------------------------------------------------
    pub static ref MONGODB_CLIENT_STORE: RwLock<std::collections::HashMap<String, mongodb::sync::Client>> = {
        RwLock::new(std::collections::HashMap::new())
    };
    // Regular expressions
    // ---------------------------------------------------------------------------------------------
    pub static ref REGEX_IS_COLOR_CODE: Regex = RegexBuilder::new(r"^(?:#|0x)(?:[a-f0-9]{3}|[a-f0-9]{6}|[a-f0-9]{8})\b|(?:rgb|hsl)a?\([^\)]*\)$").case_insensitive(true).build().unwrap();
    pub static ref REGEX_IS_DATE: Regex = Regex::new(r"^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)$").unwrap();
    pub static ref REGEX_IS_DATETIME: Regex = Regex::new(r"^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\d|2[0-3]):[0-5]\d$").unwrap();
    pub static ref REGEX_IS_PASSWORD: Regex = Regex::new(r"^[a-zA-Z0-9@#$%^&+=*!~)(]{8,256}$").unwrap();
    pub static ref REGEX_IS_TIME: Regex = Regex::new(r"^(?:[01]\d|2[0-3]):[0-5]\d$").unwrap();
}
