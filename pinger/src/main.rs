use config::Config;
use healthcheck::run_healthcheck;
use healthcheck_common::stats_file::{log_entry, EmailResult, PingResult};
use rand::{distributions::Alphanumeric, Rng};

#[cfg(feature = "gmail")]
use gmail::GmailHandler;

mod config;
mod email_service;
mod healthcheck;

#[cfg(feature = "gmail")]
mod gmail;

#[cfg(not(feature = "gmail"))]
compile_error!("exactly one email feature must be enabled (try enabling feature \"gmail\")");

fn main() {
    let config = Config::new().expect("Failed to load config!");

    let emailer = GmailHandler::new(&config).expect("Failed to create gmail client");

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
            let email_result = match email_service::email_err(ping_error, &config, &emailer) {
                Ok(()) => EmailResult::SentSuccessfully,
                Err(e) => EmailResult::FailedToSend(e),
            };

            (ping_result, email_result)
        }
    };

    log_entry(
        config.stats_file,
        state,
        ping_url,
        ping_result,
        email_result,
    )
}
