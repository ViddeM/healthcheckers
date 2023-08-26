use rust_gmail::GmailClient;

use crate::{config::GmailConfig, email_service::Emailer};

pub struct GmailHandler {
    client: GmailClient,
}

impl Emailer for GmailHandler {
    fn send_email(&self, send_to: &str, subject: &str, body: &str) -> Result<(), String> {
        if let Err(e) = self.client.send_email_blocking(send_to, subject, body) {
            return Err(format!("Failed to send email via gmail, err: {e}"));
        }

        Ok(())
    }
}

impl GmailHandler {
    pub fn new(gmail_config: &GmailConfig, send_from_email: &str) -> Result<Self, String> {
        let email_client = GmailClient::builder(
            gmail_config.service_account_file_path.clone(),
            send_from_email,
        )
        .or_else(|e| Err(format!("Failed to create gmail client, err: {e}")))?
        .mock_mode()
        .build_blocking()
        .or_else(|e| Err(format!("Failed to build gmail client, err: {e}")))?;

        Ok(Self {
            client: email_client,
        })
    }
}
