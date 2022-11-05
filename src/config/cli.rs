use clap::{Args, Parser, Subcommand};

/// The CLI for interacting with homeassistant REST APIs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Commands {
    /// The command with which to interact
    #[command(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Interact with a HomeAssistant service
    Service(Service),
}

#[derive(Args, Debug)]
pub struct Service {
    /// List all enabled by the hcli config
    #[arg(long)]
    pub list: bool,

    // TODO: this does not handle exclusivity with list well. fix it
    /// The type of servie to interact with
    #[command(subcommand)]
    pub service_type: Option<ServiceType>,
}

/// The types of services allowed by homeassistant cli
#[derive(Debug, Subcommand)]
pub enum ServiceType {
    /// Interact with a switch in HomeAssistant
    Switch(ServiceCliRequirements),
}

/// The arguments needed to call a service from the command line
#[derive(Args, Debug)]
pub struct ServiceCliRequirements {
    /// The devices "alias" internal to the hcli conf
    /// ie in: `services.switch.kitchen_lamp.entity_id: kitchen.lamp`
    /// kitchen_lamp is the alias
    pub alias: String,
}
