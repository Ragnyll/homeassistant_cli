use serde::{Serialize, Deserialize};
use std::collections::hash_map::HashMap;

/// The configurations for calling commands using the homeassistant cli.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HomeAssistantCliConfigurations {
    /// The base url with the protocol and port name.
    /// ex: http:192.168.1.132:8123
    api_base_url: String,
    device_aliases: HashMap<String, Device>,
}

/// A device stored in homeassistant
#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    /// The name of the devive defined by homeassistant
    /// currently not needed for anything, but im thinking it will prbably be useful for detection
    /// later
    name: Option<String>,
    /// The entity_id defined by homeassistant
    entity_id: String,
}
