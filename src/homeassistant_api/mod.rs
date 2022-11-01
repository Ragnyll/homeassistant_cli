use crate::rest_common::RestClient;
use std::collections::HashMap;
use thiserror::Error;

mod homeassistant_responses;
use homeassistant_responses::HomeAssistantResponse;

/// The primary wrapper around commands to interact with homeassistant on
pub struct HomeAssistant {
    /// The client to use for interacting with the homeassistant rest api
    client: RestClient,
}

impl HomeAssistant {
    /// Creates a new HomeAssistant client with the specified token and base api base_url
    /// the base url will have the format {protocol}://{hostname[:port]}
    pub fn new(bearer_token: String, base_url: String) -> Result<Self, HomeAssistantClientError> {
        Ok(Self {
            client: RestClient::new(bearer_token, format!("{}/api", base_url))?,
        })
    }

    /// Toggles a switch entity registered in homeassistant
    pub async fn toggle_switch(
        &self,
        entity_id: &str,
    ) -> Result<HomeAssistantResponse, HomeAssistantClientError> {
        let mut body = HashMap::new();
        body.insert("entity_id", entity_id);
        let _res = self.client.post("services/switch/toggle", &body).await?;
        Ok(HomeAssistantResponse::Bokay)
    }
}

#[derive(Error, Debug)]
pub enum HomeAssistantClientError {
    #[error("Error in the RestClient")]
    RestClientError(#[from] reqwest::Error),
}
