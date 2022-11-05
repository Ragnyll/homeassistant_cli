use clap::Parser;
use hcli::config::cli::{Commands, Subcommands, ServiceType};
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
    let args = Commands::parse();

    let homeassistant = get_homeassistant(conf.api_base_url.clone())?;

    match &args.subcommand {
        Subcommands::Service(s) => {
            if s.list {
                conf.get_all_service_types()
                    .iter()
                    .for_each(|x| println!("{x}"));
                return Ok(());
            } else if let Some(st) = &s.service_type {
                match st {
                    ServiceType::Switch(sclir) => {
                        let entity_id =
                            conf.get_entity_id_by_device_alias("switch", &sclir.alias)?;
                        homeassistant.toggle_switch(&entity_id).await?;
                    }
                }
            } else {
                // TODO: try to just call help method explictly from clap for this subcommand... im not sure how to do
                // that atm
                return Err(anyhow!(
                    "Bad User! Run `hcli service --help` and try again after that!"
                ));
            }
        }
        #[allow(unreachable_patterns)]
        _ => {
            return Err(anyhow!("Unknown Subcommand type!"));
        }
    }

    Ok(())
}
