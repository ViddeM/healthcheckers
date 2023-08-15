use std::{fs, io::Write, path::Path};

use chrono::{DateTime, Utc};
use serde::Serialize;

/// An entry for the statsfile.
#[derive(Serialize, Debug, Clone)]
pub struct StatsEntry {
    entry_version: u32, // Which version of stats file this is, in case the format changes in the future.
    timestamp: DateTime<Utc>,
    pinged_url: String,
    request_state: String,
    ping_result: PingResult,
    email_result: EmailResult,
}

/// The result of a healthcheck ping.
#[derive(Serialize, Debug, Clone)]
pub enum PingResult {
    /// A sucessful ping result.
    Success,
    /// A failed ping result.
    Failure(String),
}

/// The result of emailing the healthcheck result.
#[derive(Serialize, Debug, Clone)]
pub enum EmailResult {
    /// The email was sent without issues.
    SentSuccessfully,
    /// A try was made to send an email but it failed with the wrapped error.
    FailedToSend(String),
    /// No email was sent.
    NotSent,
}

/// Log an entry to the stats file at the provided `stats_file_path`.
/// Will create the file if it does not already exist.
pub fn log_entry(
    stats_file_path: String,
    state: String,
    ping_url: String,
    ping_result: PingResult,
    email_result: EmailResult,
) {
    let entry = StatsEntry {
        entry_version: 1,
        timestamp: Utc::now(),
        pinged_url: ping_url,
        request_state: state,
        ping_result,
        email_result,
    };

    let path = Path::new(&stats_file_path);
    if path.exists() {
        if !path.is_file() {
            panic!("Provided stats file path {path:?} is not a file!");
        }

        // The path exists and is a file, let's append to it.

        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(path)
            .expect("Failed to open stats file");

        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(vec![]);
        writer.serialize(entry).expect("Failed to serialize entry");
        let row = writer
            .into_inner()
            .expect("Failed to retrieve underlying csv writer");

        file.write_all(row.as_slice())
            .expect("Failed to append stats entry to file");
    } else {
        println!("Stats file does not exist, creating at {path:?}");
        let mut writer = csv::WriterBuilder::new()
            .from_path(path)
            .expect("Failed to open stats output file");

        writer.serialize(entry).expect("Failed to serialize entry");

        writer.flush().expect("Failed to flush writer");
    };
}
