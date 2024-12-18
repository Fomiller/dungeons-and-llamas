use anyhow::{self, Context};
use reqwest::{Client as HttpClient, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub url: String,
    client: HttpClient,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(url: &str) -> Self {
        ApiClient {
            url: url.to_string(),
            client: HttpClient::new(),
        }
    }

    /// Perform a GET request
    pub async fn get<T>(&self, endpoint: &str) -> anyhow::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/{}", self.url, endpoint);
        let response = self.client.get(&url).send().await?;
        response
            .json()
            .await
            .context("Failed to deserialize response for GET request")
    }

    /// Perform a POST request
    pub async fn post<T, B>(&self, endpoint: &str, body: &B) -> anyhow::Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        let url = format!("{}/{}", self.url, endpoint);
        let response = self.client.post(&url).json(body).send().await?;
        response
            .json()
            .await
            .context("Failed to deserialize response for POST request")
    }

    /// Perform a DELETE request
    pub async fn delete(&self, endpoint: &str) -> anyhow::Result<Response> {
        let url = format!("{}/{}", self.url, endpoint);
        self.client
            .delete(&url)
            .send()
            .await
            .context("Failed to send DELETE request")
    }
}

