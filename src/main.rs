use clap::Parser;
use hcli::config::cli::{Args, ServiceTypes};
use hcli::config::file::HomeAssistantConfigurations;
use hcli::homeassistant_api::HomeAssistant;
use anyhow::{anyhow, Result};

mod pass;

const APP_NAME: &str = "hcli";

pub fn get_homeassistant(base_url: String) -> Result<HomeAssistant> {
    Ok(HomeAssistant::new(pass::get_bearer_token()?, base_url)?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let conf: HomeAssistantConfigurations = confy::load(APP_NAME, None)?;
    let args = Args::parse();

    let homeassistant = get_homeassistant(conf.api_base_url.clone())?;

    match &args.service {
        ServiceTypes::Switch { alias: a } => {
            let entity_id = conf.get_entity_id_by_device_alias("switch", a)?;
            homeassistant.toggle_switch(&entity_id).await?;
        }
        #[allow(unreachable_patterns)]
        _ => {
            return Err(anyhow!("Unknown Service type!"));
        }
    }

    Ok(())
}
