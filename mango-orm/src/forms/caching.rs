//! # Caching.
//! Caching information about Forms for speed up work.
//!
//! Trait:
//! `Caching` - Methods caching information about Forms for speed up work.
//!
//! Methods:
//! `form_wig` - Get an widgets map for page template.
//! `form_json` - Get Form attributes in Json format for page templates.
//! `form_html` - Get Html Form of Model for page templates.
//! `get_cache_data` - Get cached Form data.
//!

use crate::{
    forms::{html_controls::HtmlControls, ToForm, Widget},
    store::{FormCache, FORM_CACHE},
};

// Caching information about Forms for speed up work.
// *************************************************************************************************
pub trait CachingForm: ToForm + HtmlControls {
    // Add map of widgets to cache.
    // ---------------------------------------------------------------------------------------------
    fn widgets_to_cache() -> Result<(), Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get write access in cache.
        let mut form_store = FORM_CACHE.write()?;
        // Create `FormCache` default and add map of widgets.
        let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
        let new_form_cache = FormCache {
            map_widgets,
            ..Default::default()
        };
        // Save structure `FormCache` to store.
        form_store.insert(key, new_form_cache);
        //
        Ok(())
    }

    // Get an widgets map for page template.
    // ---------------------------------------------------------------------------------------------
    fn form_wig() -> Result<std::collections::HashMap<String, Widget>, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get read access from cache.
        let mut form_store = FORM_CACHE.read()?;
        // Check if there is metadata for the Form in the cache.
        if !form_store.contains_key(key.as_str()) {
            // Unlock.
            drop(form_store);
            // Add map of widgets to cache.
            Self::widgets_to_cache()?;
            // Reaccess.
            form_store = FORM_CACHE.read()?;
        }
        // Get data and return the result.
        if let Some(form_cache) = form_store.get(&key[..]) {
            Ok(form_cache.map_widgets.clone())
        } else {
            Err(format!(
                "Form: `{}` -> Method: `form_wig()` : Failed to get data from cache.",
                Self::form_name()
            ))?
        }
    }

    // Get Form attributes in Json format for page templates.
    // ---------------------------------------------------------------------------------------------
    fn form_json() -> Result<String, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get read access from cache.
        let mut form_store = FORM_CACHE.read()?;
        // Check if there is metadata for the Form in the cache.
        if !form_store.contains_key(key.as_str()) {
            // Unlock.
            drop(form_store);
            // Add map of widgets to cache.
            Self::widgets_to_cache()?;
            // Reaccess.
            form_store = FORM_CACHE.read()?;
        }
        // Get data and return the result.
        if let Some(form_cache) = form_store.get(&key[..]) {
            Ok(serde_json::to_string(&form_cache.map_widgets.clone())?)
        } else {
            Err(format!(
                "Form: `{}` -> Method: `form_json()` : Failed to get data from cache.",
                Self::form_name()
            ))?
        }
    }

    // Get Html Form of Model for page templates.
    // ---------------------------------------------------------------------------------------------
    fn form_html() -> Result<String, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get read access from cache.
        let mut form_store = FORM_CACHE.read()?;
        // Check if there is metadata for the Form in the cache.
        if !form_store.contains_key(key.as_str()) {
            // Unlock.
            drop(form_store);
            // Add map of widgets to cache.
            Self::widgets_to_cache()?;
            // Reaccess.
            form_store = FORM_CACHE.read()?;
        }
        // Get data and return the result.
        if let Some(form_cache) = form_store.get(&key[..]) {
            Ok(Self::to_html(
                &Self::fields_name()?,
                form_cache.map_widgets.clone(),
            ))
        } else {
            Err(format!(
                "Form: `{}` -> Method: `form_html()` : Failed to get data from cache.",
                Self::form_name()
            ))?
        }
    }

    // Get cached Form data.
    // ---------------------------------------------------------------------------------------------
    fn get_cache_data() -> Result<FormCache, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get read access from cache.
        let mut form_store = FORM_CACHE.read()?;
        // Check if there is metadata for the Form in the cache.
        if !form_store.contains_key(key.as_str()) {
            // Unlock.
            drop(form_store);
            // Add map of widgets to cache.
            Self::widgets_to_cache()?;
            // Reaccess.
            form_store = FORM_CACHE.read()?;
        }
        // Get data and return the result.
        if let Some(form_cache) = form_store.get(&key[..]) {
            Ok(form_cache.clone())
        } else {
            Err(format!(
                "Form: `{}` -> Method: `get_cache_data()` : Failed to get data from cache.",
                Self::form_name()
            ))?
        }
    }
}
