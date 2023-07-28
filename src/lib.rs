use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use url::Url;

pub mod build;
pub mod config;
pub mod model;

use build::builder::Builder;
use config::SupabaseConfig;

/// A struct representing a Storage with an associated client and headers.
pub struct Storage {
    pub url: url::Url,
    pub headers: HeaderMap,
    pub client: Client,
}

impl Storage {
    /// Creates a new `Storage` instance with the provided URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL for the storage.
    ///
    /// # Example
    ///
    /// ```
    /// use storage_rs::Storage;
    ///
    /// let _ = Storage::new("https://your_project_path/storage/v1");
    /// ```
    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            url: Url::parse(&url.into()).unwrap(),
            headers: HeaderMap::new(),
            client: Client::new(),
        }
    }

    /// Creates a new `Storage` instance with provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The `SupabaseConfig` containing the necessary configuration for Supabase.
    ///
    /// # Example
    /// ```
    /// use storage_rs::{Storage, config::SupabaseConfig};
    /// use dotenv::dotenv;
    ///
    /// dotenv().ok();
    /// let config = SupabaseConfig::default();  // load values from .env file using the 'envy' crate
    /// let storage = Storage::new_with_config(config);
    /// ```
    pub fn new_with_config(config: SupabaseConfig) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", config.supabase_api_key))
                .expect("header value is invalid"),
        );

        Self {
            url: Url::parse(&config.supabase_url_storage).unwrap(),
            headers,
            client: Client::new(),
        }
    }

    /// Creates a new `Builder` instance to build an action
    ///
    /// # Example
    /// ```
    /// use storage_rs::Storage;
    ///
    /// let storage = Storage::new("https://your_project_path/storage/v1");
    /// let builder = storage.from();
    /// ```
    pub fn from(&self) -> Builder {
        Builder::new(self.url.clone(), self.headers.clone(), self.client.clone())
    }
}
