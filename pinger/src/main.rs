use config::Config;
use healthcheck::run_healthcheck;
use rand::{distributions::Alphanumeric, Rng};
use rust_gmail::GmailClient;
use stats_file::log_entry;

use crate::{
    email::email_err,
    stats_file::{EmailResult, PingResult},
};

mod config;
mod email;
mod healthcheck;
mod stats_file;

fn main() {
    let config = Config::new().expect("Failed to load config!");
    let email_client = GmailClient::builder(
        config.service_account_file_path.clone(),
        config.send_from_email.clone(),
    )
    .expect("Failed to create gmail client")
    .mock_mode()
    .build_blocking()
    .expect("Failed to build gmail client");

    let state: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let ping_url = format!("{}/api/health?state={state}", config.check_base_url);

    let (ping_result, email_result) = match run_healthcheck(state.clone(), ping_url.clone()) {
        Ok(()) => {
            println!("Healthcheck went ok");
            (PingResult::Success, EmailResult::NotSent)
        }
        Err(ping_error) => {
            eprintln!("Healthcheck failed, err {ping_error}");

            let ping_result = PingResult::Failure(ping_error.clone());
            let email_result =
                match email_err(email_client, ping_error, config.send_to_email.clone()) {
                    Ok(()) => EmailResult::SentSuccessfully,
                    Err(e) => EmailResult::FailedToSend(e),
                };

            (ping_result, email_result)
        }
    };

    log_entry(&config, state, ping_url, ping_result, email_result)
}
