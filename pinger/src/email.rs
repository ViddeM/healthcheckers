use rust_gmail::GmailClient;

pub fn email_err(
    email_client: GmailClient,
    err: String,
    send_to_email: String,
) -> Result<(), String> {
    if let Err(e) = email_client.send_email_blocking(
        &send_to_email,
        "Healthcheck failed",
        &format!("Server healthcheck has failed with error: {err}"),
    ) {
        eprintln!("Failed to send email, err: {err}");

        return Err(err.to_string());
    }

    Ok(())
}
