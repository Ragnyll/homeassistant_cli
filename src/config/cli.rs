use clap::{Parser, Subcommand};

/// The CLI for interacting with homeassistant REST APIs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The service with which to interact
    #[command(subcommand)]
    pub service: ServiceTypes,
}

#[derive(Subcommand, Debug)]
pub enum ServiceTypes {
    Switch {
        /// The alias for the entity defined in {config}/config.yaml
        alias: String,
    },
}
