pub fn run_healthcheck(state: String, ping_url: String) -> Result<(), String> {
    let response = reqwest::blocking::get(ping_url)
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
