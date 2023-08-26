use config::{Config, EmailConfig};
use gmail::GmailHandler;
use healthcheck::run_healthcheck;
use healthcheck_common::stats_file::{log_entry, EmailResult, PingResult};
use rand::{distributions::Alphanumeric, Rng};

mod config;
mod email_service;
mod gmail;
mod healthcheck;

fn main() {
    let config = Config::new().expect("Failed to load config!");

    let emailer = match &config.email_config {
        EmailConfig::Gmail(conf) => {
            GmailHandler::new(conf, &config.send_from_email).expect("Failed to create gmail client")
        }
    };

    let state: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let ping_url = format!("{}/api/health?state={state}", config.check_base_url);

    let (ping_result, ping_error, email_result, email_error) =
        match run_healthcheck(state.clone(), ping_url.clone()) {
            Ok(()) => {
                println!("Healthcheck went ok");
                (PingResult::Success, None, EmailResult::NotSent, None)
            }
            Err(ping_error) => {
                eprintln!("Healthcheck failed, err {ping_error}");

                let ping_result = PingResult::Failure;
                let (email_result, email_err) =
                    match email_service::email_err(ping_error.clone(), &config, &emailer) {
                        Ok(()) => (EmailResult::SentSuccessfully, None),
                        Err(e) => (EmailResult::FailedToSend, Some(e)),
                    };

                (ping_result, Some(ping_error), email_result, email_err)
            }
        };

    log_entry(
        &config.stats_file,
        state,
        ping_url,
        ping_result,
        ping_error,
        email_result,
        email_error,
    )
}
