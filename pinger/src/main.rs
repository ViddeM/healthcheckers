use config::Config;
use healthcheck::run_healthcheck;
use rust_gmail::GmailClient;

mod config;
mod healthcheck;

fn main() {
    let config = Config::new().expect("Failed to load config!");
    let email_client = GmailClient::builder(
        config.service_account_file_path.clone(),
        config.send_from_email.clone(),
    )
    .expect("Failed to create gmail client")
    .build_blocking()
    .expect("Failed to build gmail client");

    match run_healthcheck(&config) {
        Ok(()) => println!("Healthcheck went ok"),
        Err(e) => {
            eprintln!("Healthcheck failed, err {e}");

            email_err(email_client, e, config.send_to_email);
        }
    }
}

fn email_err(email_client: GmailClient, err: String, send_to_email: String) {
    email_client
        .send_email_blocking(
            &send_to_email,
            "Healthcheck failed",
            &format!("Server healthcheck has failed with error: {err}"),
        )
        .expect("Failed to send error email!");
}
