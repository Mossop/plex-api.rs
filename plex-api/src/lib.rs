extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_with;

use std::env;
use std::result;
use std::sync::RwLock;

use reqwest::header::HeaderMap;
use serde::de::{self, Deserialize, Deserializer, Unexpected};
use uname::uname;
use uuid::Uuid;

pub use self::my_plex::*;

mod media_container;
mod my_plex;

#[cfg(test)]
mod tests {
    use crate::{
        base_headers, X_PLEX_CLIENT_IDENTIFIER, X_PLEX_DEVICE, X_PLEX_DEVICE_NAME, X_PLEX_PLATFORM,
        X_PLEX_PLATFORM_VERSION, X_PLEX_PRODUCT, X_PLEX_PROVIDES, X_PLEX_SYNC_VERSION,
        X_PLEX_VERSION,
    };

    #[test]
    fn base_headers_contains_required_headers() {
        let headers = base_headers();
        assert_eq!(true, headers.contains_key("x-plex-provides"));
        assert_eq!(true, headers.contains_key("x-plex-product"));
        assert_eq!(true, headers.contains_key("x-plex-version"));
        assert_eq!(true, headers.contains_key("x-plex-sync-version"));
        assert_eq!(true, headers.contains_key("x-plex-platform"));
        assert_eq!(true, headers.contains_key("x-plex-platform-version"));
        assert_eq!(true, headers.contains_key("x-plex-client-identifier"));
        assert_eq!(true, headers.contains_key("x-plex-device"));
        assert_eq!(true, headers.contains_key("x-plex-device-name"));
    }

    #[test]
    fn base_headers_use_provided_values() {
        {
            let mut provides = X_PLEX_PROVIDES.write().unwrap();
            *provides = "plex_provides";
            let mut platform = X_PLEX_PLATFORM.write().unwrap();
            *platform = "plex_platform";
            let mut platform_version = X_PLEX_PLATFORM_VERSION.write().unwrap();
            *platform_version = "plex_platform_version";
            let mut product = X_PLEX_PRODUCT.write().unwrap();
            *product = "plex_product";
            let mut version = X_PLEX_VERSION.write().unwrap();
            *version = "plex_version";
            let mut device = X_PLEX_DEVICE.write().unwrap();
            *device = "plex_device";
            let mut device_name = X_PLEX_DEVICE_NAME.write().unwrap();
            *device_name = "plex_device_name";
            let mut client_identifier = X_PLEX_CLIENT_IDENTIFIER.write().unwrap();
            *client_identifier = "plex_client_identifier";
        }

        let headers = base_headers();

        let provides = X_PLEX_PROVIDES.read().unwrap();
        let platform = X_PLEX_PLATFORM.read().unwrap();
        let platform_version = X_PLEX_PLATFORM_VERSION.read().unwrap();
        let product = X_PLEX_PRODUCT.read().unwrap();
        let version = X_PLEX_VERSION.read().unwrap();
        let device = X_PLEX_DEVICE.read().unwrap();
        let device_name = X_PLEX_DEVICE_NAME.read().unwrap();
        let client_identifier = X_PLEX_CLIENT_IDENTIFIER.read().unwrap();

        assert_eq!(*provides, headers.get("x-plex-provides").unwrap());
        assert_eq!(*product, headers.get("x-plex-product").unwrap());
        assert_eq!(*version, headers.get("x-plex-version").unwrap());
        assert_eq!(
            X_PLEX_SYNC_VERSION,
            headers.get("x-plex-sync-version").unwrap()
        );
        assert_eq!(*platform, headers.get("x-plex-platform").unwrap());
        assert_eq!(
            *platform_version,
            headers.get("x-plex-platform-version").unwrap()
        );
        assert_eq!(
            *client_identifier,
            headers.get("x-plex-client-identifier").unwrap()
        );
        assert_eq!(*device, headers.get("x-plex-device").unwrap());
        assert_eq!(*device_name, headers.get("x-plex-device-name").unwrap());
    }
}

const PROJECT: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const X_PLEX_SYNC_VERSION: &str = "2";

lazy_static! {
    pub static ref X_PLEX_PROVIDES: RwLock<&'static str> = RwLock::new("controller");
    pub static ref X_PLEX_PLATFORM: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_PLATFORM_VERSION: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_PRODUCT: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_VERSION: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_DEVICE: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_DEVICE_NAME: RwLock<&'static str> = RwLock::new("");
    pub static ref X_PLEX_CLIENT_IDENTIFIER: RwLock<&'static str> = RwLock::new("");
}

fn base_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let i = uname().unwrap();

    let provides = *X_PLEX_PROVIDES.read().unwrap();
    headers.insert("X-Plex-Provides", provides.parse().unwrap());

    let mut product = *X_PLEX_PRODUCT.read().unwrap();
    if product == "" {
        product = PROJECT.unwrap_or("plex-api");
    }

    headers.insert("X-Plex-Product", product.parse().unwrap());

    let mut version = *X_PLEX_VERSION.read().unwrap();
    if version == "" {
        version = VERSION.unwrap_or("unknown");
    }

    headers.insert("X-Plex-Version", version.parse().unwrap());
    headers.insert("X-Plex-Sync-Version", X_PLEX_SYNC_VERSION.parse().unwrap());

    let mut platform = *X_PLEX_PLATFORM.read().unwrap();
    if platform == "" {
        platform = &i.sysname;
    }

    headers.insert("X-Plex-Platform", platform.parse().unwrap());

    let mut platform_version = *X_PLEX_PLATFORM_VERSION.read().unwrap();
    if platform_version == "" {
        platform_version = &i.release;
    }

    headers.insert("X-Plex-Platform-Version", platform_version.parse().unwrap());

    let mut client_identifier: String = String::from(*X_PLEX_CLIENT_IDENTIFIER.read().unwrap());
    if client_identifier == "" {
        let client_id = env::var("X_PLEX_CLIENT_IDENTIFIER");
        if client_id.is_ok() {
            client_identifier = client_id.unwrap().clone();
        } else {
            warn!(target: "plex-api", "Generating random identifier for the machine! Set X_PLEX_CLIENT_IDENTIFIER to avoid this");
            let random_uuid = Uuid::new_v4();
            client_identifier = random_uuid.to_string().clone();
        }
    }

    headers.insert(
        "X-Plex-Client-Identifier",
        client_identifier.parse().unwrap(),
    );

    let mut device = *X_PLEX_DEVICE.read().unwrap();
    if device == "" {
        device = platform
    }

    headers.insert("X-Plex-Device", device.parse().unwrap());

    let mut device_name = *X_PLEX_DEVICE_NAME.read().unwrap();
    if device_name == "" {
        device_name = &i.nodename;
    }

    headers.insert("X-Plex-Device-Name", device_name.parse().unwrap());

    headers
}

fn bool_from_int<'de, D>(deserializer: D) -> result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

fn option_bool_from_int<'de, D>(deserializer: D) -> result::Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match bool_from_int(deserializer) {
        Ok(v) => Ok(Option::from(v)),
        Err(e) => Err(e),
    }
}
