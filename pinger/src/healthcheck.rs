use rand::{distributions::Alphanumeric, Rng};

use crate::config::Config;

pub fn run_healthcheck(config: &Config) -> Result<(), String> {
    let state: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let response = reqwest::blocking::get(format!(
        "{}/api/health?state={state}",
        config.check_base_url
    ))
    .map_err(|e| format!("Failed to send request, err: {e}"))?;

    let response_status = response.status();
    if !response_status.is_success() {
        return Err(format!("Got error response {response_status} from server"));
    }

    let response_state = response
        .text()
        .map_err(|e| format!("Failed to read text response from server, err {e}"))?;

    if response_state != state {
        return Err(format!(
            "Got invalid state response from server, expected {state}, got {response_state}"
        ));
    }

    Ok(())
}
