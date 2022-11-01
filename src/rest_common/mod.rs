use reqwest::Client;

/// A wrapper around the reqwest client because for some reason the bearer_token default header
/// doesn't fucking operate in the realm of my patience
/// Has a few basic common methods for my usage
#[derive(Debug)]
pub struct RestClient {
    bearer_token: String,
    /// A base url to call all endpoints with the protocol speicified
    base_url: String,
    client: Client,
}

impl RestClient {
    /// Creates a new client that will be call the given url with the default in Self
    pub fn new(bearer_token: String, base_url: String) -> Result<Self, reqwest::Error> {
        Ok(Self {
            bearer_token,
            base_url,
            client: reqwest::Client::builder().build()?,
        })
    }

    /// a get function that assumes no body is required for the request
    /// it is up to the caller to deserialize the response to the desired type
    pub async fn get(&self, endpoint: &str) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self
            .client
            .get(format!("{}/{}", self.base_url, endpoint))
            .bearer_auth(&self.bearer_token)
            .send()
            .await?)
    }

    /// a post function
    /// it is up to the caller to deserialize the response to the desired type
    pub async fn post<T: serde::Serialize + ?Sized>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self
            .client
            .post(format!("{}/{}", self.base_url, endpoint))
            .bearer_auth(&self.bearer_token)
            .json(&body)
            .send()
            .await?)
    }
}
