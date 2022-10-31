use std::collections::HashMap;
use std::error::Error;
use std::process::Command;

const DEFAULT_HOME_ASSISTANT_IP: &str = "192.168.1.132";
const DEFAULT_HOME_ASSISTANT_PORT: &str = "8123";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bearer_token = get_password(&hcli_pass_name()?)?;
    let client = get_rest_client()?;

    toggle_switch(client, &bearer_token, "switch.kitchen_sun_lamp").await?;

    Ok(())
}

fn get_rest_client() -> Result<reqwest::Client, Box<dyn Error>> {
    Ok(reqwest::Client::builder().build()?)
}

/// Generates the homeassistant pass cli
fn hcli_pass_name() -> Result<String, Box<dyn Error>> {
    // find the hostname of the computer
    let out = Command::new("hostname").output()?;
    let hostname = std::str::from_utf8(&out.stdout)?
        .to_string()
        .trim()
        .to_string();

    Ok(format!("homeassistant/{hostname}_token"))
}

fn get_password(pass_name: &str) -> Result<String, Box<dyn Error>> {
    let out = Command::new("pass").arg(pass_name).output()?;
    Ok(std::str::from_utf8(&out.stdout)?.trim().to_string())
}

async fn toggle_switch(
    client: reqwest::Client,
    bearer_token: &str,
    entity_id: &str,
) -> Result<(), Box<dyn Error>> {
    let mut body = HashMap::new();
    body.insert("entity_id", entity_id);

    let res = client
        .post(format!(
            "http://{DEFAULT_HOME_ASSISTANT_IP}:{DEFAULT_HOME_ASSISTANT_PORT}/api/services/switch/toggle"
        ))
        .bearer_auth(bearer_token)
        .json(&body)
        .send()
        .await?;

    println!("res: {res:?}");

    Ok(())
}
