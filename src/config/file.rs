use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::collections::HashSet;
use thiserror::Error;

/// The configurations for calling commands using the homeassistant cli.
/// The default configuration is located at $HOME/.config/hcli/default-config.yml
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HomeAssistantConfigurations {
    /// The base url with the protocol and port name.
    /// ex: http://192.168.1.132:8123
    pub api_base_url: String,
    /// The various services and device aliases associated to them.
    /// ex
    /// ```yaml
    /// services:
    ///     switch:
    ///         kitchen_lamp:
    ///             entity_id: kitchen.lamp
    ///         bedroom_lamp:
    ///             entity_id: bedroom.lamp
    /// ```
    /// So that is to say the Service type is the Key
    services: HashMap<String, HashMap<String, Device>>,
}

/// A device stored in homeassistant
/// I know this struct is kinda dumb, I just want this to be easy to work with when i expand this
/// program
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Device {
    /// The entity_id defined by homeassistant
    entity_id: String,
}

impl HomeAssistantConfigurations {
    pub fn get_all_service_types(&self) -> HashSet<String> {
        self.services.clone().into_keys().collect()
    }

    /// Gets all the devices of a specific service type, eg all switches
    pub fn get_all_device_aliases_of_services_type(&self, service_type: &str) -> HashSet<String> {
        self.services
            .get(service_type)
            .unwrap_or(&HashMap::new())
            .clone()
            .into_keys()
            .collect()
    }

    /// Gets a device's entity id from its given alias in the conf
    pub fn get_entity_id_by_device_alias(
        &self,
        service_type: &str,
        alias: &str,
    ) -> Result<String, HCLIConfigError> {
        if let Some(service) = self.services.get(service_type) {
            if let Some(device) = service.get(alias) {
                return Ok(device.entity_id.clone());
            }
        };
        return Err(HCLIConfigError::NoSuchDevice {
            service_type: alias.to_string(),
            alias: alias.to_string(),
        });
    }
}

#[derive(Error, Debug)]
pub enum HCLIConfigError {
    #[error("No alias {alias} with the given alias for service_type: {service_type}")]
    NoSuchDevice { service_type: String, alias: String },
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_ex() -> HomeAssistantConfigurations {
        let d = Device {
            entity_id: String::from("kitchen.lamp"),
        };
        let mut m = HashMap::new();
        m.insert(String::from("kitchen_lamp"), d);
        let mut ds = HashMap::new();
        ds.insert(String::from("switch"), m);

        HomeAssistantConfigurations {
            api_base_url: String::from("192.168.1.1:8000"),
            services: ds,
        }
    }

    #[test]
    fn test_deserialize() {
        let to_deserialize = r#"
        api_base_url: "192.168.1.1:8000"
        services:
          switch:
            kitchen_lamp:
              entity_id: kitchen.lamp
        "#;

        let conf: HomeAssistantConfigurations = serde_yaml::from_str(to_deserialize).unwrap();
        assert_eq!(make_ex(), conf);
    }

    #[test]
    fn test_get_all_service_types() {
        let conf = make_ex();
        assert_eq!(conf.get_all_service_types().len(), 1);
        assert!(conf.get_all_service_types().contains("switch"))
    }

    #[test]
    fn test_get_all_device_aliases_of_services_type() {
        let conf = make_ex();
        assert_eq!(
            conf.get_all_device_aliases_of_services_type("switch").len(),
            1
        );
        assert!(conf
            .get_all_device_aliases_of_services_type("switch")
            .contains("kitchen_lamp"))
    }

    #[test]
    fn test_get_device_alias_entity_id() {
        let conf = make_ex();
        assert_eq!(
            conf.get_entity_id_by_device_alias("switch", "kitchen_lamp")
                .unwrap(),
            "kitchen.lamp"
        )
    }
}
