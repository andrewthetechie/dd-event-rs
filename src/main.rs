#![doc = include_str!("../README.md")]

use self::args::Args;
use anyhow::Result;
use clap::Parser;
use log::{debug, LevelFilter};
use reqwest::Client;
use std::env;

mod args;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .init();
    }

    debug!("{:?}", args);

    // get api key and api host from the env
    let api_key = match env::var("DD_API_KEY") {
        Ok(val) => val,
        Err(_e) => {
            eprintln!("Error: DD_API_KEY not set in environment");
            std::process::exit(78);
        }
    };
    let api_host = match env::var("DD_API_HOST") {
        Ok(val) => val,
        Err(_e) => "https://api.datadoghq.com/api/v1".to_string(),
    };

    let client = Client::default();
    let event_json = serde_json::to_string(&args)?;
    let response = client
        .post(&format!("{}/events", api_host))
        .header("Content-Type", "application/json")
        .header("DD-API-KEY", api_key)
        .body(event_json)
        .send()
        .await?;
    debug!("{:?}", response);
    if response.status().is_success() {
        println!("Successfully posted event to Datadog");
        Ok(())
    } else {
        eprintln!("Error when posting to datadog api: {:?}", response);
        std::process::exit(1);
    }
}
