use gcp_auth::AuthenticationManager;
use std::env;

pub struct Client {
    token: String,
    client: reqwest::Client,
    pub uri: String,
}

impl Client {
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

    pub async fn get(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.uri.clone() + path + ".json";

        self.client
            .get(url)
            .bearer_auth(self.token.as_str())
            .send()
            .await
    }

    pub async fn patch(&self, path: &str, body: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.uri.clone() + path + ".json";

        self.client
            .put(url)
            .bearer_auth(self.token.as_str())
            .body(body.to_string())
            .send()
            .await
    }

    pub async fn delete(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = self.uri.clone() + path + ".json";

        self.client
            .delete(url)
            .bearer_auth(self.token.as_str())
            .send()
            .await
    }
}
