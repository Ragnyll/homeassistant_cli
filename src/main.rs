use std::error::Error;
use hcli::homeassistant_api::HomeAssistant;

const DEFAULT_HOME_ASSISTANT_IP: &str = "192.168.1.132";
const DEFAULT_HOME_ASSISTANT_PORT: &str = "8123";

mod pass;

pub fn get_homeassistant() -> Result<HomeAssistant, Box<dyn Error>> {
    Ok(HomeAssistant::new(
        pass::get_bearer_token()?,
        format!("http://{DEFAULT_HOME_ASSISTANT_IP}:{DEFAULT_HOME_ASSISTANT_PORT}"),
    )?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let homeassistant = get_homeassistant()?;
    homeassistant.toggle_switch("switch.kitchen_sun_lamp").await?;

    Ok(())
}
