use crate::config::Config;

pub trait Emailer {
    fn send_email(&self, send_to: &str, subject: &str, body: &str) -> Result<(), String>;
}

pub fn email_err<T: Emailer>(err: String, config: &Config, emailer: &T) -> Result<(), String> {
    let subject = "Healthcheck failed";
    let body = format!("Server healthcheck has failed with error: {err}");

    if let Err(e) = emailer.send_email(&config.send_to_email, subject, &body) {
        eprintln!("Failed to send email with content: {subject} \n\n{body}\n\n Error: {e}");

        return Err(e.to_string());
    }

    Ok(())
}
