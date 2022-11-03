use std::process::Command;
use anyhow::Result;

fn hcli_pass_name() -> Result<String> {
    // find the hostname of the computer
    let out = Command::new("hostname").output()?;
    let hostname = std::str::from_utf8(&out.stdout)?
        .to_string()
        .trim()
        .to_string();

    Ok(format!("homeassistant/{hostname}_token"))
}

// Gets the homeassistant cli bearer token from the password store with the name
// "homeassistant/{hostname}_token"
pub fn get_bearer_token() -> Result<String> {
    let out = Command::new("pass").arg(hcli_pass_name()?).output()?;
    Ok(std::str::from_utf8(&out.stdout)?.trim().to_string())
}
