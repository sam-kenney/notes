use gcp_auth::AuthenticationManager;
use std::env;

/// Client to interface with Firebase Realtime Databases.
///
/// # Parameters
///
/// * `token` - An authentication token for your Firebase Database.
/// * `client` - A HTTP client to use.
/// * `uri` - The URI of the Firebase Realtime Database.
pub struct Client {
    token: String,
    client: reqwest::Client,
    pub uri: String,
}

/// A Firebase Realtime Database client.
impl Client {
    /// Create a new Firebase Realtime Database client.
    ///
    /// Automatically infers the Firebase URI from the `FIREBASE_URI` environment variable.
    /// Creates a new authentication token using the `gcp_auth` crate, which uses the
    /// `GOOGLE_APPLICATION_CREDENTIALS` environment variable to find the service account
    /// credentials.
    ///
    /// # Panics
    ///
    /// Panics if the `FIREBASE_URI` environment variable is not set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use backend::firebase::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new().await;
    /// }
    /// ```
    pub async fn new() -> Self {
        let token = AuthenticationManager::new()
            .await
            .unwrap()
            .get_token(&[
                "https://www.googleapis.com/auth/firebase.database",
                "https://www.googleapis.com/auth/userinfo.email",
            ])
            .await
            .unwrap();

        Self {
            token: token.as_str().to_string(),
            client: reqwest::Client::new(),
            uri: env::var("FIREBASE_URI").expect("FIREBASE_URI must be set"),
        }
    }

    /// Get a value from the Firebase Realtime Database.
    ///
    /// # Parameters
    ///
    /// * `path` - The path to append to the base url to access
    ///            a value in the database.
    pub async fn get(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.uri.clone() + path + ".json";

        self.client
            .get(url)
            .bearer_auth(self.token.as_str())
            .send()
            .await
    }

    /// Post a value to the Firebase Realtime Database.
    ///
    /// # Parameters
    ///
    /// * `path` - The path to append to the base url to access
    ///            a value in the database.
    /// * `body` - The JSON string body of the request.
    pub async fn patch(&self, path: &str, body: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.uri.clone() + path + ".json";

        self.client
            .put(url)
            .bearer_auth(self.token.as_str())
            .body(body.to_string())
            .send()
            .await
    }

    /// Delete a value from the Firebase Realtime Database.
    ///
    /// # Parameters
    ///
    /// * `path` - The path to append to the base url to access
    ///            a value in the database.
    pub async fn delete(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.uri.clone() + path + ".json";

        self.client
            .delete(url)
            .bearer_auth(self.token.as_str())
            .send()
            .await
    }
}
